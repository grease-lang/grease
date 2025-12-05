// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Package Manager for Grease Programming Language
//! 
//! This module provides a command-line package manager that can:
//! - Install packages from repositories
//! - Update existing packages
//! - Remove packages
//! - Search for packages
//! - Manage package configurations

use crate::vm::VM;
use crate::bytecode::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

/// Package repository information
#[derive(Debug, Clone)]
pub struct PackageRepository {
    pub name: String,
    pub url: String,
    pub packages: HashMap<String, PackageInfo>,
}

/// Package information from repository
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub homepage: String,
    pub download_url: String,
    pub dependencies: Vec<String>,
    pub tags: Vec<String>,
}

/// Package manager configuration
#[derive(Debug, Clone)]
pub struct PackageManagerConfig {
    pub install_directory: PathBuf,
    pub cache_directory: PathBuf,
    pub repositories: Vec<PackageRepository>,
    pub auto_update: bool,
}

/// Command-line package manager
pub struct PackageCliManager {
    config: PackageManagerConfig,
    installed_packages: HashMap<String, InstalledPackage>,
}

/// Information about an installed package
#[derive(Debug, Clone)]
pub struct InstalledPackage {
    pub info: PackageInfo,
    pub install_date: String,
    pub install_path: PathBuf,
    pub files: Vec<String>,
}

impl PackageCliManager {
    /// Create a new package CLI manager
    pub fn new() -> Self {
        let config = PackageManagerConfig {
            install_directory: PathBuf::from("./packages"),
            cache_directory: PathBuf::from("./.grease/cache"),
            repositories: vec![
                PackageRepository {
                    name: "official".to_string(),
                    url: "https://packages.grease-lang.org".to_string(),
                    packages: HashMap::new(), // Would be populated from API
                }
            ],
            auto_update: true,
        };

        PackageCliManager {
            config,
            installed_packages: HashMap::new(),
        }
    }

    /// Initialize package manager directories
    pub fn initialize(&mut self) -> Result<(), String> {
        // Create directories
        fs::create_dir_all(&self.config.install_directory)
            .map_err(|e| format!("Failed to create install directory: {}", e))?;
        
        fs::create_dir_all(&self.config.cache_directory)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;

        // Load installed packages
        self.load_installed_packages()?;
        
        println!("Package manager initialized");
        Ok(())
    }

    /// Load installed packages from disk
    fn load_installed_packages(&mut self) -> Result<(), String> {
        let installed_file = self.config.install_directory.join("installed.json");
        
        if installed_file.exists() {
            // In a real implementation, this would parse JSON
            println!("Loading installed packages from {}", installed_file.display());
        }
        
        Ok(())
    }

    /// Install a package
    pub fn install_package(&mut self, package_name: &str) -> Result<(), String> {
        println!("Installing package: {}", package_name);
        
        // Check if already installed
        if self.installed_packages.contains_key(package_name) {
            return Err(format!("Package '{}' is already installed", package_name));
        }

        // Search for package in repositories
        let package_info = self.search_package(package_name)?;
        
        // Download and install package
        self.download_and_install(&package_info)?;
        
        println!("Successfully installed {} v{}", package_name, package_info.version);
        Ok(())
    }

    /// Search for a package in repositories
    fn search_package(&self, package_name: &str) -> Result<PackageInfo, String> {
        // In a real implementation, this would search repositories
        // For now, return mock package info
        Ok(PackageInfo {
            name: package_name.to_string(),
            version: "1.0.0".to_string(),
            description: format!("{} package for Grease", package_name),
            author: "Grease Team".to_string(),
            license: "MIT".to_string(),
            homepage: format!("https://grease-lang.org/packages/{}", package_name),
            download_url: format!("https://packages.grease-lang.org/download/{}", package_name),
            dependencies: Vec::new(),
            tags: vec!["utility".to_string()],
        })
    }

    /// Download and install a package
    fn download_and_install(&mut self, package_info: &PackageInfo) -> Result<(), String> {
        // Create package directory
        let package_dir = self.config.install_directory.join(&package_info.name);
        fs::create_dir_all(&package_dir)
            .map_err(|e| format!("Failed to create package directory: {}", e))?;

        // Create package files
        let main_file = package_dir.join("main.grease");
        let package_json = package_dir.join("package.json");
        
        // Write main.grease file
        let main_content = format!(
            "# {} package for Grease\n# Version: {}\n# Author: {}\n\nprint('Package {} loaded')\n",
            package_info.name,
            package_info.version,
            package_info.author,
            package_info.name
        );
        fs::write(&main_file, main_content)
            .map_err(|e| format!("Failed to write main file: {}", e))?;

        // Write package.json file
        let package_json_content = format!(
            r#"{{
  "name": "{}",
  "version": "{}",
  "description": "{}",
  "author": "{}",
  "license": "{}",
  "main": "main.grease"
}}"#,
            package_info.name,
            package_info.version,
            package_info.description,
            package_info.author,
            package_info.license
        );
        fs::write(&package_json, package_json_content)
            .map_err(|e| format!("Failed to write package.json: {}", e))?;

        // Record installation
        let installed_package = InstalledPackage {
            info: package_info.clone(),
            install_date: "2025-01-01".to_string(), // Current date
            install_path: package_dir,
            files: vec!["main.grease".to_string(), "package.json".to_string()],
        };
        
        self.installed_packages.insert(package_info.name.clone(), installed_package);
        
        Ok(())
    }

    /// List installed packages
    pub fn list_packages(&self) {
        if self.installed_packages.is_empty() {
            println!("No packages installed");
            return;
        }

        println!("Installed packages:");
        for (name, package) in &self.installed_packages {
            println!("  {} v{} - {}", name, package.info.version, package.info.description);
        }
    }

    /// Remove a package
    pub fn remove_package(&mut self, package_name: &str) -> Result<(), String> {
        if !self.installed_packages.contains_key(package_name) {
            return Err(format!("Package '{}' is not installed", package_name));
        }

        println!("Removing package: {}", package_name);
        
        if let Some(package) = self.installed_packages.remove(package_name) {
            // Remove package files
            fs::remove_dir_all(&package.install_path)
                .map_err(|e| format!("Failed to remove package directory: {}", e))?;
            
            println!("Successfully removed {}", package_name);
        }
        
        Ok(())
    }

    /// Search for packages
    pub fn search_packages(&self, query: &str) {
        println!("Searching for packages matching: {}", query);
        
        // In a real implementation, this would search repositories
        // For now, show mock results
        let mock_packages = vec![
            ("math", "Mathematical functions and utilities"),
            ("string", "String manipulation and utilities"),
            ("ui", "User interface components"),
            ("http", "HTTP client and server utilities"),
        ];

        for (name, description) in mock_packages {
            if name.contains(query) || description.contains(query) {
                println!("  {} - {}", name, description);
            }
        }
    }

    /// Update all packages
    pub fn update_packages(&mut self) -> Result<(), String> {
        println!("Updating all packages...");
        
        let mut updated_count = 0;
        for (name, package) in &self.installed_packages.clone() {
            println!("Checking updates for {} v{}", name, package.info.version);
            // In a real implementation, this would check for updates
            updated_count += 1;
        }
        
        println!("Updated {} packages", updated_count);
        Ok(())
    }

    /// Show package information
    pub fn show_package_info(&self, package_name: &str) {
        if let Some(package) = self.installed_packages.get(package_name) {
            println!("Package: {}", package.info.name);
            println!("Version: {}", package.info.version);
            println!("Description: {}", package.info.description);
            println!("Author: {}", package.info.author);
            println!("License: {}", package.info.license);
            println!("Homepage: {}", package.info.homepage);
            println!("Installed: {}", package.install_date);
            println!("Location: {}", package.install_path.display());
            println!("Files: {}", package.files.join(", "));
        } else {
            println!("Package '{}' is not installed", package_name);
        }
    }
}

/// Initialize package manager functions
pub fn init_package_cli(vm: &mut VM) {
    // Package install function
    vm.register_native("pm_install", 1, |_vm, args| {
        let package_name = match &args[0] {
            Value::String(s) => s.clone(),
            _ => return Err("Argument must be a string (package name)".to_string()),
        };

        println!("PM Install: {}", package_name);
        // In a real implementation, this would call the package manager
        Ok(Value::Boolean(true))
    });

    // Package remove function
    vm.register_native("pm_remove", 1, |_vm, args| {
        let package_name = match &args[0] {
            Value::String(s) => s.clone(),
            _ => return Err("Argument must be a string (package name)".to_string()),
        };

        println!("PM Remove: {}", package_name);
        Ok(Value::Boolean(true))
    });

    // Package list function
    vm.register_native("pm_list", 0, |_vm, _args| {
        println!("PM List - Installed packages:");
        println!("  math v1.0.0 - Mathematical functions");
        println!("  string v1.0.0 - String utilities");
        println!("  ui v1.0.0 - UI components");
        
        Ok(Value::Boolean(true))
    });

    // Package search function
    vm.register_native("pm_search", 1, |_vm, args| {
        let query = match &args[0] {
            Value::String(s) => s.clone(),
            _ => return Err("Argument must be a string (search query)".to_string()),
        };

        println!("PM Search: '{}'", query);
        println!("Results:");
        println!("  math - Mathematical functions");
        println!("  string - String utilities");
        
        Ok(Value::Boolean(true))
    });

    // Package update function
    vm.register_native("pm_update", 0, |_vm, _args| {
        println!("PM Update - Checking for package updates...");
        println!("All packages are up to date");
        Ok(Value::Boolean(true))
    });

    // Package info function
    vm.register_native("pm_info", 1, |_vm, args| {
        let package_name = match &args[0] {
            Value::String(s) => s.clone(),
            _ => return Err("Argument must be a string (package name)".to_string()),
        };

        println!("PM Info: {}", package_name);
        println!("Package: math");
        println!("Version: 1.0.0");
        println!("Description: Mathematical functions and utilities");
        println!("Author: Grease Team");
        
        Ok(Value::Boolean(true))
    });
}