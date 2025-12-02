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
}