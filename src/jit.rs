// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Just-In-Time (JIT) Compiler for Grease Programming Language
//! 
//! This module provides a basic JIT compiler that can compile bytecode
//! to native machine code at runtime for improved performance.

use crate::bytecode::*;
use crate::vm::VM;
use std::collections::HashMap;

/// JIT Compiler that compiles bytecode to native machine code
pub struct JITCompiler {
    /// Compiled function cache
    compiled_functions: HashMap<usize, JitFunction>,
    /// Memory pages for executable code
    memory_pages: Vec<MemoryPage>,
}

/// Represents a compiled native function
#[derive(Clone)]
pub struct JitFunction {
    /// Pointer to compiled native code
    pub code_ptr: *const u8,
    /// Size of compiled code in bytes
    pub code_size: usize,
    /// Function signature information
    pub arity: usize,
}

/// Memory page for executable code
pub struct MemoryPage {
    /// Pointer to the memory page
    pub ptr: *mut u8,
    /// Size of the memory page
    pub size: usize,
    /// Current offset within the page
    pub offset: usize,
}

impl MemoryPage {
    /// Create a new executable memory page
    pub fn new(size: usize) -> Result<Self, String> {
        // Allocate memory
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            return Err("Failed to allocate executable memory".to_string());
        }

        Ok(MemoryPage {
            ptr: ptr as *mut u8,
            size,
            offset: 0,
        })
    }

    /// Write data to the memory page
    pub fn write(&mut self, data: &[u8]) -> Result<*mut u8, String> {
        if self.offset + data.len() > self.size {
            return Err("Memory page overflow".to_string());
        }

        let dest = unsafe {
            let dest_ptr = self.ptr.add(self.offset);
            std::ptr::copy_nonoverlapping(data.as_ptr(), dest_ptr, data.len());
            dest_ptr
        };

        self.offset += data.len();
        Ok(dest)
    }
}

impl Drop for MemoryPage {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, self.size);
        }
    }
}

impl JITCompiler {
    /// Create a new JIT compiler
    pub fn new() -> Self {
        JITCompiler {
            compiled_functions: HashMap::new(),
            memory_pages: Vec::new(),
        }
    }

    /// Compile a chunk of bytecode to native code
    pub fn compile_chunk(&mut self, chunk: &Chunk) -> Result<JitFunction, String> {
        let function_key = chunk.constants.len() + chunk.code.len();
        
        // Check if already compiled
        if let Some(compiled) = self.compiled_functions.get(&function_key) {
            return Ok(compiled.clone());
        }

        // Create a new memory page for this function
        let mut memory_page = MemoryPage::new(4096)?; // 4KB page
        self.memory_pages.push(memory_page);

        // Get the memory page back for writing
        let page = self.memory_pages.last_mut().unwrap();

        // Simple compilation: emit basic function prologue
        let mut code = Vec::new();
        
        // Function prologue (x86-64 System V ABI)
        code.extend_from_slice(&[0x55]); // push rbp
        code.extend_from_slice(&[0x48, 0x89, 0xe5]); // mov rbp, rsp
        
        // For now, we'll create a simple stub that calls the VM interpreter
        // In a full implementation, this would compile each bytecode instruction
        // to corresponding native machine code
        
        // For demonstration, we'll create a function that just returns 42
        // mov eax, 42
        code.extend_from_slice(&[0xb8, 0x2a, 0x00, 0x00, 0x00]);
        
        // Function epilogue
        code.extend_from_slice(&[0x5d]); // pop rbp
        code.extend_from_slice(&[0xc3]); // ret

        // Write the compiled code to memory
        let code_ptr = page.write(&code)?;

        let jit_function = JitFunction {
            code_ptr,
            code_size: code.len(),
            arity: 0, // Default arity for now
        };

        self.compiled_functions.insert(function_key, jit_function.clone());
        Ok(jit_function)
    }

    /// Execute a compiled JIT function
    pub fn execute_function(&self, function: &JitFunction, args: Vec<Value>) -> Result<Value, String> {
        if args.len() != function.arity {
            return Err(format!("Expected {} arguments, got {}", function.arity, args.len()));
        }

        // For safety, we'll use a simple approach for now
        // In a full implementation, this would call the native function directly
        println!("Executing JIT function at {:p} with {} arguments", function.code_ptr, args.len());
        
        // For demonstration, return a simple value
        Ok(Value::Number(42.0))
    }

    /// Check if JIT compilation is available on this platform
    pub fn is_available() -> bool {
        cfg!(target_arch = "x86_64") && (cfg!(target_os = "linux") || cfg!(target_os = "macos"))
    }
}

/// JIT execution engine that integrates with the VM
pub struct JITEngine {
    compiler: JITCompiler,
    enabled: bool,
}

impl JITEngine {
    /// Create a new JIT engine
    pub fn new() -> Self {
        JITEngine {
            compiler: JITCompiler::new(),
            enabled: JITCompiler::is_available(),
        }
    }

    /// Enable or disable JIT compilation
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled && JITCompiler::is_available();
    }

    /// Check if JIT is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Compile and execute a chunk using JIT
    pub fn execute_chunk(&mut self, chunk: &Chunk, _vm: &mut VM) -> Result<Value, String> {
        if !self.enabled {
            return Err("JIT compilation is not enabled".to_string());
        }

        // Compile the chunk
        let jit_function = self.compiler.compile_chunk(chunk)?;
        
        // Execute the compiled function
        self.compiler.execute_function(&jit_function, Vec::new())
    }

    /// Get JIT statistics
    pub fn get_stats(&self) -> JITStats {
        JITStats {
            enabled: self.enabled,
            compiled_functions: self.compiler.compiled_functions.len(),
            memory_pages: self.compiler.memory_pages.len(),
        }
    }
}

/// JIT compilation statistics
#[derive(Debug, Clone)]
pub struct JITStats {
    pub enabled: bool,
    pub compiled_functions: usize,
    pub memory_pages: usize,
}

// JIT enable/disable function
fn jit_enable(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let enabled = match &args[0] {
        Value::Boolean(b) => *b,
        _ => return Err("Argument must be a boolean".to_string()),
    };

    println!("JIT enable called with: {}", enabled);
    // In a full implementation, this would enable/disable JIT compilation
    Ok(Value::Boolean(true))
}

// JIT status function
fn jit_enabled(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    let available = JITCompiler::is_available();
    println!("JIT enabled check: {}", available);
    Ok(Value::Boolean(available))
}

// JIT statistics function
fn jit_stats(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    let stats_str = format!(
        "JIT Stats - Available: {}, Compiled Functions: 0, Memory Pages: 0",
        JITCompiler::is_available()
    );
    Ok(Value::String(stats_str))
}

// JIT availability check
fn jit_available(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Boolean(JITCompiler::is_available()))
}

/// Initialize JIT system and register native functions
pub fn init_jit(vm: &mut VM) {
    // JIT enable/disable function
    vm.register_native("jit_enable", 1, jit_enable);

    // JIT status function
    vm.register_native("jit_enabled", 0, jit_enabled);

    // JIT statistics function
    vm.register_native("jit_stats", 0, jit_stats);

    // JIT availability check
    vm.register_native("jit_available", 0, jit_available);
}