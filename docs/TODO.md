# Terminal Call Implementation TODO

## Overview
Implement comprehensive terminal call support for Grease programming language, allowing developers to execute external commands like `yt-dlp`, manage processes, and handle system operations.

## Phase1: Fix Critical Module Access Bug ⚠️ **PRIORITY: CRITICAL**

### Issue
Parser creates `PropertyAccess` instead of `ModuleAccess` for module calls like `math.add()`, preventing standard library modules from working.

### Files to Modify
- `src/parser.rs` - Fix `call()` function (lines 463-501)
- `src/compiler.rs` - Ensure `ModuleAccess` compilation works correctly  
- `src/ast.rs` - Verify `ModuleAccess` enum variant exists

### Implementation Details
1. Modify parser `call()` function to detect when left side of dot access could be a module
2. Create `ModuleAccess` expressions instead of `PropertyAccess` for module calls
3. Add module tracking to parser to distinguish modules from variables
4. Test with existing `std/math.grease` and `std/string.grease`

### Expected Fix Location
In `src/parser.rs` around line 489:
```rust
// CURRENT (BROKEN):
expr = Expression::PropertyAccess {
    object: Box::new(expr),
    property: member,
};

// NEEDED:
// Check if expr could be a module identifier
if let Expression::Identifier(ref name) = expr {
    // This should be ModuleAccess, not PropertyAccess
    expr = Expression::ModuleAccess {
        module: name.clone(),
        member: member,
    };
} else {
    expr = Expression::PropertyAccess {
        object: Box::new(expr),
        property: member,
    };
}
```

## Phase 2: Core Terminal Call Native Functions 🚀

### Create `src/native_system.rs`
New file for system-related native functions with comprehensive process control.

### Native Functions to Implement

#### Basic Command Execution
- `system_exec(command, args...)` - Execute synchronously
- `system_spawn(command, args...)` - Execute asynchronously  
- `system_shell(command_string)` - Execute shell commands

#### Process Control
- `system_wait(pid)` - Wait for process completion
- `system_kill(pid, signal)` - Send signal to process
- `system_status(pid)` - Get process status
- `system_background(command, args...)` - Run in background

#### Input/Output Handling
- `system_read_output(command, args...)` - Get command output
- `system_write_input(pid, input)` - Write to process stdin
- `system_capture(command, args...)` - Capture stdout/stderr separately

#### Environment Variables
- `system_getenv(name)` - Get environment variable
- `system_setenv(name, value)` - Set environment variable
- `system_environ()` - Get all environment variables

#### Advanced Process Control
- `system_pipe(command1, command2)` - Pipe commands
- `system_redirect(command, stdout_file, stderr_file)` - Redirect output
- `system_timeout(command, seconds)` - Execute with timeout

### Return Value Design
```rust
// For command execution results
ProcessResult {
    exit_code: i32,
    stdout: String,
    stderr: String,
    success: bool,
    pid: Option<String>,
    signal: Option<i32>,
}
```

### Error Handling Requirements
- Capture system error codes
- Detailed error messages for command not found
- Permission denied handling
- Timeout and signal handling
- Resource exhaustion protection

## Phase 3: Process Management Infrastructure 🔄

### Process Tracking System
- Add `HashMap<String, Child>` to VM for tracking spawned processes
- Implement process ID generation and management
- Add cleanup for orphaned processes
- Support for process trees and parent-child relationships

### Resource Management
- Memory limits for spawned processes
- CPU usage monitoring
- File descriptor tracking
- Timeout enforcement

### Security Model
- No whitelist/blacklist (per requirements)
- User permission system (OS-level)
- Current working directory control
- Audit logging capability

## Phase 4: Standard Library Module 📚

### Create `std/system.grease`
User-friendly wrapper functions that call native system functions.

### Module Structure
```grease
# Basic command execution
def exec(command, args...):
    """Execute command synchronously and return result"""
    return system_exec(command, args)

def spawn(command, args...):
    """Spawn command asynchronously and return process ID"""
    return system_spawn(command, args)

def shell(command):
    """Execute shell command string"""
    return system_shell(command)

# Process control
def wait(pid):
    """Wait for process to complete"""
    return system_wait(pid)

def kill(pid, signal):
    """Send signal to process"""
    return system_kill(pid, signal)

def status(pid):
    """Get process status"""
    return system_status(pid)

# I/O operations
def capture(command, args...):
    """Capture command output"""
    return system_capture(command, args)

def pipe(command1, command2):
    """Pipe output of command1 to input of command2"""
    return system_pipe(command1, command2)

# Environment variables
def getenv(name):
    """Get environment variable"""
    return system_getenv(name)

def setenv(name, value):
    """Set environment variable"""
    return system_setenv(name, value)

def environ():
    """Get all environment variables"""
    return system_environ()

# Advanced utilities
def background(command, args...):
    """Run command in background"""
    return system_background(command, args)

def redirect(command, stdout_file, stderr_file):
    """Redirect command output to files"""
    return system_redirect(command, stdout_file, stderr_file)

def timeout(command, seconds):
    """Execute command with timeout"""
    return system_timeout(command, seconds)
```

## Phase 5: Async Support ⚡

### Leverage Existing `tokio` Runtime
- Add async native functions using `tokio::process::Command`
- Implement `async_exec()` and `async_spawn()` functions
- Add callback mechanism for async completion

### Async Function Design
- `system_async_exec(command, args..., callback)` - Execute async with callback
- `system_async_wait(pid, callback)` - Wait async with callback
- `system_async_pipe(command1, command2, callback)` - Async pipe operations

## Phase 6: Advanced Features 🔧

### Real-time Output Streaming
- Separate system for piping terminal information
- Support for parsing progress bars and real-time updates
- Stream stdout/stderr separately for live monitoring
- Callback-based streaming for real-time processing

### Advanced Process Control
- Process tree visualization
- Signal handling with custom handlers
- Named pipes and FIFO support
- File system event monitoring

### Integration Features
- Job control (foreground/background process management)
- Process groups and sessions
- Resource usage monitoring
- Performance profiling for spawned processes

## Phase 7: Examples & Documentation 📖

### Create Example Files

#### `examples/terminal_calls.grease`
```grease
# Basic command execution
result = system.exec("ls", "-la")
print("Directory listing:")
print(result.stdout)

# Cross-platform command testing
print("Testing echo command:")
echo_result = system.exec("echo", "Hello from Grease!")
print(echo_result.stdout)

# Error handling
try:
    result = system.exec("nonexistent_command")
except:
    print("Command not found (expected)")

# Process management
pid = system.spawn("sleep", "5")
print("Sleep process PID: " + pid)

# Wait for completion
result = system.wait(pid)
print("Sleep completed with exit code: " + result.exit_code)
```

#### `examples/system_integration.grease`
```grease
# Environment variable handling
home = system.getenv("HOME")
print("Home directory: " + home)

# Set custom environment
system.setenv("GREASE_TEST", "Terminal calls working")
print("GREASE_TEST = " + system.getenv("GREASE_TEST"))

# Process pipeline
result = system.pipe("ls", "grep", ".grease")
print("Grease files:")
print(result.stdout)

# Background processing
pid = system.background("sleep", "1")
print("Background sleep PID: " + pid)

# Check status
status = system.status(pid)
print("Process status: " + status)
```

#### `examples/yt_dlp_integration.grease`
```grease
# YouTube download with yt-dlp (when available)
print("Testing yt-dlp integration...")

# Check if yt-dlp is available
try:
    version = system.exec("yt-dlp", "--version")
    print("yt-dlp found: " + version.stdout.split("\n")[0])
    
    # Example download (commented out to avoid actual download)
    # download = system.exec("yt-dlp", 
    #     "https://youtube.com/watch?v=VIDEO_ID", 
    #     "-o", "video.mp4",
    #     "--extract-audio"
    # )
    # 
    # if download.success {
    #     print("Download successful!")
    # } else {
    #     print("Download failed:")
    #     print(download.stderr)
    # }
except:
    print("yt-dlp not found - install with: pip install yt-dlp")
```

### Documentation Updates

#### Files to Update
- `README.md` - Add terminal call capabilities
- `AGENTS.md` - Document system module for AI agents
- `docs/TERMINAL_CALLS.md` - Comprehensive guide
- Update standard library documentation

#### Documentation Sections
- API reference for all system functions
- Security considerations and best practices
- Performance implications and optimization tips
- Troubleshooting common issues
- Cross-platform compatibility notes

## Phase 8: Testing & Validation ✅

### Test Categories

#### Unit Tests (`src/native_system.rs` tests)
- Command execution with various arguments
- Error handling for invalid commands
- Process spawning and management
- Environment variable operations
- Cross-platform command compatibility

#### Integration Tests (`tests/system_tests.rs`)
- End-to-end command execution
- Async operation workflows
- Process lifecycle management
- Security boundary testing
- Real-time output streaming

#### Example Tests
- Verify all example files run correctly
- Test with cross-platform commands (`echo`, `ls`, `dir`, etc.)
- Validate error handling scenarios
- Test with long-running processes

#### Cross-Platform Testing
- Linux: `ls`, `echo`, `cat`, `grep`
- macOS: `ls`, `echo`, `cat`, `grep`
- Windows: `dir`, `echo`, `type`, `findstr`
- Test platform-specific behaviors

## Phase 9: Performance & Security 🔒

### Performance Considerations
- Efficient process spawning
- Memory usage optimization
- Minimal overhead for simple commands
- Async operation performance
- Resource cleanup and leak prevention

### Security Features
- No built-in restrictions (per requirements)
- OS-level permission handling
- Current working directory control
- Audit logging capability
- Resource limits and monitoring

## Implementation Dependencies & Order

### Critical Path (Must be in order)
1. **Fix Module Access Bug** - Blocks all standard library functionality
2. **Basic Native Functions** - Core terminal execution capability
3. **Standard Library Module** - User-friendly API
4. **Testing** - Ensure reliability

### Parallel Development
- Advanced process control can be developed alongside basic functions
- Async support can be added after basic sync implementation
- Documentation can be written during development

### Risk Mitigation
- Comprehensive error handling for security
- Process resource limits to prevent abuse
- Extensive testing with various command types
- Fallback mechanisms for unsupported operations

## Technical Implementation Notes

### Native Function Registration
Add to `src/vm.rs` in `VM::new()` function:
```rust
// Register system native functions
vm.register_native("system_exec", 1, native_system::system_exec);
vm.register_native("system_spawn", 1, native_system::system_spawn);
vm.register_native("system_shell", 1, native_system::system_shell);
// ... etc for all system functions
```

### Module Loading Fix
The module access bug prevents standard library from working. This is the highest priority fix.

### Cross-Platform Considerations
- Use `std::process::Command` for cross-platform compatibility
- Handle different shell behaviors (`cmd.exe` vs `bash`)
- Path separator differences (`/` vs `\`)
- Environment variable handling differences

## Success Criteria

### Functional Requirements
- [ ] All system functions execute correctly
- [ ] Cross-platform commands work
- [ ] Error handling is comprehensive
- [ ] Process management is robust
- [ ] Async operations work
- [ ] Real-time output streaming functions
- [ ] All examples run without errors

### Quality Requirements
- [ ] All tests pass (including new system tests)
- [ ] No memory leaks in process management
- [ ] Error messages are helpful and detailed
- [ ] Documentation is complete and accurate
- [ ] Code follows Grease conventions
- [ ] Performance is acceptable

### Integration Requirements
- [ ] Module access bug is fixed
- [ ] Standard library modules load correctly
- [ ] System module integrates seamlessly
- [ ] Existing functionality is not broken
- [ ] LSP server handles new functions
- [ ] Build system works correctly

## Next Steps

1. **Immediate**: Fix module access bug in parser.rs
2. **High Priority**: Implement basic system native functions
3. **Medium Priority**: Create std/system.grease module
4. **Ongoing**: Add comprehensive tests and examples
5. **Final**: Update documentation and validate complete functionality

This implementation will provide Grease with comprehensive terminal call capabilities while maintaining security, performance, and cross-platform compatibility.