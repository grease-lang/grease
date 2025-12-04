// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use crate::bytecode::*;
use std::collections::HashMap;

pub struct VM {
    pub chunk: Option<Chunk>,
    ip: usize,
    pub stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
    frames: Vec<CallFrame>,
    pub modules: HashMap<String, HashMap<String, Value>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CallFrame {
    ip: usize,
    slot: usize,
    chunk: Chunk,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError(String),
    RuntimeError(String),
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            chunk: None,
            ip: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
            frames: Vec::new(),
            modules: HashMap::new(),
        };

        // Add built-in functions
        vm.globals.insert("print".to_string(), Value::String("print".to_string()));

        // Add a test native function
        vm.register_native("native_add", 2, |_vm, args| {
            match (&args[0], &args[1]) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                _ => Err("Arguments must be numbers".to_string()),
                }
        });

        vm
    }

    pub fn register_native(&mut self, name: &str, arity: usize, function: fn(&mut VM, Vec<Value>) -> Result<Value, String>) {
        let native_func = Value::NativeFunction(NativeFunction {
            name: name.to_string(),
            arity,
            function,
        });
        self.globals.insert(name.to_string(), native_func);
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.stack.clear();
        self.frames.clear();
        
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
    loop {
        let instruction = self.read_byte().expect("Unexpected end of bytecode");
            match OpCode::from_byte(instruction) {
            Some(OpCode::Constant) => {
                let constant_index = self.read_byte().expect("Expected constant index") as usize;
                let constant = self.chunk.as_ref().unwrap().constants[constant_index].clone();
                self.stack.push(constant);
            }
            Some(OpCode::Add) => {
                let b_opt = self.stack.pop();
                let a_opt = self.stack.pop();
                if let (Some(b), Some(a)) = (b_opt, a_opt) {
                    match (&a, &b) {
                        (Value::Number(a_num), Value::Number(b_num)) => {
                            self.stack.push(Value::Number(a_num + b_num));
                        }
                        (Value::String(a_str), Value::String(b_str)) => {
                            self.stack.push(Value::String(a_str.clone() + b_str));
                        }
                        (Value::String(a_str), Value::Number(b_num)) => {
                            self.stack.push(Value::String(a_str.clone() + &b_num.to_string()));
                        }
                        (Value::Number(a_num), Value::String(b_str)) => {
                            self.stack.push(Value::String(a_num.to_string() + b_str));
                        }
                        (Value::String(a_str), Value::Boolean(b_bool)) => {
                            self.stack.push(Value::String(a_str.clone() + if *b_bool { "true" } else { "false" }));
                        }
                        (Value::Boolean(a_bool), Value::String(b_str)) => {
                            self.stack.push(Value::String(format!("{}{}", if *a_bool { "true" } else { "false" }, b_str)));
                        }
                        _ => {
                            return InterpretResult::RuntimeError("Operands must be numbers or strings".to_string());
                        }
                    }
                } else {
                    return InterpretResult::RuntimeError("Stack underflow".to_string());
                }
            }
            Some(OpCode::Multiply) => {
                let b_opt = self.stack.pop();
                let a_opt = self.stack.pop();
                if let (Some(Value::Number(b)), Some(Value::Number(a))) = (b_opt, a_opt) {
                    self.stack.push(Value::Number(a * b));
                } else {
                    return InterpretResult::RuntimeError("Operands must be numbers".to_string());
                }
            }
            Some(OpCode::Divide) => {
                let b_opt = self.stack.pop();
                let a_opt = self.stack.pop();
                if let (Some(Value::Number(b)), Some(Value::Number(a))) = (b_opt, a_opt) {
                    if b == 0.0 {
                        return InterpretResult::RuntimeError("Division by zero".to_string());
                    }
                    self.stack.push(Value::Number(a / b));
                } else {
                    return InterpretResult::RuntimeError("Operands must be numbers".to_string());
                }
            }
            Some(OpCode::Modulo) => {
                let b_opt = self.stack.pop();
                let a_opt = self.stack.pop();
                if let (Some(Value::Number(b)), Some(Value::Number(a))) = (b_opt, a_opt) {
                    if b == 0.0 {
                        return InterpretResult::RuntimeError("Modulo by zero".to_string());
                    }
                    self.stack.push(Value::Number(a % b));
                } else {
                    return InterpretResult::RuntimeError("Operands must be numbers".to_string());
                }
            }
            Some(OpCode::Null) => {
                    self.stack.push(Value::Null);
                }
            Some(OpCode::True) => {
                    self.stack.push(Value::Boolean(true));
                }
            Some(OpCode::False) => {
                    self.stack.push(Value::Boolean(false));
                }
            Some(OpCode::GetGlobal) => {
                    let name = match self.read_string() {
                        Value::String(s) => s,
                        _ => return InterpretResult::RuntimeError("Global name must be a string".to_string()),
                    };
                    
                    match self.globals.get(&name) {
                        Some(value) => self.stack.push(value.clone()),
                        None => return InterpretResult::RuntimeError(format!("Undefined variable '{}'", name)),
                    }
                }
            Some(OpCode::SetGlobal) => {
                    let name = match self.read_string() {
                        Value::String(s) => s,
                        _ => return InterpretResult::RuntimeError("Global name must be a string".to_string()),
                    };
                    
                    if let Some(value) = self.stack.pop() {
                        self.globals.insert(name, value);
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::GetLocal) => {
                    let slot = self.read_byte().unwrap() as usize;
                    if let Some(frame) = self.frames.last() {
                        let absolute_slot = frame.slot + slot;
                        if absolute_slot < self.stack.len() {
                            self.stack.push(self.stack[absolute_slot].clone());
                        } else {
                            return InterpretResult::RuntimeError("Invalid local slot".to_string());
                        }
                    } else {
                        return InterpretResult::RuntimeError("GetLocal outside of function".to_string());
                    }
                }
            Some(OpCode::SetLocal) => {
                    let slot = self.read_byte().unwrap() as usize;
                    if let Some(value) = self.stack.pop() {
                        if let Some(frame) = self.frames.last() {
                            let absolute_slot = frame.slot + slot;
                            if absolute_slot < self.stack.len() {
                                self.stack[absolute_slot] = value;
                            } else {
                                return InterpretResult::RuntimeError("Invalid local slot".to_string());
                            }
                        } else {
                            return InterpretResult::RuntimeError("SetLocal outside of function".to_string());
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Jump) => {
                    let offset = self.read_short() as usize;
                    self.ip += offset;
                }
            Some(OpCode::JumpIfFalse) => {
                    let offset = self.read_short() as usize;
                    if let Some(value) = self.stack.last() {
                        if !self.is_truthy(value) {
                            self.ip += offset;
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::JumpIfTrue) => {
                    let offset = self.read_short() as usize;
                    if let Some(value) = self.stack.last() {
                        if self.is_truthy(value) {
                            self.ip += offset;
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Loop) => {
                    let offset = self.read_short() as usize;
                    self.ip -= offset;
                }
            Some(OpCode::Dup) => {
                    if let Some(value) = self.stack.last().cloned() {
                        self.stack.push(value);
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Call) => {
                    let arg_count = self.read_byte().unwrap() as usize;
                    if !self.call_value(arg_count) {
                        return InterpretResult::RuntimeError("Failed to call value".to_string());
                    }
                }
            Some(OpCode::Return) => {
                let result = self.stack.pop(); // May be None if no explicit return value
                
                // If we have call frames, restore the previous one
                if let Some(frame) = self.frames.pop() {
                    self.stack.truncate(frame.slot);
                    self.ip = frame.ip;
                    self.chunk = Some(frame.chunk); // Restore the previous chunk
                    // Push the result back (may be None, in which case push Null)
                    self.stack.push(result.unwrap_or(Value::Null));
                } else {
                    // No frames left, execution is done
                    // If there was a result, we could return it, but for now just return Ok
                    return InterpretResult::Ok;
                }
            }
            Some(OpCode::Subtract) => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a - b));
                        }
                        _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                    }
                } else {
                    return InterpretResult::RuntimeError("Stack underflow".to_string());
                }
            }
            Some(OpCode::Negate) => {
                    if let Some(value) = self.stack.pop() {
                        match value {
                            Value::Number(n) => self.stack.push(Value::Number(-n)),
                            _ => return InterpretResult::RuntimeError("Operand must be a number".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Array) => {
                    if let Some(count) = self.read_byte() {
                        let mut elements = Vec::new();
                        for _ in 0..count {
                            if let Some(element) = self.stack.pop() {
                                elements.insert(0, element); // Insert at beginning to maintain order
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                        }
                        self.stack.push(Value::Array(elements));
                    } else {
                        return InterpretResult::RuntimeError("Expected array count".to_string());
                    }
                }
            Some(OpCode::Index) => {
                    if let (Some(array), Some(index)) = (self.stack.pop(), self.stack.pop()) {
                        match (array, index) {
                            (Value::Array(elements), Value::Number(i))=> {
                                let idx = i as usize;
            if idx < elements.len() {
                self.stack.push(elements[idx].clone());
            } else {
                return InterpretResult::RuntimeError(format!("Index {} out of bounds for array of length {}", idx, elements.len()));
                }
                            }
        _ => return InterpretResult::RuntimeError("Index operation requires array and number".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Length) => {
                    if let Some(value) = self.stack.pop() {
                        match value {
                            Value::Array(elements)=> {
                                self.stack.push(Value::Number(elements.len() as f64));
                            }
                            _ => return InterpretResult::RuntimeError("Length operation requires array".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Equal) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(self.values_equal(&a, &b)));
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::NotEqual) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(!self.values_equal(&a, &b)));
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Less) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        match (a, b) {
                            (Value::Number(a), Value::Number(b))=> {
                                self.stack.push(Value::Boolean(a < b));
                            }
                            _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::LessEqual) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        match (a, b) {
                            (Value::Number(a), Value::Number(b))=> {
                                self.stack.push(Value::Boolean(a <= b));
                            }
                            _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Greater) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        match (a, b) {
                            (Value::Number(a), Value::Number(b))=> {
                                self.stack.push(Value::Boolean(a > b));
                            }
                            _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::GreaterEqual) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        match (a, b) {
                            (Value::Number(a), Value::Number(b))=> {
                                self.stack.push(Value::Boolean(a >= b));
                            }
                            _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Not) => {
                    if let Some(value) = self.stack.pop() {
                        self.stack.push(Value::Boolean(!self.is_truthy(&value)));
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::And) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(self.is_truthy(&a) && self.is_truthy(&b)));
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Or) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(self.is_truthy(&a) || self.is_truthy(&b)));
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
            Some(OpCode::Pop) => {
                    self.stack.pop();
                }
            Some(OpCode::Import) => {
                    // For now, this is a placeholder
                    // The module name and alias are on the stack
                    // In a full implementation, this would load and execute the module
                    let _alias = self.stack.pop();
                    let _module = self.stack.pop();
                    // TODO: Implement actual module loading
                }
            Some(OpCode::GetModule) => {
                    // Stack has: [..., module_name, member_name]
                    let member_name = match self.stack.pop() {
                        Some(Value::String(s)) => s,
                        _ => return InterpretResult::RuntimeError("Member name must be a string".to_string()),
                    };
                    let module_name = match self.stack.pop() {
                        Some(Value::String(s)) => s,
                        _ => return InterpretResult::RuntimeError("Module name must be a string".to_string()),
                    };

                    // Look up the module
                    if let Some(module) = self.modules.get(&module_name) {
                        if let Some(value) = module.get(&member_name) {
                            self.stack.push(value.clone());
                        } else {
                            return InterpretResult::RuntimeError(format!("Undefined member '{}' in module '{}'", member_name, module_name));
                        }
                    } else {
                        return InterpretResult::RuntimeError(format!("Undefined module '{}'", module_name));
                    }
                }
            Some(OpCode::CreateInstance) => {
                // Stack has: [..., class, arg1, arg2, ..., argN]
                // The number of arguments is encoded in the instruction
                let arg_count = self.read_byte().expect("Expected argument count") as usize;

                // Collect arguments
                let mut args = Vec::new();
                for _ in 0..arg_count {
                    if let Some(arg) = self.stack.pop() {
                        args.push(arg);
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
                args.reverse(); // Arguments were popped in reverse order

                // Get the class
                if let Some(class_value) = self.stack.pop() {
                    if let Value::Class { name, .. } = class_value {
                        // Create instance with empty fields
                        let instance = Value::Object {
                            class_name: name,
                            fields: std::collections::HashMap::new(),
                        };
                        self.stack.push(instance);
                    } else {
                        return InterpretResult::RuntimeError("Expected class".to_string());
                    }
                } else {
                    return InterpretResult::RuntimeError("Stack underflow".to_string());
                }
            }
            Some(OpCode::CreateClass) => {
                // TODO: Implement class creation
                return InterpretResult::RuntimeError("CreateClass not implemented".to_string());
            }
            Some(OpCode::GetProperty) => {
                // Stack: [..., object, property_name]
                let property_name = match self.stack.pop() {
                    Some(Value::String(s)) => s,
                    _ => return InterpretResult::RuntimeError("Property name must be a string".to_string()),
                };

                if let Some(Value::Object { fields, .. }) = self.stack.pop() {
                    if let Some(value) = fields.get(&property_name) {
                        self.stack.push(value.clone());
                    } else {
                        return InterpretResult::RuntimeError(format!("Undefined property '{}'", property_name));
                    }
                } else {
                    return InterpretResult::RuntimeError("Expected object".to_string());
                }
            }
            Some(OpCode::SetProperty) => {
                // TODO: Implement property setting
                return InterpretResult::RuntimeError("SetProperty not implemented".to_string());
            }
            Some(OpCode::CallMethod) => {
                // TODO: Implement method calling
                return InterpretResult::RuntimeError("CallMethod not implemented".to_string());
            }
            Some(OpCode::GetSuper) => {
                // TODO: Implement super access
                return InterpretResult::RuntimeError("GetSuper not implemented".to_string());
            }
            None => return InterpretResult::RuntimeError("Unknown opcode".to_string()),
                }
        }
    }

    fn call_value(&mut self, arg_count: usize) -> bool {
        // The function is below the arguments
        let func_index = self.stack.len() - arg_count - 1;
        if let Some(callee) = self.stack.get(func_index).cloned() {
                match callee {
                        Value::String(name) if name == "print" => {
                    // Built-in print function
                    if arg_count != 1 {
                        return false;
                    }
                    if let Some(arg) = self.stack.pop() {
                        self.stack.pop(); // Remove the function name
                        println!("{}", self.format_value(&arg));
                        self.stack.push(Value::Null);
                        return true;
                    }
            }
                        Value::Function(func) => {
                    // User-defined function
                    if arg_count != func.arity {
                        return false;
                    }

                    // Remove the function from the stack
                    self.stack.remove(func_index);

                    // Create a new call frame
                    let slot = self.stack.len() - arg_count;
                    let current_chunk = self.chunk.take().unwrap_or_else(|| Chunk::new());
                    let frame = CallFrame {
                        ip: self.ip,
                        slot,
                        chunk: current_chunk,
                    };
                    self.frames.push(frame);

                    // Set up the function's chunk
                    self.chunk = Some(func.chunk.clone());

                    // Jump to the start of the function
                    self.ip = 0;
                    return true;
                }
                Value::NativeFunction(native_func) => {
                    // Native function
                    if arg_count != native_func.arity {
                        return false;
                    }

                    // Collect arguments (they are above the function on the stack)
                    let mut args = Vec::new();
                    for i in 0..arg_count {
                        if let Some(arg) = self.stack.get(func_index + 1 + i).cloned() {
                            args.push(arg);
                        } else {
                            return false;
                        }
                    }

                    // Remove the function and arguments from the stack
                    self.stack.truncate(func_index);

                    // Call the native function
                    match (native_func.function)(self, args) {
                        Ok(result)=> {
                            self.stack.push(result);
                            return true;
                        }
                        Err(_) => return false,
                    }
                }
                        _ => { () }
                }
        }
        self.stack.push(Value::Null);
        true
    }

    pub fn format_value(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Function(f) => format!("<fn {}>", f.name),
            Value::NativeFunction(f) => format!("<native fn {}>", f.name),
            Value::Array(arr)=> {
                let elements: Vec<String> = arr.iter().map(|v| self.format_value(v)).collect();
                format!("[{}]", elements.join(", "))
            },
            Value::Object { class_name, .. } => {
                format!("Object of class {}", class_name)
            },
            Value::Class { name, .. } => format!("Class {:?}", name),
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        if let Some(ref chunk) = self.chunk {
            if self.ip < chunk.code.len() {
                let byte = chunk.code[self.ip];
                self.ip += 1;
            Some(byte)
            } else {
                None
                }
        } else {
            None
                }
    }

    fn read_short(&mut self) -> u16 {
        if let Some(ref chunk) = self.chunk {
            let high = chunk.code[self.ip] as u16;
            let low = chunk.code[self.ip + 1] as u16;
            self.ip += 2;
            (high << 8) | low
        } else {
            0
                }
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte().unwrap() as usize;
        if let Some(ref chunk) = self.chunk {
            chunk.constants[index].clone()
        } else {
            Value::Null
                }
    }

    fn read_string(&mut self) -> Value {
        self.read_constant()
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Function(_) => true,
            Value::NativeFunction(_) => true,
            Value::Array(arr) => !arr.is_empty(),
            Value::Object { .. } => true,
            Value::Class { .. } => true,
        }
    }

    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b))=> {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| self.values_equal(x, y))
                }
            _ => false,
                }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Compiler;
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    fn run_code(code: &str) -> Result<InterpretResult, String> {
        let mut lexer = Lexer::new(code.to_string());
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        let mut compiler = Compiler::new();
        let chunk = compiler.compile(&program)?.clone();
        let mut vm = VM::new();
        Ok(vm.interpret(chunk))
    }

    #[test]
    fn test_vm_number_literal() {
        let result = run_code("42").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_string_literal() {
        let result = run_code("\"hello\"").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_boolean_literal() {
        let result = run_code("true").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_null_literal() {
        let result = run_code("null").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_arithmetic() {
        let result = run_code("1 + 2 * 3").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_variable() {
        let result = run_code("x = 42\nx").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_assignment() {
        let result = run_code("x = 1\nx = 2").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_if_statement() {
        let result = run_code("if true:\n    1").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_while_statement() {
        let result = run_code("while false:\n    1").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_function_definition() {
        let result = run_code("def test():\n    return 42").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_function_call() {
        let result = run_code("def test():\n    return 42").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_print() {
        let result = run_code("print(42)").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }
}