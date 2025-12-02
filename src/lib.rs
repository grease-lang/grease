pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod bytecode;
pub mod compiler;
pub mod vm;
pub mod repl;
pub mod grease;

pub use token::*;
pub use lexer::*;
pub use ast::*;
pub use parser::*;
pub use bytecode::*;
pub use compiler::*;
pub use vm::*;
pub use repl::*;
pub use grease::*;

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
}