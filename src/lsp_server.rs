// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::lsp_workspace::Workspace;

pub struct GreaseLanguageServer {
    client: Client,
    workspace: Arc<Mutex<Workspace>>,
}

impl GreaseLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            workspace: Arc::new(Mutex::new(Workspace::new())),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for GreaseLanguageServer {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(false),
                trigger_characters: Some(vec![".".to_string(), " ".to_string()]),
                work_done_progress_options: Default::default(),
                all_commit_characters: None,
                completion_item: None,
            }),
            definition_provider: Some(OneOf::Left(true)),
            references_provider: Some(OneOf::Left(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    work_done_progress_options: Default::default(),
                    legend: SemanticTokensLegend {
                        token_types: vec![
                            SemanticTokenType::COMMENT,
                            SemanticTokenType::KEYWORD,
                            SemanticTokenType::STRING,
                            SemanticTokenType::NUMBER,
                            SemanticTokenType::FUNCTION,
                            SemanticTokenType::VARIABLE,
                            SemanticTokenType::OPERATOR,
                        ],
                        token_modifiers: vec![],
                    },
                    range: Some(false),
                    full: Some(SemanticTokensFullOptions::Delta { delta: Some(false) }),
                })
            ),
            ..Default::default()
        };

        Ok(InitializeResult {
            capabilities,
            server_info: Some(ServerInfo {
                name: "grease-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Grease Language Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "Grease Language Server shutting down")
            .await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text;
        let language_id = params.text_document.language_id;

        let workspace = self.workspace.lock().await;
        let doc = workspace.upsert_document(uri, text, language_id);
        
        // Send diagnostics
        let diagnostics = doc.get_diagnostics();
        drop(workspace);
        
        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let version = params.text_document.version;

        // Apply changes to get the full text
        let mut full_text = String::new();
        if let Some(doc) = self.workspace.lock().await.get_document(&uri) {
            full_text = doc.text.to_string();
        }

        for change in params.content_changes {
            match change.range {
                Some(range) => {
                    // Apply incremental change
                    let rope = ropey::Rope::from_str(&full_text);
                    let start_idx = rope.line_to_char(range.start.line as usize) + range.start.character as usize;
                    let end_idx = rope.line_to_char(range.end.line as usize) + range.end.character as usize;
                    
                    let mut text = rope.to_string();
                    text.replace_range(start_idx..end_idx, &change.text);
                    full_text = text;
                }
                None => {
                    // Full text change
                    full_text = change.text;
                }
            }
        }

        let workspace = self.workspace.lock().await;
        if let Some(doc) = workspace.update_document(&uri, full_text, version) {
            let diagnostics = doc.get_diagnostics();
            drop(workspace);
            
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.workspace.lock().await.remove_document(&uri);
        
        // Clear diagnostics for closed document
        self.client
            .publish_diagnostics(uri, Vec::new(), None)
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let workspace = self.workspace.lock().await;
        let completions = workspace.get_completions(&uri, position);
        
        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        // Get the word at the cursor position
        let word = if let Some(doc) = self.workspace.lock().await.get_document(&uri) {
            get_word_at_position(&doc.text, position)
        } else {
            None
        };

        if let Some(word) = word {
            let workspace = self.workspace.lock().await;
            let locations = workspace.find_definitions(&word, &uri, position);
            
            if !locations.is_empty() {
                return Ok(Some(GotoDefinitionResponse::Array(locations)));
            }
        }

        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        // Get the word at the cursor position
        let word = if let Some(doc) = self.workspace.lock().await.get_document(&uri) {
            get_word_at_position(&doc.text, position)
        } else {
            None
        };

        if let Some(word) = word {
            let workspace = self.workspace.lock().await;
            let locations = workspace.find_references(&word, &uri, position);
            
            if !locations.is_empty() {
                return Ok(Some(locations));
            }
        }

        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let workspace = self.workspace.lock().await;
        
        if let Some(doc) = workspace.get_document(&uri) {
            if let Some(ast) = &doc.ast {
                let symbols = extract_document_symbols(ast, &uri);
                return Ok(Some(DocumentSymbolResponse::Nested(symbols)));
            }
        }

        Ok(None)
    }

    async fn symbol(&self, params: WorkspaceSymbolParams) -> Result<Option<Vec<SymbolInformation>>> {
        let workspace = self.workspace.lock().await;
        let mut symbols = Vec::new();
        
        for (uri_str, doc_symbols) in &workspace.symbols {
            if let Ok(_uri) = Url::parse(uri_str) {
                for symbol in doc_symbols {
                    if params.query.is_empty() || 
                       symbol.name.to_lowercase().contains(&params.query.to_lowercase()) {
                        symbols.push(SymbolInformation {
                            name: symbol.name.clone(),
                            kind: symbol.kind,
                            location: symbol.location.clone(),
                            container_name: symbol.container_name.clone(),
                            tags: None,
                            deprecated: None,
                        });
                    }
                }
            }
        }

        Ok(Some(symbols))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        // Get the word at the cursor position
        let word = if let Some(doc) = self.workspace.lock().await.get_document(&uri) {
            get_word_at_position(&doc.text, position)
        } else {
            None
        };

        if let Some(word) = word {
            let hover_text = get_hover_text(&word);
            if let Some(text) = hover_text {
                return Ok(Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(text)),
                    range: None,
                }));
            }
        }

        Ok(None)
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let workspace = self.workspace.lock().await;
        
        if let Some(doc) = workspace.get_document(&uri) {
            if let Some(ast) = &doc.ast {
                let tokens = extract_semantic_tokens(ast);
                return Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                    result_id: None,
                    data: tokens,
                })));
            }
        }

        Ok(None)
    }
}

fn get_word_at_position(text: &ropey::Rope, position: Position) -> Option<String> {
    let line_idx = position.line as usize;
    if line_idx >= text.len_lines() {
        return None;
    }

    let line = text.line(line_idx).to_string();
    let char_idx = position.character as usize;
    
    if char_idx >= line.len() {
        return None;
    }

    // Find word boundaries
    let start = line[..char_idx]
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);
    
    let end = line[char_idx..]
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| char_idx + i)
        .unwrap_or(line.len());

    if start < end {
        Some(line[start..end].to_string())
    } else {
        None
    }
}

fn get_hover_text(word: &str) -> Option<String> {
    match word {
        "def" => Some("def - Define a function\n\n```grease\ndef function_name(param1, param2):\n    # function body\n    return result\n```".to_string()),
        "if" => Some("if - Conditional statement\n\n```grease\nif condition:\n    # code to execute if condition is true\n```".to_string()),
        "while" => Some("while - Loop while condition is true\n\n```grease\nwhile condition:\n    # code to execute in each iteration\n```".to_string()),
        "for" => Some("for - Loop over iterable\n\n```grease\nfor item in iterable:\n    # code to execute for each item\n```".to_string()),
        "use" => Some("use - Import a module\n\n```grease\nuse math\nuse string as str\n```".to_string()),
        "return" => Some("return - Return a value from a function\n\n```grease\nreturn value\n```".to_string()),
        "and" | "or" | "not" => Some(format!("{} - Boolean operator", word)),
        "true" | "false" => Some(format!("{} - Boolean literal", word)),
        "null" => Some("null - Null value".to_string()),
        _ => None,
    }
}

fn extract_document_symbols(ast: &crate::ast::Program, _uri: &Url) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();
    
    for statement in &ast.statements {
        match statement {
            crate::ast::Statement::FunctionDeclaration { name, parameters, body, .. } => {
                if let crate::token::TokenType::Identifier(ident) = &name.token_type {
                    let mut children = Vec::new();
                    
                    // Add parameters as child symbols
                    for (param, _) in parameters {
                        if let crate::token::TokenType::Identifier(param_name) = &param.token_type {
                            children.push(DocumentSymbol {
                                name: param_name.clone(),
                                detail: Some("parameter".to_string()),
                                kind: SymbolKind::VARIABLE,
                                tags: None,
                                range: Range::default(),
                                selection_range: Range::default(),
                                children: None,
                                deprecated: None,

                            });
                        }
                    }
                    
                    // Extract symbols from function body
                    for stmt in body {
                        extract_symbols_from_statement(stmt, &mut children);
                    }
                    
                    symbols.push(DocumentSymbol {
                        name: ident.clone(),
                        detail: Some(format!("function({})", parameters.len())),
                        kind: SymbolKind::FUNCTION,
                        tags: None,
                        range: Range::default(),
                        selection_range: Range::default(),
                        children: Some(children),
                        deprecated: None,
                    });
                }
            }
            crate::ast::Statement::VariableDeclaration { name, type_annotation, .. } => {
                if let crate::token::TokenType::Identifier(ident) = &name.token_type {
                    symbols.push(DocumentSymbol {
                        name: ident.clone(),
                        detail: type_annotation.clone(),
                        kind: SymbolKind::VARIABLE,
                        tags: None,
                        range: Range::default(),
                        selection_range: Range::default(),
                        children: None,
                        deprecated: None,
                    });
                }
            }
            _ => {}
        }
    }
    
    symbols
}

fn extract_symbols_from_statement(stmt: &crate::ast::Statement, symbols: &mut Vec<DocumentSymbol>) {
    match stmt {
        crate::ast::Statement::VariableDeclaration { name, type_annotation, .. } => {
            if let crate::token::TokenType::Identifier(ident) = &name.token_type {
                symbols.push(DocumentSymbol {
                    name: ident.clone(),
                    detail: type_annotation.clone(),
                    kind: SymbolKind::VARIABLE,
                    tags: None,
                    range: Range::default(),
                    selection_range: Range::default(),
                    children: None,
                    deprecated: None,
                });
            }
        }
        crate::ast::Statement::FunctionDeclaration { name, parameters, body, .. } => {
            if let crate::token::TokenType::Identifier(ident) = &name.token_type {
                let mut children = Vec::new();
                
                    for (param, _) in parameters {
                        if let crate::token::TokenType::Identifier(param_name) = &param.token_type {
                            children.push(DocumentSymbol {
                                name: param_name.clone(),
                                detail: Some("parameter".to_string()),
                                kind: SymbolKind::VARIABLE,
                                tags: None,
                                range: Range::default(),
                                selection_range: Range::default(),
                                children: None,
                                deprecated: None,

                            });
                        }
                    }
                
                for stmt in body {
                    extract_symbols_from_statement(stmt, &mut children);
                }
                
                symbols.push(DocumentSymbol {
                    name: ident.clone(),
                    detail: Some(format!("function({})", parameters.len())),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    range: Range::default(),
                    selection_range: Range::default(),
                    children: None,
                    deprecated: None,
                });
            }
        }
        crate::ast::Statement::Block(statements) => {
            for stmt in statements {
                extract_symbols_from_statement(stmt, symbols);
            }
        }
        _ => {}
    }
}

fn extract_semantic_tokens(_ast: &crate::ast::Program) -> Vec<SemanticToken> {
    let mut tokens = Vec::new();
    
    // This is a simplified implementation
    // In a full implementation, you'd walk the AST and generate semantic tokens
    // with proper line/column information
    
    // Example: add some basic tokens
    tokens.push(SemanticToken {
        delta_line: 0,
        delta_start: 0,
        length: 3,
        token_type: 1, // KEYWORD
        token_modifiers_bitset: 0,
    });
    
    tokens
}

pub async fn run_server() -> Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    
    let (service, socket) = LspService::new(GreaseLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
    
    Ok(())
}