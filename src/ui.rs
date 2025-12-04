// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! UI Kit for Grease Programming Language
//! 
//! This module provides a simple, pure Rust UI toolkit that integrates
//! seamlessly with the Grease virtual machine through native functions.

use crate::vm::VM;
use crate::bytecode::Value;

// UI Window creation function
fn ui_window_create(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let title = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("First argument must be a string (title)".to_string()),
    };
    let width = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err("Second argument must be a number (width)".to_string()),
    };
    let height = match &args[2] {
        Value::Number(n) => *n,
        _ => return Err("Third argument must be a number (height)".to_string()),
    };
    let window_id = match &args[3] {
        Value::String(s) => s.clone(),
        _ => return Err("Fourth argument must be a string (window_id)".to_string()),
    };

    println!("UI Window Created: '{}' ({}x{}) with ID: {}", title, width, height, window_id);
    Ok(Value::String(window_id))
}

// UI Window show function
fn ui_window_show(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let window_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Argument must be a string (window_id)".to_string()),
    };

    println!("UI Window Show: {}", window_id);
    Ok(Value::Boolean(true))
}

// UI Window hide function
fn ui_window_hide(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let window_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Argument must be a string (window_id)".to_string()),
    };

    println!("UI Window Hide: {}", window_id);
    Ok(Value::Boolean(true))
}

// UI Button add function
fn ui_button_add(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let window_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("First argument must be a string (window_id)".to_string()),
    };
    let button_id = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("Second argument must be a string (button_id)".to_string()),
    };
    let label = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err("Third argument must be a string (label)".to_string()),
    };
    let x = match &args[3] {
        Value::Number(n) => *n,
        _ => return Err("Fourth argument must be a number (x)".to_string()),
    };
    let y = match &args[4] {
        Value::Number(n) => *n,
        _ => return Err("Fifth argument must be a number (y)".to_string()),
    };
    let width = match &args[5] {
        Value::Number(n) => *n,
        _ => return Err("Sixth argument must be a number (width)".to_string()),
    };

    println!("UI Button Added: '{}' at ({}, {}) size {}x{} in window {}", 
             label, x, y, width, 30, window_id);
    Ok(Value::Boolean(true))
}

// UI Label add function
fn ui_label_add(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let window_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("First argument must be a string (window_id)".to_string()),
    };
    let label_id = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("Second argument must be a string (label_id)".to_string()),
    };
    let text = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err("Third argument must be a string (text)".to_string()),
    };
    let x = match &args[3] {
        Value::Number(n) => *n,
        _ => return Err("Fourth argument must be a number (x)".to_string()),
    };
    let y = match &args[4] {
        Value::Number(n) => *n,
        _ => return Err("Fifth argument must be a number (y)".to_string()),
    };

    println!("UI Label Added: '{}' at ({}, {}) in window {}", text, x, y, window_id);
    Ok(Value::Boolean(true))
}

// UI Input add function
fn ui_input_add(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let window_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("First argument must be a string (window_id)".to_string()),
    };
    let input_id = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("Second argument must be a string (input_id)".to_string()),
    };
    let label = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err("Third argument must be a string (label)".to_string()),
    };
    let x = match &args[3] {
        Value::Number(n) => *n,
        _ => return Err("Fourth argument must be a number (x)".to_string()),
    };
    let y = match &args[4] {
        Value::Number(n) => *n,
        _ => return Err("Fifth argument must be a number (y)".to_string()),
    };
    let width = match &args[5] {
        Value::Number(n) => *n,
        _ => return Err("Sixth argument must be a number (width)".to_string()),
    };

    println!("UI Input Added: '{}' at ({}, {}) width {} in window {}", label, x, y, width, window_id);
    Ok(Value::Boolean(true))
}

// UI Button clicked check function
fn ui_button_clicked(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let window_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("First argument must be a string (window_id)".to_string()),
    };
    let button_id = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("Second argument must be a string (button_id)".to_string()),
    };

    println!("UI Button Clicked Check: {} in window {}", button_id, window_id);
    // In a real implementation, this would check actual UI state
    Ok(Value::Boolean(false))
}

// UI Input get value function
fn ui_input_get_value(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let window_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("First argument must be a string (window_id)".to_string()),
    };
    let input_id = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err("Second argument must be a string (input_id)".to_string()),
    };

    println!("UI Input Get Value: {} in window {}", input_id, window_id);
    // In a real implementation, this would return the actual input value
    Ok(Value::String(String::new()))
}

// UI Run function
fn ui_run(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    println!("UI event loop started (simplified implementation)");
    println!("In a full implementation, this would start the actual UI event loop");
    Ok(Value::Boolean(true))
}

// UI Stop function
fn ui_stop(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    println!("UI event loop stopped");
    Ok(Value::Boolean(true))
}

/// Initialize the UI system and register all native functions
pub fn init_ui(vm: &mut VM) {
    // Window management functions
    vm.register_native("ui_window_create", 4, ui_window_create);
    vm.register_native("ui_window_show", 1, ui_window_show);
    vm.register_native("ui_window_hide", 1, ui_window_hide);
    
    // Widget creation functions
    vm.register_native("ui_button_add", 6, ui_button_add);
    vm.register_native("ui_label_add", 5, ui_label_add);
    vm.register_native("ui_input_add", 6, ui_input_add);
    
    // Event handling functions
    vm.register_native("ui_button_clicked", 2, ui_button_clicked);
    vm.register_native("ui_input_get_value", 2, ui_input_get_value);
    
    // Main UI loop functions
    vm.register_native("ui_run", 0, ui_run);
    vm.register_native("ui_stop", 0, ui_stop);
}