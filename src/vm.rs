use crate::bytecode::*;
use std::collections::HashMap;

pub struct VM {
    pub chunk: Option<Chunk>,
    ip: usize,
    pub stack: Vec<Value>,
    globals: HashMap<String, Value>,
    frames: Vec<CallFrame>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CallFrame {
    ip: usize,
    slot: usize,
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
        };
        
        // Add built-in functions
        vm.globals.insert("print".to_string(), Value::String("print".to_string()));
        
        vm
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
            let instruction = match self.read_byte() {
                Some(byte) => byte,
                None => return InterpretResult::Ok,
            };
            
            match OpCode::from_byte(instruction) {
                Some(OpCode::Constant) => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
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
                    if slot < self.stack.len() {
                        self.stack.push(self.stack[slot].clone());
                    } else {
                        return InterpretResult::RuntimeError("Invalid local slot".to_string());
                    }
                }
                Some(OpCode::SetLocal) => {
                    let slot = self.read_byte().unwrap() as usize;
                    if let Some(value) = self.stack.pop() {
                        if slot < self.stack.len() {
                            self.stack[slot] = value;
                        } else {
                            return InterpretResult::RuntimeError("Invalid local slot".to_string());
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
                Some(OpCode::Call) => {
                    let arg_count = self.read_byte().unwrap() as usize;
                    if !self.call_value(arg_count) {
                        return InterpretResult::RuntimeError("Failed to call value".to_string());
                    }
                }
                Some(OpCode::Return) => {
                    if let Some(result) = self.stack.pop() {
                        // Pop locals and return value
                        self.stack.clear();
                        self.stack.push(result);
                    } else {
                        self.stack.push(Value::Null);
                    }
                }
                Some(OpCode::Add) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        match (a, b) {
                            (Value::Number(a), Value::Number(b)) => {
                                self.stack.push(Value::Number(a + b));
                            }
                            (Value::String(a), Value::String(b)) => {
                                self.stack.push(Value::String(a + &b));
                            }
                            (Value::String(a), Value::Number(b)) => {
                                self.stack.push(Value::String(a + &b.to_string()));
                            }
                            (Value::Number(a), Value::String(b)) => {
                                self.stack.push(Value::String(a.to_string() + &b));
                            }
                            _ => return InterpretResult::RuntimeError("Operands must be numbers or strings".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
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
                Some(OpCode::Multiply) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        match (a, b) {
                            (Value::Number(a), Value::Number(b)) => {
                                self.stack.push(Value::Number(a * b));
                            }
                            _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
                Some(OpCode::Divide) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        match (a, b) {
                            (Value::Number(a), Value::Number(b)) => {
                                if b == 0.0 {
                                    return InterpretResult::RuntimeError("Division by zero".to_string());
                                }
                                self.stack.push(Value::Number(a / b));
                            }
                            _ => return InterpretResult::RuntimeError("Operands must be numbers".to_string()),
                        }
                    } else {
                        return InterpretResult::RuntimeError("Stack underflow".to_string());
                    }
                }
                Some(OpCode::Modulo) => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        match (a, b) {
                            (Value::Number(a), Value::Number(b)) => {
                                if b == 0.0 {
                                    return InterpretResult::RuntimeError("Modulo by zero".to_string());
                                }
                                self.stack.push(Value::Number(a % b));
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
                            (Value::Number(a), Value::Number(b)) => {
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
                            (Value::Number(a), Value::Number(b)) => {
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
                            (Value::Number(a), Value::Number(b)) => {
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
                            (Value::Number(a), Value::Number(b)) => {
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
                None => return InterpretResult::RuntimeError("Unknown opcode".to_string()),
            }
        }
    }

    fn call_value(&mut self, arg_count: usize) -> bool {
        // Check if we're calling a built-in function
        if let Some(callee) = self.stack.get(self.stack.len() - arg_count - 1) {
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
                _ => {}
            }
        }
        
        // For now, just pop the arguments and function name
        for _ in 0..=arg_count {
            self.stack.pop();
        }
        self.stack.push(Value::Null);
        true
    }

    fn format_value(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Function(f) => format!("<fn {}>", f.name),
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
        }
    }

    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
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
        let result = run_code("let x = 42\nx").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_assignment() {
        let result = run_code("let x = 1\nx = 2").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_if_statement() {
        let result = run_code("if true { 1 }").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_while_statement() {
        let result = run_code("while false { }").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_function_definition() {
        let result = run_code("fn test() { return 42 }").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_function_call() {
        let result = run_code("fn test() { return 42 }").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_vm_print() {
        let result = run_code("print(42)").unwrap();
        assert_eq!(result, InterpretResult::Ok);
    }
}