# Grease WebAssembly Library

This is the WebAssembly library for Grease programming language. It provides WebAssembly compilation and execution functionality that can be used as a separate library.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
grease-webassembly = { path = "../grease-webassembly" }
```

## Usage

```rust
use grease_webassembly::init_webassembly;
use grease::vm::VM;

fn main() {
    let mut vm = VM::new();
    grease_webassembly::init_webassembly(&mut vm);
    
    // Now you can use WebAssembly functions in Grease scripts
    // wasm_init();
    // wasm_compile("print('Hello from WASM!')");
    // wasm_available();
    // wasm_stats();
}
```

## Features

- WebAssembly compilation from Grease bytecode
- JavaScript wrapper generation
- Web API integration
- Runtime statistics
- Cross-platform support

## Example

```grease
# Initialize WebAssembly runtime
wasm_init()

# Check if WebAssembly is available
if wasm_available():
    print("WebAssembly is available!")
    
    # Get runtime statistics
    stats = wasm_stats()
    print(stats)
    
    # Compile some code to WebAssembly
    result = wasm_compile("print('Hello from WebAssembly!')")
    print(result)
else:
    print("WebAssembly is not available")
```