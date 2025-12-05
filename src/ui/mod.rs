pub mod theme;
pub mod components;
pub mod layout;
pub mod events;
pub mod app;

pub use theme::*;
pub use components::*;
pub use layout::*;
pub use events::*;
pub use app::*;

use dioxus::prelude::*;

/// Main UI entry point for Grease applications
pub fn launch_ui<T: Component>(root: T) {
    #[cfg(target_family = "wasm")]
    {
        dioxus_web::launch(root);
    }
    
    #[cfg(target_os = "android")]
    {
        dioxus_mobile::launch(root);
    }
    
    #[cfg(target_os = "ios")]
    {
        dioxus_mobile::launch(root);
    }
    
    #[cfg(not(any(target_family = "wasm", target_os = "android", target_os = "ios")))]
    {
        dioxus_desktop::launch(root);
    }
}