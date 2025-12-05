# Grease UI System Documentation

## Overview

Grease features a **Hybrid UI System** that combines the flexibility of VM-based UI components with the performance of pure Rust rendering. This system eliminates the traditional 30-second startup times associated with interpreted GUI applications while maintaining full scripting capabilities.

## Architecture

```
Source Code → Lexer → Tokens → Parser → AST → Compiler → Bytecode → VM → Execution
                                                    ↓
                                              Hybrid UI System
                                                    ↓
                                            Dioxus + eframe Rendering
```

### Performance Comparison

| Approach | Startup Time | Render Time | Memory Usage | Flexibility |
|----------|-------------|-------------|-------------|------------|
| **Pure VM** | ~30 seconds | Slow | High | Maximum |
| **Hybrid UI** | ~200ms | Fast | Medium | High |
| **Pure Rust** | ~100ms | Fastest | Low | Limited |

## UI Function Types

### 1. Traditional VM-Based UI Functions

These functions provide maximum flexibility but go through the VM interpreter:

```grease
# Window management
ui_window_create(title, width, height, window_id)
ui_window_show(window_id)
ui_window_hide(window_id)

# Widget creation
ui_button_add(window_id, button_id, label, x, y, width)
ui_label_add(window_id, label_id, text, x, y)
ui_input_add(window_id, input_id, label, x, y, width)

# Event handling
ui_button_clicked(window_id, button_id)
ui_input_get_value(window_id, input_id)

# UI control
ui_run()
ui_stop()
```

**Use Cases:**
- Dynamic UI generation
- Complex layout calculations
- Runtime widget modification
- Script-heavy applications

### 2. High-Performance Hybrid UI Functions

These functions bypass the VM for native Rust performance:

```grease
# High-performance widget creation
button_id = ui_create_button_pure(label, callback_name)
label_id = ui_create_label_pure(text)
input_id = ui_create_input_pure(placeholder)

# Component management
ui_add_hybrid_component(window_id, component_id)
ui_get_component_value(component_id)
ui_component_clicked(component_id)
```

**Use Cases:**
- Static UI elements
- Performance-critical applications
- Large numbers of widgets
- Real-time interfaces

## Usage Examples

### Basic Hybrid UI Application

```grease
# Create main window
ui_window_create("My App", 800, 600, "main_window")

# Mix traditional and hybrid approaches
ui_button_add("main_window", "vm_btn", "VM Button", 10, 10, 120)
fast_button = ui_create_button_pure("Fast Button", "handle_fast_click")

# Add hybrid component to window
ui_add_hybrid_component("main_window", fast_button)

# Show window and start event loop
ui_window_show("main_window")

# Event handling
while true:
    if ui_button_clicked("main_window", "vm_btn"):
        print("VM button clicked!")
    
    if ui_component_clicked(fast_button):
        print("Fast button clicked!")
    
    # Small delay to prevent busy waiting
    break
```

### Performance-Optimized UI

```grease
# Create window
ui_window_create("Performance Demo", 1024, 768, "perf_window")

# Create many high-performance buttons
for i in range(0, 100):
    button_id = ui_create_button_pure("Button " + i, "handle_button_" + i)
    ui_add_hybrid_component("perf_window", button_id)

# Show and run
ui_window_show("perf_window")
ui_run()
```

## Advanced Features

### Template Caching

The hybrid UI system automatically caches common UI patterns for instant rendering:

- **Button Templates**: Pre-compiled button styles and layouts
- **Input Templates**: Cached input field configurations
- **Layout Templates**: Reusable layout patterns
- **Theme Templates**: Consistent styling across components

### Lazy Loading

UI components are loaded progressively to improve startup performance:

1. **Immediate**: Core window and essential widgets
2. **Background**: Non-critical components loaded asynchronously
3. **On-Demand**: Components created when first accessed

### Error Handling

The UI system provides comprehensive error handling:

```grease
# Safe UI operations with error checking
result = ui_create_button_pure("Test", "callback")
if result != null:
    print("Button created successfully")
else:
    print("Failed to create button")
```

## Performance Benchmarks

Use the built-in benchmarking system to test UI performance:

```grease
# Run performance benchmarks
benchmark = UIBenchmark.new()
results = benchmark.run_full_benchmark([10, 50, 100, 500])

# Print comparison
benchmark.print_comparison(100)
```

### Expected Results

For 100 widgets:
- **VM-based UI**: ~5000ms startup, ~2000ms render
- **Hybrid UI**: ~200ms startup, ~100ms render  
- **Pure Rust**: ~100ms startup, ~50ms render

## Best Practices

### 1. Choose the Right Approach

- **Use Hybrid UI** for: Static elements, performance-critical UI, large widget counts
- **Use VM UI** for: Dynamic content, complex layouts, script-heavy interfaces
- **Mix Both** for: Balanced performance and flexibility

### 2. Optimize Startup Performance

```grease
# Good: Create essential UI first
essential_button = ui_create_button_pure("Start", "start_app")
ui_add_hybrid_component("main_window", essential_button)

# Load secondary UI in background
# (handled automatically by lazy loading)
```

### 3. Handle Events Efficiently

```grease
# Efficient event handling loop
while ui_running():
    # Check hybrid components first (faster)
    if ui_component_clicked(essential_button):
        handle_start()
    
    # Then check VM components
    if ui_button_clicked("main_window", "settings_btn"):
        show_settings()
```

## Integration with LSP

The Language Server Protocol provides full support for UI functions:

- **Auto-completion** for all UI functions
- **Function signatures** with parameter hints
- **Documentation** on hover
- **Error checking** for UI function calls

Type `ui_` in any LSP-enabled editor to see all available UI functions.

## Troubleshooting

### Common Issues

1. **Slow Startup**: Use more hybrid UI functions instead of VM-based ones
2. **UI Not Showing**: Ensure `ui_window_show()` is called before `ui_run()`
3. **Events Not Working**: Check that components are added to windows with `ui_add_hybrid_component()`
4. **Memory Usage**: Monitor component creation and reuse component IDs where possible

### Debug Mode

Enable verbose output for UI debugging:

```bash
grease --verbose your_script.grease
```

This will show UI initialization, component creation, and event handling details.

## Future Enhancements

Planned improvements to the UI system:

- [ ] Advanced layout managers (grid, flexbox)
- [ ] Rich text and styling support
- [ ] Custom widget creation
- [ ] Animation and transitions
- [ ] Theme system integration
- [ ] Accessibility features
- [ ] Touch and gesture support

## Examples Repository

See the `examples/` directory for complete working examples:

- `hybrid_ui_performance.grease` - Performance comparison demo
- `ui_example.grease` - Basic UI usage
- `performance_example.grease` - Performance optimization techniques

---

**Grease UI System**: High-performance GUI development with scripting flexibility! 🚀