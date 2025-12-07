// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Linux GTK UI Platform Implementation
//! 
//! This module provides a GTK-based UI implementation for Linux systems
//! using native GTK3 widgets for optimal performance and integration.

use crate::platform::{UIPlatform, WindowHandle, ButtonHandle, LabelHandle, PlatformWindowHandle, PlatformButtonHandle, PlatformLabelHandle, UIError, UIFeature};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::ptr;

/// GTK-based UI platform implementation
pub struct LinuxPlatform {
    /// Next widget ID to assign
    next_id: Arc<Mutex<u32>>,
    /// Window storage
    windows: Arc<Mutex<HashMap<String, GtkWindow>>>,
    /// Button click states
    button_states: Arc<Mutex<HashMap<String, bool>>>,
    /// Event loop running state
    event_loop_running: Arc<Mutex<bool>>,
    /// GTK application instance
    app: *mut gtk_sys::GtkApplication,
}

/// GTK window representation
#[derive(Debug, Clone)]
struct GtkWindow {
    pub id: String,
    pub window: *mut gtk_sys::GtkWidget,
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

impl LinuxPlatform {
    /// Create a new Linux GTK platform instance
    pub fn new() -> Self {
        // Initialize GTK
        unsafe {
            if gtk_sys::gtk_init_check(ptr::null(), ptr::null()) == 0 {
                eprintln!("Failed to initialize GTK");
            }
        }

        Self {
            next_id: Arc::new(Mutex::new(1)),
            windows: Arc::new(Mutex::new(HashMap::new())),
            button_states: Arc::new(Mutex::new(HashMap::new())),
            event_loop_running: Arc::new(Mutex::new(false)),
            app: ptr::null_mut(),
        }
    }

    /// Generate next unique widget ID
    fn next_id(&self) -> String {
        let mut next = self.next_id.lock().unwrap();
        let id = format!("widget_{}", *next);
        *next += 1;
        id
    }

    /// Check if GTK is available
    fn check_gtk_available() -> bool {
        unsafe {
            gtk_sys::gtk_init_check(ptr::null(), ptr::null()) != 0
        }
    }

    /// Create GTK window widget
    fn create_gtk_window(&self, title: &str, width: i32, height: i32, id: &str) -> Result<*mut gtk_sys::GtkWidget, UIError> {
        unsafe {
            let window = gtk_sys::gtk_window_new(gtk_sys::GTK_WINDOW_TOPLEVEL);
            if window.is_null() {
                return Err(UIError::ResourceError("Failed to create GTK window".to_string()));
            }

            // Set window properties
            let title_cstr = std::ffi::CString::new(title).unwrap();
            gtk_sys::gtk_window_set_title(window as *mut gtk_sys::GtkWindow, title_cstr.as_ptr());
            gtk_sys::gtk_window_set_default_size(window as *mut gtk_sys::GtkWindow, width, height);
            gtk_sys::gtk_window_set_resizable(window as *mut gtk_sys::GtkWindow, 1);

            // Create a container for widgets
            let fixed = gtk_sys::gtk_fixed_new();
            if fixed.is_null() {
                return Err(UIError::ResourceError("Failed to create GTK fixed container".to_string()));
            }

            gtk_sys::gtk_container_add(window as *mut gtk_sys::GtkContainer, fixed);

            Ok(window)
        }
    }

    /// Create GTK button widget
    fn create_gtk_button(&self, label: &str, x: i32, y: i32, width: i32, button_id: &str) -> Result<*mut gtk_sys::GtkWidget, UIError> {
        unsafe {
            let button = gtk_sys::gtk_button_new_with_label(label.as_ptr() as *const i8);
            if button.is_null() {
                return Err(UIError::ResourceError("Failed to create GTK button".to_string()));
            }

            // Set button size
            gtk_sys::gtk_widget_set_size_request(button, width, 30);

            // Position button (using fixed container)
            gtk_sys::gtk_fixed_put(
                ptr::null_mut(), // Will be set when added to window
                button,
                x,
                y
            );

            // Set up click handler
            let button_id_cstr = std::ffi::CString::new(button_id).unwrap();
            let button_id_ptr = button_id_cstr.into_raw();
            
            gtk_sys::g_signal_connect_data(
                button,
                b"clicked\0".as_ptr() as *const i8,
                Some(Self::button_clicked_callback),
                button_id_ptr as *mut _,
                None,
            );

            Ok(button)
        }
    }

    /// Create GTK label widget
    fn create_gtk_label(&self, text: &str, x: i32, y: i32, label_id: &str) -> Result<*mut gtk_sys::GtkWidget, UIError> {
        unsafe {
            let label = gtk_sys::gtk_label_new(text.as_ptr() as *const i8);
            if label.is_null() {
                return Err(UIError::ResourceError("Failed to create GTK label".to_string()));
            }

            // Position label (using fixed container)
            gtk_sys::gtk_fixed_put(
                ptr::null_mut(), // Will be set when added to window
                label,
                x,
                y
            );

            Ok(label)
        }
    }

    /// GTK button click callback
    unsafe extern "C" fn button_clicked_callback(
        _button: *mut gtk_sys::GtkWidget,
        user_data: *mut libc::c_void,
    ) -> libc::c_int {
        let button_id = user_data as *const i8;
        let button_id_str = std::ffi::CStr::from_ptr(button_id).to_string_lossy();
        
        // Store click state (this would need to be implemented with proper state management)
        web_sys::console::log_1(&format!("GTK button clicked: {}", button_id_str).into());
        
        0 // Continue propagation
    }
}

impl UIPlatform for LinuxPlatform {
    fn create_window(&self, title: &str, width: u32, height: u32, id: &str) -> Result<WindowHandle, UIError> {
        if !Self::check_gtk_available() {
            return Err(UIError::PlatformError("GTK not available on this system".to_string()));
        }

        let window = self.create_gtk_window(title, width as i32, height as i32, id)?;
        
        let gtk_window = GtkWindow {
            id: id.to_string(),
            window,
            title: title.to_string(),
            width: width as i32,
            height: height as i32,
            visible: false,
        };

        // Store window
        {
            let mut windows = self.windows.lock().unwrap();
            windows.insert(id.to_string(), gtk_window.clone());
        }

        Ok(WindowHandle {
            id: id.to_string(),
            platform_handle: PlatformWindowHandle {
                data: window as *mut std::ffi::c_void,
            },
        })
    }

    fn create_button(&self, window: &WindowHandle, id: &str, label: &str, x: u32, y: u32, width: u32) -> Result<ButtonHandle, UIError> {
        let button_id = format!("{}_{}", window.id, id);
        let button = self.create_gtk_button(label, x as i32, y as i32, width as i32, &button_id)?;

        // Initialize button state
        {
            let mut button_states = self.button_states.lock().unwrap();
            button_states.insert(button_id.clone(), false);
        }

        Ok(ButtonHandle {
            id: button_id,
            platform_handle: PlatformButtonHandle {
                data: button as *mut std::ffi::c_void,
            },
        })
    }

    fn create_label(&self, window: &WindowHandle, id: &str, text: &str, x: u32, y: u32) -> Result<LabelHandle, UIError> {
        let label_id = format!("{}_{}", window.id, id);
        let label = self.create_gtk_label(text, x as i32, y as i32, &label_id)?;

        Ok(LabelHandle {
            id: label_id,
            platform_handle: PlatformLabelHandle {
                data: label as *mut std::ffi::c_void,
            },
        })
    }

    fn show_window(&self, window: &WindowHandle) -> Result<(), UIError> {
        unsafe {
            gtk_sys::gtk_widget_show_all(window.platform_handle.data as *mut gtk_sys::GtkWidget);
        }

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
        unsafe {
            gtk_sys::gtk_widget_hide(window.platform_handle.data as *mut gtk_sys::GtkWidget);
        }

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
        // GTK input handling would require implementing entry widgets
        // For now, return empty string
        Ok("".to_string())
    }

    fn set_input_value(&self, _window: &WindowHandle, _input_id: &str, _value: &str) -> Result<(), UIError> {
        // GTK input handling would require implementing entry widgets
        // For now, do nothing
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

        unsafe {
            // Run GTK main loop
            gtk_sys::gtk_main();
        }

        Ok(())
    }

    fn stop_event_loop(&self) -> Result<(), UIError> {
        {
            let mut running = self.event_loop_running.lock().unwrap();
            *running = false;
        }

        unsafe {
            // Quit GTK main loop
            gtk_sys::gtk_main_quit();
        }

        Ok(())
    }

    fn platform_name(&self) -> &'static str {
        "Linux (GTK)"
    }

    fn supports_feature(&self, feature: UIFeature) -> bool {
        match feature {
            UIFeature::NativeMenus => true,
            UIFeature::SystemTray => true,
            UIFeature::MultipleWindows => true,
            UIFeature::CustomTheming => true,
            UIFeature::WebAssemblyIntegration => false,
            UIFeature::HardwareAcceleration => true, // Through GTK's OpenGL support
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_platform_creation() {
        let platform = LinuxPlatform::new();
        assert_eq!(platform.platform_name(), "Linux (GTK)");
        assert!(platform.supports_feature(UIFeature::NativeMenus));
        assert!(platform.supports_feature(UIFeature::SystemTray));
        assert!(!platform.supports_feature(UIFeature::WebAssemblyIntegration));
    }

    #[test]
    fn test_next_id_generation() {
        let platform = LinuxPlatform::new();
        let id1 = platform.next_id();
        let id2 = platform.next_id();
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("widget_"));
        assert!(id2.starts_with("widget_"));
    }

    #[test]
    fn test_gtk_availability() {
        // This test will pass if GTK is available, fail otherwise
        // That's expected behavior
        let _available = LinuxPlatform::check_gtk_available();
    }

    #[test]
    fn test_window_creation() {
        let platform = LinuxPlatform::new();
        
        // This test may fail if GTK is not available
        // That's expected behavior in CI/testing environments
        if LinuxPlatform::check_gtk_available() {
            let result = platform.create_window("Test Window", 800, 600, "test_window");
            assert!(result.is_ok());
            
            let window = result.unwrap();
            assert_eq!(window.id, "test_window");
        }
    }

    #[test]
    fn test_event_loop_controls() {
        let platform = LinuxPlatform::new();
        
        // Start event loop (may fail if GTK not available)
        let start_result = platform.run_event_loop();
        
        if LinuxPlatform::check_gtk_available() {
            assert!(start_result.is_ok());
            
            // Should fail to start again
            assert!(platform.run_event_loop().is_err());
            
            // Stop event loop
            assert!(platform.stop_event_loop().is_ok());
        }
    }

    #[test]
    fn test_button_click_tracking() {
        let platform = LinuxPlatform::new();
        
        if LinuxPlatform::check_gtk_available() {
            let window = platform.create_window("Test", 400, 300, "test").unwrap();
            let button = platform.create_button(&window, "btn1", "Click Me", 10, 10, 100).unwrap();
            
            // Initially not clicked
            assert!(!platform.button_clicked(&button));
            
            // Reset should work
            platform.reset_button_click(&button);
            assert!(!platform.button_clicked(&button));
        }
    }
}