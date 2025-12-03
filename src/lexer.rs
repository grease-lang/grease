// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    indent_stack: Vec<usize>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0],
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            match self.scan_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => {},
                Err(e) => return Err(e),
            }
        }
        
        // Add remaining dedents
        while self.indent_stack.len() > 1 {
            tokens.push(Token::new(TokenType::Dedent, self.line, self.column));
            self.indent_stack.pop();
        }
        
        tokens.push(Token::new(TokenType::EOF, self.line, self.column));
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Ok(None);
        }

        let c = self.current_char();
        
        match c {
            '(' => {
                self.advance();
                Ok(Some(Token::new(TokenType::LeftParen, self.line, self.column)))
            }
            ')' => {
                self.advance();
                Ok(Some(Token::new(TokenType::RightParen, self.line, self.column)))
            }
            '{' => {
                self.advance();
                Ok(Some(Token::new(TokenType::LeftBrace, self.line, self.column)))
            }
            '}' => {
                self.advance();
                Ok(Some(Token::new(TokenType::RightBrace, self.line, self.column)))
            }
            '[' => {
                self.advance();
                Ok(Some(Token::new(TokenType::LeftBracket, self.line, self.column)))
            }
            ']' => {
                self.advance();
                Ok(Some(Token::new(TokenType::RightBracket, self.line, self.column)))
            }
            ',' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Comma, self.line, self.column)))
            }
            '.' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Dot, self.line, self.column)))
            }
            ':' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Colon, self.line, self.column)))
            }
            ';' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Semicolon, self.line, self.column)))
            }
            '+' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Plus, self.line, self.column)))
            }
            '-' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Minus, self.line, self.column)))
            }
            '*' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Multiply, self.line, self.column)))
            }
            '#' => {
                self.skip_comment();
                Ok(None)
            }
            '/' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Divide, self.line, self.column)))
            }
            '%' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Modulo, self.line, self.column)))
            }
            '=' => {
                self.advance();
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::Equal, self.line, self.column)))
                } else {
                    Ok(Some(Token::new(TokenType::Assign, self.line, self.column)))
                }
            }
            '!' => {
                self.advance();
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::NotEqual, self.line, self.column)))
                } else {
                    Err(format!("Unexpected character '!' at line {}, column {}", self.line, self.column))
                }
            }
            '<' => {
                self.advance();
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::LessEqual, self.line, self.column)))
                } else {
                    Ok(Some(Token::new(TokenType::Less, self.line, self.column)))
                }
            }
            '>' => {
                self.advance();
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::GreaterEqual, self.line, self.column)))
                } else {
                    Ok(Some(Token::new(TokenType::Greater, self.line, self.column)))
                }
            }
            '"' => self.string(),
            '\'' => self.char_string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            '\n' => self.newline(),
            _ => Err(format!("Unexpected character '{}' at line {}, column {}", c, self.line, self.column)),
        }
    }

    fn identifier(&mut self) -> Result<Option<Token>, String> {
        let start = self.position;
        while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        
        let text: String = self.input[start..self.position].iter().collect();
        let token_type = match text.as_str() {
            "def" => TokenType::Fn,
            "if" => TokenType::If,
            "elif" => TokenType::Elif,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "return" => TokenType::Return,
            "use" => TokenType::Use,
            "as" => TokenType::As,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            "class" => TokenType::Class,
            "new" => TokenType::New,
            "self" => TokenType::SelfKw,
            "super" => TokenType::Super,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            "str" => TokenType::Identifier(text),
            _ => TokenType::Identifier(text),
        };
        
        Ok(Some(Token::new(token_type, self.line, self.column)))
    }

    fn number(&mut self) -> Result<Option<Token>, String> {
        let start = self.position;
        let mut has_dot = false;
        
        while !self.is_at_end() && (self.current_char().is_numeric() || self.current_char() == '.') {
            if self.current_char() == '.' {
                if has_dot {
                    break;
                }
                has_dot = true;
            }
            self.advance();
        }
        
        let text: String = self.input[start..self.position].iter().collect();
        match text.parse::<f64>() {
            Ok(value) => Ok(Some(Token::new(TokenType::Number(value), self.line, self.column))),
            Err(_) => Err(format!("Invalid number '{}' at line {}, column {}", text, self.line, self.column)),
        }
    }

    fn string(&mut self) -> Result<Option<Token>, String> {
        self.advance(); // skip opening quote
        let start = self.position;
        
        while !self.is_at_end() && self.current_char() != '"' {
            if self.current_char() == '\n' {
                return Err(format!("Unterminated string at line {}", self.line));
            }
            self.advance();
        }
        
        if self.is_at_end() {
            return Err(format!("Unterminated string at line {}", self.line));
        }
        
        let text: String = self.input[start..self.position].iter().collect();
        self.advance(); // skip closing quote
        
        Ok(Some(Token::new(TokenType::String(text), self.line, self.column)))
    }

    fn char_string(&mut self) -> Result<Option<Token>, String> {
        self.advance(); // skip opening quote
        let start = self.position;
        
        while !self.is_at_end() && self.current_char() != '\'' {
            if self.current_char() == '\n' {
                return Err(format!("Unterminated string at line {}", self.line));
            }
            self.advance();
        }
        
        if self.is_at_end() {
            return Err(format!("Unterminated string at line {}", self.line));
        }
        
        let text: String = self.input[start..self.position].iter().collect();
        self.advance(); // skip closing quote
        
        Ok(Some(Token::new(TokenType::String(text), self.line, self.column)))
    }

    fn newline(&mut self) -> Result<Option<Token>, String> {
        self.advance();
        self.line += 1;
        self.column = 1;
        
        // Handle indentation
        let mut indent_level = 0;
        let mut peek_pos = self.position;
        while peek_pos < self.input.len() && self.input[peek_pos].is_whitespace() && self.input[peek_pos] != '\n' {
            if self.input[peek_pos] == ' ' {
                indent_level += 1;
            } else if self.input[peek_pos] == '\t' {
                indent_level += 4; // treat tab as 4 spaces
            }
            peek_pos += 1;
        }
        
        let current_indent = *self.indent_stack.last().unwrap();
        
        if indent_level > current_indent {
            self.indent_stack.push(indent_level);
            Ok(Some(Token::new(TokenType::Indent, self.line, self.column)))
        } else if indent_level < current_indent {
            self.indent_stack.pop();
            Ok(Some(Token::new(TokenType::Dedent, self.line, self.column)))
        } else {
            Ok(Some(Token::new(TokenType::Newline, self.line, self.column)))
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.current_char().is_whitespace() && self.current_char() != '\n' {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.current_char() != '\n' {
            self.advance();
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.current_char() != expected {
            return false;
        }
        self.advance();
        true
    }

    fn current_char(&self) -> char {
        self.input[self.position]
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_numbers() {
        let mut lexer = Lexer::new("42 3.14".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3); // two numbers + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number(42.0));
        assert_eq!(tokens[1].token_type, TokenType::Number(3.14));
    }

    #[test]
    fn test_tokenize_strings() {
        let mut lexer = Lexer::new("\"hello\" 'world'".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::String("hello".to_string()));
        assert_eq!(tokens[1].token_type, TokenType::String("world".to_string()));
    }

    #[test]
    fn test_tokenize_identifiers() {
        let mut lexer = Lexer::new("def x y_z".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Fn);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("x".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::Identifier("y_z".to_string()));
    }

    #[test]
    fn test_tokenize_operators() {
        let mut lexer = Lexer::new("+ - * / == != < > <= >=".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 11); // 10 operators + EOF
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Multiply);
        assert_eq!(tokens[3].token_type, TokenType::Divide);
        assert_eq!(tokens[4].token_type, TokenType::Equal);
        assert_eq!(tokens[5].token_type, TokenType::NotEqual);
        assert_eq!(tokens[6].token_type, TokenType::Less);
        assert_eq!(tokens[7].token_type, TokenType::Greater);
        assert_eq!(tokens[8].token_type, TokenType::LessEqual);
        assert_eq!(tokens[9].token_type, TokenType::GreaterEqual);
        assert_eq!(tokens[10].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_keywords() {
        let mut lexer = Lexer::new("def if elif else while for in return use as true false null class new self super".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 18); // 17 keywords + EOF
        assert_eq!(tokens[0].token_type, TokenType::Fn);
        assert_eq!(tokens[1].token_type, TokenType::If);
        assert_eq!(tokens[2].token_type, TokenType::Elif);
        assert_eq!(tokens[3].token_type, TokenType::Else);
        assert_eq!(tokens[4].token_type, TokenType::While);
        assert_eq!(tokens[5].token_type, TokenType::For);
        assert_eq!(tokens[6].token_type, TokenType::In);
        assert_eq!(tokens[7].token_type, TokenType::Return);
        assert_eq!(tokens[8].token_type, TokenType::Use);
        assert_eq!(tokens[9].token_type, TokenType::As);
        assert_eq!(tokens[10].token_type, TokenType::True);
        assert_eq!(tokens[11].token_type, TokenType::False);
        assert_eq!(tokens[12].token_type, TokenType::Null);
        assert_eq!(tokens[13].token_type, TokenType::EOF);
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"hello".to_string());
        assert!(lexer.tokenize().is_err());
    }
}