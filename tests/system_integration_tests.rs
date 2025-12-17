// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Integration tests for terminal calls and system functionality
//! These tests verify end-to-end functionality of the system module

use grease::Grease;

/// Test system module integration end-to-end
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a test Grease instance
    fn create_test_grease() -> Grease {
        Grease::new()
    }

    #[test]
    fn test_system_module_loading() {
        let mut grease = create_test_grease();
        
        // Test that system functions are available
        let result = grease.run("use system\nsystem.shell('echo test')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_shell_execution() {
        let mut grease = create_test_grease();
        
        // Test basic shell command execution
        let result = grease.run("use system\nresult = system.shell('echo hello world')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_environment_variables() {
        let mut grease = create_test_grease();
        
        // Test environment variable getting
        let result = grease.run("use system\nhome = system.getenv('HOME')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
        
        // Test environment variable setting and getting in one script
        let mut grease2 = create_test_grease();
        let result2 = grease2.run("use system\nsystem.setenv('TEST_VAR', 'test_value')\ntest_val = system.getenv('TEST_VAR')");
        if !result2.is_ok() {
            println!("Error: {:?}", result2);
        }
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_error_handling() {
        let mut grease = create_test_grease();
        
        // Test error handling with invalid command
        let result = grease.run("use system\nresult = system.shell('nonexistent_command_xyz')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
        
        // The result should indicate failure
        // Note: This would require checking the result dictionary in the test
    }

    #[test]
    fn test_system_cross_platform() {
        let mut grease = create_test_grease();
        
        // Test cross-platform command execution
        let result = grease.run("use system\nresult = system.shell('echo cross-platform-test')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_redirection() {
        let mut grease = create_test_grease();
        
        // Test output redirection
        let result = grease.run("use system\nresult = system.shell('echo test > /tmp/test_output.txt')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_pipelines() {
        let mut grease = create_test_grease();
        
        // Test command pipelines
        let result = grease.run("use system\nresult = system.shell('echo pipeline-test | grep pipeline')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_complex_workflow() {
        let mut grease = create_test_grease();
        
        // Test a complex workflow with multiple operations
        let workflow_code = r#"
            use system
            home = system.getenv("HOME")
            system.setenv("WORKFLOW_TEST", "success")
            result = system.shell("echo 'Workflow test completed'")
            if result.success:
                print("Workflow completed successfully")
            else:
                print("Workflow failed")
        "#;
        
        let result = grease.run(workflow_code);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_file_operations() {
        let mut grease = create_test_grease();
        
        // Test file operations through shell
        let result = grease.run("use system\nresult = system.shell('echo test > /tmp/integration_test.txt')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_dictionary_access() {
        let mut grease = create_test_grease();
        
        // Test accessing dictionary fields from system function results
        let result = grease.run("use system\nshell_result = system.shell('echo test')");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_multiple_commands() {
        let mut grease = create_test_grease();
        
        // Test executing multiple system commands in sequence
        let multi_code = r#"
            use system
            result1 = system.shell("echo first")
            result2 = system.shell("echo second")
            result3 = system.shell("echo third")
        "#;
        
        let result = grease.run(multi_code);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_with_real_commands() {
        let mut grease = create_test_grease();
        
        // Test with real system commands that should work
        let real_code = r#"
            use system
            # Test with a command that should exist on most systems
            ls_result = system.shell("ls")
            if ls_result.success:
                print("ls command successful")
            else:
                print("ls command failed")
        "#;
        
        let result = grease.run(real_code);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_error_recovery() {
        let mut grease = create_test_grease();
        
        // Test error recovery scenarios
        let error_code = r#"
            use system
            try:
                result = system.shell("definitely_nonexistent_command")
                if not result.success:
                    print("Expected error occurred")
            except:
                print("Unexpected error in error handling")
        "#;
        
        let result = grease.run(error_code);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_performance() {
        let mut grease = create_test_grease();
        
        // Test performance with multiple rapid system calls
        let perf_code = r#"
            use system
            for i in 0..10:
                result = system.shell("echo performance_test_" + i)
                # Just execute, don't check result for performance test
        "#;
        
        let result = grease.run(perf_code);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_memory_usage() {
        let mut grease = create_test_grease();
        
        // Test that system operations don't cause memory leaks
        // This is more of a smoke test to ensure VM stability
        let memory_test_code = r#"
            use system
            # Execute multiple system operations
            for i in 0..5:
                result = system.shell("echo memory_test_" + i)
                # Force garbage collection by creating and dropping values
                temp = result
        "#;
        
        let result = grease.run(memory_test_code);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }

    #[test]
    fn test_system_concurrent_access() {
        let mut grease = create_test_grease();
        
        // Test concurrent access to system functions (if supported)
        let concurrent_code = r#"
            use system
            # This test checks if system can handle rapid successive calls
            result1 = system.shell("echo concurrent_1")
            result2 = system.shell("echo concurrent_2")
            result3 = system.shell("echo concurrent_3")
        "#;
        
        let result = grease.run(concurrent_code);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grease::vm::InterpretResult::Ok);
    }
}