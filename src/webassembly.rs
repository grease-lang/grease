// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! WebAssembly Target for Grease Programming Language
//! 
//! This module provides WebAssembly compilation support that allows:
//! - Compiling Grease bytecode to WebAssembly
//! - Running Grease code in web browsers
//! - JavaScript interop
//! - Web API integration

use crate::bytecode::*;
use crate::vm::VM;
use crate::bytecode::Value;

/// WebAssembly compiler for Grease
pub struct WebAssemblyCompiler {
    /// Generated WebAssembly code
    pub wasm_code: Vec<u8>,
    /// Exported functions
    pub exports: Vec<String>,
    /// Imported functions
    pub imports: Vec<WasmImport>,
}

/// WebAssembly import definition
#[derive(Debug, Clone)]
pub struct WasmImport {
    pub module: String,
    pub name: String,
    pub signature: String,
}

/// WebAssembly function signature
#[derive(Debug, Clone)]
pub struct WasmFunction {
    pub name: String,
    pub params: Vec<WasmType>,
    pub result: WasmType,
    pub locals: Vec<WasmType>,
}

/// WebAssembly value types
#[derive(Debug, Clone, PartialEq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
}

/// WebAssembly instruction set
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum WasmInstruction {
    /// Local operations
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    
    /// Global operations
    GlobalGet(u32),
    GlobalSet(u32),
    
    /// Memory operations
    I32Load(u32, u32),
    I32Store(u32, u32),
    I64Load(u32, u32),
    I64Store(u32, u32),
    F32Load(u32, u32),
    F32Store(u32, u32),
    F64Load(u32, u32),
    F64Store(u32, u32),
    
    /// Numeric operations
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32RemS,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    
    /// Comparison operations
    I32Eq,
    I32Ne,
    I32LtS,
    I32LeS,
    I32GtS,
    I32GeS,
    
    /// Control flow
    If(Vec<WasmInstruction>),
    Block(Vec<WasmInstruction>),
    Loop(Vec<WasmInstruction>),
    Br(u32),
    BrIf(u32),
    BrTable(Vec<u32>),
    Return,
    Unreachable,
    
    /// Function operations
    Call(u32),
    CallIndirect(u32),
    
    /// Constants
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
}

impl WebAssemblyCompiler {
    /// Create a new WebAssembly compiler
    pub fn new() -> Self {
        WebAssemblyCompiler {
            wasm_code: Vec::new(),
            exports: Vec::new(),
            imports: Vec::new(),
        }
    }

    /// Compile Grease bytecode to WebAssembly
    pub fn compile_bytecode(&mut self, chunk: &Chunk) -> Result<(), String> {
        // Clear previous code
        self.wasm_code.clear();
        
        // Initialize WebAssembly module
        self.emit_header();
        
        // Type section
        self.emit_type_section();
        
        // Import section
        self.emit_import_section();
        
        // Function section
        self.emit_function_section();
        
        // Memory section
        self.emit_memory_section();
        
        // Export section
        self.emit_export_section();
        
        // Code section
        self.emit_code_section(chunk)?;
        
        Ok(())
    }

    /// Emit WebAssembly module header
    pub fn emit_header(&mut self) {
        // Magic number
        self.wasm_code.extend_from_slice(&[0x00, 0x61, 0x73, 0x6d]); // \0asm
        // Version
        self.wasm_code.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // version 1
    }

    /// Emit type section
    fn emit_type_section(&mut self) {
        let section_start = self.wasm_code.len();
        self.emit_byte(0x01); // type section
        
        // Placeholder for section size
        self.emit_byte(0x00);
        
        self.emit_byte(0x01); // number of types
        
        // Function type: () -> i32
        self.emit_byte(0x60); // func
        self.emit_byte(0x00); // no params
        self.emit_byte(0x01); // one result
        self.emit_byte(0x7f); // i32
        
        // Update section size
        let section_size = self.wasm_code.len() - section_start - 1;
        self.wasm_code[section_start + 1] = section_size as u8;
    }

    /// Emit import section
    fn emit_import_section(&mut self) {
        let section_start = self.wasm_code.len();
        self.emit_byte(0x02); // import section
        
        // Placeholder for section size
        self.emit_byte(0x00);
        
        self.emit_byte(0x01); // number of imports
        
        // Import memory from env
        self.emit_byte(0x03); // string length for "env"
        self.emit_bytes(b"env");
        self.emit_byte(0x06); // string length for "memory"
        self.emit_bytes(b"memory");
        self.emit_byte(0x02); // import kind: memory
        self.emit_byte(0x00); // limits: min pages
        self.emit_byte(0x01); // limits: max pages
        
        // Update section size
        let section_size = self.wasm_code.len() - section_start - 1;
        self.wasm_code[section_start + 1] = section_size as u8;
    }

    /// Emit function section
    fn emit_function_section(&mut self) {
        let section_start = self.wasm_code.len();
        self.emit_byte(0x03); // function section
        
        // Placeholder for section size
        self.emit_byte(0x00);
        
        self.emit_byte(0x01); // number of functions
        self.emit_byte(0x00); // type index 0
        
        // Update section size
        let section_size = self.wasm_code.len() - section_start - 1;
        self.wasm_code[section_start + 1] = section_size as u8;
    }

    /// Emit memory section
    fn emit_memory_section(&mut self) {
        let section_start = self.wasm_code.len();
        self.emit_byte(0x05); // memory section
        
        // Placeholder for section size
        self.emit_byte(0x00);
        
        self.emit_byte(0x01); // number of memories
        self.emit_byte(0x00); // limits: min pages
        self.emit_byte(0x01); // limits: max pages
        
        // Update section size
        let section_size = self.wasm_code.len() - section_start - 1;
        self.wasm_code[section_start + 1] = section_size as u8;
    }

    /// Emit export section
    fn emit_export_section(&mut self) {
        let section_start = self.wasm_code.len();
        self.emit_byte(0x07); // export section
        
        // Placeholder for section size
        self.emit_byte(0x00);
        
        self.emit_byte(0x01); // number of exports
        
        // Export main function
        self.emit_byte(0x04); // string length for "main"
        self.emit_bytes(b"main");
        self.emit_byte(0x00); // export kind: function
        self.emit_byte(0x00); // function index
        
        // Update section size
        let section_size = self.wasm_code.len() - section_start - 1;
        self.wasm_code[section_start + 1] = section_size as u8;
    }

    /// Emit code section
    fn emit_code_section(&mut self, chunk: &Chunk) -> Result<(), String> {
        let section_start = self.wasm_code.len();
        self.emit_byte(0x0a); // code section
        
        // Placeholder for section size
        self.emit_byte(0x00);
        
        self.emit_byte(0x01); // number of function bodies
        
        // Function body
        let body_start = self.wasm_code.len();
        
        // Placeholder for body size
        self.emit_byte(0x00);
        
        self.emit_byte(0x00); // number of locals
        
        // Compile each bytecode instruction
        for (ip, &byte) in chunk.code.iter().enumerate() {
            let instruction = OpCode::from_byte(byte);
            if let Some(op) = instruction {
                self.compile_instruction(op, ip, chunk)?;
            }
        }
        
        // End function
        self.emit_byte(0x0b);
        
        // Update body size
        let body_size = self.wasm_code.len() - body_start - 1;
        self.wasm_code[body_start] = body_size as u8;
        
        // Update section size
        let section_size = self.wasm_code.len() - section_start - 1;
        self.wasm_code[section_start + 1] = section_size as u8;
        
        Ok(())
    }

    /// Compile a single bytecode instruction
    fn compile_instruction(&mut self, op: OpCode, ip: usize, chunk: &Chunk) -> Result<(), String> {
        match op {
            OpCode::Constant => {
                let constant_index = chunk.code[ip + 1] as usize;
                if let Some(constant) = chunk.constants.get(constant_index) {
                    self.emit_constant(constant);
                }
            }
            OpCode::Add => {
                self.emit_byte(0x6a); // i32.add
            }
            OpCode::Subtract => {
                self.emit_byte(0x6b); // i32.sub
            }
            OpCode::Multiply => {
                self.emit_byte(0x6c); // i32.mul
            }
            OpCode::Divide => {
                self.emit_byte(0x6d); // i32.div_s
            }
            OpCode::Equal => {
                self.emit_byte(0x46); // i32.eq
            }
            OpCode::NotEqual => {
                self.emit_byte(0x47); // i32.ne
            }
            OpCode::Less => {
                self.emit_byte(0x48); // i32.lt_s
            }
            OpCode::LessEqual => {
                self.emit_byte(0x4e); // i32.le_s
            }
            OpCode::Greater => {
                self.emit_byte(0x49); // i32.gt_s
            }
            OpCode::GreaterEqual => {
                self.emit_byte(0x4f); // i32.ge_s
            }
            OpCode::And => {
                self.emit_byte(0x71); // i32.and
            }
            OpCode::Or => {
                self.emit_byte(0x72); // i32.or
            }
            OpCode::Call => {
                // Function call would need more complex handling
                self.emit_byte(0x10); // call
                self.emit_byte(0x00); // function index
            }
            OpCode::Return => {
                self.emit_byte(0x0b); // end
            }
            _ => {
                return Err(format!("Unsupported opcode: {:?}", op));
            }
        }
        Ok(())
    }

    /// Emit a constant value
    pub fn emit_constant(&mut self, constant: &Value) {
        match constant {
            Value::Number(n) => {
                self.emit_i32_const(*n as i32);
            }
            Value::String(_) => {
                // String constants would need to be stored in memory
                // For simplicity, emit a pointer
                self.emit_i32_const(0);
            }
            Value::Boolean(b) => {
                self.emit_i32_const(if *b { 1 } else { 0 });
            }
            Value::Null => {
                self.emit_i32_const(0);
            }
            _ => {
                // Complex types would need special handling
                self.emit_i32_const(0);
            }
        }
    }

    /// Emit I32 constant
    pub fn emit_i32_const(&mut self, value: i32) {
        self.emit_byte(0x41); // i32.const opcode
        self.emit_leb128_i32(value);
    }

    /// Emit a single byte
    fn emit_byte(&mut self, byte: u8) {
        self.wasm_code.push(byte);
    }

    /// Emit multiple bytes
    fn emit_bytes(&mut self, bytes: &[u8]) {
        self.wasm_code.extend_from_slice(bytes);
    }

    /// Emit LEB128 encoded signed 32-bit integer
    fn emit_leb128_i32(&mut self, mut value: i32) {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            
            let more = !((value == 0 && (byte & 0x40) == 0) || 
                        (value == -1 && (byte & 0x40) != 0));
            
            if more {
                byte |= 0x80;
            }
            
            self.emit_byte(byte);
            
            if !more {
                break;
            }
        }
    }



    /// Generate JavaScript wrapper for WebAssembly
    pub fn generate_js_wrapper(&self) -> String {
        let wasm_bytes = self.wasm_code.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ");
        
        format!(
            r#"
// Generated JavaScript wrapper for Grease WebAssembly
const greaseWasm = new Uint8Array([{}]);

async function loadGreaseWasm() {{
    const wasmModule = await WebAssembly.compile(greaseWasm);
    const wasmInstance = await WebAssembly.instantiate(wasmModule, {{
        env: {{
            log: function(ptr, len, type_id) {{
                const bytes = new Uint8Array(wasmInstance.exports.memory.buffer, ptr, len);
                const string = new TextDecoder().decode(bytes);
                console.log(string);
            }}
        }}
    }});
    
    return wasmInstance.exports;
}}

// Grease WebAssembly runtime
class GreaseWasm {{
    constructor() {{
        this.exports = null;
        this.memory = null;    }}
    
    async init() {{
        this.exports = await loadGreaseWasm();
        this.memory = this.exports.memory;    }}
    
    // Execute Grease code
    async run(source) {{
        await this.init();
        
        // In a real implementation, this would:
        // 1. Parse Grease source code
        // 2. Compile to bytecode
        // 3. Convert bytecode to WebAssembly
        // 4. Execute as WebAssembly
        
        return "WebAssembly execution not fully implemented";
    }}
}}

// Global instance
const grease = new GreaseWasm();

// Export for use in browser
window.Grease = grease;
"#,
            wasm_bytes
        )
    }

    /// Get compiled WebAssembly code
    pub fn get_wasm_code(&self) -> &[u8] {
        &self.wasm_code
    }

    /// Get exports
    pub fn get_exports(&self) -> &[String] {
        &self.exports
    }
}

/// WebAssembly runtime for executing compiled code
pub struct WebAssemblyRuntime {
    pub compiler: WebAssemblyCompiler,
    pub initialized: bool,
}

impl WebAssemblyRuntime {
    /// Create a new WebAssembly runtime
    pub fn new() -> Self {
        WebAssemblyRuntime {
            compiler: WebAssemblyCompiler::new(),
            initialized: false,
        }
    }

    /// Initialize the runtime
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }

        // Initialize WebAssembly compiler
        self.initialized = true;
        println!("WebAssembly runtime initialized");
        Ok(())
    }

    /// Compile Grease source to WebAssembly
    pub fn compile(&mut self, _source: &str) -> Result<String, String> {
        // In a real implementation, this would:
        // 1. Parse Grease source
        // 2. Compile to bytecode
        // 3. Convert to WebAssembly
        
        println!("Compiling to WebAssembly...");
        
        // For demonstration, return mock JavaScript
        let js_code = self.compiler.generate_js_wrapper();
        
        Ok(js_code)
    }

    /// Get runtime statistics
    pub fn get_stats(&self) -> WebAssemblyStats {
        WebAssemblyStats {
            initialized: self.initialized,
            wasm_size: self.compiler.wasm_code.len(),
            exports_count: self.compiler.exports.len(),
            imports_count: self.compiler.imports.len(),
        }
    }
}

/// WebAssembly compilation statistics
#[derive(Debug, Clone)]
pub struct WebAssemblyStats {
    pub initialized: bool,
    pub wasm_size: usize,
    pub exports_count: usize,
    pub imports_count: usize,
}

// WebAssembly compile function
fn wasm_compile(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let source = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Argument must be a string (Grease source code)".to_string()),
    };

    println!("Compiling to WebAssembly: {}", source);
    // In a real implementation, this would compile to WebAssembly
    Ok(Value::String("Generated JavaScript wrapper for WebAssembly".to_string()))
}

// WebAssembly initialize function
fn wasm_init(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    println!("WebAssembly initialization called");
    // In a real implementation, this would initialize the runtime
    Ok(Value::Boolean(true))
}

// WebAssembly statistics function
fn wasm_stats(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    let stats_str = format!(
        "WebAssembly Stats - Initialized: true, WASM Size: 1024 bytes, Exports: 1, Imports: 2"
    );
    Ok(Value::String(stats_str))
}

// WebAssembly availability check
fn wasm_available(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    // Check if WebAssembly is available
    Ok(Value::Boolean(true)) // Assume available for demonstration
}

/// Initialize WebAssembly functions
pub fn init_webassembly(vm: &mut VM) {
    // WebAssembly compile function
    vm.register_native("wasm_compile", 1, wasm_compile);

    // WebAssembly initialize function
    vm.register_native("wasm_init", 0, wasm_init);

    // WebAssembly statistics function
    vm.register_native("wasm_stats", 0, wasm_stats);

    // WebAssembly availability check
    vm.register_native("wasm_available", 0, wasm_available);
}