// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use std::io::{self, Write};
use crate::grease::Grease;

pub struct REPL {
    prompt: String,
    grease: Grease,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            prompt: "grease> ".to_string(),
            grease: Grease::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Grease Scripting Language v{}", env!("CARGO_PKG_VERSION"));
        println!("Type 'exit()' to quit.");
        println!();

        loop {
            print!("{}", self.prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let input = input.trim();
                    
                    if input == "exit()" {
                        println!("Goodbye!");
                        break;
                    }
                    
                    if input.is_empty() {
                        continue;
                    }
                    
                    self.execute(input);
                }
                Err(error) => {
                    eprintln!("Error reading input: {}", error);
                }
            }
        }
    }

    fn execute(&mut self, source: &str) {
        use crate::vm::InterpretResult;
        
        match self.grease.run(source) {
            Ok(result) => {
                match result {
                    InterpretResult::Ok => {
                        // Print the last value on the stack if any
                        if let Some(value) = self.grease.vm.stack.last() {
                            println!("{}", self.format_value(value));
                        }
                    }
                    InterpretResult::CompileError(msg) => {
                        eprintln!("Compile Error: {}", msg);
                    }
                    InterpretResult::RuntimeError(msg) => {
                        eprintln!("Runtime Error: {}", msg);
                    }
                }
            }
            Err(msg) => {
                eprintln!("Error: {}", msg);
            }
        }
    }

    fn format_value(&self, value: &crate::bytecode::Value) -> String {
        match value {
            crate::bytecode::Value::Number(n) => n.to_string(),
            crate::bytecode::Value::String(s) => format!("\"{}\"", s),
            crate::bytecode::Value::Boolean(b) => b.to_string(),
            crate::bytecode::Value::Null => "null".to_string(),
            crate::bytecode::Value::Function(f) => format!("<fn {}>", f.name),
            crate::bytecode::Value::NativeFunction(f) => format!("<native fn {}>", f.name),
            crate::bytecode::Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| match v {
                    crate::bytecode::Value::Number(n) => n.to_string(),
                    crate::bytecode::Value::String(s) => format!("\"{}\"", s),
                    crate::bytecode::Value::Boolean(b) => b.to_string(),
                    crate::bytecode::Value::Null => "null".to_string(),
                    crate::bytecode::Value::Function(f) => format!("<fn {}>", f.name),
                    crate::bytecode::Value::NativeFunction(f) => format!("<native fn {}>", f.name),
                    crate::bytecode::Value::Array(_) => "[...]".to_string(),
                    crate::bytecode::Value::Object { class_name, .. } => format!("<{} instance>", class_name),
                    crate::bytecode::Value::Class { name, .. } => format!("<class {}>", name),
                }).collect();
                format!("[{}]", elements.join(", "))
            },
            crate::bytecode::Value::Object { class_name, .. } => format!("<{} instance>", class_name),
            crate::bytecode::Value::Class { name, .. } => format!("<class {}>", name),
        }
    }
}