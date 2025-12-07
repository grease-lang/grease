// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Version Compatibility System
//! 
//! This module provides version comparison and compatibility checking
//! between Grease core and modules with warning system for mismatched versions.

use std::collections::HashMap;
use std::env;

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

/// Version information for core and modules
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub core_version: String,
    pub ui_version: Option<String>,

    pub compatibility: HashMap<String, CompatibilityLevel>,
}

/// Version parsing and comparison utilities
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// Parse version string in format "major.minor.patch"
    pub fn parse(version_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = version_str.split('.').collect();
        
        if parts.len() != 3 {
            return Err(format!("Invalid version format: {}", version_str));
        }
        
        let major = parts[0].parse::<u32>()
            .map_err(|_| format!("Invalid major version: {}", parts[0]))?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| format!("Invalid minor version: {}", parts[1]))?;
        let patch = parts[2].parse::<u32>()
            .map_err(|_| format!("Invalid patch version: {}", parts[2]))?;
        
        Ok(Version { major, minor, patch })
    }
    
    /// Compare two versions
    pub fn compare(&self, other: &Version) -> CompatibilityLevel {
        if self.major != other.major {
            CompatibilityLevel::Incompatible
        } else if self.minor != other.minor {
            CompatibilityLevel::WarningMajor
        } else if self.patch != other.patch {
            CompatibilityLevel::WarningMinor
        } else {
            CompatibilityLevel::Compatible
        }
    }
}

impl VersionInfo {
    /// Create new version info with current core version
    pub fn new() -> Self {
        let core_version = env!("CARGO_PKG_VERSION").to_string();
        
        Self {
            core_version,
            ui_version: None,

            compatibility: HashMap::new(),
        }
    }
    
    /// Add module version information
    pub fn with_ui_version(mut self, version: String) -> Self {
        self.ui_version = Some(version);
        self
    }
    

    
    /// Assess compatibility between core and modules
    pub fn assess_compatibility(&mut self) {
        let core_version = match Version::parse(&self.core_version) {
            Ok(v) => v,
            Err(_) => {
                println!("⚠️  Warning: Could not parse core version '{}'", self.core_version);
                return;
            }
        };
        
        // Assess UI module compatibility
        if let Some(ref ui_version) = self.ui_version {
            match Version::parse(ui_version) {
                Ok(ui_ver) => {
                    let compatibility = core_version.compare(&ui_ver);
                    self.compatibility.insert("ui".to_string(), compatibility.clone());
                    
                    self.print_compatibility_warning("UI", ui_version, &compatibility);
                }
                Err(_) => {
                    println!("⚠️  Warning: Could not parse UI module version '{}'", ui_version);
                    self.compatibility.insert("ui".to_string(), CompatibilityLevel::WarningMajor);
                }
            }
        }

    }
    
    /// Print compatibility warning with user-friendly message
    fn print_compatibility_warning(&self, module_name: &str, module_version: &str, compatibility: &CompatibilityLevel) {
        let (icon, message, action) = match compatibility {
            CompatibilityLevel::Compatible => {
                ("✅", "fully compatible", "No action needed")
            }
            CompatibilityLevel::WarningMinor => {
                ("⚠️", "minor version difference", "Should work fine")
            }
            CompatibilityLevel::WarningMajor => {
                ("⚠️", "major version difference", "Expect some issues")
            }
            CompatibilityLevel::Incompatible => {
                ("❌", "incompatible", "Will not work")
            }
        };
        
        println!(
            "{} {} module version {} vs Core {}: {}",
            icon, module_name, module_version, self.core_version, message
        );
        
        if *compatibility != CompatibilityLevel::Compatible {
            println!("💡 Recommended action: {}", action);
        }
    }
    
    /// Get compatibility level for a specific module
    pub fn get_module_compatibility(&self, module: &str) -> CompatibilityLevel {
        self.compatibility
            .get(module)
            .copied()
            .unwrap_or(CompatibilityLevel::Compatible)
    }
    
    /// Check if all modules are compatible
    pub fn all_compatible(&self) -> bool {
        self.compatibility.values().all(|&level| {
            level == CompatibilityLevel::Compatible || level == CompatibilityLevel::WarningMinor
        })
    }
    
    /// Check if any module is incompatible
    pub fn has_incompatible(&self) -> bool {
        self.compatibility.values().any(|&level| {
            level == CompatibilityLevel::Incompatible
        })
    }
    
    /// Get summary of compatibility status
    pub fn get_summary(&self) -> String {
        let mut summary = Vec::new();
        
        if let Some(ref ui_version) = self.ui_version {
            let ui_compat = self.get_module_compatibility("ui");
            summary.push(format!("UI: {} ({})", ui_version, self.format_compatibility(&ui_compat)));
        }
        

        
        if summary.is_empty() {
            "No modules loaded".to_string()
        } else {
            format!("Core: {} | {}", self.core_version, summary.join(" | "))
        }
    }
    
    /// Format compatibility level for display
    fn format_compatibility(&self, compatibility: &CompatibilityLevel) -> &'static str {
        match compatibility {
            CompatibilityLevel::Compatible => "✅ Compatible",
            CompatibilityLevel::WarningMinor => "⚠️ Minor Warning",
            CompatibilityLevel::WarningMajor => "⚠️ Major Warning",
            CompatibilityLevel::Incompatible => "❌ Incompatible",
        }
    }
    
    /// Print detailed compatibility report
    pub fn print_detailed_report(&self) {
        println!("\n📋 Version Compatibility Report");
        println!("================================");
        println!("Core Version: {}", self.core_version);
        
        if let Some(ref ui_version) = self.ui_version {
            let ui_compat = self.get_module_compatibility("ui");
            println!("UI Module: {} - {}", ui_version, self.format_compatibility(&ui_compat));
        }
        

        
        println!("\n🎯 Overall Status: {}", 
            if self.all_compatible() {
                "✅ All modules compatible"
            } else if self.has_incompatible() {
                "❌ Some modules incompatible"
            } else {
                "⚠️ Some modules have warnings"
            }
        );
        println!("================================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_parsing() {
        // Test valid version parsing
        assert!(Version::parse("1.2.3").is_ok());
        let version = Version::parse("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        
        // Test invalid version parsing
        assert!(Version::parse("1.2").is_err());
        assert!(Version::parse("1.2.3.4").is_err());
        assert!(Version::parse("invalid").is_err());
    }
    
    #[test]
    fn test_version_comparison_identical() {
        let v1 = Version { major: 1, minor: 2, patch: 3 };
        let v2 = Version { major: 1, minor: 2, patch: 3 };
        
        assert_eq!(v1.compare(&v2), CompatibilityLevel::Compatible);
    }
    
    #[test]
    fn test_version_comparison_patch_difference() {
        let v1 = Version { major: 1, minor: 2, patch: 3 };
        let v2 = Version { major: 1, minor: 2, patch: 4 };
        
        assert_eq!(v1.compare(&v2), CompatibilityLevel::WarningMinor);
    }
    
    #[test]
    fn test_version_comparison_minor_difference() {
        let v1 = Version { major: 1, minor: 2, patch: 3 };
        let v2 = Version { major: 1, minor: 3, patch: 0 };
        
        assert_eq!(v1.compare(&v2), CompatibilityLevel::WarningMajor);
    }
    
    #[test]
    fn test_version_comparison_major_difference() {
        let v1 = Version { major: 1, minor: 0, patch: 0 };
        let v2 = Version { major: 2, minor: 0, patch: 0 };
        
        assert_eq!(v1.compare(&v2), CompatibilityLevel::Incompatible);
    }
    
    #[test]
    fn test_version_info_creation() {
        let mut info = VersionInfo::new();
        assert_eq!(info.core_version, env!("CARGO_PKG_VERSION"));
        assert!(info.ui_version.is_none());

        assert!(info.compatibility.is_empty());
        
        info = info.with_ui_version("0.1.0".to_string());
        assert_eq!(info.ui_version, Some("0.1.0".to_string()));
        

    }
    
    #[test]
    fn test_compatibility_assessment() {
        let mut info = VersionInfo::new();
        info = info.with_ui_version("0.1.0".to_string());
        info.assess_compatibility();
        
        // Should be WarningMinor since patch version differs (0.1.1 vs 0.1.0)
        assert_eq!(info.get_module_compatibility("ui"), CompatibilityLevel::WarningMinor);
        
        // Test with exact same version
        let mut info2 = VersionInfo::new();
        info2 = info2.with_ui_version(env!("CARGO_PKG_VERSION").to_string());
        info2.assess_compatibility();
        assert_eq!(info2.get_module_compatibility("ui"), CompatibilityLevel::Compatible);
    }
    
    #[test]
    fn test_compatibility_summary() {
        let mut info = VersionInfo::new();
        info = info.with_ui_version("0.2.0".to_string());
        info.assess_compatibility();
        
        let summary = info.get_summary();
        assert!(summary.contains("UI: 0.2.0"));
        assert!(summary.contains("Major Warning"));
        

    }
    
    #[test]
    fn test_all_compatible() {
        let mut info = VersionInfo::new();
        info = info.with_ui_version(env!("CARGO_PKG_VERSION").to_string());
        info.assess_compatibility();
        
        assert!(info.all_compatible());
        assert!(!info.has_incompatible());
    }
    
    #[test]
    fn test_has_incompatible() {
        let mut info = VersionInfo::new();
        info = info.with_ui_version("2.0.0".to_string()); // Different major version
        info.assess_compatibility();
        
        assert!(!info.all_compatible());
        assert!(info.has_incompatible());
    }
    
    #[test]
    fn test_format_compatibility() {
        let info = VersionInfo::new();
        
        assert_eq!(info.format_compatibility(&CompatibilityLevel::Compatible), "✅ Compatible");
        assert_eq!(info.format_compatibility(&CompatibilityLevel::WarningMinor), "⚠️ Minor Warning");
        assert_eq!(info.format_compatibility(&CompatibilityLevel::WarningMajor), "⚠️ Major Warning");
        assert_eq!(info.format_compatibility(&CompatibilityLevel::Incompatible), "❌ Incompatible");
    }
    
    #[test]
    fn test_version_info_display() {
        let mut info = VersionInfo::new();
        info = info.with_ui_version("0.1.0".to_string());
        info.assess_compatibility();
        
        // Should not panic when getting summary
        let _summary = info.get_summary();
    }
}