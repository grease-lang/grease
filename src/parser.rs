// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use crate::token::{Token, TokenType};
use crate::ast::{Expression, Statement, Program};
use std::iter::Peekable;
use std::vec::IntoIter;

#[allow(dead_code)]
fn parse_expr(input: &str) -> Result<Expression, String> {
    let mut lexer = crate::lexer::Lexer::new(input.to_string());
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.expression()
}

#[allow(dead_code)]
fn parse_program(input: &str) -> Result<Program, String> {
    let mut lexer = crate::lexer::Lexer::new(input.to_string());
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    previous: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            previous: None,
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut program = Program { statements: Vec::new() };

        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            if let Some(stmt) = self.declaration()? {
                program.statements.push(stmt);
            } else {
                return Err(format!("Unexpected token at line {}", self.tokens.peek().map(|t| t.line).unwrap_or(0)));
            }
        }

        Ok(program)
    }

    fn skip_newlines(&mut self) {
        while self.match_token(&TokenType::Newline) {
            // Skip
        }
    }

    fn declaration(&mut self) -> Result<Option<Statement>, String> {
        if self.match_token(&TokenType::Fn) {
            Ok(Some(self.function_declaration()?))
        } else if self.match_token(&TokenType::Use) {
            Ok(Some(self.use_statement()?))
        } else {
            self.statement()
        }
    }



    fn function_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume_identifier("Expected function name")?;

        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;

        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let param_name = if self.match_token(&TokenType::SelfKw) {
                    self.previous.clone().unwrap()
                } else {
                    self.consume_identifier("Expected parameter name")?
                };
                parameters.push((param_name, None)); // No type annotations

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        self.consume(TokenType::Colon, "Expected ':' after function signature")?;

        let body = self.block()?;

        Ok(Statement::FunctionDeclaration {
            name,
            parameters,
            return_type: None, // No return type annotations
            body,
        })
    }

    fn use_statement(&mut self) -> Result<Statement, String> {
        let module_token = self.consume_identifier("Expected module name after 'use'")?;
        let module = if let TokenType::Identifier(ref name) = module_token.token_type {
            name.clone()
        } else {
            return Err("Expected identifier for module name".to_string());
        };

        let alias = if self.match_token(&TokenType::As) {
            let alias_token = self.consume_identifier("Expected alias name after 'as'")?;
            if let TokenType::Identifier(ref name) = alias_token.token_type {
                Some(name.clone())
            } else {
                return Err("Expected identifier for alias".to_string());
            }
        } else {
            None
        };

        Ok(Statement::Use { module, alias })
    }

    fn statement(&mut self) -> Result<Option<Statement>, String> {
        if self.match_token(&TokenType::If) {
            Ok(Some(self.if_statement()?))
        } else if self.match_token(&TokenType::While) {
            Ok(Some(self.while_statement()?))
        } else if self.match_token(&TokenType::For) {
            Ok(Some(self.for_statement()?))
        } else if self.match_token(&TokenType::Class) {
            Ok(Some(self.class_statement()?))
        } else if self.match_token(&TokenType::Try) {
            Ok(Some(self.try_statement()?))
        } else if self.match_token(&TokenType::Throw) {
            Ok(Some(self.throw_statement()?))
        } else if self.match_token(&TokenType::Return) {
            Ok(Some(self.return_statement()?))
        } else if self.check(&TokenType::LeftBrace) {
            Ok(Some(self.block_statement()?))
        } else if self.check(&TokenType::RustInline) {
            Ok(Some(self.rust_inline_statement()?))
        } else if self.check(&TokenType::AsmInline) {
            Ok(Some(self.asm_inline_statement()?))
        } else if self.is_assignment_statement() {
            Ok(Some(self.assignment_statement()?))
        } else {
            let expr = self.expression()?;
            // Only consume newline if it exists (for REPL compatibility)
            self.match_token(&TokenType::Newline);
            Ok(Some(Statement::Expression(expr)))
        }
    }

    fn if_statement(&mut self) -> Result<Statement, String> {
        let condition = self.expression()?;
        self.consume(TokenType::Colon, "Expected ':' after if condition")?;
        let then_branch = self.block()?;

        let mut else_branch = None;
        if self.match_token(&TokenType::Elif) {
            else_branch = Some(vec![self.if_statement()?]);
        } else if self.match_token(&TokenType::Else) {
            self.consume(TokenType::Colon, "Expected ':' after else")?;
            else_branch = Some(self.block()?);
        }

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn try_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Colon, "Expected ':' after try")?;
        let try_block = self.block()?;
        self.consume(TokenType::Catch, "Expected 'catch' after try block")?;
        self.consume(TokenType::Colon, "Expected ':' after catch")?;
        let catch_block = self.block()?;
        Ok(Statement::Try {
            try_block,
            catch_block,
        })
    }

    fn throw_statement(&mut self) -> Result<Statement, String> {
        let value = if self.check(&TokenType::Newline) || self.check(&TokenType::EOF) {
            None
        } else {
            Some(self.expression()?)
        };
        Ok(Statement::Throw { value })
    }

    fn while_statement(&mut self) -> Result<Statement, String> {
        let condition = self.expression()?;
        self.consume(TokenType::Colon, "Expected ':' after while condition")?;
        let body = self.block()?;

        Ok(Statement::While {
            condition,
            body,
        })
    }

    fn for_statement(&mut self) -> Result<Statement, String> {
        let variable = self.consume_identifier("Expected variable name")?;
        self.consume(TokenType::In, "Expected 'in' after for variable")?;
        let iterable = self.expression()?;
        self.consume(TokenType::Colon, "Expected ':' after for clause")?;
        let body = self.block()?;

        Ok(Statement::For {
            variable,
            iterable,
            body,
        })
    }

    fn return_statement(&mut self) -> Result<Statement, String> {
        let value = if !self.check(&TokenType::Newline) && !self.is_at_end() {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.match_token(&TokenType::Newline);
        
        Ok(Statement::Return { value })
    }

    fn block_statement(&mut self) -> Result<Statement, String> {
        Ok(Statement::Block(self.block()?))
    }

    fn is_assignment_statement(&mut self) -> bool {
        // Check if this looks like an assignment: identifier [: Type] = ...
        if let Some(token) = self.tokens.peek() {
            if let TokenType::Identifier(_) = &token.token_type {
                let mut temp_tokens = self.tokens.clone();
                temp_tokens.next(); // consume identifier
                if let Some(next_token) = temp_tokens.next() {
                    if matches!(next_token.token_type, TokenType::Assign) {
                        return true;
                    } else if matches!(next_token.token_type, TokenType::Colon) {
                        if let Some(type_token) = temp_tokens.next() {
                            if let TokenType::Identifier(_) = &type_token.token_type {
                                if let Some(assign_token) = temp_tokens.next() {
                                    return matches!(assign_token.token_type, TokenType::Assign);
                                }
                            }
                        }
                    }
                }
                false
            } else {
                false
            }
        } else {
            false
        }
    }

    fn assignment_statement(&mut self) -> Result<Statement, String> {
        let name = self.consume_identifier("Expected variable name")?;

        let mut type_annotation = None;
        if self.match_token(&TokenType::Colon) {
            let type_token = self.consume_identifier("Expected type name")?;
            type_annotation = Some(match &type_token.token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err("Expected type name".to_string()),
            });
        }

        self.consume(TokenType::Assign, "Expected '=' after variable name")?;
        let initializer = self.expression()?;
        self.match_token(&TokenType::Newline);

        Ok(Statement::VariableDeclaration {
            name,
            type_annotation,
            initializer: Some(initializer),
        })
    }

    fn block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        // Skip the newline after the colon
        self.match_token(&TokenType::Newline);

        // Expect an indent
        self.consume(TokenType::Indent, "Expected indented block")?;

        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenType::Dedent) {
                break;
            }
            if let Some(statement) = self.declaration()? {
                statements.push(statement);
            }
        }

        self.consume(TokenType::Dedent, "Expected end of indented block")?;

        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expression, String> {
        let expr = self.logical_or()?;
        
        // Check for assignment
        if self.match_token(&TokenType::Assign) {
            let value = self.expression()?;
            
            match expr {
                Expression::Identifier(name) => {
                    return Ok(Expression::Assignment {
                        name,
                        value: Box::new(value),
                    });
                }
                Expression::PropertyAccess { object, property } => {
                    return Ok(Expression::PropertyAssignment {
                        object,
                        property,
                        value: Box::new(value),
                    });
                }
                _ => return Err("Invalid assignment target".to_string()),
            }
        }
        
        Ok(expr)
    }



    fn logical_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.logical_and()?;
        
        while self.match_token(&TokenType::Or) {
            let operator = self.previous.clone().unwrap();
            let right = self.logical_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.equality()?;
        
        while self.match_token(&TokenType::And) {
            let operator = self.previous.clone().unwrap();
            let right = self.equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.comparison()?;
        
        while self.match_token(&TokenType::Equal) || self.match_token(&TokenType::NotEqual) {
            let operator = self.previous.clone().unwrap();
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.term()?;
        
        while self.match_token(&TokenType::Greater) || 
              self.match_token(&TokenType::GreaterEqual) || 
              self.match_token(&TokenType::Less) || 
              self.match_token(&TokenType::LessEqual) {
            let operator = self.previous.clone().unwrap();
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor()?;
        
        while self.match_token(&TokenType::Plus) || self.match_token(&TokenType::Minus) {
            let operator = self.previous.clone().unwrap();
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.unary()?;
        
        while self.match_token(&TokenType::Multiply) || 
              self.match_token(&TokenType::Divide) || 
              self.match_token(&TokenType::Modulo) {
            let operator = self.previous.clone().unwrap();
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_token(&TokenType::Not) || self.match_token(&TokenType::Minus) {
            let operator = self.previous.clone().unwrap();
            let right = self.unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }
        
        self.call()
    }

    fn call(&mut self) -> Result<Expression, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&TokenType::Dot) {
                let member = self.consume_identifier("Expected property name after '.'")?;
                if self.match_token(&TokenType::LeftParen) {
                    // method call
                    let mut arguments = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            arguments.push(self.expression()?);
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
                    expr = Expression::MethodCall {
                        object: Box::new(expr),
                        method: member,
                        arguments,
                    };
                } else {
                    // property access
                    expr = Expression::PropertyAccess {
                        object: Box::new(expr),
                        property: member,
                    };
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, String> {
        let mut arguments = Vec::new();
        
        if !self.check(&TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
        
        Ok(Expression::Call {
            callee: Box::new(callee),
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.tokens.peek() {
            let token_type = token.token_type.clone();
            match token_type {
                TokenType::Number(value) => {
                    self.advance();
                    return Ok(Expression::Number(value));
                }
                TokenType::String(value) => {
                    self.advance();
                    return Ok(Expression::String(value));
                }
                TokenType::True => {
                    self.advance();
                    return Ok(Expression::Boolean(true));
                }
                TokenType::False => {
                    self.advance();
                    return Ok(Expression::Boolean(false));
                }
                TokenType::Null => {
                    self.advance();
                    return Ok(Expression::Null);
                }
                TokenType::New => {
                    self.advance();
                    let class = self.primary()?;
                    self.consume(TokenType::LeftParen, "Expected '(' after new")?;
                    let mut arguments = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            arguments.push(self.expression()?);
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
                    return Ok(Expression::NewInstance {
                        class: Box::new(class),
                        arguments,
                    });
                }
                TokenType::Super => {
                    self.advance();
                    self.consume(TokenType::LeftParen, "Expected '(' after super")?;
                    let method = if self.check(&TokenType::RightParen) {
                        None
                    } else {
                        Some(self.consume_identifier("Expected method name")?)
                    };
                    let mut arguments = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            arguments.push(self.expression()?);
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenType::RightParen, "Expected ')' after super")?;
                    return Ok(Expression::SuperCall {
                        method,
                        arguments,
                    });
                }
                TokenType::Identifier(_) => {
                    let token = self.advance().unwrap().clone();
                    return Ok(Expression::Identifier(token));
                }
                TokenType::SelfKw => {
                    let token = self.advance().unwrap().clone();
                    return Ok(Expression::Identifier(token));
                }
                TokenType::LeftParen => {
                    self.advance();
                    let expr = self.expression()?;
                    self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                    return Ok(Expression::Grouping(Box::new(expr)));
                }
                TokenType::LeftBracket => {
                    self.advance();
                    let mut elements = Vec::new();
                    
                    if !self.check(&TokenType::RightBracket) {
                        loop {
                            elements.push(self.expression()?);
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    
                    self.consume(TokenType::RightBracket, "Expected ']' after array elements")?;
                    return Ok(Expression::Array(elements));
                }
                TokenType::LeftBrace => {
                    self.advance();
                    let mut pairs = Vec::new();
                    
                    if !self.check(&TokenType::RightBrace) {
                        loop {
                            let key = self.expression()?;
                            self.consume(TokenType::Colon, "Expected ':' after dictionary key")?;
                            let value = self.expression()?;
                            pairs.push((key, value));
                            
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    
                    self.consume(TokenType::RightBrace, "Expected '}' after dictionary pairs")?;
                    return Ok(Expression::Dictionary(pairs));
                }
                _ => {}
            }
        }
        
        Err(format!("Unexpected token at line {}", self.current_line()))
    }

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if let Some(token) = self.tokens.peek() {
            if std::mem::discriminant(&token.token_type) == std::mem::discriminant(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if let Some(token) = self.tokens.peek() {
            std::mem::discriminant(&token.token_type) == std::mem::discriminant(token_type)
        } else {
            false
        }
    }

    fn advance(&mut self) -> Option<Token> {
        self.previous = self.tokens.next();
        self.previous.clone()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, String> {
        if self.check(&token_type) {
            Ok(self.advance().unwrap())
        } else {
            Err(format!("{} at line {}", message, self.current_line()))
        }
    }



    fn is_at_end(&mut self) -> bool {
        match self.tokens.peek() {
            Some(token) => matches!(token.token_type, TokenType::EOF),
            None => true,
        }
    }

    fn class_statement(&mut self) -> Result<Statement, String> {
        let name = self.consume_identifier("Expected class name")?;
        let superclass = if self.match_token(&TokenType::LeftParen) {
            let super_name = self.consume_identifier("Expected superclass name")?;
            self.consume(TokenType::RightParen, "Expected ')' after superclass")?;
            Some(super_name)
        } else {
            None
        };
        self.consume(TokenType::Colon, "Expected ':' after class declaration")?;
        
        // Skip the newline after the colon
        self.match_token(&TokenType::Newline);
        
        // Expect an indent
        self.consume(TokenType::Indent, "Expected indented block")?;
        
        let mut methods = Vec::new();
        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenType::Dedent) {
                break;
            }
            if self.match_token(&TokenType::Fn) {
                let method = self.function_declaration()?;
                methods.push(method);
            } else {
                return Err("Expected method declaration in class".to_string());
            }
        }
        
        self.consume(TokenType::Dedent, "Expected end of indented block")?;
        Ok(Statement::ClassDeclaration {
            name,
            superclass,
            methods,
        })
    }

    fn consume_identifier(&mut self, message: &str) -> Result<Token, String> {
        if let Some(token) = self.tokens.peek() {
            if let TokenType::Identifier(_) = &token.token_type {
                return Ok(self.tokens.next().unwrap());
            }
        }
        Err(message.to_string())
    }

    fn current_line(&self) -> usize {
        self.previous.as_ref().map(|t| t.line).unwrap_or(0)
    }

    fn rust_inline_statement(&mut self) -> Result<Statement, String> {
        let token = self.advance().unwrap();
        if let TokenType::RustInline = token.token_type {
            Ok(Statement::RustInline { code: token.lexeme })
        } else {
            Err("Expected rust inline block".to_string())
        }
    }

    fn asm_inline_statement(&mut self) -> Result<Statement, String> {
        let token = self.advance().unwrap();
        if let TokenType::AsmInline = token.token_type {
            Ok(Statement::AsmInline { code: token.lexeme })
        } else {
            Err("Expected asm inline block".to_string())
        }
    }




}