//! Educational PDE solving with step-by-step explanations

pub mod message_registry;
pub mod wrapper;

pub use message_registry::{get_pde_message, PdeMessageKey};
pub use wrapper::*;
