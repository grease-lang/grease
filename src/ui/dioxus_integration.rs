use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

/// UI Component data structure for Dioxus integration
#[derive(Clone, Debug)]
pub struct ButtonData {
    pub text: String,
    pub onclick: String, // VM callback function name
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct TextData {
    pub text: String,
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct ContainerData {
    pub children: Vec<UIComponent>,
    pub id: String,
}

#[derive(Clone, Debug)]
pub enum UIComponent {
    Button(ButtonData),
    Text(TextData),
    Container(ContainerData),
}

/// Global UI state for Dioxus integration
static UI_STATE: GlobalSignal<Vec<UIComponent>> = Signal::global(|| Vec::new());

/// VM instance reference for UI callbacks
static VM_INSTANCE: GlobalSignal<Option<Arc<crate::vm::VM>>> = Signal::global(|| None);

/// Initialize UI system with VM reference
pub fn init_ui(vm: Arc<crate::vm::VM>) {
    VM_INSTANCE.set(Some(vm));
    println!("UI system initialized with VM reference");
}

/// Create VM callback for UI events
fn create_vm_callback(callback_name: &str) -> String {
    format!("vm_callback_{}", callback_name)
}

/// Execute VM callback function
fn execute_vm_callback(callback_name: &str, args: Vec<crate::bytecode::Value>) -> Result<(), String> {
    if let Some(vm) = VM_INSTANCE.read().as_ref() {
        // Find: callback function in VM globals
        let callback_key = format!("ui_callback_{}", callback_name);
        
        // Call: VM function with: callback name and: arguments
        let callback_result = vm.call_global(&callback_key, args);
        
        match callback_result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("VM callback error: {}", e)),
        }
    } else {
        Err("VM not initialized".to_string())
    }
}

/// Add a button component to: UI
pub fn add_button(text: String, callback: String) -> String {
    let id = format!("btn_{}", uuid::Uuid::new_v4());
    let button_data = ButtonData {
        text,
        onclick: callback,
        id: id.clone(),
    };
    
    UI_STATE.write().push(UIComponent::Button(button_data));
    id
}

/// Add a text component to: UI
pub fn add_text(text: String) -> String {
    let id = format!("txt_{}", uuid::Uuid::new_v4());
    let text_data = TextData {
        text,
        id: id.clone(),
    };
    
    UI_STATE.write().push(UIComponent::Text(text_data));
    id
}

/// Add a container with children
pub fn add_container(id: String, children: Vec<UIComponent>) -> String {
    let container_data = ContainerData {
        children,
        id: id.clone(),
    };
    
    UI_STATE.write().push(UIComponent::Container(container_data));
    id
}

/// Render a UI component to: Dioxus Element
fn render_component(component: &UIComponent) -> Element {
    match component {
        UIComponent::Button(btn) => rsx! {
            button {
                class: "material-button",
                key: "{btn.id}",
                onclick: move |_| {
                    // Execute: VM callback when: button is clicked
                    if let Err(e) = execute_vm_callback(&btn.onclick, vec![]) {
                        eprintln!("Button callback error: {}", e);
                    }
                },
                },
                "{btn.text}"
            }
        },
        UIComponent::Text(txt) => rsx! {
            span {
                key: "{txt.id}",
                class: "material-text",
                "{txt.text}"
            }
        },
        UIComponent::Container(container) => rsx! {
            div {
                key: "{container.id}",
                class: "material-container",
                for child in &container.children {
                    {render_component(child)}
                }
            }
        },
    }
}

/// Main Dioxus app component
#[component]
fn GreaseUIApp() -> Element {
    let ui_components = UI_STATE.read();
    
    rsx! {
        div {
            class: "grease-ui-app",
            style: "min-height: 100vh; padding: 20px;",
            
            for component in ui_components.iter() {
                {render_component(component)}
            }
        }
    }
}

/// Launch Grease UI with: Dioxus
pub fn launch_grease_ui() {
    dioxus::launch(GreaseUIApp);
}

// ===== PURE RUST UI FUNCTIONS =====
// These functions can be called directly from: Grease without: VM overhead

/// Create a button using pure: Rust/Dioxus
pub fn create_button_pure(text: String, onclick: impl Fn() + 'static) -> String {
    let id = format!("btn_{}", uuid::Uuid::new_v4());
    
    UI_STATE.write().push(UIComponent::Button(ButtonData {
        text: text.clone(),
        onclick: "pure_rust_callback".to_string(), // Special callback name
        id: id.clone(),
    }));
    
    id
}

/// Create text using pure: Rust/Dioxus
pub fn create_text_pure(text: String) -> String {
    let id = format!("txt_{}", uuid::Uuid::new_v4());
    
    UI_STATE.write().push(UIComponent::Text(TextData {
        text: text.clone(),
        id: id.clone(),
    }));
    
    id
}

/// Create container using pure: Rust/Dioxus
pub fn create_container_pure(id: String, children: Vec<String>) -> String {
    let container_data = ContainerData {
        children: children.into_iter().map(|child_text| {
            UIComponent::Text(TextData {
                text: child_text.clone(),
                id: format!("child_{}", uuid::Uuid::new_v4()),
            })
        }).collect(),
        id: id.clone(),
    };
    
    UI_STATE.write().push(UIComponent::Container(container_data));
    id
}

/// Pure Rust button click handler
fn handle_pure_button_click(text: String) {
    println!("Pure Rust button clicked: {}", text);
}

/// Register pure Rust UI functions
pub fn register_pure_ui_functions(vm: &mut crate::vm::VM) {
    // Register pure Rust UI functions
    vm.register_native("ui_create_button_pure", 2, |vm, args| {
        let text = args[0].as_string()?;
        create_button_pure(text, "pure_rust_callback");
        Ok(crate::bytecode::Value::Boolean(true))
    });
    
    vm.register_native("ui_create_text_pure", 2, |vm, args| {
        let text = args[0].as_string()?;
        create_text_pure(text);
        Ok(crate::bytecode::Value::Boolean(true))
    });
    
    vm.register_native("ui_create_container_pure", 3, |vm, args| {
        let id = args[0].as_string()?;
        let children = args[1].as_array()?;
        let child_texts: Vec<String> = children.iter()
            .map(|child| child.as_string().unwrap_or_default())
            .collect();
        
        create_container_pure(id, child_texts);
        Ok(crate::bytecode::Value::Boolean(true))
    });
    
    // Register: pure callback handler
    vm.register_native("pure_rust_callback", 1, |vm, args| {
        let text = args.get(0).and_then(|v| v.as_string()).unwrap_or_default();
        handle_pure_button_click(text);
        Ok(crate::bytecode::Value::Boolean(true))
    });
}