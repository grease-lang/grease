use grease::lexer::Lexer;
use grease::parser::Parser;

fn main() {
    let input = "print(\"Result: \" + (true and false))".to_string();
    let mut lexer = Lexer::new(input);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("Tokens:");
            for (i, token) in tokens.iter().enumerate() {
                println!("  {}: {:?}", i, token);
            }
            
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(program) => {
                    println!("Parsed successfully:");
                    for statement in &program.statements {
                        println!("  {:?}", statement);
                    }
                }
                Err(e) => {
                    println!("Parse error: {}", e);
                }
            }
        }
        Err(e) => println!("Lex error: {}", e),
    }
}