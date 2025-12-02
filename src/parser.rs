use crate::token::{Token, TokenType};
use crate::ast::{Expression, Statement, Program};
use std::iter::Peekable;
use std::vec::IntoIter;

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
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            if let Some(statement) = self.declaration()? {
                statements.push(statement);
            }
        }
        
        Ok(Program { statements })
    }

    fn declaration(&mut self) -> Result<Option<Statement>, String> {
        if self.match_token(&TokenType::Let) {
            Ok(Some(self.variable_declaration()?))
        } else if self.match_token(&TokenType::Fn) {
            Ok(Some(self.function_declaration()?))
        } else {
            self.statement()
        }
    }

    fn variable_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume_identifier("Expected variable name")?;
        
        let mut type_annotation = None;
        if self.match_token(&TokenType::Colon) {
            let type_token = self.consume_identifier("Expected type name")?;
            type_annotation = Some(match &type_token.token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err("Expected type name".to_string()),
            });
        }
        
        let mut initializer = None;
        if self.match_token(&TokenType::Assign) {
            initializer = Some(self.expression()?);
        }
        
        self.match_token(&TokenType::Newline);
        
        Ok(Statement::VariableDeclaration {
            name,
            type_annotation,
            initializer,
        })
    }

    fn function_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume_identifier("Expected function name")?;
        
        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let param_name = self.consume_identifier("Expected parameter name")?;
                
                let mut param_type = None;
                if self.match_token(&TokenType::Colon) {
                    let type_token = self.consume_identifier("Expected type name")?;
                    param_type = Some(match &type_token.token_type {
                        TokenType::Identifier(name) => name.clone(),
                        _ => return Err("Expected type name".to_string()),
                    });
                }
                
                parameters.push((param_name, param_type));
                
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
        
        let mut return_type = None;
        if self.match_token(&TokenType::Colon) {
            let type_token = self.consume_identifier("Expected return type")?;
            return_type = Some(match &type_token.token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err("Expected type name".to_string()),
            });
        }
        
        let body = self.block()?;
        
        Ok(Statement::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }

    fn statement(&mut self) -> Result<Option<Statement>, String> {
        if self.match_token(&TokenType::If) {
            Ok(Some(self.if_statement()?))
        } else if self.match_token(&TokenType::While) {
            Ok(Some(self.while_statement()?))
        } else if self.match_token(&TokenType::For) {
            Ok(Some(self.for_statement()?))
        } else if self.match_token(&TokenType::Return) {
            Ok(Some(self.return_statement()?))
        } else if self.check(&TokenType::LeftBrace) {
            Ok(Some(self.block_statement()?))
        } else {
            let expr = self.expression()?;
            // Only consume newline if it exists (for REPL compatibility)
            self.match_token(&TokenType::Newline);
            Ok(Some(Statement::Expression(expr)))
        }
    }

    fn if_statement(&mut self) -> Result<Statement, String> {
        let condition = self.expression()?;
        let then_branch = self.block()?;
        
        let mut else_branch = None;
        if self.match_token(&TokenType::Else) {
            if self.check(&TokenType::If) {
                else_branch = Some(vec![self.if_statement()?]);
            } else {
                else_branch = Some(self.block()?);
            }
        }
        
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Statement, String> {
        let condition = self.expression()?;
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

    fn block(&mut self) -> Result<Vec<Statement>, String> {
        self.consume(TokenType::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(statement) = self.declaration()? {
                statements.push(statement);
            }
        }
        
        self.consume(TokenType::RightBrace, "Expected '}' after block")?;
        
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expression, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, String> {
        let expr = self.logical_or()?;
        
        if self.match_token(&TokenType::Assign) {
            let value = self.assignment()?;
            
            match expr {
                Expression::Identifier(_) => {
                    Ok(Expression::Assignment {
                        name: self.previous().unwrap(),
                        value: Box::new(value),
                    })
                }
                _ => Err("Invalid assignment target".to_string()),
            }
        } else {
            Ok(expr)
        }
    }

    fn logical_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.logical_and()?;
        
        while self.match_token(&TokenType::Or) {
            let operator = self.previous().unwrap();
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
            let operator = self.previous().unwrap();
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
            let operator = self.previous().unwrap();
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
            let operator = self.previous().unwrap();
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
            let operator = self.previous().unwrap();
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
            let operator = self.previous().unwrap();
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
            let operator = self.previous().unwrap();
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
        
        while self.match_token(&TokenType::LeftParen) {
            expr = self.finish_call(expr)?;
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
                TokenType::Identifier(value) => {
                    self.advance();
                    return Ok(Expression::Identifier(value));
                }
                TokenType::LeftParen => {
                    self.advance();
                    let expr = self.expression()?;
                    self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                    return Ok(Expression::Grouping(Box::new(expr)));
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

    fn previous(&self) -> Option<Token> {
        self.previous.clone()
    }

    fn current_line(&mut self) -> usize {
        match self.tokens.peek() {
            Some(token) => token.line,
            None => 0,
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<Token, String> {
        match self.tokens.peek() {
            Some(token) => {
                match &token.token_type {
                    TokenType::Identifier(_) => {
                        Ok(self.advance().unwrap())
                    }
                    _ => Err(format!("{} at line {}", message, token.line))
                }
            }
            None => Err(format!("{} at end of input", message))
        }
    }
}