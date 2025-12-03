// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

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
                let param_name = self.consume_identifier("Expected parameter name")?;
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
        } else if self.match_token(&TokenType::Return) {
            Ok(Some(self.return_statement()?))
        } else if self.check(&TokenType::LeftBrace) {
            Ok(Some(self.block_statement()?))
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
        self.logical_or()
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
        let mut methods = Vec::new();
        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            if self.match_token(&TokenType::Fn) {
                let method = self.function_declaration()?;
                methods.push(method);
            } else {
                return Err("Expected method declaration in class".to_string());
            }
        }
        Ok(Statement::ClassDeclaration {
            name,
            superclass,
            methods,
        })
    }

    #[test]
    fn test_parse_number() {
        let expr = parse_expr("42").unwrap();
        assert!(matches!(expr, Expression::Number(42.0)));
    }

    #[test]
    fn test_parse_string() {
        let expr = parse_expr("\"hello\"").unwrap();
        assert!(matches!(expr, Expression::String(s) if s == "hello"));
    }

    #[test]
    fn test_parse_boolean() {
        let expr = parse_expr("true").unwrap();
        assert!(matches!(expr, Expression::Boolean(true)));
    }

    #[test]
    fn test_parse_null() {
        let expr = parse_expr("null").unwrap();
        assert!(matches!(expr, Expression::Null));
    }

    #[test]
    fn test_parse_identifier() {
        let expr = parse_expr("x").unwrap();
        match expr {
            Expression::Identifier(token) => {
                assert_eq!(token.token_type, TokenType::Identifier("x".to_string()));
            }
            _ => panic!("Expected identifier"),
        }
    }

    #[test]
    fn test_parse_binary_expression() {
        let expr = parse_expr("1 + 2").unwrap();
        match expr {
            Expression::Binary { left, operator, right } => {
                assert!(matches!(*left, Expression::Number(1.0)));
                assert_eq!(operator.token_type, TokenType::Plus);
                assert!(matches!(*right, Expression::Number(2.0)));
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_unary_expression() {
        let expr = parse_expr("-5").unwrap();
        match expr {
            Expression::Unary { operator, right } => {
                assert_eq!(operator.token_type, TokenType::Minus);
                assert!(matches!(*right, Expression::Number(5.0)));
            }
            _ => panic!("Expected unary expression"),
        }
    }



    #[test]
    fn test_parse_call() {
        let expr = parse_expr("print(42)").unwrap();
        match expr {
            Expression::Call { callee, arguments } => {
                match *callee {
                    Expression::Identifier(ref token) => {
                        assert_eq!(token.token_type, TokenType::Identifier("print".to_string()));
                    }
                    _ => panic!("Expected identifier callee"),
                }
                assert_eq!(arguments.len(), 1);
                assert!(matches!(arguments[0], Expression::Number(42.0)));
            }
            _ => panic!("Expected call expression"),
        }
    }

    #[test]
    fn test_parse_variable_declaration() {
        let program = parse_program("x = 42").unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::VariableDeclaration { name, type_annotation, initializer } => {
                assert_eq!(name.token_type, TokenType::Identifier("x".to_string()));
                assert!(type_annotation.is_none());
                assert!(matches!(initializer.as_ref().unwrap(), Expression::Number(42.0)));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_variable_declaration_with_type() {
        let program = parse_program("x: Number = 42").unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::VariableDeclaration { name, type_annotation, initializer } => {
                assert_eq!(name.token_type, TokenType::Identifier("x".to_string()));
                assert_eq!(type_annotation.as_ref().unwrap(), "Number");
                assert!(matches!(initializer.as_ref().unwrap(), Expression::Number(42.0)));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let program = parse_program("if true:\n    print(1)").unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::If { condition, then_branch, else_branch } => {
                assert!(matches!(condition, Expression::Boolean(true)));
                assert_eq!(then_branch.len(), 1);
                assert!(else_branch.is_none());
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn test_parse_while_statement() {
        let program = parse_program("while true:\n    print(1)").unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::While { condition, body } => {
                assert!(matches!(condition, Expression::Boolean(true)));
                assert_eq!(body.len(), 1);
            }
            _ => panic!("Expected while statement"),
        }
    }

    #[test]
    fn test_parse_use_statement() {
        let program = parse_program("use math").unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Use { module, alias } => {
                assert_eq!(module, "math");
                assert_eq!(*alias, None);
            }
            _ => panic!("Expected use statement"),
        }
    }

    #[test]
    fn test_parse_use_statement_with_alias() {
        let program = parse_program("use math as m").unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Use { module, alias } => {
                assert_eq!(module, "math");
                assert_eq!(*alias, Some("m".to_string()));
            }
            _ => panic!("Expected use statement"),
        }
    }


}