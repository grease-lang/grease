use grease::lexer::Lexer;

fn main() {
    let input = "use std/math".to_string();
    let mut lexer = Lexer::new(input);
    match lexer.tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}