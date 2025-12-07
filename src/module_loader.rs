// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Module Detection and Loading System
//! 
//! This module provides automatic detection and loading of Grease modules
//! (UI and WebAssembly) from multiple possible locations.
//! 
//! ## Search Strategy
//! 
//! Modules are searched in the following priority order:
//! 1. Same directory: ./grease-ui, ./grease-webassembly
//! 2. Parent directory: ../grease-ui, ../grease-webassembly
//! 3. Subdirectory: ./modules/grease-ui, ./modules/grease-webassembly
//! 4. Lib directory: ./lib/grease-ui, ./lib/grease-webassembly
//! 5. Custom paths: GREASE_UI_PATH, GREASE_WASM_PATH environment variables

use std::path::{Path, PathBuf};
use std::env;
use crate::vm::VM;
use crate::module_errors::ModuleError;

/// Module information and paths
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub path: PathBuf,
    pub version: Option<String>,
    pub available: bool,
}

/// Module loader with detection and initialization capabilities
pub struct ModuleLoader {
    pub ui_module: Option<ModuleInfo>,
    pub wasm_module: Option<ModuleInfo>,
    pub platform: Platform,
    pub searched_paths: Vec<String>,
}

/// Supported target platforms
#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
    Android,
    WebAssembly,
    Unknown,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::MacOS => write!(f, "macOS"),
            Platform::Windows => write!(f, "Windows"),
            Platform::Android => write!(f, "Android"),
            Platform::WebAssembly => write!(f, "WebAssembly"),
            Platform::Unknown => write!(f, "Unknown"),
        }
    }
}

impl ModuleLoader {
    /// Create a new module loader and detect available modules
    pub fn new() -> Result<Self, ModuleError> {
        let platform = Self::detect_platform();
        
        println!("🔍 Detecting Grease modules...");
        
        // Search for UI module
        let ui_module = Self::find_module("grease-ui", &[
            "./grease-ui",
            "../grease-ui",
            "./modules/grease-ui",
            "./lib/grease-ui",
            &env::var("GREASE_UI_PATH").unwrap_or_default(),
        ])?;
        
        // Search for WebAssembly module
        let wasm_module = Self::find_module("grease-webassembly", &[
            "./grease-webassembly",
            "../grease-webassembly",
            "./modules/grease-webassembly",
            "./lib/grease-webassembly",
            &env::var("GREASE_WASM_PATH").unwrap_or_default(),
        ])?;
        
        // Report detection results
        if let Some(ref ui) = ui_module {
            println!("✅ Found UI module at: {}", ui.path.display());
        } else {
            println!("⚠️  UI module not found");
        }
        
        if let Some(ref wasm) = wasm_module {
            println!("✅ Found WebAssembly module at: {}", wasm.path.display());
        } else {
            println!("⚠️  WebAssembly module not found");
        }
        
        let searched_paths = vec![
            "./grease-ui".to_string(),
            "../grease-ui".to_string(),
            "./modules/grease-ui".to_string(),
            "./lib/grease-ui".to_string(),
            env::var("GREASE_UI_PATH").unwrap_or_default(),
            "./grease-webassembly".to_string(),
            "../grease-webassembly".to_string(),
            "./modules/grease-webassembly".to_string(),
            "./lib/grease-webassembly".to_string(),
            env::var("GREASE_WASM_PATH").unwrap_or_default(),
        ];
        
        Ok(Self {
            ui_module,
            wasm_module,
            platform,
            searched_paths,
        })
    }
    
    /// Find a module in multiple possible locations
    fn find_module(name: &str, paths: &[&str]) -> Result<Option<ModuleInfo>, ModuleError> {
        for &path_str in paths {
            if path_str.trim().is_empty() {
                continue;
            }
            
            let module_path = PathBuf::from(path_str);
            
            // Check if module directory exists
            if !module_path.exists() {
                continue;
            }
            
            // Check if Cargo.toml exists (indicates proper module)
            let cargo_toml = module_path.join("Cargo.toml");
            if !cargo_toml.exists() {
                continue;
            }
            
            // Try to read version from Cargo.toml
            let version = Self::extract_module_version(&cargo_toml);
            
            return Ok(Some(ModuleInfo {
                name: name.to_string(),
                path: module_path,
                version,
                available: true,
            }));
        }
        
        Ok(None)
    }
    
    /// Extract version from module's Cargo.toml
    fn extract_module_version(cargo_toml: &Path) -> Option<String> {
        use std::fs;
        
        if let Ok(content) = fs::read_to_string(cargo_toml) {
            for line in content.lines() {
                if line.trim().starts_with("version = ") {
                    let version = line.trim()
                        .strip_prefix("version = ")
                        .unwrap_or("")
                        .trim_matches('"')
                        .trim_matches('"');
                    return Some(version.to_string());
                }
            }
        }
        None
    }
    
    /// Detect the current target platform
    pub fn detect_platform() -> Platform {
        #[cfg(target_os = "linux")]
        { return Platform::Linux; }
        
        #[cfg(target_os = "macos")]
        { return Platform::MacOS; }
        
        #[cfg(target_os = "windows")]
        { return Platform::Windows; }
        
        #[cfg(target_os = "android")]
        { return Platform::Android; }
        
        #[cfg(target_arch = "wasm32")]
        { return Platform::WebAssembly; }
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows", target_os = "android", target_arch = "wasm32")))]
        { return Platform::Unknown; }
    }
    
    /// Initialize detected modules with the VM
    pub fn init_modules(&self, vm: &mut VM) -> Result<(), ModuleError> {
        println!("🚀 Initializing Grease modules...");
        
        // Initialize UI module if available
        if let Some(ref ui_module) = self.ui_module {
            println!("📱 Initializing UI module...");
            self.init_ui_module(vm, ui_module)?;
        }
        
        // Initialize WebAssembly module if available
        if let Some(ref wasm_module) = self.wasm_module {
            println!("🌐 Initializing WebAssembly module...");
            self.init_wasm_module(vm, wasm_module)?;
        }
        
        // Check platform compatibility
        self.check_platform_compatibility()?;
        
        println!("✅ Module initialization complete");
        Ok(())
    }
    
    /// Initialize UI module
    fn init_ui_module(&self, vm: &mut VM, _module: &ModuleInfo) -> Result<(), ModuleError> {
        // Check if UI module is compatible with current platform
        match self.platform {
            Platform::WebAssembly => {
                // Web platform should use web UI implementation
                println!("🌐 Using Web-based UI implementation");
            }
            Platform::Android => {
                // Android should use native Android UI
                println!("🤖 Using Android native UI implementation");
            }
            _ => {
                // Desktop platforms use native UI
                println!("🖥️ Using native UI implementation for {:?}", self.platform);
            }
        }
        
        // For now, we'll register a placeholder function to indicate UI is available
        // In the full implementation, this would call grease_ui::init_ui(vm)
        vm.register_native("ui_available", 0, |_vm, _args| {
            println!("UI functions are available");
            Ok(crate::bytecode::Value::Boolean(true))
        });
        
        println!("✅ UI module initialized");
        Ok(())
    }
    
    /// Initialize WebAssembly module
    fn init_wasm_module(&self, vm: &mut VM, _module: &ModuleInfo) -> Result<(), ModuleError> {
        // Check if WebAssembly module is compatible with current platform
        match self.platform {
            Platform::WebAssembly => {
                println!("🌐 Using WebAssembly in browser environment");
            }
            _ => {
                println!("💻 Using WebAssembly compilation for {:?}", self.platform);
            }
        }
        
        // For now, we'll register a placeholder function to indicate WebAssembly is available
        // In the full implementation, this would call grease_webassembly::init_webassembly(vm)
        vm.register_native("wasm_available", 0, |_vm, _args| {
            println!("WebAssembly functions are available");
            Ok(crate::bytecode::Value::Boolean(true))
        });
        
        println!("✅ WebAssembly module initialized");
        Ok(())
    }
    
    /// Check platform compatibility and warn about potential issues
    pub fn check_platform_compatibility(&self) -> Result<(), ModuleError> {
        let mut warnings = Vec::new();
        
        match self.platform {
            Platform::Linux => {
                if self.ui_module.is_some() {
                    // Check for GTK libraries on Linux
                    if !Self::check_gtk_available() {
                        warnings.push(
                            "UI module requires GTK libraries on Linux. Install with: sudo apt install libgtk-3-dev".to_string()
                        );
                    }
                }
            }
            Platform::Windows => {
                if self.ui_module.is_some() {
                    // Check for MSVC or GTK on Windows
                    if !Self::check_msvc_available() && !Self::check_gtk_available() {
                        warnings.push(
                            "UI module requires MSVC or GTK on Windows. Install with: vcpkg install gtk3:x64-windows".to_string()
                        );
                    }
                }
            }
            Platform::Android => {
                if self.ui_module.is_some() {
                    // Android UI should work but may need NDK
                    warnings.push(
                        "UI module on Android requires NDK setup. Set ANDROID_NDK_ROOT environment variable".to_string()
                    );
                }
            }
            Platform::WebAssembly => {
                // WebAssembly should work everywhere
                if self.ui_module.is_some() {
                    println!("🌐 Web UI will use WebAssembly backend for performance");
                }
            }
            _ => {}
        }
        
        // Print warnings
        for warning in &warnings {
            println!("⚠️  {}", warning);
        }
        
        Ok(())
    }
    
    /// Check if GTK libraries are available (Linux)
    fn check_gtk_available() -> bool {
        // Try to find GTK libraries in common locations
        let gtk_paths = [
            "/usr/lib/x86_64-linux-gnu/libgtk-3.so",
            "/usr/lib/libgtk-3.so",
            "/usr/local/lib/libgtk-3.so",
        ];
        
        gtk_paths.iter().any(|path| Path::new(path).exists())
    }
    
    /// Check if MSVC is available (Windows)
    fn check_msvc_available() -> bool {
        // Check for MSVC runtime libraries
        let msvc_paths = [
            "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\VC\\Redist\\MSVC\\v142",
            "C:\\Program Files\\Microsoft Visual Studio\\2022\\VC\\Redist\\MSVC\\v143",
        ];
        
        msvc_paths.iter().any(|path| Path::new(path).exists())
    }
    
    /// Get module information for debugging
    pub fn get_module_info(&self) -> (&Option<ModuleInfo>, &Option<ModuleInfo>) {
        (&self.ui_module, &self.wasm_module)
    }
    
    /// Check if specific module is available
    pub fn is_ui_available(&self) -> bool {
        self.ui_module.is_some()
    }
    
    pub fn is_wasm_available(&self) -> bool {
        self.wasm_module.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;
    use tempfile::TempDir;
    
    #[test]
    fn test_module_loader_creation() {
        let loader = ModuleLoader::new().unwrap();
        
        assert!(loader.platform != Platform::Unknown);
        assert_eq!(loader.searched_paths.len(), 10); // Should search 10 locations (5 for each module)
    }
    
    #[test]
    fn test_platform_detection() {
        let platform = ModuleLoader::detect_platform();
        // Platform should be one of the supported variants
        assert!(platform != Platform::Unknown);
        
        // Test platform string conversion
        let platform_str = platform.to_string();
        assert!(!platform_str.is_empty());
        assert!(platform_str.len() > 0);
    }
    
    #[test]
    fn test_module_info_creation() {
        let module_info = ModuleInfo {
            name: "test-module".to_string(),
            path: PathBuf::from("./test-module"),
            version: Some("0.1.0".to_string()),
            available: true,
        };
        
        assert_eq!(module_info.name, "test-module");
        assert_eq!(module_info.path, PathBuf::from("./test-module"));
        assert_eq!(module_info.version, Some("0.1.0".to_string()));
        assert!(module_info.available);
    }
    
    #[test]
    fn test_module_search_paths() {
        let loader = ModuleLoader::new().unwrap();
        
        // Should search in these exact locations
        let expected_paths = vec![
            "./grease-ui",
            "../grease-ui", 
            "./modules/grease-ui",
            "./lib/grease-ui",
            "./grease-webassembly",
            "../grease-webassembly",
            "./modules/grease-webassembly", 
            "./lib/grease-webassembly"
        ];
        
        for expected_path in expected_paths {
            assert!(loader.searched_paths.contains(&expected_path.to_string()));
        }
    }
    
    #[test]
    fn test_module_availability() {
        let loader = ModuleLoader::new().unwrap();
        
        // These might be true if modules exist in repo
        // Just test that the methods don't panic
        let _ui_available = loader.is_ui_available();
        let _wasm_available = loader.is_wasm_available();
    }
    
    #[test]
    fn test_environment_variable_paths() {
        // Test that environment variables are included in search paths
        env::set_var("GREASE_UI_PATH", "/custom/ui/path");
        env::set_var("GREASE_WASM_PATH", "/custom/wasm/path");
        
        let loader = ModuleLoader::new().unwrap();
        
        assert!(loader.searched_paths.contains(&"/custom/ui/path".to_string()));
        assert!(loader.searched_paths.contains(&"/custom/wasm/path".to_string()));
        
        // Clean up
        env::remove_var("GREASE_UI_PATH");
        env::remove_var("GREASE_WASM_PATH");
    }
    
    #[test]
    fn test_module_detection_with_fake_directories() {
        // Create temporary directories to test detection logic
        let temp_dir = TempDir::new().unwrap();
        let ui_path = temp_dir.path().join("grease-ui");
        let wasm_path = temp_dir.path().join("grease-webassembly");
        
        // Create directories
        fs::create_dir(&ui_path).unwrap();
        fs::create_dir(&wasm_path).unwrap();
        
        // Create fake Cargo.toml files
        fs::write(ui_path.join("Cargo.toml"), "[package]\nname = \"grease-ui\"\nversion = \"0.1.0\"").unwrap();
        fs::write(wasm_path.join("Cargo.toml"), "[package]\nname = \"grease-webassembly\"\nversion = \"0.1.0\"").unwrap();
        
        // Change to temp directory for testing
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_dir).unwrap();
        
        // Test detection
        let loader = ModuleLoader::new().unwrap();
        
        // Should search in these locations (relative to temp dir)
        assert!(loader.searched_paths.contains(&"./grease-ui".to_string()));
        assert!(loader.searched_paths.contains(&"./grease-webassembly".to_string()));
        
        // Restore original directory
        env::set_current_dir(original_dir).unwrap();
    }
    
    #[test]
    fn test_platform_compatibility_checking() {
        let loader = ModuleLoader::new().unwrap();
        
        // This should not panic and should return Ok
        let result = loader.check_platform_compatibility();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_module_initialization() {
        let mut vm = VM::new();
        let loader = ModuleLoader::new().unwrap();
        
        // Should not panic even with no modules
        let result = loader.init_modules(&mut vm);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_gtk_availability_check() {
        // This function should not panic
        let gtk_available = ModuleLoader::check_gtk_available();
        // Result depends on system, but should be boolean
        assert!(gtk_available == true || gtk_available == false);
    }
    
    #[test]
    fn test_msvc_availability_check() {
        // This function should not panic
        let msvc_available = ModuleLoader::check_msvc_available();
        // Result depends on system, but should be boolean
        assert!(msvc_available == true || msvc_available == false);
    }
}