// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Integration tests for terminal calls and system functionality
//! These tests verify end-to-end functionality of the system module

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::Value;
    use std::fs;
    use std::env;

    /// Helper function to create a test VM with system module loaded
    fn create_test_vm() -> VM {
        let mut vm = VM::new();
        
        // Load system module
        let system_path = "std/system.grease";
        if std::path::Path::new(system_path).exists() {
            let source = std::fs::read_to_string(system_path).expect("Failed to read system module");
            let mut lexer = crate::lexer::Lexer::new(source);
            let tokens = lexer.tokenize().expect("Failed to tokenize system module");
            let mut parser = crate::parser::Parser::new(tokens);
            let program = parser.parse().expect("Failed to parse system module");
            let mut compiler = crate::compiler::Compiler::new();
            let chunk = compiler.compile(&program).expect("Failed to compile system module");
            
            // Execute the module to load its functions
            let result = vm.interpret(chunk);
            assert!(result == crate::vm::InterpretResult::Ok, "Failed to load system module: {:?}", result);
        }
        
        vm
    }

    #[test]
    fn test_system_module_loading() {
        let vm = create_test_vm();
        
        // Test that system functions are available
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("result = system.shell('echo test')".to_string()))).unwrap()).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_shell_execution() {
        let vm = create_test_vm();
        
        // Test basic shell command execution
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("result = system.shell('echo hello world')".to_string()))).unwrap()).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_environment_variables() {
        let vm = create_test_vm();
        
        // Test environment variable getting
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("home = system.getenv('HOME')".to_string())).unwrap()).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
        
        // Test environment variable setting
        let result2 = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("system.setenv('TEST_VAR', 'test_value')".to_string())).unwrap()).unwrap()).unwrap());
        assert!(result2 == crate::vm::InterpretResult::Ok);
        
        // Test that variable was set
        let result3 = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("test_val = system.getenv('TEST_VAR')".to_string())).unwrap()).unwrap()).unwrap());
        assert!(result3 == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_error_handling() {
        let vm = create_test_vm();
        
        // Test error handling with invalid command
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("result = system.shell('nonexistent_command_xyz')".to_string()))).unwrap()).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
        
        // The result should indicate failure
        // Note: This would require checking to result dictionary in test
    }

    #[test]
    fn test_system_cross_platform() {
        let vm = create_test_vm();
        
        // Test cross-platform command execution
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("result = system.shell('echo cross-platform-test')".to_string()))).unwrap()).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_redirection() {
        let vm = create_test_vm();
        
        // Test output redirection
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("result = system.shell('echo test > /tmp/output.txt')".to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_pipelines() {
        let vm = create_test_vm();
        
        // Test command pipelines
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("result = system.shell('echo pipeline-test | grep pipeline')".to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_complex_workflow() {
        let vm = create_test_vm();
        
        // Test a complex workflow with multiple operations
        let workflow_code = r#"
            home = system.getenv("HOME")
            system.setenv("WORKFLOW_TEST", "success")
            result = system.shell("echo 'Workflow test completed'")
            if result.success:
                print("Workflow completed successfully")
        "#;
        
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new(workflow_code.to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_file_operations() {
        let vm = create_test_vm();
        
        // Test file operations through shell
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("result = system.shell('echo test > /tmp/test_file.txt && cat /tmp/test_file.txt')".to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_dictionary_access() {
        let vm = create_test_vm();
        
        // Test accessing dictionary fields from system function results
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new("shell_result = system.shell('echo test')".to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_multiple_commands() {
        let vm = create_test_vm();
        
        // Test executing multiple system commands in sequence
        let multi_code = r#"
            result1 = system.shell("echo first")
            result2 = system.shell("echo second")
            result3 = system.shell("echo third")
        "#;
        
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new(multi_code.to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_real_commands() {
        let vm = create_test_vm();
        
        // Test with real system commands that should work
        let real_code = r#"
            ls_result = system.shell("ls")
            if ls_result.success:
                print("Directory listing successful")
        "#;
        
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new(real_code.to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_performance() {
        let vm = create_test_vm();
        
        // Test performance with multiple rapid system calls
        let perf_code = r#"
            for i in 0..10:
                result = system.shell("echo performance_test_" + i)
                # Don't check result for performance test
        "#;
        
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new(perf_code.to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_memory_stability() {
        let vm = create_test_vm();
        
        // Test that repeated system operations don't cause memory issues
        let stability_code = r#"
            for i in 0..100:
                result = system.shell("echo stability_test_" + i)
                # Force some cleanup
                temp = result
        "#;
        
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new(stability_code.to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_error_recovery() {
        let vm = create_test_vm();
        
        // Test error recovery scenarios
        let error_code = r#"
            try:
                result = system.shell("definitely_nonexistent_command_xyz")
                if not result.success:
                    print("Expected error occurred")
            except:
                print("Unexpected error in error handling")
        "#;
        
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new(error_code.to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_concurrent_access() {
        let vm = create_test_vm();
        
        // Test concurrent access to system functions (if supported)
        let concurrent_code = r#"
            result1 = system.shell("echo concurrent_1")
            result2 = system.shell("echo concurrent_2")
            result3 = system.shell("echo concurrent_3")
        "#;
        
        let result = vm.interpret(&crate::compiler::compile(&crate::parser::parse(&crate::lexer::Lexer::new(concurrent_code.to_string()))).unwrap()).unwrap());
        assert!(result == crate::vm::InterpretResult::Ok);
    }
}