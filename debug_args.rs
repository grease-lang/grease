use std::io;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Args: {:?}", args);
        if args.len() > 3 {
            println!("Eval arg: '{}'", args[2]);
        }
    }
}