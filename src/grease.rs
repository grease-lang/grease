use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::{VM, InterpretResult};

pub struct Grease {
    pub vm: VM,
    pub verbose: bool,
}

impl Grease {
    pub fn new() -> Self {
        Grease {
            vm: VM::new(),
            verbose: false,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn run(&mut self, source: &str) -> Result<InterpretResult, String> {
        if self.verbose {
            eprintln!("🔍 Lexical analysis...");
        }
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize()?;

        if self.verbose {
            eprintln!("📝 Parsing...");
        }
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;

        if self.verbose {
            eprintln!("⚙️  Compilation...");
        }
        let mut compiler = Compiler::new();
        let chunk = compiler.compile(&program)?.clone();

        if self.verbose {
            eprintln!("🚀 Interpretation...");
        }
        let result = self.vm.interpret(chunk);
        Ok(result)
    }
}