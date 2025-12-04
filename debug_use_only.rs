use grease::lexer::Lexer;
use grease::parser::Parser;

fn main() {
    let input = "use std/math;".to_string();
    let mut lexer = Lexer::new(input);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("All tokens:");
            for (i, token) in tokens.iter().enumerate() {
                println!("  {}: {:?}", i, token);
            }
            
            let mut parser = Parser::new(tokens);
            
            // Try to parse just the use statement
            match parser.use_statement() {
                Ok(use_stmt) => {
                    println!("Use statement parsed: {:?}", use_stmt);
                }
                Err(e) => {
                    println!("Use statement error: {}", e);
                }
            }
            
            // Check what tokens are left
            println!("Tokens remaining after use statement:");
            if let Some(token) = parser.tokens.peek() {
                println!("  Next token: {:?}", token);
            } else {
                println!("  No tokens remaining");
            }
        }
        Err(e) => println!("Lex error: {}", e),
    }
}