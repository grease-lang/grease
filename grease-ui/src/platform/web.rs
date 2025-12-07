// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Web-based UI Platform Implementation
//! 
//! This module provides a WebAssembly-based UI implementation that works
//! in web browsers using DOM manipulation and JavaScript interop.

use crate::platform::{UIPlatform, WindowHandle, ButtonHandle, LabelHandle, PlatformWindowHandle, PlatformButtonHandle, PlatformLabelHandle, UIError, UIFeature};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Web-based UI platform implementation
pub struct WebPlatform {
    /// Next widget ID to assign
    next_id: Arc<Mutex<u32>>,
    /// Window storage
    windows: Arc<Mutex<HashMap<String, WebWindow>>>,
    /// Button click states
    button_states: Arc<Mutex<HashMap<String, bool>>>,
    /// Event loop running state
    event_loop_running: Arc<Mutex<bool>>,
}

/// Web window representation
#[derive(Debug, Clone)]
struct WebWindow {
    pub id: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

impl WebPlatform {
    /// Create a new web platform instance
    pub fn new() -> Self {
        Self {
            next_id: Arc::new(Mutex::new(1)),
            windows: Arc::new(Mutex::new(HashMap::new())),
            button_states: Arc::new(Mutex::new(HashMap::new())),
            event_loop_running: Arc::new(Mutex::new(false)),
        }
    }

    /// Generate next unique widget ID
    fn next_id(&self) -> String {
        let mut next = self.next_id.lock().unwrap();
        let id = format!("widget_{}", *next);
        *next += 1;
        id
    }

    /// Execute JavaScript code in browser context
    fn execute_js(&self, code: &str) -> Result<(), UIError> {
        #[cfg(target_arch = "wasm32")]
        {
            // In WebAssembly, we would use wasm-bindgen to execute JavaScript
            // For now, this is a placeholder
            web_sys::console::log_1(&format!("Executing JS: {}", code).into());
            Ok(())
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // For testing outside of browser
            println!("Mock JS execution: {}", code);
            Ok(())
        }
    }

    /// Create DOM element for window
    fn create_window_dom(&self, window: &WebWindow) -> String {
        format!(
            r#"
            if (!document.getElementById('{id}')) {{
                const div = document.createElement('div');
                div.id = '{id}';
                div.style.position = 'fixed';
                div.style.left = '50%';
                div.style.top = '50%';
                div.style.transform = 'translate(-50%, -50%)';
                div.style.width = '{width}px';
                div.style.height = '{height}px';
                div.style.border = '1px solid #ccc';
                div.style.backgroundColor = '#fff';
                div.style.boxShadow = '0 4px 8px rgba(0,0,0,0.1)';
                div.style.zIndex = '1000';
                div.innerHTML = '<h3>{title}</h3><div id="{id}_content"></div>';
                document.body.appendChild(div);
            }}
            "#,
            id = window.id,
            title = window.title,
            width = window.width,
            height = window.height
        )
    }

    /// Create DOM element for button
    fn create_button_dom(&self, window_id: &str, button_id: &str, label: &str, x: u32, y: u32, width: u32) -> String {
        format!(
            r#"
            const button = document.createElement('button');
            button.id = '{button_id}';
            button.textContent = '{label}';
            button.style.position = 'absolute';
            button.style.left = '{x}px';
            button.style.top = '{y}px';
            button.style.width = '{width}px';
            button.style.padding = '8px 16px';
            button.style.backgroundColor = '#007bff';
            button.style.color = 'white';
            button.style.border = 'none';
            button.style.borderRadius = '4px';
            button.style.cursor = 'pointer';
            button.onclick = function() {{
                window.greaseButtonClicked = window.greaseButtonClicked || {{}};
                window.greaseButtonClicked['{button_id}'] = true;
            }};
            const content = document.getElementById('{window_id}_content');
            if (content) {{
                content.appendChild(button);
            }}
            "#,
            button_id = button_id,
            label = label,
            x = x,
            y = y,
            width = width,
            window_id = window_id
        )
    }

    /// Create DOM element for label
    fn create_label_dom(&self, window_id: &str, label_id: &str, text: &str, x: u32, y: u32) -> String {
        format!(
            r#"
            const label = document.createElement('div');
            label.id = '{label_id}';
            label.textContent = '{text}';
            label.style.position = 'absolute';
            label.style.left = '{x}px';
            label.style.top = '{y}px';
            label.style.fontFamily = 'Arial, sans-serif';
            label.style.fontSize = '14px';
            label.style.color = '#333';
            const content = document.getElementById('{window_id}_content');
            if (content) {{
                content.appendChild(label);
            }}
            "#,
            label_id = label_id,
            text = text,
            x = x,
            y = y,
            window_id = window_id
        )
    }
}

impl UIPlatform for WebPlatform {
    fn create_window(&self, title: &str, width: u32, height: u32, id: &str) -> Result<WindowHandle, UIError> {
        let window = WebWindow {
            id: id.to_string(),
            title: title.to_string(),
            width,
            height,
            visible: false,
        };

        // Store window
        {
            let mut windows = self.windows.lock().unwrap();
            windows.insert(id.to_string(), window.clone());
        }

        // Create DOM elements
        let js_code = self.create_window_dom(&window);
        self.execute_js(&js_code)?;

        Ok(WindowHandle {
            id: id.to_string(),
            platform_handle: PlatformWindowHandle {
                data: std::ptr::null_mut(), // Web handles are managed differently
            },
        })
    }

    fn create_button(&self, window: &WindowHandle, id: &str, label: &str, x: u32, y: u32, width: u32) -> Result<ButtonHandle, UIError> {
        let button_id = format!("{}_{}", window.id, id);
        
        // Create DOM elements
        let js_code = self.create_button_dom(&window.id, &button_id, label, x, y, width);
        self.execute_js(&js_code)?;

        // Initialize button state
        {
            let mut button_states = self.button_states.lock().unwrap();
            button_states.insert(button_id.clone(), false);
        }

        Ok(ButtonHandle {
            id: button_id,
            platform_handle: PlatformButtonHandle {
                data: std::ptr::null_mut(),
            },
        })
    }

    fn create_label(&self, window: &WindowHandle, id: &str, text: &str, x: u32, y: u32) -> Result<LabelHandle, UIError> {
        let label_id = format!("{}_{}", window.id, id);
        
        // Create DOM elements
        let js_code = self.create_label_dom(&window.id, &label_id, text, x, y);
        self.execute_js(&js_code)?;

        Ok(LabelHandle {
            id: label_id,
            platform_handle: PlatformLabelHandle {
                data: std::ptr::null_mut(),
            },
        })
    }

    fn show_window(&self, window: &WindowHandle) -> Result<(), UIError> {
        let js_code = format!(
            "const elem = document.getElementById('{}'); if (elem) {{ elem.style.display = 'block'; }}",
            window.id
        );
        self.execute_js(&js_code)?;

        // Update window state
        {
            let mut windows = self.windows.lock().unwrap();
            if let Some(w) = windows.get_mut(&window.id) {
                w.visible = true;
            }
        }

        Ok(())
    }

    fn hide_window(&self, window: &WindowHandle) -> Result<(), UIError> {
        let js_code = format!(
            "const elem = document.getElementById('{}'); if (elem) {{ elem.style.display = 'none'; }}",
            window.id
        );
        self.execute_js(&js_code)?;

        // Update window state
        {
            let mut windows = self.windows.lock().unwrap();
            if let Some(w) = windows.get_mut(&window.id) {
                w.visible = false;
            }
        }

        Ok(())
    }

    fn button_clicked(&self, button: &ButtonHandle) -> bool {
        let mut button_states = self.button_states.lock().unwrap();
        
        #[cfg(target_arch = "wasm32")]
        {
            // In WebAssembly, check global JavaScript state
            if let Some(clicked) = web_sys::window()
                .and_then(|w| w.grease_button_clicked())
                .and_then(|obj| js_sys::Reflect::get(&obj, &button.id.into()).ok())
                .and_then(|val| val.as_bool())
            {
                // Reset the state after checking
                web_sys::console::log_1(&format!("Button {} clicked: {}", button.id, clicked).into());
                return clicked;
            }
        }
        
        // For testing or non-wasm environments
        button_states.get(&button.id).copied().unwrap_or(false)
    }

    fn reset_button_click(&self, button: &ButtonHandle) {
        let mut button_states = self.button_states.lock().unwrap();
        button_states.insert(button.id.clone(), false);

        #[cfg(target_arch = "wasm32")]
        {
            // Reset JavaScript state
            if let Some(window) = web_sys::window() {
                let _ = window.grease_reset_button(&button.id.into());
            }
        }
    }

    fn get_input_value(&self, window: &WindowHandle, input_id: &str) -> Result<String, UIError> {
        let full_id = format!("{}_{}", window.id, input_id);
        let js_code = format!(
            "const elem = document.getElementById('{}'); if (elem && elem.value) {{ elem.value; }} else {{ '' }}",
            full_id
        );
        
        #[cfg(target_arch = "wasm32")]
        {
            // In WebAssembly, execute and get result
            if let Some(result) = web_sys::window()
                .and_then(|w| w.eval(&js_code.into()).ok())
                .and_then(|val| val.as_string())
            {
                return Ok(result);
            }
        }
        
        // For testing or non-wasm environments
        Ok("".to_string())
    }

    fn set_input_value(&self, window: &WindowHandle, input_id: &str, value: &str) -> Result<(), UIError> {
        let full_id = format!("{}_{}", window.id, input_id);
        let js_code = format!(
            "const elem = document.getElementById('{}'); if (elem) {{ elem.value = '{}'; }}",
            full_id, value
        );
        self.execute_js(&js_code)
    }

    fn run_event_loop(&self) -> Result<(), UIError> {
        {
            let mut running = self.event_loop_running.lock().unwrap();
            if *running {
                return Err(UIError::EventLoopError("Event loop already running".to_string()));
            }
            *running = true;
        }

        #[cfg(target_arch = "wasm32")]
        {
            // In WebAssembly, the browser handles the event loop
            // We just need to set up event handlers
            self.execute_js("
                window.greaseEventLoopRunning = true;
                console.log('Grease UI event loop started');
            ")?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // For testing, simulate event loop
            println!("Mock event loop started for web platform");
        }

        Ok(())
    }

    fn stop_event_loop(&self) -> Result<(), UIError> {
        {
            let mut running = self.event_loop_running.lock().unwrap();
            *running = false;
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.execute_js("
                window.greaseEventLoopRunning = false;
                console.log('Grease UI event loop stopped');
            ")?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            println!("Mock event loop stopped for web platform");
        }

        Ok(())
    }

    fn platform_name(&self) -> &'static str {
        "Web"
    }

    fn supports_feature(&self, feature: UIFeature) -> bool {
        match feature {
            UIFeature::WebAssemblyIntegration => true,
            UIFeature::MultipleWindows => true,
            UIFeature::CustomTheming => true,
            UIFeature::NativeMenus => false, // Web has different menu system
            UIFeature::SystemTray => false, // Not available in web browsers
            UIFeature::HardwareAcceleration => true, // Browser handles this
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_platform_creation() {
        let platform = WebPlatform::new();
        assert_eq!(platform.platform_name(), "Web");
        assert!(platform.supports_feature(UIFeature::WebAssemblyIntegration));
        assert!(!platform.supports_feature(UIFeature::SystemTray));
    }

    #[test]
    fn test_next_id_generation() {
        let platform = WebPlatform::new();
        let id1 = platform.next_id();
        let id2 = platform.next_id();
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("widget_"));
        assert!(id2.starts_with("widget_"));
    }

    #[test]
    fn test_window_creation() {
        let platform = WebPlatform::new();
        let result = platform.create_window("Test Window", 800, 600, "test_window");
        
        assert!(result.is_ok());
        let window = result.unwrap();
        assert_eq!(window.id, "test_window");
    }

    #[test]
    fn test_button_creation() {
        let platform = WebPlatform::new();
        let window = platform.create_window("Test", 400, 300, "test").unwrap();
        let result = platform.create_button(&window, "btn1", "Click Me", 10, 10, 100);
        
        assert!(result.is_ok());
        let button = result.unwrap();
        assert_eq!(button.id, "test_btn1");
    }

    #[test]
    fn test_label_creation() {
        let platform = WebPlatform::new();
        let window = platform.create_window("Test", 400, 300, "test").unwrap();
        let result = platform.create_label(&window, "label1", "Hello World", 10, 50);
        
        assert!(result.is_ok());
        let label = result.unwrap();
        assert_eq!(label.id, "test_label1");
    }

    #[test]
    fn test_event_loop_controls() {
        let platform = WebPlatform::new();
        
        // Start event loop
        assert!(platform.run_event_loop().is_ok());
        
        // Should fail to start again
        assert!(platform.run_event_loop().is_err());
        
        // Stop event loop
        assert!(platform.stop_event_loop().is_ok());
        
        // Should be able to start again
        assert!(platform.run_event_loop().is_ok());
    }

    #[test]
    fn test_button_click_tracking() {
        let platform = WebPlatform::new();
        let window = platform.create_window("Test", 400, 300, "test").unwrap();
        let button = platform.create_button(&window, "btn1", "Click Me", 10, 10, 100).unwrap();
        
        // Initially not clicked
        assert!(!platform.button_clicked(&button));
        
        // Reset should work
        platform.reset_button_click(&button);
        assert!(!platform.button_clicked(&button));
    }
}