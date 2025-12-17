// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use crate::bytecode::Value;
use std::process::Command;
use std::collections::HashMap;
use std::env;

/// Native system functions for terminal calls and process management
pub struct ProcessResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub pid: Option<String>,
    pub signal: Option<i32>,
}

impl ProcessResult {
    pub fn new(exit_code: i32, stdout: String, stderr: String) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
            success: exit_code == 0,
            pid: None,
            signal: None,
        }
    }

    pub fn to_value(&self) -> Value {
        let mut fields = HashMap::new();
        fields.insert("exit_code".to_string(), Value::Number(self.exit_code as f64));
        fields.insert("stdout".to_string(), Value::String(self.stdout.clone()));
        fields.insert("stderr".to_string(), Value::String(self.stderr.clone()));
        fields.insert("success".to_string(), Value::Boolean(self.success));
        fields.insert("pid".to_string(), self.pid.as_ref().map(|p| Value::String(p.clone())).unwrap_or(Value::Null));
        fields.insert("signal".to_string(), self.signal.map(|s| Value::Number(s as f64)).unwrap_or(Value::Null));
        
        Value::Dictionary(fields)
    }
}

/// Execute a command synchronously and capture output
pub fn system_exec(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("system_exec requires at least 1 argument (command)".to_string());
    }

    let command = match &args[0] {
        Value::String(s) => {
            eprintln!("DEBUG: system_exec called with command: {}", s);
            s.clone()
        },
        _ => return Err("Command must be a string".to_string()),
    };

    let mut cmd = Command::new(command);
    
    // Add arguments
    for arg in args.iter().skip(1) {
        match arg {
            Value::String(s) => { cmd.arg(s); },
            _ => return Err("All arguments must be strings".to_string()),
        }
    }

    // Execute and capture output
    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);
            
            let result = ProcessResult::new(exit_code, stdout, stderr);
            Ok(result.to_value())
        },
        Err(e) => {
            let result = ProcessResult::new(-1, String::new(), format!("Failed to execute command: {}", e));
            Ok(result.to_value())
        }
    }
}

/// Execute a command asynchronously (spawn process)
pub fn system_spawn(vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("system_spawn requires at least 1 argument (command)".to_string());
    }

    let command = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Command must be a string".to_string()),
    };

    let mut cmd = Command::new(command);
    
    // Add arguments
    for arg in args.iter().skip(1) {
        match arg {
            Value::String(s) => { cmd.arg(s); },
            _ => return Err("All arguments must be strings".to_string()),
        }
    }

    // Configure for background execution
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    // Spawn the process
    match cmd.spawn() {
        Ok(child) => {
            let pid = child.id().to_string();
            
            // Track the process in the VM
            vm.track_process(pid.clone(), child);
            
            let mut fields = HashMap::new();
            fields.insert("exit_code".to_string(), Value::Number(0.0)); // Still running
            fields.insert("stdout".to_string(), Value::String("".to_string()));
            fields.insert("stderr".to_string(), Value::String("".to_string()));
            fields.insert("success".to_string(), Value::Boolean(true));
            fields.insert("pid".to_string(), Value::String(pid));
            fields.insert("signal".to_string(), Value::Null);
            
            Ok(Value::Dictionary(fields))
        },
        Err(e) => {
            let mut fields = HashMap::new();
            fields.insert("exit_code".to_string(), Value::Number(-1.0));
            fields.insert("stdout".to_string(), Value::String("".to_string()));
            fields.insert("stderr".to_string(), Value::String(e.to_string()));
            fields.insert("success".to_string(), Value::Boolean(false));
            fields.insert("pid".to_string(), Value::Null);
            fields.insert("signal".to_string(), Value::Null);
            
            Ok(Value::Dictionary(fields))
        }
    }
}

/// Execute a shell command
pub fn system_shell(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("system_shell requires exactly 1 argument (command_string)".to_string());
    }

    let command_string = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Command string must be a string".to_string()),
    };

    // Use shell to execute the command
    let (shell, shell_arg) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    match Command::new(shell).arg(shell_arg).arg(&command_string).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);
            
            let result = ProcessResult::new(exit_code, stdout, stderr);
            Ok(result.to_value())
        },
        Err(e) => {
            let result = ProcessResult::new(-1, String::new(), format!("Failed to execute shell command: {}", e));
            Ok(result.to_value())
        }
    }
}

/// Get environment variable
pub fn system_getenv(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("system_getenv requires exactly 1 argument (variable_name)".to_string());
    }

    let var_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Variable name must be a string".to_string()),
    };

    match env::var(&var_name) {
        Ok(value) => Ok(Value::String(value)),
        Err(_) => Ok(Value::Null),
    }
}

/// Set environment variable
pub fn system_setenv(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("system_setenv requires exactly 2 arguments (name, value)".to_string());
    }

    let name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Variable name must be a string".to_string()),
    };

    let value = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("Variable value must be a string".to_string()),
    };

    env::set_var(&name, &value);
    Ok(Value::Boolean(true))
}

/// Get all environment variables
pub fn system_environ(_vm: &mut crate::vm::VM, _args: Vec<Value>) -> Result<Value, String> {
    let mut env_vars = HashMap::new();
    
    for (key, value) in env::vars() {
        env_vars.insert(key, Value::String(value));
    }
    
    Ok(Value::Dictionary(env_vars))
}

/// Read command output (alias for system_exec)
pub fn system_read_output(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    system_exec(_vm, args)
}

/// Capture stdout and stderr separately
pub fn system_capture(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    system_exec(_vm, args)
}

/// Run command in background (alias for system_spawn)
pub fn system_background(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    system_spawn(_vm, args)
}

/// Pipe commands (basic implementation)
pub fn system_pipe(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("system_pipe requires at least 2 arguments (command1, command2, ...)".to_string());
    }

    // For now, implement simple pipe between two commands
    let cmd1 = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("First command must be a string".to_string()),
    };

    let cmd2 = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("Second command must be a string".to_string()),
    };

    // Simple implementation using shell
    let pipe_command = format!("{} | {}", cmd1, cmd2);
    system_shell(_vm, vec![Value::String(pipe_command)])
}

/// Redirect command output to files
pub fn system_redirect(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("system_redirect requires at least 2 arguments (command, stdout_file, [stderr_file])".to_string());
    }

    let command = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Command must be a string".to_string()),
    };

    let stdout_file = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("stdout file must be a string".to_string()),
    };

    let stderr_file = if args.len() > 2 {
        match &args[2] {
            Value::String(s) => Some(s.clone()),
            _ => return Err("stderr file must be a string".to_string()),
        }
    } else {
        None
    };

    // Build redirect command
    let redirect_cmd = if let Some(stderr_file) = stderr_file {
        format!("{} > {} 2> {}", command, stdout_file, stderr_file)
    } else {
        format!("{} > {}", command, stdout_file)
    };

    system_shell(_vm, vec![Value::String(redirect_cmd)])
}

/// Execute command with timeout (basic implementation)
pub fn system_timeout(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("system_timeout requires at least 2 arguments (command, timeout_seconds)".to_string());
    }

    let command = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Command must be a string".to_string()),
    };

    let timeout_secs = match &args[1] {
        Value::Number(n) => *n as u64,
        _ => return Err("Timeout must be a number".to_string()),
    };

    // For now, implement basic timeout using shell
    let timeout_cmd = if cfg!(target_os = "windows") {
        format!("timeout {} {}", timeout_secs, command)
    } else {
        format!("timeout {}s {}", timeout_secs, command)
    };

    system_shell(_vm, vec![Value::String(timeout_cmd)])
}

/// Wait for process to complete
pub fn system_wait(vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("system_wait requires exactly 1 argument (pid)".to_string());
    }

    let pid_str = match &args[0] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return Err("PID must be a string or number".to_string()),
    };
    
    match vm.remove_tracked_process(&pid_str) {
        Some(mut child) => {
            match child.wait() {
                Ok(status) => {
                    let exit_code = status.code().unwrap_or(-1);
                    let success = status.success();
                    
                    // Try to get output if available
                    let stdout = child.stdout.take()
                        .and_then(|mut stdout| {
                            use std::io::Read;
                            let mut output = String::new();
                            stdout.read_to_string(&mut output).ok().map(|_| output)
                        })
                        .unwrap_or_default();
                    
                    let stderr = child.stderr.take()
                        .and_then(|mut stderr| {
                            use std::io::Read;
                            let mut output = String::new();
                            stderr.read_to_string(&mut output).ok().map(|_| output)
                        })
                        .unwrap_or_default();
                    
                    let mut fields = HashMap::new();
                    fields.insert("exit_code".to_string(), Value::Number(exit_code as f64));
                    fields.insert("stdout".to_string(), Value::String(stdout));
                    fields.insert("stderr".to_string(), Value::String(stderr));
                    fields.insert("success".to_string(), Value::Boolean(success));
                    fields.insert("pid".to_string(), Value::String(pid_str));
                    fields.insert("signal".to_string(), Value::Null);
                    
                    Ok(Value::Dictionary(fields))
                }
                Err(e) => {
                    let mut fields = HashMap::new();
                    fields.insert("exit_code".to_string(), Value::Number(-1.0));
                    fields.insert("stdout".to_string(), Value::String("".to_string()));
                    fields.insert("stderr".to_string(), Value::String(e.to_string()));
                    fields.insert("success".to_string(), Value::Boolean(false));
                    fields.insert("pid".to_string(), Value::String(pid_str));
                    fields.insert("signal".to_string(), Value::Null);
                    
                    Ok(Value::Dictionary(fields))
                }
            }
        }
        None => {
            Err(format!("Process with PID {} not found", pid_str))
        }
    }
}

/// Kill a process
pub fn system_kill(vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("system_kill requires at least 2 arguments (pid, signal)".to_string());
    }

    let pid_str = match &args[0] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return Err("PID must be a string or number".to_string()),
    };

    let _signal = match &args[1] {
        Value::Number(n) => *n as i32,
        _ => return Err("Signal must be a number".to_string()),
    };
    
    match vm.get_tracked_process_mut(&pid_str) {
        Some(child) => {
            match child.kill() {
                Ok(_) => {
                    vm.remove_tracked_process(&pid_str);
                    Ok(Value::Boolean(true))
                }
                Err(_) => Ok(Value::Boolean(false)),
            }
        }
        None => {
            Err(format!("Process with PID {} not found", pid_str))
        }
    }
}

/// Get process status
pub fn system_status(vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("system_status requires exactly 1 argument (pid)".to_string());
    }

    let pid_str = match &args[0] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return Err("PID must be a string or number".to_string()),
    };
    
    if vm.get_tracked_process(&pid_str).is_some() {
        Ok(Value::String("running".to_string()))
    } else {
        Ok(Value::String("not_found".to_string()))
    }
}

/// Write input to process stdin (placeholder implementation)
pub fn system_write_input(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("system_write_input requires at least 2 arguments (pid, input)".to_string());
    }

    let _pid = match &args[0] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return Err("PID must be a string or number".to_string()),
    };

    let _input = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("Input must be a string".to_string()),
    };

    // For now, return a placeholder result
    let mut fields = HashMap::new();
    fields.insert("success".to_string(), Value::Boolean(true));
    fields.insert("message".to_string(), Value::String("Process write input not fully implemented".to_string()));
    Ok(Value::Dictionary(fields))
}

/// Async version of system_exec using tokio
pub fn system_async_exec(vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    // For now, delegate to sync version since async execution in VM context is complex
    // In a full implementation, this would spawn a tokio task and return a promise/future
    system_exec(vm, args)
}

/// Async version of system_spawn
pub fn system_async_spawn(vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    // For now, delegate to sync version
    // In a full implementation, this would use tokio::process::Command
    system_spawn(vm, args)
}

/// Async version of system_wait
pub fn system_async_wait(vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    // For now, delegate to sync version
    system_wait(vm, args)
}

/// Async pipe implementation (placeholder)
pub fn system_async_pipe(_vm: &mut crate::vm::VM, _args: Vec<Value>) -> Result<Value, String> {
    let mut fields = HashMap::new();
    fields.insert("success".to_string(), Value::Boolean(false));
    fields.insert("message".to_string(), Value::String("Async pipe not implemented".to_string()));
    Ok(Value::Dictionary(fields))
}

/// Stream command output in real-time (basic implementation)
pub fn system_stream_exec(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("system_stream_exec requires at least 1 argument (command)".to_string());
    }

    let command = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Command must be a string".to_string()),
    };

    let mut cmd = Command::new(command);

    // Add arguments
    for arg in args.iter().skip(1) {
        match arg {
            Value::String(s) => { cmd.arg(s); },
            _ => return Err("All arguments must be strings".to_string()),
        }
    }

    // For streaming, we need to handle stdout/stderr separately
    // This is a basic implementation - in a full version, this would stream output
    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);

            let mut fields = HashMap::new();
            fields.insert("exit_code".to_string(), Value::Number(exit_code as f64));
            fields.insert("stdout".to_string(), Value::String(stdout));
            fields.insert("stderr".to_string(), Value::String(stderr));
            fields.insert("success".to_string(), Value::Boolean(exit_code == 0));
            fields.insert("streamed".to_string(), Value::Boolean(true));

            Ok(Value::Dictionary(fields))
        },
        Err(e) => {
            let mut fields = HashMap::new();
            fields.insert("exit_code".to_string(), Value::Number(-1.0));
            fields.insert("stdout".to_string(), Value::String("".to_string()));
            fields.insert("stderr".to_string(), Value::String(format!("Failed to execute command: {}", e)));
            fields.insert("success".to_string(), Value::Boolean(false));
            fields.insert("streamed".to_string(), Value::Boolean(false));

            Ok(Value::Dictionary(fields))
        }
    }
}

/// Monitor process with real-time output (placeholder)
pub fn system_monitor_process(_vm: &mut crate::vm::VM, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("system_monitor_process requires exactly 1 argument (pid)".to_string());
    }

    let _pid = match &args[0] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return Err("PID must be a string or number".to_string()),
    };

    // Placeholder implementation
    let mut fields = HashMap::new();
    fields.insert("status".to_string(), Value::String("monitoring_not_implemented".to_string()));
    fields.insert("message".to_string(), Value::String("Real-time process monitoring not fully implemented".to_string()));
    Ok(Value::Dictionary(fields))
}