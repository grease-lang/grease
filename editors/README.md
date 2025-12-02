# Grease Language Server Protocol (LSP) Implementation

This directory contains configuration files for using the Grease Language Server with various editors.

## VSCode Extension

### Installation

1. Build the Grease language server:
   ```bash
   cargo build --release
   ```

2. Install the VSCode extension:
   ```bash
   cd editors/vscode
   npm install
   npm run compile
   code --install-extension .
   ```

### Manual Installation

Alternatively, you can manually install the extension:

1. Copy the `editors/vscode` directory to your VSCode extensions folder
2. Update the `package.json` with the correct path to the `grease` binary
3. Install dependencies and compile

### Features

- Syntax highlighting for `.grease` files
- Auto-completion for keywords and symbols
- Go to definition
- Find references
- Error checking and diagnostics
- Hover information

## Neovim Configuration

### Prerequisites

- Neovim 0.5+ with LSP support
- `nvim-lspconfig` plugin

### Setup

1. Build the Grease language server:
   ```bash
   cargo build --release
   ```

2. Add the Grease LSP configuration to your Neovim config:

   ```lua
   -- Add to your init.lua
   dofile('/path/to/grease/editors/neovim/grease-lsp.lua')
   ```

   Or copy the content of `grease-lsp.lua` to your configuration.

3. Ensure the `grease` binary is in your PATH or update the `cmd` in the configuration.

### Features

- LSP completion
- Go to definition (`gd`)
- Hover documentation (`K`)
- Find references (`gr`)
- Diagnostics and error checking
- Basic syntax highlighting

## Basic Editor Configuration (Kate, etc.)

For editors that support external LSP servers:

1. Build the language server:
   ```bash
   cargo build --release
   ```

2. Configure your editor to use:
   - Command: `grease lsp`
   - Language: `grease`
   - File extensions: `.grease`

## Language Server Features

The Grease LSP server provides:

- **Text Document Synchronization**: Real-time file updates
- **Completion**: Keywords, functions, and variables
- **Go to Definition**: Navigate to symbol definitions
- **Find References**: Locate all symbol usages
- **Hover**: Documentation and type information
- **Diagnostics**: Syntax and semantic error reporting
- **Document Symbols**: Outline view of file structure
- **Workspace Symbols**: Search across all files
- **Semantic Tokens**: Enhanced syntax highlighting

## Testing

To test the LSP server:

```bash
# Start the server
./target/release/grease lsp

# In another terminal, test with a client
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"rootUri":"file:///tmp","capabilities":{}}}' | ./target/release/grease lsp
```

## Troubleshooting

### Common Issues

1. **Server not found**: Ensure the `grease` binary is in your PATH
2. **Permission denied**: Make the binary executable (`chmod +x grease`)
3. **Diagnostics not showing**: Check that the file is saved and has `.grease` extension
4. **Completion not working**: Verify the LSP client is properly configured

### Debug Logging

Enable debug logging for troubleshooting:

```bash
RUST_LOG=debug ./target/release/grease lsp
```

### File Association

Make sure your editor associates `.grease` files with the Grease language:

- VSCode: Automatic via extension
- Neovim: Configured in the provided setup
- Other editors: Manual configuration may be required