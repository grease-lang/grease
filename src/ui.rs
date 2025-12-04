// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! UI Kit for Grease Programming Language
//!
//! This module provides a simple, pure Rust UI toolkit that integrates
//! seamlessly with the Grease virtual machine through native functions.
//! Built on egui/eframe for cross-platform desktop and future web support.

use crate::vm::VM;
use crate::bytecode::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use eframe::egui;
use eframe::{App, Frame};
use eframe::egui::{Context, Ui};

static UI_STATE: Mutex<Option<Arc<Mutex<UiState>>>> = Mutex::new(None);

fn get_ui_state() -> Result<Arc<Mutex<UiState>>, String> {
    let state = UI_STATE.lock().unwrap();
    match &*state {
        Some(s) => Ok(Arc::clone(s)),
        None => Err("UI system not initialized. Call ui_run() first.".to_string()),
    }
}

/// Represents a UI window
#[derive(Clone)]
struct UiWindow {
    title: String,
    width: f64,
    height: f64,
    visible: bool,
}

/// Represents a UI button
#[derive(Clone)]
struct UiButton {
    label: String,
    x: f64,
    y: f64,
    width: f64,
    clicked: bool,
}

/// Represents a UI label
#[derive(Clone)]
struct UiLabel {
    text: String,
    x: f64,
    y: f64,
}

/// Represents a UI input field
#[derive(Clone)]
struct UiInput {
    label: String,
    x: f64,
    y: f64,
    width: f64,
    value: String,
}

/// Global UI state shared between VM and UI thread
pub struct UiState {
    pub windows: HashMap<String, UiWindow>,
    pub buttons: HashMap<String, UiButton>,
    pub labels: HashMap<String, UiLabel>,
    pub inputs: HashMap<String, UiInput>,
    pub running: bool,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            buttons: HashMap::new(),
            labels: HashMap::new(),
            inputs: HashMap::new(),
            running: false,
        }
    }
}

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

    let state = get_ui_state()?;
    let mut state = state.lock().unwrap();

    let window = UiWindow {
        title,
        width,
        height,
        visible: false,
    };

    state.windows.insert(window_id.clone(), window);
    Ok(Value::String(window_id))
}

// UI Window show function
fn ui_window_show(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let window_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Argument must be a string (window_id)".to_string()),
    };

    let state = get_ui_state()?;
    let mut state = state.lock().unwrap();

    if let Some(window) = state.windows.get_mut(&window_id) {
        window.visible = true;
        Ok(Value::Boolean(true))
    } else {
        Err(format!("Window '{}' not found", window_id))
    }
}

// UI Window hide function
fn ui_window_hide(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let window_id = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Argument must be a string (window_id)".to_string()),
    };

    let state = get_ui_state()?;
    let mut state = state.lock().unwrap();

    if let Some(window) = state.windows.get_mut(&window_id) {
        window.visible = false;
        Ok(Value::Boolean(true))
    } else {
        Err(format!("Window '{}' not found", window_id))
    }
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

    let state = get_ui_state()?;
    let mut state = state.lock().unwrap();

    // Check if window exists
    if !state.windows.contains_key(&window_id) {
        return Err(format!("Window '{}' not found", window_id));
    }

    let button = UiButton {
        label,
        x,
        y,
        width,
        clicked: false,
    };

    let full_button_id = format!("{}_{}", window_id, button_id);
    state.buttons.insert(full_button_id, button);
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

    let state = get_ui_state()?;
    let mut state = state.lock().unwrap();

    // Check if window exists
    if !state.windows.contains_key(&window_id) {
        return Err(format!("Window '{}' not found", window_id));
    }

    let label = UiLabel {
        text,
        x,
        y,
    };

    let full_label_id = format!("{}_{}", window_id, label_id);
    state.labels.insert(full_label_id, label);
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

    let state = get_ui_state()?;
    let mut state = state.lock().unwrap();

    // Check if window exists
    if !state.windows.contains_key(&window_id) {
        return Err(format!("Window '{}' not found", window_id));
    }

    let input = UiInput {
        label,
        x,
        y,
        width,
        value: String::new(),
    };

    let full_input_id = format!("{}_{}", window_id, input_id);
    state.inputs.insert(full_input_id, input);
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

    let state = get_ui_state()?;
    let mut state = state.lock().unwrap();

    let full_button_id = format!("{}_{}", window_id, button_id);
    if let Some(button) = state.buttons.get_mut(&full_button_id) {
        let was_clicked = button.clicked;
        button.clicked = false; // Reset after checking
        Ok(Value::Boolean(was_clicked))
    } else {
        Err(format!("Button '{}' not found in window '{}'", button_id, window_id))
    }
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

    let state = get_ui_state()?;
    let state = state.lock().unwrap();

    let full_input_id = format!("{}_{}", window_id, input_id);
    if let Some(input) = state.inputs.get(&full_input_id) {
        Ok(Value::String(input.value.clone()))
    } else {
        Err(format!("Input '{}' not found in window '{}'", input_id, window_id))
    }
}

// UI Run function
fn ui_run(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    // Initialize UI state if not already done
    {
        let mut ui_state = UI_STATE.lock().unwrap();
        if ui_state.is_none() {
            *ui_state = Some(Arc::new(Mutex::new(UiState::new())));
        }
    }

    let state = get_ui_state()?;
    {
        let mut state = state.lock().unwrap();
        state.running = true;
    }

    // Create eframe options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // Clone the Arc for the UI thread
    let ui_state_arc = Arc::clone(&state);

    // Run eframe (this will block until the window is closed)
    eframe::run_native(
        "Grease UI",
        options,
        Box::new(move |_cc| {
            // Create a wrapper that holds the Arc
            struct UiApp {
                state: Arc<Mutex<UiState>>,
            }

            impl App for UiApp {
                fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
                    let state = &mut *self.state.lock().unwrap();

                    // Render all visible windows
                    for (window_id, window) in &state.windows.clone() {
                        if window.visible {
                            egui::Window::new(&window.title)
                                .default_size([window.width as f32, window.height as f32])
                                .show(ctx, |ui| {
                                    Self::render_window_content(state, ui, window_id);
                                });
                        }
                    }

                    // Request repaint to keep the UI responsive
                    ctx.request_repaint();
                }
            }

            impl UiApp {
                fn render_window_content(state: &mut UiState, ui: &mut Ui, window_id: &str) {
                    // Render labels
                    for (label_id, label) in &state.labels.clone() {
                        if label_id.starts_with(&format!("{}_", window_id)) {
                            ui.label(&label.text);
                        }
                    }

                    // Render inputs
                    for (input_id, input) in &mut state.inputs {
                        if input_id.starts_with(&format!("{}_", window_id)) {
                            ui.horizontal(|ui| {
                                ui.label(&input.label);
                                ui.text_edit_singleline(&mut input.value);
                            });
                        }
                    }

                    // Render buttons
                    for (button_id, button) in &mut state.buttons {
                        if button_id.starts_with(&format!("{}_", window_id)) {
                            if ui.button(&button.label).clicked() {
                                button.clicked = true;
                            }
                        }
                    }
                }
            }

            Ok(Box::new(UiApp { state: ui_state_arc }))
        }),
    ).map_err(|e| format!("Failed to start UI: {}", e))?;

    Ok(Value::Boolean(true))
}

// UI Stop function
fn ui_stop(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    let state = get_ui_state()?;
    let mut state = state.lock().unwrap();
    state.running = false;
    Ok(Value::Boolean(true))
}

/// Initialize the UI system and register all native functions
pub fn init_ui(vm: &mut VM) {
    println!("DEBUG: Initializing UI functions");
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