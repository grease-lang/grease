use std::env;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Eval arg: '{}'", args[2]);
        println!("Length: {}", args[2].len());
        for (i, c) in args[2].chars().enumerate() {
            println!("  {}: '{}' ({})", i, c, c as u32);
        }
    }
}