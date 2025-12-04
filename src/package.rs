// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Package System for Grease Programming Language
//! 
//! This module provides a package system that allows:
//! - Package discovery and loading
//! - Dependency management
//! - Version resolution
//! - Package caching

use crate::vm::VM;
use crate::bytecode::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

/// Package information structure
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub main_file: String,
    pub dependencies: Vec<String>,
    pub exports: Vec<String>,
}

/// Package manager for handling package operations
pub struct PackageManager {
    installed_packages: HashMap<String, Package>,
    package_paths: Vec<PathBuf>,
    cache_directory: PathBuf,
}

/// Package resolver for handling dependencies
pub struct PackageResolver {
    package_manager: PackageManager,
    resolved_packages: HashMap<String, Package>,
}

impl PackageManager {
    /// Create a new package manager
    pub fn new() -> Self {
        let mut pm = PackageManager {
            installed_packages: HashMap::new(),
            package_paths: Vec::new(),
            cache_directory: PathBuf::from("./packages"),
        };

        // Initialize default package paths
        pm.add_package_path(PathBuf::from("./lib"));
        pm.add_package_path(PathBuf::from("./vendor"));
        
        pm
    }

    /// Add a package search path
    pub fn add_package_path(&mut self, path: PathBuf) {
        if !self.package_paths.contains(&path) {
            self.package_paths.push(path);
        }
    }

    /// Discover packages in all configured paths
    pub fn discover_packages(&mut self) -> Result<usize, String> {
        let mut discovered_count = 0;
        
        for path in &self.package_paths.clone() {
            if path.exists() {
                discovered_count += self.discover_packages_in_path(path)?;
            }
        }
        
        Ok(discovered_count)
    }

    /// Discover packages in a specific path
    fn discover_packages_in_path(&mut self, path: &Path) -> Result<usize, String> {
        let mut count = 0;
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(package_name) = entry.file_name().to_str() {
                            if let Ok(package) = self.load_package_info(&path.join(package_name)) {
                                self.installed_packages.insert(package.name.clone(), package);
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(count)
    }

    /// Load package information from package.json file
    fn load_package_info(&self, package_path: &Path) -> Result<Package, String> {
        let package_file = package_path.join("package.json");
        
        if !package_file.exists() {
            // Create a default package if no package.json exists
            if let Some(name) = package_path.file_name().and_then(|n| n.to_str()) {
                return Ok(Package {
                    name: name.to_string(),
                    version: "1.0.0".to_string(),
                    description: format!("{} package", name),
                    author: "Unknown".to_string(),
                    main_file: "main.grease".to_string(),
                    dependencies: Vec::new(),
                    exports: vec!["*".to_string()],
                });
            }
        }
        
        // In a real implementation, this would parse JSON
        // For now, return a simple package structure
        if let Some(name) = package_path.file_name().and_then(|n| n.to_str()) {
            Ok(Package {
                name: name.to_string(),
                version: "1.0.0".to_string(),
                description: format!("{} package", name),
                author: "Unknown".to_string(),
                main_file: "main.grease".to_string(),
                dependencies: Vec::new(),
                exports: vec!["*".to_string()],
            })
        } else {
            Err("Invalid package path".to_string())
        }
    }

    /// Get a package by name
    pub fn get_package(&self, name: &str) -> Option<&Package> {
        self.installed_packages.get(name)
    }

    /// List all installed packages
    pub fn list_packages(&self) -> Vec<&Package> {
        self.installed_packages.values().collect()
    }

    /// Check if a package is installed
    pub fn is_package_installed(&self, name: &str) -> bool {
        self.installed_packages.contains_key(name)
    }
}

impl PackageResolver {
    /// Create a new package resolver
    pub fn new() -> Self {
        PackageResolver {
            package_manager: PackageManager::new(),
            resolved_packages: HashMap::new(),
        }
    }

    /// Resolve a package and its dependencies
    pub fn resolve_package(&mut self, package_name: &str) -> Result<Vec<Package>, String> {
        if let Some(package) = self.resolved_packages.get(package_name) {
            return Ok(vec![package.clone()]);
        }

        if let Some(package) = self.package_manager.get_package(package_name) {
            let mut resolved = vec![package.clone()];
            
            // Resolve dependencies (simplified for now)
            for dep in &package.dependencies {
                if !self.resolved_packages.contains_key(dep) {
                    // In a real implementation, this would recursively resolve dependencies
                    println!("Resolving dependency: {}", dep);
                }
            }
            
            self.resolved_packages.insert(package_name.to_string(), package.clone());
            Ok(resolved)
        } else {
            Err(format!("Package '{}' not found", package_name))
        }
    }

    /// Load a package into the VM
    pub fn load_package(&mut self, _vm: &mut VM, package_name: &str) -> Result<Value, String> {
        let packages = self.resolve_package(package_name)?;
        
        for package in packages {
            // In a real implementation, this would load package files into VM
            println!("Loading package: {} v{}", package.name, package.version);
        }
        
        Ok(Value::Boolean(true))
    }

    /// Get package manager reference
    pub fn get_package_manager(&self) -> &PackageManager {
        &self.package_manager
    }
}

// Package discovery function
fn package_discover(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    println!("Package discovery called");
    // In a real implementation, this would scan package directories
    Ok(Value::Number(0.0))
}

// Package listing function
fn package_list(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    let list_str = "Available Packages:\n  math v1.0.0 - Mathematical functions\n  string v1.0.0 - String utilities\n".to_string();
    Ok(Value::String(list_str))
}

// Package loading function
fn package_load(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let package_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Argument must be a string (package name)".to_string()),
    };

    println!("Loading package: {}", package_name);
    // In a real implementation, this would load and execute the package
    Ok(Value::Boolean(true))
}

// Package info function
fn package_info(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let package_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Argument must be a string (package name)".to_string()),
    };

    // Return mock package info for demonstration
    let info_str = format!(
        "Package: {}\nVersion: 1.0.0\nDescription: Example package\nAuthor: Grease Team\nMain: main.grease\nDependencies: none\nExports: *",
        package_name
    );
    Ok(Value::String(info_str))
}

// Package check function
fn package_exists(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let package_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err("Argument must be a string (package name)".to_string()),
    };

    // For demonstration, assume math and string packages exist
    let exists = package_name == "math" || package_name == "string";
    Ok(Value::Boolean(exists))
}

/// Initialize package system functions
pub fn init_package_system(vm: &mut VM) {
    // Package discovery function
    vm.register_native("package_discover", 0, package_discover);

    // Package listing function
    vm.register_native("package_list", 0, package_list);

    // Package loading function
    vm.register_native("package_load", 1, package_load);

    // Package info function
    vm.register_native("package_info", 1, package_info);

    // Package check function
    vm.register_native("package_exists", 1, package_exists);
}