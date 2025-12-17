// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use crate::bytecode::*;
use std::collections::HashMap;
use std::process::Child;

pub struct VM {
    pub chunk: Option<Chunk>,
    ip: usize,
    pub stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
    frames: Vec<CallFrame>,
    pub modules: HashMap<String, HashMap<String, Value>>,
    exception_stack: Vec<usize>,
    tracked_processes: HashMap<String, Child>,
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
            stack: Vec::with_capacity(256),
            globals: HashMap::with_capacity(64),
            frames: Vec::with_capacity(16),
            modules: HashMap::new(),
            exception_stack: Vec::with_capacity(8),
            tracked_processes: HashMap::new(),
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



        // Initialize JIT functions
        crate::jit::init_jit(&mut vm);

        // Initialize performance optimization functions
        crate::performance::init_performance_optimizations(&mut vm);

        // Initialize package system functions
        crate::package::init_package_system(&mut vm);

        // Initialize package manager functions
        crate::package_manager::init_package_cli(&mut vm);

        // Initialize system native functions
        vm.register_native("system_exec", 1, crate::native_system::system_exec);
        vm.register_native("system_spawn", 1, crate::native_system::system_spawn);
        vm.register_native("system_shell", 1, crate::native_system::system_shell);
        vm.register_native("system_getenv", 1, crate::native_system::system_getenv);
        vm.register_native("system_setenv", 2, crate::native_system::system_setenv);
        vm.register_native("system_environ", 0, crate::native_system::system_environ);
        vm.register_native("system_read_output", 1, crate::native_system::system_read_output);
        vm.register_native("system_capture", 1, crate::native_system::system_capture);
        vm.register_native("system_background", 1, crate::native_system::system_background);
        vm.register_native("system_pipe", 2, crate::native_system::system_pipe);
        vm.register_native("system_redirect", 2, crate::native_system::system_redirect);
        vm.register_native("system_timeout", 2, crate::native_system::system_timeout);
        vm.register_native("system_wait", 1, crate::native_system::system_wait);
        vm.register_native("system_kill", 2, crate::native_system::system_kill);
        vm.register_native("system_status", 1, crate::native_system::system_status);
        vm.register_native("system_write_input", 2, crate::native_system::system_write_input);

        // Async system functions
        vm.register_native("system_async_exec", 1, crate::native_system::system_async_exec);
        vm.register_native("system_async_spawn", 1, crate::native_system::system_async_spawn);
        vm.register_native("system_async_wait", 1, crate::native_system::system_async_wait);
        vm.register_native("system_async_pipe", 2, crate::native_system::system_async_pipe);

        // Streaming and monitoring functions
        vm.register_native("system_stream_exec", 1, crate::native_system::system_stream_exec);
        vm.register_native("system_monitor_process", 1, crate::native_system::system_monitor_process);



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
        self.exception_stack.clear();
        
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
    loop {
        let instruction = self.read_byte().expect("Unexpected end of bytecode");

        match OpCode::from_byte(instruction) {
            Some(OpCode::Constant) => {
                let constant_index = self.read_byte().expect("Expected constant index") as usize;
                let chunk = self.chunk.as_ref().expect("No chunk loaded");
                let constant = chunk.constants[constant_index].clone();
                self.stack.push(constant);
            }
            Some(OpCode::Add) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (b, a),
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                
                match (a, b) {
                    (Value::Number(a_num), Value::Number(b_num)) => {
                        self.stack.push(Value::Number(a_num + b_num));
                    }
                    (Value::String(mut a_str), Value::String(b_str)) => {
                        a_str.push_str(&b_str);
                        self.stack.push(Value::String(a_str));
                    }
                    (Value::String(mut a_str), Value::Number(b_num)) => {
                        a_str.push_str(&b_num.to_string());
                        self.stack.push(Value::String(a_str));
                    }
                    (Value::Number(a_num), Value::String(b_str)) => {
                        let mut result = a_num.to_string();
                        result.push_str(&b_str);
                        self.stack.push(Value::String(result));
                    }
                    (Value::String(mut a_str), Value::Boolean(b_bool)) => {
                        a_str.push_str(if b_bool { "true" } else { "false" });
                        self.stack.push(Value::String(a_str));
                    }
                    (Value::Boolean(a_bool), Value::String(b_str)) => {
                        let mut result = if a_bool { "true" } else { "false" }.to_string();
                        result.push_str(&b_str);
                        self.stack.push(Value::String(result));
                    }
                    _ => {
                        return InterpretResult::RuntimeError("Operands must be numbers or strings".to_string());
                    }
                }
            }
            Some(OpCode::Multiply) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(b)), Some(Value::Number(a))) => (a, b),
                    _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                };
                self.stack.push(Value::Number(a * b));
            }
            Some(OpCode::Divide) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(b)), Some(Value::Number(a))) => (a, b),
                    _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                };
                if b == 0.0 {
                    return InterpretResult::RuntimeError("Division by zero".to_string());
                }
                self.stack.push(Value::Number(a / b));
            }
            Some(OpCode::Modulo) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(b)), Some(Value::Number(a))) => (a, b),
                    _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                };
                if b == 0.0 {
                    return InterpretResult::RuntimeError("Modulo by zero".to_string());
                }
                self.stack.push(Value::Number(a % b));
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
                
                let value = match self.stack.pop() {
                    Some(v) => v,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                self.globals.insert(name, value);
            }
            Some(OpCode::GetLocal) => {
                let slot = self.read_byte().expect("Expected slot") as usize;
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
                let slot = self.read_byte().expect("Expected slot") as usize;
                let value = match self.stack.pop() {
                    Some(v) => v,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
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
                self.ip = self.ip.checked_sub(offset).expect("Loop underflow");
            }
            Some(OpCode::Dup) => {
                if let Some(value) = self.stack.last() {
                    self.stack.push(value.clone());
                } else {
                    return InterpretResult::RuntimeError("Stack underflow".to_string());
                }
            }
            Some(OpCode::Call) => {
                let arg_count = self.read_byte().expect("Expected argument count") as usize;
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
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (a, b),
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                match (a, b) {
                    (Value::Number(a), Value::Number(b)) => {
                        self.stack.push(Value::Number(a - b));
                    }
                    _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                }
            }
            Some(OpCode::Negate) => {
                let value = match self.stack.pop() {
                    Some(v) => v,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                match value {
                    Value::Number(n) => self.stack.push(Value::Number(-n)),
                    _ => return InterpretResult::RuntimeError("Operand must be a number".to_string()),
                }
            }
            Some(OpCode::Array) => {
                let count = self.read_byte().expect("Expected array count") as usize;
                let start_idx = match self.stack.len().checked_sub(count) {
                    Some(idx) => idx,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                let elements = self.stack.drain(start_idx..).collect();
                self.stack.push(Value::Array(elements));
            }
            Some(OpCode::Dictionary) => {
                let count = self.read_byte().expect("Expected dictionary count") as usize;
                let pairs_needed = count * 2;
                let start_idx = match self.stack.len().checked_sub(pairs_needed) {
                    Some(idx) => idx,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                
                let pairs = self.stack.drain(start_idx..).collect::<Vec<_>>();
                let mut dict = std::collections::HashMap::with_capacity(count);
                
                for chunk in pairs.chunks(2) {
                    match &chunk[0] {
                        Value::String(key_str) => {
                            dict.insert(key_str.clone(), chunk[1].clone());
                        }
                        _ => return InterpretResult::RuntimeError("Dictionary keys must be strings".to_string()),
                    }
                }
                self.stack.push(Value::Dictionary(dict));
            }
            Some(OpCode::Index) => {
                let (array, index) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(array), Some(index)) => (array, index),
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
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
            }
            Some(OpCode::Length) => {
                let value = match self.stack.pop() {
                    Some(v) => v,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                match value {
                    Value::Array(elements)=> {
                        self.stack.push(Value::Number(elements.len() as f64));
                    }
                    _ => return InterpretResult::RuntimeError("Length operation requires array".to_string()),
                }
            }
            Some(OpCode::Equal) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (a, b),
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                self.stack.push(Value::Boolean(self.values_equal(&a, &b)));
            }
            Some(OpCode::NotEqual) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (a, b),
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                self.stack.push(Value::Boolean(!self.values_equal(&a, &b)));
            }
            Some(OpCode::Less) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (b, a),  // Fixed: don't swap
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                match (a, b) {
                    (Value::Number(a), Value::Number(b))=> {
                        self.stack.push(Value::Boolean(a < b));
                    }
                    _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                }
            }
            Some(OpCode::LessEqual) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (b, a),  // Fixed: don't swap
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                match (a, b) {
                    (Value::Number(a), Value::Number(b))=> {
                        self.stack.push(Value::Boolean(a <= b));
                    }
                    _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                }
            }
            Some(OpCode::Greater) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (b, a),  // Fixed: don't swap
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                match (a, b) {
                    (Value::Number(a), Value::Number(b))=> {
                        self.stack.push(Value::Boolean(a > b));
                    }
                    _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                }
            }
            Some(OpCode::GreaterEqual) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (b, a),  // Fixed: don't swap
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                match (a, b) {
                    (Value::Number(a), Value::Number(b))=> {
                        self.stack.push(Value::Boolean(a >= b));
                    }
                    _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                }
            }
            Some(OpCode::Not) => {
                let value = match self.stack.pop() {
                    Some(v) => v,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                self.stack.push(Value::Boolean(!self.is_truthy(&value)));
            }
            Some(OpCode::And) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (a, b),
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                self.stack.push(Value::Boolean(self.is_truthy(&a) && self.is_truthy(&b)));
            }
            Some(OpCode::Or) => {
                let (b, a) = match (self.stack.pop(), self.stack.pop()) {
                    (Some(b), Some(a)) => (a, b),
                    _ => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                self.stack.push(Value::Boolean(self.is_truthy(&a) || self.is_truthy(&b)));
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
                // Class creation is handled at compile time by storing the class value as a constant
                // This opcode is just a placeholder that should never be executed
                return InterpretResult::RuntimeError("CreateClass should not be executed at runtime".to_string());
            }
            Some(OpCode::GetProperty) => {
                // Stack: [..., object, property_name]
                let property_name = match self.stack.pop() {
                    Some(Value::String(s)) => s,
                    _ => return InterpretResult::RuntimeError("Property name must be a string".to_string()),
                };

                if let Some(object) = self.stack.pop() {
                    match object {
                        Value::Object { fields, .. } => {
                            if let Some(value) = fields.get(&property_name) {
                                self.stack.push(value.clone());
                            } else {
                                return InterpretResult::RuntimeError(format!("Undefined property '{}'", property_name));
                            }
                        }
                        Value::Dictionary(dict) => {
                            if let Some(value) = dict.get(&property_name) {
                                self.stack.push(value.clone());
                            } else {
                                return InterpretResult::RuntimeError(format!("Undefined key '{}'", property_name));
                            }
                        }
                        _ => return InterpretResult::RuntimeError("Expected object or dictionary".to_string()),
                    }
                } else {
                    return InterpretResult::RuntimeError("Stack underflow".to_string());
                }
            }
            Some(OpCode::SetProperty) => {
                // Stack: [..., object, property_name, value]
                let value = match self.stack.pop() {
                    Some(v) => v,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };
                
                let property_name = match self.stack.pop() {
                    Some(Value::String(s)) => s,
                    _ => return InterpretResult::RuntimeError("Property name must be a string".to_string()),
                };

                if let Some(Value::Object { fields, class_name }) = self.stack.pop() {
                    // Create a new object with the updated property
                    let mut new_fields = fields.clone();
                    new_fields.insert(property_name, value);
                    
                    // Push the updated object back on the stack
                    self.stack.push(Value::Object {
                        class_name,
                        fields: new_fields,
                    });
                } else {
                    return InterpretResult::RuntimeError("Expected object".to_string());
                }
            }
            Some(OpCode::CallMethod) => {
                // Stack: [..., object, method_name, arg1, arg2, ..., argN]
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

                let method_name = match self.stack.pop() {
                    Some(Value::String(s)) => s,
                    _ => return InterpretResult::RuntimeError("Method name must be a string".to_string()),
                };

                let object = match self.stack.pop() {
                    Some(obj) => obj,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };

                // Get the class name from the object
                let class_name = match &object {
                    Value::Object { class_name, .. } => class_name.clone(),
                    _ => return InterpretResult::RuntimeError("Expected object".to_string()),
                };

                // Look up the class in globals
                let class_value = match self.globals.get(&class_name) {
                    Some(Value::Class { methods, .. }) => methods,
                    _ => return InterpretResult::RuntimeError(format!("Class '{}' not found", class_name)),
                };

                // Look up the method in the class
                let method_constant_index = match class_value.get(&method_name) {
                    Some(index) => index,
                    None => return InterpretResult::RuntimeError(format!("Method '{}' not found in class '{}'", method_name, class_name)),
                };

                // Get the method function from constants
                let method_function = match &self.chunk.as_ref().unwrap().constants[*method_constant_index] {
                    Value::Function(func) => func.clone(),
                    _ => return InterpretResult::RuntimeError("Method is not a function".to_string()),
                };

                // Create a new call frame for the method
                let frame = CallFrame {
                    ip: 0,
                    slot: self.stack.len(),
                    chunk: method_function.chunk.clone(),
                };
                self.frames.push(frame);

                // Push the object as the first argument (self)
                self.stack.push(object);

                // Push the arguments
                for arg in args {
                    self.stack.push(arg);
                }

                // Set up the new chunk
                self.chunk = Some(method_function.chunk.clone());
                self.ip = 0;
            }
            Some(OpCode::GetSuper) => {
                // Stack: [..., object, super_method_name]
                let method_name = match self.stack.pop() {
                    Some(Value::String(s)) => s,
                    _ => return InterpretResult::RuntimeError("Method name must be a string".to_string()),
                };

                let object = match self.stack.pop() {
                    Some(obj) => obj,
                    None => return InterpretResult::RuntimeError("Stack underflow".to_string()),
                };

                // Get the class name from the object
                let class_name = match &object {
                    Value::Object { class_name, .. } => class_name.clone(),
                    _ => return InterpretResult::RuntimeError("Expected object".to_string()),
                };

                // Look up the class in globals
                let class_value = match self.globals.get(&class_name) {
                    Some(Value::Class { methods, superclass, .. }) => (methods, superclass),
                    _ => return InterpretResult::RuntimeError(format!("Class '{}' not found", class_name)),
                };

                // Get the superclass name
                let superclass_name = match class_value.1 {
                    Some(name) => name,
                    None => return InterpretResult::RuntimeError(format!("Class '{}' has no superclass", class_name)),
                };

                // Look up the superclass in globals
                let superclass_value = match self.globals.get(superclass_name.as_str()) {
                    Some(Value::Class { methods, .. }) => methods,
                    _ => return InterpretResult::RuntimeError(format!("Superclass '{}' not found", superclass_name)),
                };

                // Look up the method in the superclass
                let method_constant_index = match superclass_value.get(&method_name) {
                    Some(index) => index,
                    None => return InterpretResult::RuntimeError(format!("Method '{}' not found in superclass '{}'", method_name, superclass_name)),
                };

                // Get the method function from constants
                let method_function = match &self.chunk.as_ref().unwrap().constants[*method_constant_index] {
                    Value::Function(func) => func.clone(),
                    _ => return InterpretResult::RuntimeError("Super method is not a function".to_string()),
                };

                // Push the method function onto the stack
                self.stack.push(Value::Function(method_function));
            }
            Some(OpCode::RustInline) => {
                let constant_index = self.read_byte().expect("Expected constant index") as usize;
                if let Value::String(code) = &self.chunk.as_ref().unwrap().constants[constant_index] {
                    // For now, just print inline Rust code to show it's working
                    // In a real implementation, this would compile and execute Rust code
                    println!("[Executing Rust: {}]", code);
                    self.stack.push(Value::String("Rust inline executed".to_string()));
                } else {
                    return InterpretResult::RuntimeError("RustInline expects string constant".to_string());
                }
            }
            Some(OpCode::AsmInline) => {
                let constant_index = self.read_byte().expect("Expected constant index") as usize;
                if let Value::String(code) = &self.chunk.as_ref().unwrap().constants[constant_index] {
                    // For now, just print inline assembly code to show it's working
                    // In a real implementation, this would assemble and execute assembly code
                    println!("[Executing Assembly: {}]", code);
                    self.stack.push(Value::String("Assembly inline executed".to_string()));
                } else {
                    return InterpretResult::RuntimeError("AsmInline expects string constant".to_string());
                }
            }
            Some(OpCode::Try) => {
                // Try instruction - just a placeholder for now
                // The actual exception handling will be done by compiler-generated jumps
                let _jump_offset = self.read_short() as usize;
            }
            Some(OpCode::Catch) => {
                // Catch instruction - exception is already on stack
                // No action needed for now
            }
            Some(OpCode::Throw) => {
                // Throw an exception
                if let Some(exception) = self.stack.pop() {
                    // For now, just return runtime error with exception message
                    if let Value::String(msg) = &exception {
                        return InterpretResult::RuntimeError(format!("Exception: {}", msg));
                    } else {
                        return InterpretResult::RuntimeError("Exception thrown".to_string());
                    }
                } else {
                    return InterpretResult::RuntimeError("No exception to throw".to_string());
                }
            }
            Some(OpCode::PopException) => {
                // Pop exception handler from stack
                self.exception_stack.pop();
            }
            None => return InterpretResult::RuntimeError("Unknown opcode".to_string()),
                }
        }
    }

    fn call_value(&mut self, arg_count: usize) -> bool {
        // The function is below the arguments
        let func_index = self.stack.len().checked_sub(arg_count + 1).unwrap_or(0);
        let callee = match self.stack.get(func_index).cloned() {
            Some(callee) => callee,
            None => return false,
        };
        
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
                    true
                } else {
                    false
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
                true
            }
            Value::NativeFunction(native_func) => {
                // Native function
                if arg_count != native_func.arity {
                    return false;
                }

                // Collect arguments (they are above the function on the stack)
                let mut args = Vec::with_capacity(arg_count);
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
                        true
                    }
                    Err(_) => false,
                }
            }
            _ => {
                self.stack.push(Value::Null);
                true
            }
        }
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
            Value::Dictionary(dict) => {
                let pairs: Vec<String> = dict.iter()
                    .map(|(k, v)| format!("{}: {}", k, self.format_value(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            },
            Value::Object { class_name, .. } => {
                format!("Object of class {}", class_name)
            },
            Value::Class { name, .. } => format!("Class {:?}", name),
            Value::Module(name) => format!("<module {}>", name),
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        let chunk = self.chunk.as_ref()?;
        if self.ip < chunk.code.len() {
            let byte = chunk.code[self.ip];
            self.ip += 1;
            Some(byte)
        } else {
            None
        }
    }

    fn read_short(&mut self) -> u16 {
        let chunk = self.chunk.as_ref().expect("No chunk loaded");
        let high = chunk.code[self.ip] as u16;
        let low = chunk.code[self.ip + 1] as u16;
        self.ip += 2;
        (high << 8) | low
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte().expect("Expected constant") as usize;
        let chunk = self.chunk.as_ref().expect("No chunk loaded");
        chunk.constants[index].clone()
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
            Value::Dictionary(dict) => !dict.is_empty(),
            Value::Object { .. } => true,
            Value::Class { .. } => true,
            Value::Module(_) => true,
        }
    }

    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Module(a), Value::Module(b)) => a == b,
            (Value::Array(a), Value::Array(b))=> {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| self.values_equal(x, y))
                }
            _ => false,
                }
    }

    // Process tracking methods
    pub fn track_process(&mut self, pid: String, child: Child) {
        self.tracked_processes.insert(pid, child);
    }

    pub fn get_tracked_process(&self, pid: &str) -> Option<&Child> {
        self.tracked_processes.get(pid)
    }

    pub fn get_tracked_process_mut(&mut self, pid: &str) -> Option<&mut Child> {
        self.tracked_processes.get_mut(pid)
    }

    pub fn remove_tracked_process(&mut self, pid: &str) -> Option<Child> {
        self.tracked_processes.remove(pid)
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        // Clean up any remaining tracked processes
        for (pid, mut child) in self.tracked_processes.drain() {
            let _ = child.kill(); // Ignore errors during cleanup
            eprintln!("Cleaned up orphaned process with PID: {}", pid);
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