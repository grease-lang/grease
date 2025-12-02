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
            '/' => {
                self.advance();
                if self.match_char('/') {
                    self.skip_comment();
                    Ok(None)
                } else {
                    Ok(Some(Token::new(TokenType::Divide, self.line, self.column)))
                }
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
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
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