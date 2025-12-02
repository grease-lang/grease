// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    
    // Keywords
    Fn,
    If,
    Elif,
    Else,
    While,
    For,
    In,
    Return,
    Use,
    As,
    True,
    False,
    Null,
    
    // Operators
    Assign,       // =
    Plus,         // +
    Minus,        // -
    Multiply,     // *
    Divide,       // /
    Modulo,       // %
    
    // Comparison
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    
    // Logical
    And,          // and
    Or,           // or
    Not,          // not
    
    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    Semicolon,    // ;
    
    // Special
    Newline,
    Indent,
    Dedent,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}