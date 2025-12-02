#!/usr/bin/env python3
"""Simple test script for Grease LSP server"""

import subprocess
import json
import sys

def send_message(proc, message):
    """Send a JSON-RPC message to the LSP server"""
    content = json.dumps(message)
    content_bytes = content.encode('utf-8')
    content_length = len(content_bytes)
    header = f"Content-Length: {content_length}\r\n\r\n".encode('utf-8')
    proc.stdin.write(header + content_bytes)
    proc.stdin.flush()

def read_message(proc):
    """Read a JSON-RPC message from the LSP server"""
    # Read headers
    headers = {}
    while True:
        line = proc.stdout.readline().decode('utf-8').strip()
        if not line:
            break
        if ':' in line:
            key, value = line.split(':', 1)
            headers[key.strip()] = value.strip()
    
    # Read content
    content_length = int(headers.get('Content-Length', 0))
    if content_length > 0:
        content = proc.stdout.read(content_length).decode('utf-8')
        return json.loads(content)
    return None

def test_lsp():
    """Test the Grease LSP server"""
    # Start the LSP server
    proc = subprocess.Popen(
        ['./target/release/grease', 'lsp'],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=False
    )
    
    try:
        # Initialize
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "rootUri": "file:///home/coder/Projects/grease",
                "capabilities": {
                    "textDocument": {
                        "completion": {
                            "completionItem": {
                                "snippetSupport": True
                            }
                        }
                    }
                }
            }
        }
        
        print("Sending initialize request...")
        send_message(proc, init_request)
        
        # Read response
        response = read_message(proc)
        if response:
            print("Initialize response:", json.dumps(response, indent=2))
        
        # Send initialized notification
        initialized_notification = {
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {}
        }
        
        print("Sending initialized notification...")
        send_message(proc, initialized_notification)
        
        # Open a document
        open_request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///home/coder/Projects/grease/test_lsp.grease",
                    "languageId": "grease",
                    "version": 1,
                    "text": open("test_lsp.grease").read()
                }
            }
        }
        
        print("Opening document...")
        send_message(proc, open_request)
        
        # Test completion
        completion_request = {
            "jsonrpc": "2.0",
            "id": 3,
            "method": "textDocument/completion",
            "params": {
                "textDocument": {
                    "uri": "file:///home/coder/Projects/grease/test_lsp.grease"
                },
                "position": {
                    "line": 0,
                    "character": 4
                }
            }
        }
        
        print("Requesting completion...")
        send_message(proc, completion_request)
        
        response = read_message(proc)
        if response:
            print("Completion response:", json.dumps(response, indent=2))
        
        print("LSP server test completed successfully!")
        
    except Exception as e:
        print(f"Error: {e}")
    finally:
        proc.terminate()
        proc.wait()

if __name__ == "__main__":
    test_lsp()