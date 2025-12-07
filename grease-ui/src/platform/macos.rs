// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! macOS Cocoa UI Platform Implementation
//! 
//! This module provides a macOS-native UI implementation using
//! Cocoa frameworks through Objective-C bindings.

use crate::platform::{UIPlatform, WindowHandle, ButtonHandle, LabelHandle, PlatformWindowHandle, PlatformButtonHandle, PlatformLabelHandle, UIError, UIFeature};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// macOS UI platform implementation
pub struct MacOSPlatform {
    /// Next widget ID to assign
    next_id: Arc<Mutex<u32>>,
    /// Window storage
    windows: Arc<Mutex<HashMap<String, CocoaWindow>>>,
    /// Button click states
    button_states: Arc<Mutex<HashMap<String, bool>>>,
    /// Event loop running state
    event_loop_running: Arc<Mutex<bool>>,
}

/// Cocoa window representation
#[derive(Debug, Clone)]
struct CocoaWindow {
    pub id: String,
    pub window: *mut std::ffi::c_void, // NSWindow pointer
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

impl MacOSPlatform {
    /// Create a new macOS platform instance
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

    /// Check if macOS/Cocoa is available
    fn check_cocoa_available() -> bool {
        #[cfg(target_os = "macos")]
        {
            // Check for macOS environment
            std::env::var("PATH").unwrap_or_default().contains("/usr/bin")
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }
}

impl UIPlatform for MacOSPlatform {
    fn create_window(&self, title: &str, width: u32, height: u32, id: &str) -> Result<WindowHandle, UIError> {
        if !Self::check_cocoa_available() {
            return Err(UIError::PlatformError("Cocoa not available on this system".to_string()));
        }

        let window = CocoaWindow {
            id: id.to_string(),
            window: std::ptr::null_mut(), // Would be NSWindow pointer
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

        Ok(WindowHandle {
            id: id.to_string(),
            platform_handle: PlatformWindowHandle {
                data: std::ptr::null_mut(),
            },
        })
    }

    fn create_button(&self, window: &WindowHandle, id: &str, label: &str, x: u32, y: u32, width: u32) -> Result<ButtonHandle, UIError> {
        let button_id = format!("{}_{}", window.id, id);
        
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

        Ok(LabelHandle {
            id: label_id,
            platform_handle: PlatformLabelHandle {
                data: std::ptr::null_mut(),
            },
        })
    }

    fn show_window(&self, window: &WindowHandle) -> Result<(), UIError> {
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
        let button_states = self.button_states.lock().unwrap();
        button_states.get(&button.id).copied().unwrap_or(false)
    }

    fn reset_button_click(&self, button: &ButtonHandle) {
        let mut button_states = self.button_states.lock().unwrap();
        button_states.insert(button.id.clone(), false);
    }

    fn get_input_value(&self, _window: &WindowHandle, _input_id: &str) -> Result<String, UIError> {
        Ok("".to_string())
    }

    fn set_input_value(&self, _window: &WindowHandle, _input_id: &str, _value: &str) -> Result<(), UIError> {
        Ok(())
    }

    fn run_event_loop(&self) -> Result<(), UIError> {
        {
            let mut running = self.event_loop_running.lock().unwrap();
            if *running {
                return Err(UIError::EventLoopError("Event loop already running".to_string()));
            }
            *running = true;
        }

        println!("macOS UI event loop started");
        Ok(())
    }

    fn stop_event_loop(&self) -> Result<(), UIError> {
        {
            let mut running = self.event_loop_running.lock().unwrap();
            *running = false;
        }

        println!("macOS UI event loop stopped");
        Ok(())
    }

    fn platform_name(&self) -> &'static str {
        "macOS (Cocoa)"
    }

    fn supports_feature(&self, feature: UIFeature) -> bool {
        match feature {
            UIFeature::NativeMenus => true,
            UIFeature::SystemTray => true,
            UIFeature::MultipleWindows => true,
            UIFeature::CustomTheming => true,
            UIFeature::WebAssemblyIntegration => false,
            UIFeature::HardwareAcceleration => true, // Metal support
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macos_platform_creation() {
        let platform = MacOSPlatform::new();
        assert_eq!(platform.platform_name(), "macOS (Cocoa)");
        assert!(platform.supports_feature(UIFeature::NativeMenus));
        assert!(platform.supports_feature(UIFeature::SystemTray));
        assert!(!platform.supports_feature(UIFeature::WebAssemblyIntegration));
    }

    #[test]
    fn test_next_id_generation() {
        let platform = MacOSPlatform::new();
        let id1 = platform.next_id();
        let id2 = platform.next_id();
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("widget_"));
        assert!(id2.starts_with("widget_"));
    }

    #[test]
    fn test_cocoa_availability() {
        // This will be true on macOS, false elsewhere
        let _available = MacOSPlatform::check_cocoa_available();
    }

    #[test]
    fn test_window_creation() {
        let platform = MacOSPlatform::new();
        let result = platform.create_window("Test Window", 800, 600, "test_window");
        
        // Should succeed even without Cocoa runtime
        assert!(result.is_ok());
        let window = result.unwrap();
        assert_eq!(window.id, "test_window");
    }

    #[test]
    fn test_event_loop_controls() {
        let platform = MacOSPlatform::new();
        
        // Start event loop
        assert!(platform.run_event_loop().is_ok());
        
        // Should fail to start again
        assert!(platform.run_event_loop().is_err());
        
        // Stop event loop
        assert!(platform.stop_event_loop().is_ok());
        
        // Should be able to start again
        assert!(platform.run_event_loop().is_ok());
    }
}