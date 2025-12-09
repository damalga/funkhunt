// src/tui/mod.rs

// Re-exportar todo lo público
pub mod components;
pub mod events;
pub mod popup;
pub mod render;
pub mod state;

// Re-exportar tipos principales
pub use events::handle_key_event;
pub use render::{init, render, restore};
pub use state::{AppAction, TuiState};
