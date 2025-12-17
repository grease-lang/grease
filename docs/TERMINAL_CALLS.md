# Terminal Calls and System Integration

Grease provides comprehensive terminal call capabilities, allowing you to execute external commands, manage processes, and integrate with system operations.

## Overview

The `system` module provides a safe and user-friendly interface to execute terminal commands and manage processes. All functions return structured results with consistent error handling.

## Basic Command Execution

### `system.exec(command, args...)`

Execute a command synchronously and wait for completion.

```grease
use system

# Simple command
result = system.exec("echo", "Hello World")
print(result.stdout)  # Hello World
print(result.success) # true
print(result.exit_code) # 0

# Command with multiple arguments
result = system.exec("ls", "-la", "/tmp")
if result.success:
    print("Directory listing:")
    print(result.stdout)
else:
    print("Command failed:")
    print(result.stderr)
```

### `system.shell(command_string)`

Execute a shell command string. Useful for complex commands with pipes and redirection.

```grease
use system

# Simple shell command
result = system.shell("echo 'Hello from shell'")
print(result.stdout)

# Complex command with pipes
result = system.shell("ls -la | grep '\.grease' | wc -l")
print("Grease files count: " + result.stdout)
```

## Process Management

### `system.spawn(command, args...)`

Spawn a command asynchronously in the background.

```grease
use system

# Start a background process
pid = system.spawn("sleep", "10")
print("Process started with PID: " + pid)

# Check status
status = system.status(pid)
print("Process status: " + status)  # "running" or "not_found"
```

### `system.wait(pid)`

Wait for a background process to complete.

```grease
use system

pid = system.spawn("echo", "Background task")
result = system.wait(pid)
print("Process completed with exit code: " + result.exit_code)
```

### `system.kill(pid, signal)`

Send a signal to a running process.

```grease
use system

pid = system.spawn("sleep", "100")
# Send SIGTERM (signal 15)
success = system.kill(pid, 15)
print("Kill signal sent: " + success)
```

## Environment Variables

### `system.getenv(name)`

Get an environment variable.

```grease
use system

home = system.getenv("HOME")
path = system.getenv("PATH")
user = system.getenv("USER")

print("Home directory: " + home)
print("Current user: " + user)
```

### `system.setenv(name, value)`

Set an environment variable.

```grease
use system

system.setenv("GREASE_TEST", "terminal_calls_working")
test_value = system.getenv("GREASE_TEST")
print("Test value: " + test_value)
```

### `system.environ()`

Get all environment variables as a dictionary.

```grease
use system

all_vars = system.environ()
print("Environment variables loaded: " + all_vars)
print("PATH = " + all_vars["PATH"])
```

## Advanced Operations

### `system.pipe(command1, command2)`

Pipe output from one command to another.

```grease
use system

result = system.pipe("echo 'hello world'", "grep 'hello'")
print(result.stdout)  # hello world
```

### `system.redirect(command, stdout_file, stderr_file)`

Redirect command output to files.

```grease
use system

# Redirect stdout only
result = system.redirect("echo 'test output'", "output.txt")

# Redirect both stdout and stderr
result = system.redirect("ls /tmp /nonexistent", "output.txt", "error.txt")
```

### `system.timeout(command, seconds)`

Execute a command with a timeout.

```grease
use system

# Timeout after 5 seconds
result = system.timeout("sleep 10", 5)
if result.success:
    print("Command completed within timeout")
else:
    print("Command timed out or failed")
```

## Async Operations

### `system.async_exec(command, args...)`

Execute a command asynchronously (currently delegates to sync version).

```grease
use system

result = system.async_exec("echo", "async execution")
print(result.stdout)
```

## Streaming and Monitoring

### `system.stream_exec(command, args...)`

Execute a command with real-time output streaming.

```grease
use system

result = system.stream_exec("ls", "-la")
print("Streaming result: " + result.streamed)
print(result.stdout)
```

### `system.monitor_process(pid)`

Monitor a process with real-time output (placeholder implementation).

```grease
use system

status = system.monitor_process("12345")
print("Monitoring status: " + status.status)
```

## Result Structure

All system functions return a consistent result structure:

```grease
{
    exit_code: Number,    # Process exit code (0 = success)
    stdout: String,       # Standard output
    stderr: String,       # Standard error
    success: Boolean,     # true if exit_code == 0
    pid: String,          # Process ID (for spawned processes)
    signal: Number        # Signal number (if terminated by signal)
}
```

## Error Handling

System functions handle errors gracefully and return structured error information:

```grease
use system

result = system.exec("nonexistent_command")
if not result.success:
    print("Command failed with exit code: " + result.exit_code)
    print("Error output: " + result.stderr)
```

## Security Considerations

- Commands are executed with the same permissions as the Grease process
- No built-in command whitelisting or blacklisting
- Environment variables are inherited from the parent process
- Process management respects OS-level security policies

## Cross-Platform Compatibility

The system module works on Linux, macOS, and Windows:

- **Linux/macOS**: Uses `/bin/sh` for shell commands
- **Windows**: Uses `cmd.exe` for shell commands
- Path separators and environment variable handling are platform-aware
- Command availability may vary between platforms

## Examples

### File Processing Pipeline

```grease
use system

# Create test files
system.shell("echo 'line 1' > test1.txt")
system.shell("echo 'line 2' > test2.txt")

# Process files
result = system.shell("cat test1.txt test2.txt | sort | uniq")
print("Processed output:")
print(result.stdout)

# Clean up
system.shell("rm test1.txt test2.txt")
```

### Background Task Management

```grease
use system

# Start multiple background tasks
pids = []
pids.add(system.spawn("sleep", "2"))
pids.add(system.spawn("sleep", "3"))
pids.add(system.spawn("sleep", "1"))

print("Started " + pids.length() + " background tasks")

# Wait for all to complete
for pid in pids:
    result = system.wait(pid)
    print("Task " + pid + " completed with code " + result.exit_code)
```

### System Information Gathering

```grease
use system

# Get system information
os_result = system.shell("uname -a")
cpu_result = system.shell("nproc")
mem_result = system.shell("free -h")

print("OS: " + os_result.stdout)
print("CPU cores: " + cpu_result.stdout)
print("Memory: " + mem_result.stdout)
```

## Integration with External Tools

The system module enables integration with external tools like `yt-dlp`, `ffmpeg`, `curl`, etc.:

```grease
use system

# Check if yt-dlp is available
check = system.shell("which yt-dlp")
if check.success:
    print("yt-dlp is available")

    # Example download (commented for safety)
    # result = system.shell("yt-dlp -x --audio-format mp3 https://youtube.com/watch?v=VIDEO_ID")
    # if result.success:
    #     print("Download completed")
    # else:
    #     print("Download failed: " + result.stderr)
else:
    print("yt-dlp not found - install with: pip install yt-dlp")
```

This comprehensive terminal call system makes Grease suitable for automation, system administration, and integration with external tools and services.