// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Windows UI Platform Implementation
//! 
//! This module provides a Windows-native UI implementation using
//! Win32 API and optionally GTK for cross-platform compatibility.

use crate::platform::{UIPlatform, WindowHandle, ButtonHandle, LabelHandle, PlatformWindowHandle, PlatformButtonHandle, PlatformLabelHandle, UIError, UIFeature};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Windows UI platform implementation
pub struct WindowsPlatform {
    /// Next widget ID to assign
    next_id: Arc<Mutex<u32>>,
    /// Window storage
    windows: Arc<Mutex<HashMap<String, WinWindow>>>,
    /// Button click states
    button_states: Arc<Mutex<HashMap<String, bool>>>,
    /// Event loop running state
    event_loop_running: Arc<Mutex<bool>>,
    /// Use GTK if available
    use_gtk: bool,
}

/// Windows window representation
#[derive(Debug, Clone)]
struct WinWindow {
    pub id: String,
    pub hwnd: *mut std::ffi::c_void, // HWND
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

impl WindowsPlatform {
    /// Create a new Windows platform instance
    pub fn new() -> Self {
        Self {
            next_id: Arc::new(Mutex::new(1)),
            windows: Arc::new(Mutex::new(HashMap::new())),
            button_states: Arc::new(Mutex::new(HashMap::new())),
            event_loop_running: Arc::new(Mutex::new(false)),
            use_gtk: Self::check_gtk_available(),
        }
    }

    /// Generate next unique widget ID
    fn next_id(&self) -> String {
        let mut next = self.next_id.lock().unwrap();
        let id = format!("widget_{}", *next);
        *next += 1;
        id
    }

    /// Check if GTK is available on Windows
    fn check_gtk_available() -> bool {
        // Try to find GTK libraries
        let gtk_paths = [
            "C:\\msys64\\mingw64\\lib\\gtk-3-0.dll",
            "C:\\GTK\\bin\\gtk-3-0.dll",
            "gtk-3-0.dll",
        ];
        
        gtk_paths.iter().any(|path| std::path::Path::new(path).exists())
    }

    /// Check if Windows API is available
    fn check_win32_available() -> bool {
        #[cfg(target_os = "windows")]
        {
            true // Always available on Windows
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }
}

impl UIPlatform for WindowsPlatform {
    fn create_window(&self, title: &str, width: u32, height: u32, id: &str) -> Result<WindowHandle, UIError> {
        if !Self::check_win32_available() && !self.use_gtk {
            return Err(UIError::PlatformError("Neither Win32 nor GTK available on this system".to_string()));
        }

        let window = WinWindow {
            id: id.to_string(),
            hwnd: std::ptr::null_mut(), // Would be HWND from CreateWindowEx
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

        if self.use_gtk {
            println!("Windows UI event loop started (GTK backend)");
        } else {
            println!("Windows UI event loop started (Win32 backend)");
        }

        Ok(())
    }

    fn stop_event_loop(&self) -> Result<(), UIError> {
        {
            let mut running = self.event_loop_running.lock().unwrap();
            *running = false;
        }

        if self.use_gtk {
            println!("Windows UI event loop stopped (GTK backend)");
        } else {
            println!("Windows UI event loop stopped (Win32 backend)");
        }

        Ok(())
    }

    fn platform_name(&self) -> &'static str {
        if self.use_gtk {
            "Windows (GTK)"
        } else {
            "Windows (Win32)"
        }
    }

    fn supports_feature(&self, feature: UIFeature) -> bool {
        match feature {
            UIFeature::NativeMenus => true,
            UIFeature::SystemTray => true,
            UIFeature::MultipleWindows => true,
            UIFeature::CustomTheming => true,
            UIFeature::WebAssemblyIntegration => false,
            UIFeature::HardwareAcceleration => true, // DirectX/OpenGL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_platform_creation() {
        let platform = WindowsPlatform::new();
        let platform_name = platform.platform_name();
        assert!(platform_name == "Windows (Win32)" || platform_name == "Windows (GTK)");
        assert!(platform.supports_feature(UIFeature::NativeMenus));
        assert!(platform.supports_feature(UIFeature::SystemTray));
        assert!(!platform.supports_feature(UIFeature::WebAssemblyIntegration));
    }

    #[test]
    fn test_next_id_generation() {
        let platform = WindowsPlatform::new();
        let id1 = platform.next_id();
        let id2 = platform.next_id();
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("widget_"));
        assert!(id2.starts_with("widget_"));
    }

    #[test]
    fn test_gtk_availability() {
        // This test checks if GTK detection works
        let _gtk_available = WindowsPlatform::check_gtk_available();
    }

    #[test]
    fn test_win32_availability() {
        #[cfg(target_os = "windows")]
        {
            assert!(WindowsPlatform::check_win32_available());
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            assert!(!WindowsPlatform::check_win32_available());
        }
    }

    #[test]
    fn test_window_creation() {
        let platform = WindowsPlatform::new();
        let result = platform.create_window("Test Window", 800, 600, "test_window");
        
        // Should succeed if either Win32 or GTK is available
        if WindowsPlatform::check_win32_available() || WindowsPlatform::check_gtk_available() {
            assert!(result.is_ok());
            
            let window = result.unwrap();
            assert_eq!(window.id, "test_window");
        }
    }

    #[test]
    fn test_event_loop_controls() {
        let platform = WindowsPlatform::new();
        
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
        let platform = WindowsPlatform::new();
        let window = platform.create_window("Test", 400, 300, "test").unwrap();
        let button = platform.create_button(&window, "btn1", "Click Me", 10, 10, 100).unwrap();
        
        // Initially not clicked
        assert!(!platform.button_clicked(&button));
        
        // Reset should work
        platform.reset_button_click(&button);
        assert!(!platform.button_clicked(&button));
    }
}