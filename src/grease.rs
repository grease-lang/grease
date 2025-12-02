// SPDX-License-Identifier: Apache-2.0

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::{VM, InterpretResult};
use crate::linter::{Linter, LintError};
use std::fs;
use std::path::Path;

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

    pub fn register_native(&mut self, name: &str, arity: usize, function: fn(&mut crate::vm::VM, Vec<crate::bytecode::Value>) -> Result<crate::bytecode::Value, String>) {
        self.vm.register_native(name, arity, function);
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

        // Handle uses before compilation
        self.process_uses(&program)?;

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

    pub fn lint(&mut self, source: &str) -> Result<Vec<LintError>, String> {
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
            eprintln!("🔎 Linting...");
        }
        let mut linter = Linter::new();
        let errors = linter.lint(&program);

        Ok(errors)
    }

    fn process_uses(&mut self, program: &crate::ast::Program) -> Result<(), String> {
        for statement in &program.statements {
            if let crate::ast::Statement::Use { module, alias } = statement {
                self.load_module(module, alias.as_ref())?;
            }
        }
        Ok(())
    }

    fn load_module(&mut self, module_name: &str, alias: Option<&String>) -> Result<(), String> {
        // Check for import cycles (simple check - prevent re-importing the same module)
        let module_key = alias.unwrap_or(&module_name.to_string()).clone();
        if self.vm.modules.contains_key(&module_key) {
            return Err(format!("Circular import detected for module '{}'", module_name));
        }

        // Try to find the module file
        let module_path = format!("{}.grease", module_name);
        let mut paths_to_try = vec![
            module_path.clone(),
            format!("modules/{}", module_path),
        ];

        // Add standard library paths
        if !module_name.contains('/') && !module_name.contains('\\') {
            paths_to_try.push(format!("std/{}.grease", module_name));
        }

        // Handle relative imports
        if module_name.starts_with('.') {
            // For relative imports, we need the current file's directory
            // For now, assume we're in the current working directory
            // In a real implementation, we'd track the importing file's path
            let relative_path = if module_name.starts_with("./") {
                module_name[2..].to_string()
            } else if module_name.starts_with("../") {
                // Go up one directory
                format!("../{}", &module_name[3..])
            } else {
                module_name[1..].to_string() // Remove leading .
            };
            paths_to_try.insert(0, format!("{}.grease", relative_path));
        }

        let mut source = None;
        for path in paths_to_try {
            if Path::new(&path).exists() {
                source = Some(fs::read_to_string(&path).map_err(|e| format!("Failed to read module {}: {}", path, e))?);
                break;
            }
        }

        let source = source.ok_or_else(|| format!("Module '{}' not found. Searched in current directory, modules/, and std/", module_name))?;
        if self.verbose {
            eprintln!("📦 Loading module '{}' from source:\n{}", module_name, source);
        }

        // Parse and execute the module
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let module_program = parser.parse()?;

        // Compile the module
        let mut compiler = Compiler::new();
        let chunk = compiler.compile(&module_program)?.clone();

        // Execute the module in a new VM instance to capture its globals
        let mut module_vm = VM::new();
        let result = module_vm.interpret(chunk);
        if let InterpretResult::RuntimeError(e) = result {
            return Err(format!("Error executing module {}: {}", module_name, e));
        }

        if self.verbose {
            eprintln!("📦 Module '{}' loaded with {} symbols", module_name, module_vm.globals.len());
        }

        // Make the module's globals available
        let module_key = alias.unwrap_or(&module_name.to_string()).clone();
        self.vm.modules.insert(module_key, module_vm.globals.clone());

        Ok(())
    }
}