// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Module Error Handling System
//! 
//! This module provides comprehensive error handling for the Grease module system
//! with user-friendly error messages and actionable solutions.

use std::fmt;
use toml;

/// Comprehensive error types for module operations
#[derive(Debug, PartialEq)]
pub enum ModuleError {
    /// Module not found in any of the searched locations
    ModuleNotFound {
        module: String,
        searched_paths: Vec<String>,
    },
    
    /// Module build failed during compilation
    BuildFailed {
        module: String,
        error: String,
        suggestions: Vec<String>,
    },
    
    /// Version mismatch between core and module
    VersionMismatch {
        module: String,
        core_version: String,
        module_version: String,
        compatibility: CompatibilityLevel,
    },
    
    /// Required dependency missing for module functionality
    DependencyMissing {
        module: String,
        dependency: String,
        platform: String,
        install_cmd: String,
    },
    
    /// Module not supported on current platform
    PlatformUnsupported {
        module: String,
        platform: String,
        reason: String,
        alternatives: Vec<String>,
    },
    
    /// Module initialization failed
    InitializationFailed {
        module: String,
        error: String,
    },
    
    /// IO error during module operations
    IoError {
        operation: String,
        path: String,
        error: String,
    },
}

/// Version compatibility levels
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CompatibilityLevel {
    /// Same major and minor version - fully compatible
    Compatible,
    /// Different patch version - should work fine
    WarningMinor,
    /// Different minor version - may have some issues
    WarningMajor,
    /// Different major version - likely to have problems
    Incompatible,
}

impl ModuleError {
    /// Generate user-friendly error message with actionable solutions
    pub fn user_message(&self) -> String {
        match self {
            ModuleError::ModuleNotFound { module, searched_paths } => {
                format!(
                    "❌ Module '{}' not found\n\n🔍 Searched in:\n{}\n\n💡 To fix:\n1. Copy module folder to one of the locations above\n2. Set environment variable:\n   export GREASE_{}_PATH=/path/to/{}\n3. Ensure module folder contains Cargo.toml",
                    module,
                    searched_paths.iter().map(|p| format!("  - {}", p)).collect::<Vec<_>>().join("\n"),
                    module.to_uppercase().replace("-", "_"),
                    module
                )
            }
            
            ModuleError::BuildFailed { module, error, suggestions } => {
                format!(
                    "❌ Module '{}' build failed\n\n🔧 Error: {}\n\n💡 Suggestions:\n{}\n\n📖 For help, see: docs/TROUBLESHOOTING.md",
                    module,
                    error,
                    suggestions.iter().map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("\n")
                )
            }
            
            ModuleError::VersionMismatch { module, core_version, module_version, compatibility } => {
                let (severity, action) = match compatibility {
                    CompatibilityLevel::Compatible => ("✅", "No action needed"),
                    CompatibilityLevel::WarningMinor => ("⚠️", "Should work fine"),
                    CompatibilityLevel::WarningMajor => ("⚠️", "Expect some issues"),
                    CompatibilityLevel::Incompatible => ("❌", "Will not work"),
                };
                
                format!(
                    "{} Version mismatch: Module '{}' ({}) vs Core ({})\n\n🎯 Compatibility: {:?}\n\n💡 Recommended action: {}",
                    severity, module, module_version, core_version, compatibility, action
                )
            }
            
            ModuleError::DependencyMissing { module, dependency, platform, install_cmd } => {
                format!(
                    "❌ Missing dependency for '{}' module\n\n📦 Dependency: '{}' on {}\n\n💡 To install:\n   {}\n\n📖 For platform setup, see: docs/PLATFORM_SETUP.md",
                    module, dependency, platform, install_cmd
                )
            }
            
            ModuleError::PlatformUnsupported { module, platform, reason, alternatives } => {
                format!(
                    "❌ Module '{}' not supported on {}\n\n🚫 Reason: {}\n\n💡 Alternatives:\n{}\n\n📖 For platform support, see: docs/PLATFORM_SETUP.md",
                    module, platform, reason,
                    alternatives.iter().map(|a| format!("  - {}", a)).collect::<Vec<_>>().join("\n")
                )
            }
            
            ModuleError::InitializationFailed { module, error } => {
                format!(
                    "❌ Failed to initialize '{}' module\n\n🔧 Error: {}\n\n💡 Try:\n1. Check module version compatibility\n2. Verify all dependencies are installed\n3. See docs/TROUBLESHOOTING.md",
                    module, error
                )
            }
            
            ModuleError::IoError { operation, path, error } => {
                format!(
                    "❌ IO error during {}\n\n📁 Path: {}\n\n🔧 Error: {}\n\n💡 Check:\n1. File permissions\n2. Disk space\n3. Path validity",
                    operation, path, error
                )
            }
        }
    }
    
    /// Get error severity level for logging
    pub fn severity(&self) -> &'static str {
        match self {
            ModuleError::ModuleNotFound { .. } => "error",
            ModuleError::BuildFailed { .. } => "error",
            ModuleError::VersionMismatch { compatibility, .. } => {
                match compatibility {
                    CompatibilityLevel::Compatible => "info",
                    CompatibilityLevel::WarningMinor => "warning",
                    CompatibilityLevel::WarningMajor => "warning",
                    CompatibilityLevel::Incompatible => "error",
                }
            }
            ModuleError::DependencyMissing { .. } => "error",
            ModuleError::PlatformUnsupported { .. } => "error",
            ModuleError::InitializationFailed { .. } => "error",
            ModuleError::IoError { .. } => "error",
        }
    }
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.user_message())
    }
}

impl std::error::Error for ModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<std::io::Error> for ModuleError {
    fn from(error: std::io::Error) -> Self {
        ModuleError::IoError {
            operation: "file operation".to_string(),
            path: "unknown".to_string(), // std::io::Error doesn't have path() method
            error: error.to_string(),
        }
    }
}

impl From<toml::de::Error> for ModuleError {
    fn from(error: toml::de::Error) -> Self {
        ModuleError::BuildFailed {
            module: "unknown".to_string(),
            error: error.to_string(),
            suggestions: vec![
                "Check Cargo.toml syntax".to_string(),
                "Verify all dependencies are available".to_string(),
                "Ensure Rust version compatibility".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    
    #[test]
    fn test_module_not_found_error_message() {
        let error = ModuleError::ModuleNotFound {
            module: "grease-ui".to_string(),
            searched_paths: vec![
                "./grease-ui".to_string(),
                "../grease-ui".to_string(),
                "./modules/grease-ui".to_string(),
            ],
        };
        
        let message = error.user_message();
        assert!(message.contains("Module 'grease-ui' not found"));
        assert!(message.contains("./grease-ui"));
        assert!(message.contains("GREASE_UI_PATH"));
        assert!(message.contains("Cargo.toml"));
    }
    
    #[test]
    fn test_version_mismatch_error_message() {
        let error = ModuleError::VersionMismatch {
            module: "grease-ui".to_string(),
            core_version: "0.1.1".to_string(),
            module_version: "0.2.0".to_string(),
            compatibility: CompatibilityLevel::WarningMajor,
        };
        
        let message = error.user_message();
        assert!(message.contains("Version mismatch"));
        assert!(message.contains("grease-ui"));
        assert!(message.contains("0.2.0"));
        assert!(message.contains("0.1.1"));
        assert!(message.contains("Expect some issues"));
    }
    
    #[test]
    fn test_dependency_missing_error_message() {
        let error = ModuleError::DependencyMissing {
            module: "grease-ui".to_string(),
            dependency: "GTK3".to_string(),
            platform: "Linux".to_string(),
            install_cmd: "sudo apt install libgtk-3-dev".to_string(),
        };
        
        let message = error.user_message();
        assert!(message.contains("Missing dependency"));
        assert!(message.contains("GTK3"));
        assert!(message.contains("Linux"));
        assert!(message.contains("sudo apt install libgtk-3-dev"));
        assert!(message.contains("PLATFORM_SETUP.md"));
    }
    
    #[test]
    fn test_build_failed_error_message() {
        let error = ModuleError::BuildFailed {
            module: "grease-ui".to_string(),
            error: "Could not compile GTK3".to_string(),
            suggestions: vec![
                "Install GTK3 development libraries".to_string(),
                "Check Rust version compatibility".to_string(),
            ],
        };
        
        let message = error.user_message();
        assert!(message.contains("Module 'grease-ui' build failed"));
        assert!(message.contains("Could not compile GTK3"));
        assert!(message.contains("Install GTK3 development libraries"));
        assert!(message.contains("TROUBLESHOOTING.md"));
    }
    
    #[test]
    fn test_platform_unsupported_error_message() {
        let error = ModuleError::PlatformUnsupported {
            module: "grease-ui".to_string(),
            platform: "FreeBSD".to_string(),
            reason: "GTK3 not available on FreeBSD".to_string(),
            alternatives: vec![
                "Use web-based UI".to_string(),
                "Try terminal interface".to_string(),
            ],
        };
        
        let message = error.user_message();
        assert!(message.contains("Module 'grease-ui' not supported on FreeBSD"));
        assert!(message.contains("GTK3 not available on FreeBSD"));
        assert!(message.contains("Use web-based UI"));
        assert!(message.contains("PLATFORM_SETUP.md"));
    }
    
    #[test]
    fn test_initialization_failed_error_message() {
        let error = ModuleError::InitializationFailed {
            module: "grease-ui".to_string(),
            error: "Failed to initialize GTK context".to_string(),
        };
        
        let message = error.user_message();
        assert!(message.contains("Failed to initialize 'grease-ui' module"));
        assert!(message.contains("Failed to initialize GTK context"));
        assert!(message.contains("TROUBLESHOOTING.md"));
    }
    
    #[test]
    fn test_io_error_message() {
        let error = ModuleError::IoError {
            operation: "reading Cargo.toml".to_string(),
            path: "/path/to/module/Cargo.toml".to_string(),
            error: "Permission denied".to_string(),
        };
        
        let message = error.user_message();
        assert!(message.contains("IO error during reading Cargo.toml"));
        assert!(message.contains("/path/to/module/Cargo.toml"));
        assert!(message.contains("Permission denied"));
        assert!(message.contains("File permissions"));
    }
    
    #[test]
    fn test_error_severity() {
        // Test error severity for different error types
        assert_eq!(ModuleError::ModuleNotFound { 
            module: "test".to_string(), 
            searched_paths: vec![] 
        }.severity(), "error");
        
        assert_eq!(ModuleError::BuildFailed {
            module: "test".to_string(),
            error: "test error".to_string(),
            suggestions: vec![],
        }.severity(), "error");
        
        assert_eq!(ModuleError::VersionMismatch {
            module: "test".to_string(),
            core_version: "0.1.0".to_string(),
            module_version: "0.1.0".to_string(),
            compatibility: CompatibilityLevel::Compatible,
        }.severity(), "info");
        
        assert_eq!(ModuleError::VersionMismatch {
            module: "test".to_string(),
            core_version: "0.1.0".to_string(),
            module_version: "0.1.1".to_string(),
            compatibility: CompatibilityLevel::WarningMinor,
        }.severity(), "warning");
        
        assert_eq!(ModuleError::VersionMismatch {
            module: "test".to_string(),
            core_version: "0.1.0".to_string(),
            module_version: "0.2.0".to_string(),
            compatibility: CompatibilityLevel::WarningMajor,
        }.severity(), "warning");
        
        assert_eq!(ModuleError::VersionMismatch {
            module: "test".to_string(),
            core_version: "0.1.0".to_string(),
            module_version: "2.0.0".to_string(),
            compatibility: CompatibilityLevel::Incompatible,
        }.severity(), "error");
        
        assert_eq!(ModuleError::DependencyMissing {
            module: "test".to_string(),
            dependency: "test".to_string(),
            platform: "test".to_string(),
            install_cmd: "test".to_string(),
        }.severity(), "error");
        
        assert_eq!(ModuleError::PlatformUnsupported {
            module: "test".to_string(),
            platform: "test".to_string(),
            reason: "test".to_string(),
            alternatives: vec![],
        }.severity(), "error");
        
        assert_eq!(ModuleError::InitializationFailed {
            module: "test".to_string(),
            error: "test".to_string(),
        }.severity(), "error");
        
        assert_eq!(ModuleError::IoError {
            operation: "test".to_string(),
            path: "test".to_string(),
            error: "test".to_string(),
        }.severity(), "error");
    }
    
    #[test]
    fn test_error_display_trait() {
        let error = ModuleError::ModuleNotFound {
            module: "test-module".to_string(),
            searched_paths: vec!["./test-module".to_string()],
        };
        
        let display_str = format!("{}", error);
        assert!(display_str.contains("Module 'test-module' not found"));
    }
    
    #[test]
    fn test_error_source() {
        let error = ModuleError::ModuleNotFound {
            module: "test".to_string(),
            searched_paths: vec![],
        };
        
        assert!(error.source().is_none());
    }
    
    #[test]
    fn test_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let module_error = ModuleError::from(io_error);
        
        match module_error {
            ModuleError::IoError { operation, path, error } => {
                assert_eq!(operation, "file operation");
                assert_eq!(path, "unknown");
                assert!(error.contains("File not found"));
            }
            _ => panic!("Expected IoError variant"),
        }
    }
    
    #[test]
    fn test_from_toml_error() {
        // Create a simple TOML parsing error by trying to parse invalid TOML
        let toml_result: Result<toml::Value, toml::de::Error> = toml::from_str("invalid toml");
        let toml_error = toml_result.unwrap_err();
        let module_error = ModuleError::from(toml_error);
        
        match module_error {
            ModuleError::BuildFailed { module, error, suggestions } => {
                assert_eq!(module, "unknown");
                // Check that error contains some TOML-related message (not necessarily exact string)
                assert!(error.contains("TOML") || error.contains("parse") || error.contains("invalid"));
                assert!(!suggestions.is_empty());
                assert!(suggestions.iter().any(|s| s.contains("Cargo.toml syntax")));
            }
            _ => panic!("Expected BuildFailed variant"),
        }
    }
}