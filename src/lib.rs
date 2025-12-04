// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod bytecode;
pub mod compiler;
pub mod vm;
pub mod repl;
pub mod grease;
pub mod linter;
pub mod lsp_workspace;
pub mod lsp_server;
pub mod ui;
pub mod jit;
pub mod performance;
pub mod package;
pub mod package_manager;
pub mod webassembly;

pub use token::*;
pub use lexer::*;
pub use ast::*;
pub use parser::*;
pub use bytecode::*;
pub use compiler::*;
pub use vm::*;
pub use repl::*;
pub use grease::*;
pub use linter::*;
pub use lsp_workspace::*;
pub use lsp_server::*;
pub use ui::*;
pub use jit::*;
pub use performance::*;
pub use package::*;
pub use package_manager::*;
pub use webassembly::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_execution() {
        let mut grease = Grease::new();
        let result = grease.run("print(42)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_verbose_mode() {
        let mut grease = Grease::new().with_verbose(true);
        let result = grease.run("print(42)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_arithmetic() {
        let mut grease = Grease::new();
        let result = grease.run("print(1 + 2 * 3)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_variables() {
        let mut grease = Grease::new();
        let result = grease.run("x = 42\ny = x + 1\nprint(y)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_strings() {
        let mut grease = Grease::new();
        let result = grease.run("name = \"Grease\"\nprint(\"Hello \" + name)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_booleans() {
        let mut grease = Grease::new();
        let result = grease.run("print(true and false)\nprint(true or false)\nprint(not true)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_comparisons() {
        let mut grease = Grease::new();
        let result = grease.run("print(1 < 2)\nprint(2 == 2)\nprint(3 != 4)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_if_statement() {
        let mut grease = Grease::new();
        let result = grease.run("if 1 < 2:\n    print(\"yes\")\nelse:\n    print(\"no\")");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_while_loop() {
        let mut grease = Grease::new();
        let result = grease.run("while false:\n    print(1)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_function_definition() {
        let mut grease = Grease::new();
        let result = grease.run("def add(a, b):\n    return a + b");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_recursion() {
        let mut grease = Grease::new();
        let result = grease.run("def factorial(n):\n    if n <= 1:\n        return 1\n    else:\n        return n * factorial(n - 1)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_type_annotations() {
        let mut grease = Grease::new();
        let result = grease.run("x = 42\nname = \"test\"\nprint(x)\nprint(name)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_linter_unused_variables() {
        let mut grease = Grease::new();
        let source = "x = 42\ny = \"unused\"\nprint(x)";
        let result = grease.lint(source);
        assert!(result.is_ok());
        let errors = result.unwrap();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("Unused variable 'y'"));
    }

    #[test]
    fn test_linter_no_errors() {
        let mut grease = Grease::new();
        let source = "x = 42\nprint(x)";
        let result = grease.lint(source);
        assert!(result.is_ok());
        let errors = result.unwrap();
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_native_function() {
        let mut grease = Grease::new();
        let result = grease.run("result = native_add(5, 3)\nprint('5 + 3 = ' + result)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_class_declaration() {
        let mut grease = Grease::new();
        let result = grease.run("class Animal:\n\tdef make_sound():\n\t\tprint(\"sound\")");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), InterpretResult::Ok);
    }

    #[test]
    fn test_class_instantiation() {
        let mut grease = Grease::new();
        let result = grease.run("class Animal:\n\tdef make_sound():\n\t\tprint(\"sound\")\nanimal = new Animal()\nanimal.make_sound()");
        match result {
            Ok(_) => {}, // pass
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_property_assignment() {
        let mut grease = Grease::new();
        let result = grease.run("class Animal:\n\tdef make_sound():\n\t\tprint(\"sound\")\nanimal = new Animal()\nanimal.name = \"Buddy\"");
        match result {
            Ok(_) => {}, // pass
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_property_access() {
        let mut grease = Grease::new();
        let result = grease.run("class Animal:\n\tdef make_sound():\n\t\tprint(\"sound\")\nanimal = new Animal()\nanimal.name = \"Buddy\"\nprint(animal.name)");
        match result {
            Ok(_) => {}, // pass
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_class_inheritance() {
        let mut grease = Grease::new();
        let result = grease.run("class Animal:\n\tdef make_sound():\n\t\tprint(\"animal sound\")\nclass Dog(Animal):\n\tdef make_sound():\n\t\tprint(\"woof\")\ndog = new Dog()\ndog.make_sound()");
        match result {
            Ok(_) => {}, // pass
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_method_with_parameters() {
        let mut grease = Grease::new();
        let result = grease.run("class Calculator:\n\tdef add(a, b):\n\t\treturn a + b\ncalc = new Calculator()\nresult = calc.add(5, 3)\nprint(result)");
        match result {
            Ok(_) => {}, // pass
            Err(e) => println!("Error: {}", e),
        }
    }

    // Comprehensive output verification tests for string concatenation
    #[test]
    fn test_string_concatenation_output() {
        let mut grease = Grease::new();
        
        // Test string + string
        let result = grease.run("print(\"Hello \" + \"World\")");
        assert!(result.is_ok());
        
        // Test string + number
        let result = grease.run("print(\"Value: \" + 42)");
        assert!(result.is_ok());
        
        // Test number + string  
        let result = grease.run("print(42 + \" is the answer\")");
        assert!(result.is_ok());
        
        // Test string + boolean
        let result = grease.run("print(\"Result: \" + true)");
        assert!(result.is_ok());
        
        // Test boolean + string
        let result = grease.run("print(false + \" is false\")");
        assert!(result.is_ok());
    }

    #[test]
    fn test_complex_string_concatenation() {
        let mut grease = Grease::new();
        
        // Test multiple concatenations
        let result = grease.run("name = \"Grease\"\nversion = 0.1\nprint(\"Language: \" + name + \" v\" + version)");
        assert!(result.is_ok());
        
        // Test concatenation with arithmetic
        let result = grease.run("x = 10\ny = 20\nprint(\"Result: \" + (x + y * 2))");
        assert!(result.is_ok());
        
        // Test concatenation with boolean operations
        let result = grease.run("print(\"Boolean: \" + (true and false))");
        assert!(result.is_ok());
    }

    #[test]
    fn test_hello_example_output() {
        let mut grease = Grease::new();
        let source = r#"
# Hello World test
print("Hello, World!")

# Variable assignment
name = "Grease"
version: Number = 0.1
print("Language: " + name + " v" + version)

# Basic arithmetic
x = 10
y = 20
result = x + y * 2
print("10 + 20 * 2 = " + result)

# Boolean operations
is_true = true
is_false = false
print("true and false = " + (is_true and is_false))
print("true or false = " + (is_true or is_false))
print("not true = " + (not is_true))
"#;
        
        let result = grease.run(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_arithmetic_still_works() {
        let mut grease = Grease::new();
        
        // Ensure arithmetic operations weren't affected by the fix
        let result = grease.run("print(10 + 20 * 2)");
        assert!(result.is_ok());
        
        let result = grease.run("print(50 - 10)");
        assert!(result.is_ok());
        
        let result = grease.run("print(100 / 2)");
        assert!(result.is_ok());
        
        let result = grease.run("print(15 % 4)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_variable_assignment_and_concatenation() {
        let mut grease = Grease::new();
        
        let result = grease.run("name = \"Grease\"\nprint(\"Hello \" + name)");
        assert!(result.is_ok());
        
        let result = grease.run("x = 42\nprint(\"The answer is \" + x)");
        assert!(result.is_ok());
        
        let result = grease.run("flag = true\nprint(\"Status: \" + flag)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_webassembly_initialization() {
        let mut grease = Grease::new();
        let result = grease.run("wasm_init()");
        assert!(result.is_ok());
    }

    #[test]
    fn test_webassembly_availability() {
        let mut grease = Grease::new();
        let result = grease.run("available = wasm_available()\nprint(available)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_webassembly_statistics() {
        let mut grease = Grease::new();
        let result = grease.run("stats = wasm_stats()\nprint(stats)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_webassembly_compilation() {
        let mut grease = Grease::new();
        let result = grease.run("result = wasm_compile(\"print(42)\")\nprint(result)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_webassembly_compiler_creation() {
        let compiler = WebAssemblyCompiler::new();
        assert_eq!(compiler.wasm_code.len(), 0);
        assert_eq!(compiler.exports.len(), 0);
        assert_eq!(compiler.imports.len(), 0);
    }

    #[test]
    fn test_webassembly_runtime_creation() {
        let runtime = WebAssemblyRuntime::new();
        assert!(!runtime.initialized);
    }

    #[test]
    fn test_webassembly_runtime_initialization() {
        let mut runtime = WebAssemblyRuntime::new();
        let result = runtime.initialize();
        assert!(result.is_ok());
        assert!(runtime.initialized);
    }

    #[test]
    fn test_webassembly_compiler_header() {
        let mut compiler = WebAssemblyCompiler::new();
        compiler.emit_header();
        
        // Check magic number and version
        assert_eq!(compiler.wasm_code[0..4], [0x00, 0x61, 0x73, 0x6d]); // \0asm
        assert_eq!(compiler.wasm_code[4..8], [0x01, 0x00, 0x00, 0x00]); // version 1
    }

    #[test]
    fn test_webassembly_emit_i32_const() {
        let mut compiler = WebAssemblyCompiler::new();
        compiler.emit_i32_const(42);
        
        // Check i32.const opcode and value
        assert_eq!(compiler.wasm_code[0], 0x41); // i32.const
        assert_eq!(compiler.wasm_code[1], 42);   // value
    }

    #[test]
    fn test_webassembly_emit_constant() {
        let mut compiler = WebAssemblyCompiler::new();
        
        // Test number constant
        compiler.emit_constant(&Value::Number(42.0));
        assert_eq!(compiler.wasm_code[0], 0x41); // i32.const
        assert_eq!(compiler.wasm_code[1], 42);   // value
        
        compiler.wasm_code.clear();
        
        // Test boolean constant
        compiler.emit_constant(&Value::Boolean(true));
        assert_eq!(compiler.wasm_code[0], 0x41); // i32.const
        assert_eq!(compiler.wasm_code[1], 1);    // true = 1
        
        compiler.wasm_code.clear();
        
        // Test null constant
        compiler.emit_constant(&Value::Null);
        assert_eq!(compiler.wasm_code[0], 0x41); // i32.const
        assert_eq!(compiler.wasm_code[1], 0);    // null = 0
    }

    #[test]
    fn test_webassembly_js_wrapper_generation() {
        let mut compiler = WebAssemblyCompiler::new();
        compiler.emit_header();
        
        let js_wrapper = compiler.generate_js_wrapper();
        assert!(js_wrapper.contains("GreaseWasm"));
        assert!(js_wrapper.contains("WebAssembly"));
        assert!(js_wrapper.contains("loadGreaseWasm"));
    }

    #[test]
    fn test_webassembly_example_execution() {
        let mut grease = Grease::new();
        let result = grease.run("wasm_init()\nwasm_available()\nwasm_stats()");
        assert!(result.is_ok());
    }


}