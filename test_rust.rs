use grease::Grease;

fn main() {
    let mut grease = Grease::new();
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
    match result {
        Ok(interp_result) => println!("Success: {:?}", interp_result),
        Err(e) => println!("Error: {}", e),
    }
}
