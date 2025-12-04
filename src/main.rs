// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use clap_mangen::Man;
use grease::Grease;
use grease::repl::REPL;
use grease::vm::InterpretResult;
use grease::lsp_server::run_server;
use std::fs;
use std::io;

#[derive(Parser)]
#[command(name = "grease")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A modern scripting language written in Rust")]
#[command(long_about = "Grease is a scripting language written in pure Rust. It compiles to bytecode and runs on a custom VM.\n\nThe high-performance oil for your Rust engine.")]
struct Args {
    /// Execute inline code
    #[arg(short, long)]
    eval: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// File to execute
    file: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Generate manpage
    Manpage,
    /// Lint Grease source code
    Lint {
        /// File to lint
        file: String,
    },
    /// Start Language Server Protocol server
    Lsp,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Completions { shell }) => {
            generate(shell, &mut Args::command(), "grease", &mut io::stdout());
        }
        Some(Commands::Manpage) => {
            let man = Man::new(Args::command());
            man.render(&mut io::stdout()).unwrap();
        }
        Some(Commands::Lint { file }) => {
            match fs::read_to_string(&file) {
                Ok(source) => {
                    let mut grease = Grease::new().with_verbose(args.verbose);
                    match grease.lint(&source) {
                        Ok(errors) => {
                            if errors.is_empty() {
                                println!("No lint errors found.");
                            } else {
                                for error in errors {
                                    println!("{}:{}:{}: {}", file, error.line, error.column, error.message);
                                }
                                std::process::exit(1);
                            }
                        }
                        Err(msg) => {
                            eprintln!("Lint Error: {}", msg);
                            std::process::exit(1);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error reading file '{}': {}", file, err);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Lsp) => {
            // Start LSP server
            if let Err(e) = tokio::runtime::Runtime::new().unwrap().block_on(run_server()) {
                eprintln!("LSP server error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            if let Some(code) = args.eval {
                // Execute inline code
                let mut grease = Grease::new().with_verbose(args.verbose);
                match grease.run(&code) {
                    Ok(result) => match result {
                        InterpretResult::Ok => {}
                        InterpretResult::CompileError(msg) => {
                            eprintln!("Compile Error: {}", msg);
                            std::process::exit(1);
                        }
                        InterpretResult::RuntimeError(msg) => {
                            eprintln!("Runtime Error: {}", msg);
                            std::process::exit(1);
                        }
                    },
                    Err(msg) => {
                        eprintln!("Error: {}", msg);
                        std::process::exit(1);
                    }
                }
            } else if let Some(filename) = args.file {
                // Run script file
                match fs::read_to_string(&filename) {
                    Ok(source) => {
                        let mut grease = Grease::new().with_verbose(args.verbose);
                        match grease.run(&source) {
                            Ok(result) => match result {
                                InterpretResult::Ok => {}
                                InterpretResult::CompileError(msg) => {
                                    eprintln!("Compile Error: {}", msg);
                                    std::process::exit(1);
                                }
                                InterpretResult::RuntimeError(msg) => {
                                    eprintln!("Runtime Error: {}", msg);
                                    std::process::exit(1);
                                }
                            },
                            Err(msg) => {
                                eprintln!("Error: {}", msg);
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading file '{}': {}", filename, err);
                        std::process::exit(1);
                    }
                }
            } else {
                // Run REPL
                let mut repl = REPL::new();
                repl.run();
            }
        }
    }
}
