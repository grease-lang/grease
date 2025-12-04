use grease::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("class Animal:\n    fn make_sound():\n        print(\"test\")".to_string());
    match lexer.tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}