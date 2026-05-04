//! Reusable Dioxus authentication package.

mod component;
mod prompt;
mod service;

pub use component::AuthenticationComponent;
pub use service::{
    current_demo_origin_warning, demo_origin_warning, AuthenticationService,
    AuthenticationSessionConfig, AuthenticationStatus, DEFAULT_PASSKEY_EXPIRATION_HOURS,
};
