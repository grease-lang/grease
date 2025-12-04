// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::lsp_types::*;
use crate::ast::{Program, Statement};
use crate::lexer::Lexer;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Document {
    pub uri: Url,
    pub text: Rope,
    pub version: i32,
    pub language_id: String,
    pub ast: Option<Program>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Document {
    pub fn new(uri: Url, text: String, language_id: String) -> Self {
        let rope = Rope::from_str(&text);
        Self {
            uri,
            text: rope,
            version: 0,
            language_id,
            ast: None,
            diagnostics: Vec::new(),
        }
    }

    pub fn update(&mut self, text: String, version: i32) {
        self.text = Rope::from_str(&text);
        self.version = version;
        self.ast = None;
        self.diagnostics.clear();
    }

    pub fn parse(&mut self) -> Result<(), String> {
        let source = self.text.to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        
        match parser.parse() {
            Ok(program) => {
                self.ast = Some(program);
                self.diagnostics.clear();
                Ok(())
            }
            Err(e) => {
                let error_msg = e.clone();
                self.diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(0, 0),
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("grease-lsp".to_string()),
                    message: error_msg.clone(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
                Err(e)
            }
        }
    }

    pub fn get_diagnostics(&self) -> Vec<Diagnostic> {
        self.diagnostics.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub container_name: Option<String>,
    pub range: Option<Range>,
    pub selection_range: Option<Range>,
    pub children: Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub documents: DashMap<Url, Document>,
    pub symbols: HashMap<String, Vec<Symbol>>,
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            documents: DashMap::new(),
            symbols: HashMap::new(),
        }
    }

    pub fn get_document(&self, uri: &Url) -> Option<Document> {
        self.documents.get(uri).map(|doc| doc.clone())
    }

    pub fn upsert_document(&self, uri: Url, text: String, language_id: String) -> Document {
        let mut doc = Document::new(uri.clone(), text, language_id);
        let _ = doc.parse();
        
        if let Some(existing) = self.documents.get(&uri) {
            doc.version = existing.version + 1;
        }
        
        self.documents.insert(uri.clone(), doc.clone());
        doc
    }

    pub fn update_document(&self, uri: &Url, text: String, version: i32) -> Option<Document> {
        if let Some(mut doc) = self.documents.get_mut(uri) {
            doc.update(text, version);
            let _ = doc.parse();
            Some(doc.clone())
        } else {
            None
        }
    }

    pub fn remove_document(&self, uri: &Url) -> Option<Document> {
        self.documents.remove(uri).map(|(_, doc)| doc)
    }

    pub fn find_symbols_in_document(&mut self, uri: &Url) {
        if let Some(doc) = self.documents.get(uri) {
            let mut symbols = Vec::new();
            
            if let Some(ast) = &doc.ast {
                for statement in &ast.statements {
                    self.extract_symbols_from_statement(statement, uri, &mut symbols, None);
                }
            }
            
            self.symbols.insert(uri.to_string(), symbols);
        }
    }

    fn extract_symbols_from_statement(
        &self,
        stmt: &Statement,
        uri: &Url,
        symbols: &mut Vec<Symbol>,
        container: Option<String>,
    ) {
        match stmt {
            Statement::VariableDeclaration { name, .. } => {
                if let TokenType::Identifier(ident) = &name.token_type {
                    symbols.push(Symbol {
                        name: ident.clone(),
                        kind: SymbolKind::VARIABLE,
                        location: Location {
                            uri: uri.clone(),
                            range: self.token_to_range(name),
                        },
                        container_name: container.clone(),
                        range: Some(self.token_to_range(name)),
                        selection_range: Some(self.token_to_range(name)),
                        children: Vec::new(),
                    });
                }
            }
            Statement::FunctionDeclaration { name, parameters, body, .. } => {
                if let TokenType::Identifier(ident) = &name.token_type {
                    let func_symbol = Symbol {
                        name: ident.clone(),
                        kind: SymbolKind::FUNCTION,
                        location: Location {
                            uri: uri.clone(),
                            range: self.token_to_range(name),
                        },
                        container_name: container.clone(),
                        range: Some(self.token_to_range(name)),
                        selection_range: Some(self.token_to_range(name)),
                        children: Vec::new(),
                    };
                    symbols.push(func_symbol);
                    
                    // Extract parameter symbols
                    for (param, _) in parameters {
                        if let TokenType::Identifier(param_name) = &param.token_type {
                            symbols.push(Symbol {
                                name: param_name.clone(),
                                kind: SymbolKind::VARIABLE,
                                location: Location {
                                    uri: uri.clone(),
                                    range: self.token_to_range(param),
                                },
                                container_name: Some(ident.clone()),
                                range: Some(self.token_to_range(param)),
                                selection_range: Some(self.token_to_range(param)),
                                children: Vec::new(),
                            });
                        }
                    }
                    
                    // Extract symbols from function body
                    for stmt in body {
                        self.extract_symbols_from_statement(stmt, uri, symbols, Some(ident.clone()));
                    }
                }
            }
            Statement::Block(statements) => {
                for stmt in statements {
                    self.extract_symbols_from_statement(stmt, uri, symbols, container.clone());
                }
            }
            Statement::If { then_branch, else_branch, .. } => {
                for stmt in then_branch {
                    self.extract_symbols_from_statement(stmt, uri, symbols, container.clone());
                }
                if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        self.extract_symbols_from_statement(stmt, uri, symbols, container.clone());
                    }
                }
            }
            Statement::While { body, .. } => {
                for stmt in body {
                    self.extract_symbols_from_statement(stmt, uri, symbols, container.clone());
                }
            }
            Statement::For { body, .. } => {
                for stmt in body {
                    self.extract_symbols_from_statement(stmt, uri, symbols, container.clone());
                }
            }
            Statement::ClassDeclaration { name, methods, .. } => {
                if let TokenType::Identifier(ident) = &name.token_type {
                    symbols.push(Symbol {
                        name: ident.clone(),
                        kind: SymbolKind::CLASS,
                        location: Location {
                            uri: uri.clone(),
                            range: self.token_to_range(name),
                        },
                        container_name: container.clone(),
                        range: Some(self.token_to_range(name)),
                        selection_range: Some(self.token_to_range(name)),
                        children: Vec::new(),
                    });
                    // Extract method symbols
                    for method in methods {
                        if let Statement::FunctionDeclaration { name: method_name, .. } = method {
                            if let TokenType::Identifier(method_ident) = &method_name.token_type {
                                symbols.last_mut().unwrap().children.push(Symbol {
                                    name: method_ident.clone(),
                                    kind: SymbolKind::METHOD,
                                    location: Location {
                                        uri: uri.clone(),
                                        range: self.token_to_range(method_name),
                                    },
                                    container_name: Some(ident.clone()),
                                    range: Some(self.token_to_range(method_name)),
                                    selection_range: Some(self.token_to_range(method_name)),
                                    children: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn token_to_range(&self, token: &crate::token::Token) -> Range {
        // Convert token position to LSP range
        // This is a simplified implementation - you'd need to map line/column properly
        Range {
            start: Position::new((token.line - 1) as u32, 0),
            end: Position::new((token.line - 1) as u32, 100), // Simplified end position
        }
    }

    pub fn find_definitions(&self, name: &str, _uri: &Url, _position: Position) -> Vec<Location> {
        let mut locations = Vec::new();
        
        // Find all symbols with the given name
        for symbols in self.symbols.values() {
            for symbol in symbols {
                if symbol.name == name {
                    locations.push(symbol.location.clone());
                }
            }
        }
        
        locations
    }

    pub fn find_references(&self, _name: &str, _uri: &Url, _position: Position) -> Vec<Location> {
        let locations = Vec::new();
        
        // This is a simplified implementation
        // In a full implementation, you'd need to:
        // 1. Parse all documents to find identifier references
        // 2. Filter by the symbol at the given position
        // 3. Return all locations where this symbol is referenced
        
        locations
    }

    pub fn get_completions(&self, uri: &Url, _position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Add language keywords
        let keywords = vec![
            "def", "if", "elif", "else", "while", "for", "in",
            "return", "use", "as", "true", "false", "null",
            "class", "new", "self", "super",
            "and", "or", "not"
        ];
        
        for keyword in keywords {
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(format!("{} keyword", keyword)),
                documentation: Some(Documentation::String(format!("Grease keyword: {}", keyword))),
                deprecated: None,
                preselect: None,
                sort_text: None,
                filter_text: None,
                insert_text: None,
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                insert_text_mode: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                commit_characters: None,
                data: None,
                tags: None,
                label_details: None,
            });
        }
        
        // Add symbols from current document
        if let Some(doc_symbols) = self.symbols.get(&uri.to_string()) {
            for symbol in doc_symbols {
                completions.push(CompletionItem {
                    label: symbol.name.clone(),
                    kind: Some(match symbol.kind {
                        SymbolKind::FUNCTION => CompletionItemKind::FUNCTION,
                        SymbolKind::VARIABLE => CompletionItemKind::VARIABLE,
                        _ => CompletionItemKind::VARIABLE,
                    }),
                    detail: Some(format!("{} ({})", symbol.name, format_symbol_kind(symbol.kind))),
                    documentation: None,
                    deprecated: None,
                    preselect: None,
                    sort_text: None,
                    filter_text: None,
                    insert_text: None,
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    insert_text_mode: None,
                    text_edit: None,
                    additional_text_edits: None,
                    command: None,
                    commit_characters: None,
                    data: None,
                    tags: None,
                    label_details: None,
                });
            }
        }
        
        completions
    }
}

fn format_symbol_kind(kind: SymbolKind) -> &'static str {
    match kind {
        SymbolKind::FUNCTION => "function",
        SymbolKind::VARIABLE => "variable",
        _ => "symbol",
    }
}

// Re-export TokenType for use in this module
use crate::token::TokenType;