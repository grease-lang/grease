use grease::Grease;

fn main() {
    let mut grease = Grease::new().with_verbose(true);
    let source = "class Animal:\n    fn make_sound():\n        print(\"test\")";
    match grease.run(source) {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
}