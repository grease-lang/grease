// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Platform Abstraction for UI Module
//! 
//! This module provides a cross-platform abstraction layer for UI functionality
//! that can be implemented differently on each target platform:
//! - Linux: GTK native integration
//! - macOS: Cocoa native integration  
//! - Windows: Win32/GTK support
//! - Android: Native Android Activities and Views


use std::fmt;
use crate::module_errors::UIError;

/// UI handle for window management
#[derive(Debug, Clone)]
pub struct WindowHandle {
    pub id: String,
    pub platform_handle: PlatformWindowHandle,
}

/// UI handle for button widgets
#[derive(Debug, Clone)]
pub struct ButtonHandle {
    pub id: String,
    pub platform_handle: PlatformButtonHandle,
}

/// UI handle for label widgets
#[derive(Debug, Clone)]
pub struct LabelHandle {
    pub id: String,
    pub platform_handle: PlatformLabelHandle,
}

/// Platform-specific window handle (opaque to users)
#[derive(Debug, Clone)]
pub struct PlatformWindowHandle {
    // Platform-specific implementation details
    pub(crate) data: *mut std::ffi::c_void,
}

/// Platform-specific button handle (opaque to users)
#[derive(Debug, Clone)]
pub struct PlatformButtonHandle {
    // Platform-specific implementation details
    pub(crate) data: *mut std::ffi::c_void,
}

/// Platform-specific label handle (opaque to users)
#[derive(Debug, Clone)]
pub struct PlatformLabelHandle {
    // Platform-specific implementation details
    pub(crate) data: *mut std::ffi::c_void,
}

/// Cross-platform UI platform trait
pub trait UIPlatform {
    /// Create a new window with specified properties
    fn create_window(&self, title: &str, width: u32, height: u32, id: &str) -> Result<WindowHandle, UIError>;
    
    /// Create a button widget in the specified window
    fn create_button(&self, window: &WindowHandle, id: &str, label: &str, x: u32, y: u32, width: u32) -> Result<ButtonHandle, UIError>;
    
    /// Create a label widget in the specified window
    fn create_label(&self, window: &WindowHandle, id: &str, text: &str, x: u32, y: u32) -> Result<LabelHandle, UIError>;
    
    /// Show a window and make it visible
    fn show_window(&self, window: &WindowHandle) -> Result<(), UIError>;
    
    /// Hide a window
    fn hide_window(&self, window: &WindowHandle) -> Result<(), UIError>;
    
    /// Check if a button was clicked since last check
    fn button_clicked(&self, button: &ButtonHandle) -> bool;
    
    /// Reset button clicked state
    fn reset_button_click(&self, button: &ButtonHandle);
    
    /// Get text value from an input field
    fn get_input_value(&self, window: &WindowHandle, input_id: &str) -> Result<String, UIError>;
    
    /// Set text value for an input field
    fn set_input_value(&self, window: &WindowHandle, input_id: &str, value: &str) -> Result<(), UIError>;
    
    /// Run the main UI event loop
    fn run_event_loop(&self) -> Result<(), UIError>;
    
    /// Stop the UI event loop
    fn stop_event_loop(&self) -> Result<(), UIError>;
    
    /// Get platform name for debugging
    fn platform_name(&self) -> &'static str;
    
    /// Check if platform supports specific features
    fn supports_feature(&self, feature: UIFeature) -> bool;
}

/// UI features that may or may not be supported on all platforms
#[derive(Debug, Clone, PartialEq)]
pub enum UIFeature {
    NativeMenus,
    SystemTray,
    MultipleWindows,
    CustomTheming,

    HardwareAcceleration,
}

/// UI error types
#[derive(Debug, PartialEq)]
pub enum UIError {
    /// Platform-specific error
    PlatformError(String),
    
    /// Window operation failed
    WindowError(String),
    
    /// Widget operation failed
    WidgetError(String),
    
    /// Event loop error
    EventLoopError(String),
    
    /// Feature not supported on current platform
    UnsupportedFeature(UIFeature),
    
    /// Resource allocation failed
    ResourceError(String),
}

impl fmt::Display for UIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIError::PlatformError(msg) => write!(f, "Platform error: {}", msg),
            UIError::WindowError(msg) => write!(f, "Window error: {}", msg),
            UIError::WidgetError(msg) => write!(f, "Widget error: {}", msg),
            UIError::EventLoopError(msg) => write!(f, "Event loop error: {}", msg),
            UIError::UnsupportedFeature(feature) => write!(f, "Unsupported feature: {:?}", feature),
            UIError::ResourceError(msg) => write!(f, "Resource error: {}", msg),
        }
    }
}

impl std::error::Error for UIError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    
    fn description(&self) -> Option<String> {
        Some(format!("UI Error: {}", self))
    }
}

/// Default implementation that reports all features as unsupported
pub struct UnsupportedPlatform;

impl UIPlatform for UnsupportedPlatform {
    fn create_window(&self, _title: &str, _width: u32, _height: u32, _id: &str) -> Result<WindowHandle, UIError> {
        Err(UIError::PlatformError("UI not supported on this platform".to_string()))
    }
    
    fn create_button(&self, _window: &WindowHandle, _id: &str, _label: &str, _x: u32, _y: u32, _width: u32) -> Result<ButtonHandle, UIError> {
        Err(UIError::PlatformError("UI not supported on this platform".to_string()))
    }
    
    fn create_label(&self, _window: &WindowHandle, _id: &str, _text: &str, _x: u32, _y: u32) -> Result<LabelHandle, UIError> {
        Err(UIError::PlatformError("UI not supported on this platform".to_string()))
    }
    
    fn show_window(&self, _window: &WindowHandle) -> Result<(), UIError> {
        Err(UIError::PlatformError("UI not supported on this platform".to_string()))
    }
    
    fn hide_window(&self, _window: &WindowHandle) -> Result<(), UIError> {
        Err(UIError::PlatformError("UI not supported on this platform".to_string()))
    }
    
    fn button_clicked(&self, _button: &ButtonHandle) -> bool {
        false
    }
    
    fn reset_button_click(&self, _button: &ButtonHandle) {
        // No-op for unsupported platform
    }
    
    fn get_input_value(&self, _window: &WindowHandle, _input_id: &str) -> Result<String, UIError> {
        Err(UIError::PlatformError("UI not supported on this platform".to_string()))
    }
    
    fn set_input_value(&self, _window: &WindowHandle, _input_id: &str, _value: &str) -> Result<(), UIError> {
        Err(UIError::PlatformError("UI not supported on this platform".to_string()))
    }
    
    fn run_event_loop(&self) -> Result<(), UIError> {
        Err(UIError::PlatformError("UI not supported on this platform".to_string()))
    }
    
    fn stop_event_loop(&self) -> Result<(), UIError> {
        Err(UIError::PlatformError("UI not supported on this platform".to_string()))
    }
    
    fn platform_name(&self) -> &'static str {
        "Unsupported"
    }
    
    fn supports_feature(&self, _feature: UIFeature) -> bool {
        false
    }
}

// Platform module declarations
#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "android")]
pub mod android;



/// Create a platform-specific UI implementation based on compile-time target
pub fn create_platform() -> Box<dyn UIPlatform> {
    #[cfg(target_os = "linux")]
    {
        Box::new(crate::platform::linux::LinuxPlatform::new())
    }
    
    #[cfg(target_os = "macos")]
    {
        Box::new(crate::platform::macos::MacOSPlatform::new())
    }
    
    #[cfg(target_os = "windows")]
    {
        Box::new(crate::platform::windows::WindowsPlatform::new())
    }
    
    #[cfg(target_os = "android")]
    {
        Box::new(crate::platform::android::AndroidPlatform::new())
    }
    

    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows", target_os = "android")))]
    {
        Box::new(UnsupportedPlatform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unsupported_platform() {
        let platform = UnsupportedPlatform;
        assert_eq!(platform.platform_name(), "Unsupported");
        assert!(!platform.supports_feature(UIFeature::NativeMenus));
    }
    
    #[test]
    fn test_ui_error_display() {
        let error = UIError::PlatformError("test error".to_string());
        assert_eq!(error.to_string(), "UI Error: Platform error: test error");
    }
    
    #[test]
    fn test_ui_handle_creation() {
        let window = WindowHandle {
            id: "test-window".to_string(),
            platform_handle: PlatformWindowHandle { data: std::ptr::null_mut() },
        };
        
        assert_eq!(window.id, "test-window");
    }
}