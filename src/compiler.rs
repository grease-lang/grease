// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use crate::ast::*;
use crate::bytecode::*;
use crate::token::{Token, TokenType};

pub struct Compiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
}

#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
        }
    }

    pub fn compile(&mut self, program: &Program) -> Result<&Chunk, String> {
        for statement in &program.statements {
            self.compile_statement(statement)?;
        }
        
        self.emit_return();
        Ok(&self.chunk)
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
                // Don't pop for now - this needs more thought about REPL vs script mode
                // self.emit_byte(OpCode::Pop);
            }
            Statement::VariableDeclaration { name, type_annotation: _, initializer } => {
                if let Some(initializer) = initializer {
                    self.compile_expression(initializer)?;
                } else {
                    self.emit_byte(OpCode::Null);
                }
                
                self.declare_variable(&name)?;
                self.define_variable(&name)?;
            }
            Statement::FunctionDeclaration { name, parameters, return_type: _, body } => {
                self.declare_variable(&name)?;
                self.mark_initialized();
                
                let function = self.compile_function(name, parameters, body)?;
                let constant = self.chunk.add_constant(Value::Function(function));
                self.emit_bytes(OpCode::Constant, constant as u8);
                
                self.define_variable(&name)?;
            }
            Statement::Return { value } => {
                if let Some(value) = value {
                    self.compile_expression(value)?;
                } else {
                    self.emit_byte(OpCode::Null);
                }
                self.emit_byte(OpCode::Return);
            }
            Statement::If { condition, then_branch, else_branch } => {
                self.compile_expression(condition)?;
                
                let else_jump = self.emit_jump(OpCode::JumpIfFalse);
                self.emit_byte(OpCode::Pop);
                
                self.compile_block(then_branch)?;
                
                let else_jump_2 = self.emit_jump(OpCode::Jump);
                self.patch_jump(else_jump);
                self.emit_byte(OpCode::Pop);
                
                if let Some(else_branch) = else_branch {
                    self.compile_block(else_branch)?;
                }
                
                self.patch_jump(else_jump_2);
            }
            Statement::While { condition, body } => {
                let loop_start = self.chunk.code.len();
                
                self.compile_expression(condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse);
                self.emit_byte(OpCode::Pop); // Pop condition result
                
                self.compile_block(body)?;
                
                self.emit_loop(loop_start);
                self.patch_jump(exit_jump);
                // No pop here - condition was already popped before loop body
            }
            Statement::For { variable, iterable, body } => {
                self.compile_expression(iterable)?;
                
                self.declare_variable(variable)?;
                self.define_variable(variable)?; // array in variable
                
                let variable_name = match &variable.token_type {
                    TokenType::Identifier(name) => name,
                    _ => return Err("Expected identifier".to_string()),
                };
                
                let zero_constant = self.chunk.add_constant(Value::Number(0.0));
                self.emit_bytes(OpCode::Constant, zero_constant as u8); // index
                let name_constant = self.chunk.add_constant(Value::String(variable_name.clone()));
                self.emit_bytes(OpCode::GetGlobal, name_constant as u8); // get array
                self.emit_byte(OpCode::Length); // length
                
                let loop_start = self.chunk.code.len();
                
                self.emit_byte(OpCode::Dup); // dup index
                self.emit_bytes(OpCode::GetGlobal, name_constant as u8); // get array
                self.emit_byte(OpCode::Index); // element
                self.emit_bytes(OpCode::SetGlobal, name_constant as u8); // set variable to element
                
                self.compile_block(body)?;
                
                let one_constant = self.chunk.add_constant(Value::Number(1.0));
                self.emit_bytes(OpCode::Constant, one_constant as u8);
                self.emit_byte(OpCode::Add); // index + 1
                self.emit_byte(OpCode::Dup); // dup new index
                self.emit_byte(OpCode::Dup); // dup length
                self.emit_byte(OpCode::Less); // index < length
                let jump = self.emit_jump(OpCode::JumpIfTrue);
                let offset = loop_start as i16 - (jump as i16 + 2);
                self.chunk.code[jump] = (offset >> 8) as u8;
                self.chunk.code[jump + 1] = (offset & 0xff) as u8;
                
                self.emit_byte(OpCode::Pop); // pop index+1
                self.emit_byte(OpCode::Pop); // pop length
                self.emit_byte(OpCode::Pop); // pop index
            }
            Statement::Block(statements) => {
                self.begin_scope();
                for statement in statements {
                    self.compile_statement(statement)?;
                }
                self.end_scope();
            }
            Statement::Use { module: _, alias: _ } => {
                // Imports are handled at the Grease level, not compiled to bytecode
            }
            Statement::ClassDeclaration { name, superclass, methods } => {
                // Compile class definition
                // Store methods in a class object
                let mut method_map = std::collections::HashMap::new();
                for method in methods {
                    if let Statement::FunctionDeclaration { name: method_name, parameters, return_type: _, body } = method {
                        let function = self.compile_function(&method_name, &parameters, &body)?;
                        let method_constant = self.chunk.add_constant(Value::Function(function));
                        method_map.insert(method_name.lexeme.clone(), method_constant);
                    }
                }
                
                // Create class object
                let class_value = Value::Class {
                    name: name.lexeme.clone(),
                    methods: method_map,
                    superclass: superclass.as_ref().map(|s| s.lexeme.clone()),
                };
                let class_constant = self.chunk.add_constant(class_value);
                
                // Define class as global
                self.emit_bytes(OpCode::Constant, class_constant as u8);
                 self.declare_variable(&name)?;
                 self.define_variable(&name)?;
             }
             Statement::Try { try_block, catch_block } => {
                 // For now, just compile try block and ignore catch
                 // TODO: Implement proper exception handling with jumps
                 self.begin_scope();
                 for stmt in try_block {
                     self.compile_statement(stmt)?;
                 }
                 self.end_scope();
                 
                 // Compile catch block
                 self.begin_scope();
                 for stmt in catch_block {
                     self.compile_statement(stmt)?;
                 }
                 self.end_scope();
             }
             Statement::Throw { value } => {
                 if let Some(val) = value {
                     self.compile_expression(val)?;
                 } else {
                     self.emit_byte(OpCode::Null);
                 }
                 self.emit_byte(OpCode::Throw);
             }
             Statement::RustInline { code } => {
                 let constant = self.chunk.add_constant(Value::String(code.clone()));
                 self.emit_bytes(OpCode::RustInline, constant as u8);
             }
             Statement::AsmInline { code } => {
                 let constant = self.chunk.add_constant(Value::String(code.clone()));
                 self.emit_bytes(OpCode::AsmInline, constant as u8);
             }
        }
        
        Ok(())
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<(), String> {
        match expression {
            Expression::Number(value) => {
                let constant = self.chunk.add_constant(Value::Number(*value));
                self.emit_bytes(OpCode::Constant, constant as u8);
            }
            Expression::String(value) => {
                let constant = self.chunk.add_constant(Value::String(value.clone()));
                self.emit_bytes(OpCode::Constant, constant as u8);
            }
            Expression::Boolean(value) => {
                if *value {
                    self.emit_byte(OpCode::True);
                } else {
                    self.emit_byte(OpCode::False);
                }
            }
            Expression::Null => {
                self.emit_byte(OpCode::Null);
            }
            Expression::Identifier(ref token) => {
                if let TokenType::Identifier(ref name) = token.token_type {
                    if let Some(local) = self.resolve_local(name) {
                        self.emit_bytes(OpCode::GetLocal, local as u8);
                    } else {
                        let constant = self.chunk.add_constant(Value::String(name.clone()));
                        self.emit_bytes(OpCode::GetGlobal, constant as u8);
                    }
                } else {
                    // Should not happen
                    panic!("Identifier token is not Identifier type");
                }
            }
            Expression::Binary { left, operator, right } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                
                match operator.token_type {
                    TokenType::Plus => self.emit_byte(OpCode::Add),
                    TokenType::Minus => self.emit_byte(OpCode::Subtract),
                    TokenType::Multiply => self.emit_byte(OpCode::Multiply),
                    TokenType::Divide => self.emit_byte(OpCode::Divide),
                    TokenType::Modulo => self.emit_byte(OpCode::Modulo),
                    TokenType::Equal => self.emit_byte(OpCode::Equal),
                    TokenType::NotEqual => self.emit_byte(OpCode::NotEqual),
                    TokenType::Less => self.emit_byte(OpCode::Less),
                    TokenType::LessEqual => self.emit_byte(OpCode::LessEqual),
                    TokenType::Greater => self.emit_byte(OpCode::Greater),
                    TokenType::GreaterEqual => self.emit_byte(OpCode::GreaterEqual),
                    TokenType::And => self.emit_byte(OpCode::And),
                    TokenType::Or => self.emit_byte(OpCode::Or),
                    _ => return Err(format!("Unknown binary operator: {:?}", operator.token_type)),
                }
            }
            Expression::Unary { operator, right } => {
                self.compile_expression(right)?;
                
                match operator.token_type {
                    TokenType::Minus => self.emit_byte(OpCode::Negate),
                    TokenType::Not => self.emit_byte(OpCode::Not),
                    _ => return Err(format!("Unknown unary operator: {:?}", operator.token_type)),
                }
            }
            Expression::Assignment { name, value } => {
                self.compile_expression(value)?;
                
                let variable_name = match &name.token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err("Expected identifier in assignment".to_string()),
                };
                
                if let Some(local) = self.resolve_local(&variable_name) {
                    self.emit_bytes(OpCode::SetLocal, local as u8);
                } else {
                    let constant = self.chunk.add_constant(Value::String(variable_name));
                    self.emit_bytes(OpCode::SetGlobal, constant as u8);
                }
            }
            Expression::PropertyAssignment { object, property, value } => {
                self.compile_expression(object)?;
                let property_constant = self.chunk.add_constant(Value::String(property.lexeme.clone()));
                self.emit_bytes(OpCode::Constant, property_constant as u8);
                self.compile_expression(value)?;
                self.emit_byte(OpCode::SetProperty);
            }
            Expression::Call { callee, arguments } => {
                self.compile_expression(callee)?;


                for arg in arguments {
                    self.compile_expression(arg)?;
                }

                self.emit_bytes(OpCode::Call, arguments.len() as u8);
            }
            Expression::ModuleAccess { module, member } => {
                // Get module name
                let module_name = match &module.token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err("Expected identifier for module".to_string()),
                };

                // Get member name
                let member_name = match &member.token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err("Expected identifier for member".to_string()),
                };

                // Emit module name as constant
                let module_constant = self.chunk.add_constant(Value::String(module_name));
                self.emit_bytes(OpCode::Constant, module_constant as u8);

                // Emit member name as constant
                let member_constant = self.chunk.add_constant(Value::String(member_name));
                self.emit_bytes(OpCode::Constant, member_constant as u8);

                // Emit get module opcode
                self.emit_byte(OpCode::GetModule);
            }
            Expression::Grouping(expr) => {
                self.compile_expression(expr)?;
            }
            Expression::Array(elements) => {
                for element in elements {
                    self.compile_expression(element)?;
                }
                self.emit_bytes(OpCode::Array, elements.len() as u8);
            }
            Expression::Dictionary(pairs) => {
                for (key, value) in pairs {
                    self.compile_expression(key)?;
                    self.compile_expression(value)?;
                }
                self.emit_bytes(OpCode::Dictionary, pairs.len() as u8);
            }
            Expression::Index { array, index } => {
                self.compile_expression(array)?;
                self.compile_expression(index)?;
                self.emit_byte(OpCode::Index);
            }
            Expression::NewInstance { class, arguments } => {
                self.compile_expression(class)?;
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                self.emit_bytes(OpCode::CreateInstance, arguments.len() as u8);
            }
            Expression::PropertyAccess { object, property } => {
                self.compile_expression(object)?;
                let property_constant = self.chunk.add_constant(Value::String(property.lexeme.clone()));
                self.emit_bytes(OpCode::Constant, property_constant as u8);
                self.emit_byte(OpCode::GetProperty);
            }
            Expression::MethodCall { object, method, arguments } => {
                self.compile_expression(object)?;
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                    let method_constant = self.chunk.add_constant(Value::String(method.lexeme.clone()));
                self.emit_bytes(OpCode::Constant, method_constant as u8);
                self.emit_bytes(OpCode::CallMethod, arguments.len() as u8);
            }
            Expression::SuperCall { method, arguments } => {
                // For super(), resolve from class hierarchy
                if let Some(method) = method {
                let method_constant = self.chunk.add_constant(Value::String(method.lexeme.clone()));
                    self.emit_bytes(OpCode::Constant, method_constant as u8);
                } else {
                    self.emit_byte(OpCode::Null); // No method for constructor
                }
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                self.emit_bytes(OpCode::GetSuper, arguments.len() as u8);
            }
            Expression::RustInline { code } => {
                let constant = self.chunk.add_constant(Value::String(code.clone()));
                self.emit_bytes(OpCode::RustInline, constant as u8);
            }
            Expression::AsmInline { code } => {
                let constant = self.chunk.add_constant(Value::String(code.clone()));
                self.emit_bytes(OpCode::AsmInline, constant as u8);
            }
        }
        
        Ok(())
    }

    fn compile_function(&mut self, name: &Token, parameters: &Vec<(Token, Option<String>)>, body: &Vec<Statement>) -> Result<Function, String> {
        let mut compiler = Compiler::new();
        compiler.begin_scope();
        
        // Add parameters as locals
        for (param, _) in parameters {
            let _param_name = match &param.token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err("Expected parameter name".to_string()),
            };
            compiler.declare_variable(param)?;
            compiler.define_variable(param)?;
        }
        
        for statement in body {
            compiler.compile_statement(statement)?;
        }
        
        compiler.end_scope();
        compiler.emit_return();
        
        let function_name = match &name.token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => "anonymous".to_string(),
        };
        
        Ok(Function {
            name: function_name,
            arity: parameters.len(),
            chunk: compiler.chunk.clone(),
        })
    }

    fn compile_block(&mut self, statements: &Vec<Statement>) -> Result<(), String> {
        self.begin_scope();
        for statement in statements {
            self.compile_statement(statement)?;
        }
        self.end_scope();
        Ok(())
    }

    fn declare_variable(&mut self, name: &Token) -> Result<(), String> {
        if self.scope_depth > 0 {
            let variable_name = match &name.token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err("Expected identifier".to_string()),
            };
            
            // Check if variable already exists in current scope
            for local in self.locals.iter().rev() {
                if local.depth < self.scope_depth {
                    break;
                }
                if local.name == variable_name {
                    return Err("Variable already declared in this scope".to_string());
                }
            }
            
            self.add_local(variable_name);
        }
        Ok(())
    }

    fn define_variable(&mut self, name: &Token) -> Result<(), String> {
        if self.scope_depth > 0 {
            self.mark_initialized();
            Ok(())
        } else {
            let variable_name = match &name.token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err("Expected identifier".to_string()),
            };
            let constant = self.chunk.add_constant(Value::String(variable_name));
            self.emit_bytes(OpCode::SetGlobal, constant as u8);
            Ok(())
        }
    }

    fn add_local(&mut self, name: String) {
        self.locals.push(Local {
            name,
            depth: self.scope_depth,
        });
    }

    fn mark_initialized(&mut self) {
        if let Some(local) = self.locals.last_mut() {
            local.depth = self.scope_depth;
        }
    }

    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                return Some(i);
            }
        }
        None
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        
        while let Some(local) = self.locals.last() {
            if local.depth > self.scope_depth {
                self.emit_byte(OpCode::Pop);
                self.locals.pop();
            } else {
                break;
            }
        }
    }

    fn emit_byte(&mut self, byte: OpCode) {
        self.chunk.write(byte.to_byte(), 0);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: u8) {
        self.emit_byte(byte1);
        self.chunk.write(byte2, 0);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.emit_byte(instruction);
        self.chunk.write(0, 0);
        self.chunk.write(0, 0);
        self.chunk.code.len() - 3
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code.len() - offset - 3;
        
        if jump > u16::MAX as usize {
            panic!("Too much code to jump over");
        }
        
        self.chunk.code[offset + 1] = ((jump >> 8) & 0xff) as u8;
        self.chunk.code[offset + 2] = (jump & 0xff) as u8;
    }

    fn emit_loop(&mut self, loop_start: usize) {
        let offset = self.chunk.code.len() - loop_start + 2;
        
        if offset > u16::MAX as usize {
            panic!("Loop body too large");
        }
        
        self.emit_byte(OpCode::Loop);
        self.chunk.write(((offset >> 8) & 0xff) as u8, 0);
        self.chunk.write((offset & 0xff) as u8, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    fn compile_code(code: &str) -> Result<Chunk, String> {
        let mut lexer = Lexer::new(code.to_string());
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        let mut compiler = Compiler::new();
        compiler.compile(&program).map(|chunk| chunk.clone())
    }

    #[test]
    fn test_compile_expression() {
        let chunk = compile_code("42").unwrap();
        // Check that bytecode was generated
        assert!(!chunk.code.is_empty());
    }

    #[test]
    fn test_compile_variable_declaration() {
        let chunk = compile_code("let x = 42").unwrap();
        assert!(!chunk.code.is_empty());
    }

    #[test]
    fn test_compile_assignment() {
        let chunk = compile_code("let x = 1\nx = 2").unwrap();
        assert!(!chunk.code.is_empty());
    }

    #[test]
    fn test_compile_binary_expression() {
        let chunk = compile_code("1 + 2").unwrap();
        assert!(!chunk.code.is_empty());
    }

    #[test]
    fn test_compile_if_statement() {
        let chunk = compile_code("if true:\n    1").unwrap();
        assert!(!chunk.code.is_empty());
    }

    #[test]
    fn test_compile_while_statement() {
        let chunk = compile_code("while true:\n    1").unwrap();
        assert!(!chunk.code.is_empty());
    }

    #[test]
    fn test_compile_function() {
        let chunk = compile_code("def test():\n    return 1").unwrap();
        assert!(!chunk.code.is_empty());
    }

    #[test]
    fn test_compile_call() {
        let chunk = compile_code("print(42)").unwrap();
        assert!(!chunk.code.is_empty());
    }

    #[test]
    fn test_compile_use() {
        let chunk = compile_code("use math").unwrap();
        // Use statements don't emit opcodes since they're processed before compilation
        assert_eq!(chunk.code, vec![13]);
    }

    #[test]
    fn test_compile_use_with_alias() {
        let chunk = compile_code("use math as m").unwrap();
        // Use statements don't emit opcodes since they're processed before compilation
        assert_eq!(chunk.code, vec![13]);
    }


}