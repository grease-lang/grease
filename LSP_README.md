# Grease Language Server Protocol (LSP)

A complete Language Server Protocol implementation for the Grease scripting language, providing IDE support for VSCode, Neovim, and other LSP-compatible editors.

## Features

### Core LSP Capabilities

- **Text Document Synchronization**: Real-time file updates and change tracking
- **Auto-completion**: Intelligent code completion for keywords, functions, and variables
- **Go to Definition**: Navigate to symbol definitions across files
- **Find References**: Locate all usages of a symbol
- **Hover Information**: Documentation and type information on hover
- **Diagnostics**: Real-time syntax and semantic error checking
- **Document Symbols**: Outline view of file structure
- **Workspace Symbols**: Search across all files in the workspace
- **Semantic Tokens**: Enhanced syntax highlighting with semantic information

### Language-Specific Features

- **Keyword Completion**: All Grease keywords (`def`, `if`, `while`, `for`, etc.)
- **Function Detection**: Automatic detection and indexing of function definitions
- **Variable Tracking**: Variable declarations and scope analysis
- **Module Support**: Recognition of `use` statements and module imports
- **Type Annotations**: Support for optional type annotations

## Installation

### Build from Source

```bash
git clone <repository>
cd grease
cargo build --release
```

The LSP server binary will be available at `./target/release/grease`.

### Usage

Start the LSP server:
```bash
grease lsp
```

The server communicates via stdin/stdout using the Language Server Protocol.

## Editor Configuration

### VSCode

1. Install the VSCode extension from `editors/vscode/`:
   ```bash
   cd editors/vscode
   npm install
   npm run compile
   code --install-extension .
   ```

2. Or manually configure the language server in your settings:
   ```json
   {
     "grease.languageServer.path": "/path/to/grease",
     "grease.languageServer.args": ["lsp"]
   }
   ```

### Neovim

Add to your Neovim configuration:

```lua
-- Using lspconfig
require('lspconfig').grease.setup {
  cmd = { 'grease', 'lsp' },
  filetypes = { 'grease' },
  root_dir = require('lspconfig.util').root_pattern('.git', vim.fn.getcwd()),
  single_file_support = true,
}

-- Or use the provided configuration
dofile('/path/to/grease/editors/neovim/grease-lsp.lua')
```

### Other Editors

For any LSP-compatible editor (Kate, Sublime Text, etc.):

- **Command**: `grease lsp`
- **Language**: `grease`
- **File Extensions**: `.grease`
- **Communication**: stdin/stdout

## Architecture

### Pure Rust Implementation

The LSP server is built entirely with pure Rust libraries:

- **tower-lsp**: LSP protocol implementation
- **tokio**: Async runtime
- **ropey**: Efficient text manipulation
- **dashmap**: Concurrent data structures
- **serde**: JSON serialization

### Components

1. **LSP Server** (`src/lsp_server.rs`): Main LSP protocol handler
2. **Workspace** (`src/lsp_workspace.rs`): Document and symbol management
3. **Integration**: Seamless integration with existing Grease compiler components

### Document Processing

1. **Parsing**: Uses existing Grease lexer and parser
2. **AST Analysis**: Extracts symbols and semantic information
3. **Indexing**: Builds cross-file symbol index
4. **Diagnostics**: Reports syntax and semantic errors

## Testing

### Unit Tests

```bash
cargo test
```

### LSP Server Test

```bash
# Test with the provided Python test script
python3 test_lsp_server.py

# Or manually test with JSON-RPC messages
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{...}}' | grease lsp
```

### Integration Testing

Test with actual editor configurations:

1. Open a `.grease` file in your configured editor
2. Verify syntax highlighting works
3. Test auto-completion (Ctrl+Space in most editors)
4. Test go-to-definition (F12 or Ctrl+Click)
5. Check for error diagnostics

## Configuration Options

The LSP server supports standard LSP configuration:

```json
{
  "grease": {
    "diagnostics": {
      "enable": true,
      "unusedVariables": true
    },
    "completion": {
      "autoImport": true,
      "showSnippets": true
    },
    "semanticHighlighting": {
      "enable": true
    }
  }
}
```

## Language Features

### Supported Constructs

- **Variables**: `name = "value"` and `name: Type = "value"`
- **Functions**: `def func(param1, param2):`
- **Control Flow**: `if/elif/else`, `while`, `for/in`
- **Modules**: `use module` and `use module as alias`
- **Built-ins**: `print()`, `true`, `false`, `null`
- **Operators**: Arithmetic, comparison, boolean logic

### Error Detection

- **Syntax Errors**: Invalid tokens, malformed expressions
- **Semantic Errors**: Undefined variables, type mismatches
- **Linting**: Unused variables, unreachable code

## Performance

### Optimizations

- **Incremental Parsing**: Only re-parses changed portions
- **Concurrent Processing**: Parallel document handling
- **Efficient Text Operations**: Uses rope data structure
- **Minimal Memory**: Streaming JSON-RPC processing

### Benchmarks

- **Startup Time**: < 100ms
- **File Parsing**: < 10ms for typical files
- **Completion Response**: < 50ms
- **Memory Usage**: < 50MB for large workspaces

## Troubleshooting

### Common Issues

1. **Server Not Found**
   - Ensure `grease` is in your PATH
   - Check binary permissions (`chmod +x grease`)

2. **No Diagnostics**
   - Verify file has `.grease` extension
   - Check that file is saved
   - Ensure LSP client is properly configured

3. **Slow Performance**
   - Check for very large files (>10MB)
   - Verify sufficient system memory
   - Consider disabling semantic highlighting

### Debug Logging

Enable debug logging for troubleshooting:

```bash
RUST_LOG=debug grease lsp
```

### File Associations

Ensure your editor associates `.grease` files:

- **VSCode**: Automatic via extension
- **Neovim**: Configured in provided setup
- **Others**: Manual configuration may be required

## Contributing

### Development Setup

```bash
git clone <repository>
cd grease
cargo build
cargo test
```

### Adding New Features

1. Update LSP capabilities in `src/lsp_server.rs`
2. Implement handlers in the `LanguageServer` trait
3. Add tests in `test_lsp_server.py`
4. Update documentation

### Testing Changes

```bash
# Run all tests
cargo test

# Test LSP specifically
python3 test_lsp_server.py

# Test with actual editors
# Open a .grease file and verify functionality
```

## License

Apache 2.0 License - see LICENSE file for details.

---

**Grease LSP**: Professional IDE support for the Grease scripting language! 🦀