use leptos::RwSignal;

pub mod components;
pub mod data_loading;
pub mod utils;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppLanguage(pub RwSignal<String>);
