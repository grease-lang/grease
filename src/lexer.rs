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
            tokens.push(Token::new(TokenType::Dedent, "".to_string(), self.line, self.column));
            self.indent_stack.pop();
        }

        tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line, self.column));
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
                Ok(Some(Token::new(TokenType::LeftParen, "(".to_string(), self.line, self.column)))
            }
            ')' => {
                self.advance();
                Ok(Some(Token::new(TokenType::RightParen, ")".to_string(), self.line, self.column)))
            }
            '{' => {
                self.advance();
                Ok(Some(Token::new(TokenType::LeftBrace, "{".to_string(), self.line, self.column)))
            }
            '}' => {
                self.advance();
                Ok(Some(Token::new(TokenType::RightBrace, "}".to_string(), self.line, self.column)))
            }
            '[' => {
                self.advance();
                Ok(Some(Token::new(TokenType::LeftBracket, "[".to_string(), self.line, self.column)))
            }
            ']' => {
                self.advance();
                Ok(Some(Token::new(TokenType::RightBracket, "]".to_string(), self.line, self.column)))
            }
            ',' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Comma, ",".to_string(), self.line, self.column)))
            }
            '.' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Dot, ".".to_string(), self.line, self.column)))
            }
            ':' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Colon, ":".to_string(), self.line, self.column)))
            }
            ';' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Semicolon, ";".to_string(), self.line, self.column)))
            }
            '+' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Plus, "+".to_string(), self.line, self.column)))
            }
            '-' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Minus, "-".to_string(), self.line, self.column)))
            }
            '*' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Multiply, "*".to_string(), self.line, self.column)))
            }
            '#' => {
                self.skip_comment();
                Ok(None)
            }
            '/' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Divide, "/".to_string(), self.line, self.column)))
            }
            '%' => {
                self.advance();
                Ok(Some(Token::new(TokenType::Modulo, "%".to_string(), self.line, self.column)))
            }
            '=' => {
                self.advance();
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::Equal, "==".to_string(), self.line, self.column)))
                } else {
                    Ok(Some(Token::new(TokenType::Assign, "=".to_string(), self.line, self.column)))
                }
            }
            '!' => {
                self.advance();
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::NotEqual, "!=".to_string(), self.line, self.column)))
                } else {
                    Err(format!("Unexpected character '!' at line {}, column {}", self.line, self.column))
                }
            }
            '<' => {
                self.advance();
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::LessEqual, "<=".to_string(), self.line, self.column)))
                } else {
                    Ok(Some(Token::new(TokenType::Less, "<".to_string(), self.line, self.column)))
                }
            }
            '>' => {
                self.advance();
                if self.match_char('=') {
                    Ok(Some(Token::new(TokenType::GreaterEqual, ">=".to_string(), self.line, self.column)))
                } else {
                    Ok(Some(Token::new(TokenType::Greater, ">".to_string(), self.line, self.column)))
                }
            }
            '"' => self.string(),
            '\'' => self.char_string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => {
                // Check if this is the start of an inline block
                if self.check_inline_block_start() {
                    self.inline_block()
                } else {
                    self.identifier()
                }
            },
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
            "throw" => TokenType::Throw,
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
            "rust" => TokenType::Identifier(text.clone()),
            "asm" => TokenType::Identifier(text.clone()),
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "str" => TokenType::Identifier(text.clone()),
            _ => TokenType::Identifier(text.clone()),
        };
        
        Ok(Some(Token::new(token_type, text, self.line, self.column)))
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
            Ok(value) => Ok(Some(Token::new(TokenType::Number(value), text, self.line, self.column))),
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
        
        Ok(Some(Token::new(TokenType::String(text.clone()), text, self.line, self.column)))
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
        
        Ok(Some(Token::new(TokenType::String(text.clone()), text, self.line, self.column)))
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
            Ok(Some(Token::new(TokenType::Indent, "".to_string(), self.line, self.column)))
        } else if indent_level < current_indent {
            self.indent_stack.pop();
            Ok(Some(Token::new(TokenType::Dedent, "".to_string(), self.line, self.column)))
        } else {
            Ok(Some(Token::new(TokenType::Newline, "\n".to_string(), self.line, self.column)))
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

    fn check_inline_block_start(&mut self) -> bool {
        let start_pos = self.position;
        
        // Check if the current identifier is "rust" or "asm"
        while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        
        let text: String = self.input[start_pos..self.position].iter().collect();
        let is_inline_keyword = text == "rust" || text == "asm";
        
        // Reset position
        self.position = start_pos;
        
        if !is_inline_keyword {
            return false;
        }
        
        // Skip the identifier
        while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        
        // Skip whitespace
        while !self.is_at_end() && self.current_char().is_whitespace() && self.current_char() != '\n' {
            self.advance();
        }
        
        // Check if next character is '{'
        let has_brace = !self.is_at_end() && self.current_char() == '{';
        
        // Reset position
        self.position = start_pos;
        
        has_brace
    }
    
    fn inline_block(&mut self) -> Result<Option<Token>, String> {
        let start_pos = self.position;
        
        // Read the keyword (rust or asm)
        while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        
        let keyword: String = self.input[start_pos..self.position].iter().collect();
        
        // Skip whitespace
        while !self.is_at_end() && self.current_char().is_whitespace() && self.current_char() != '\n' {
            self.advance();
        }
        
        // Expect '{'
        if self.is_at_end() || self.current_char() != '{' {
            return Err(format!("Expected '{{' after {} at line {}, column {}", keyword, self.line, self.column));
        }
        self.advance(); // skip '{'
        
        // Find matching closing brace
        let mut brace_count = 1;
        let code_start = self.position;
        
        while !self.is_at_end() && brace_count > 0 {
            if self.current_char() == '{' {
                brace_count += 1;
            } else if self.current_char() == '}' {
                brace_count -= 1;
            } else if self.current_char() == '\n' {
                self.line += 1;
                self.column = 1;
                self.advance();
                continue;
            }
            self.advance();
        }
        
        if brace_count > 0 {
            return Err(format!("Unterminated inline {} block at line {}", keyword, self.line));
        }
        
        // Extract the code (everything between the braces)
        let code_end = self.position - 1; // exclude the closing '}'
        let code: String = self.input[code_start..code_end].iter().collect();
        
        let token_type = if keyword == "rust" {
            TokenType::RustInline
        } else {
            TokenType::AsmInline
        };
        
        Ok(Some(Token::new(token_type, code, self.line, self.column)))
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
        assert_eq!(tokens[13].token_type, TokenType::Class);
        assert_eq!(tokens[14].token_type, TokenType::New);
        assert_eq!(tokens[15].token_type, TokenType::SelfKw);
        assert_eq!(tokens[16].token_type, TokenType::Super);
        assert_eq!(tokens[17].token_type, TokenType::EOF);
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"hello".to_string());
        assert!(lexer.tokenize().is_err());
    }
}