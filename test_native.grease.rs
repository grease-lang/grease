use grease::Grease;
use grease::bytecode::Value;

fn main() {
    let mut grease = Grease::new();

    // Register a native function
    grease.register_native("native_add", 2, |vm, args| {
        match (&args[0], &args[1]) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            _ => Err("Arguments must be numbers".to_string()),
        }
    });

    let result = grease.run("result = native_add(5, 3)\nprint('5 + 3 = ' + result)");
    match result {
        Ok(_) => println!("Native function call successful!"),
        Err(e) => println!("Error: {}", e),
    }
}