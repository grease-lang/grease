# Grease UI Library

This is the UI library for the Grease programming language. It provides UI functionality that can be used as a separate library.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
grease-ui = { path = "../grease-ui", features = ["ui"] }
```

## Usage

```rust
use grease_ui::init_ui;
use grease::vm::VM;

fn main() {
    let mut vm = VM::new();
    grease_ui::init_ui(&mut vm);
    
    // Now you can use UI functions in Grease scripts
    // ui_window_create("My Window", 800, 600);
    // ui_button_create("Click me", 100, 100);
    // ui_run();
}
```

## Features

- Window management
- Button creation and handling
- Label display
- Input fields
- Hybrid UI system with Dioxus integration
- Performance benchmarking

## System Dependencies

On Linux, you need to install GTK development libraries:

```bash
# Ubuntu/Debian
sudo apt-get install libgtk-3-dev libgdk-pixbuf2.0-dev libpango1.0-dev libatk1.0-dev libcairo-gobject2

# Fedora/RHEL
sudo dnf install gtk3-devel gdk-pixbuf2-devel pango-devel atk-devel cairo-devel

# Arch Linux
sudo pacman -S gtk3 gdk-pixbuf2 pango atk cairo
```