use std::collections::HashMap;

use leptos::{create_signal, ReadSignal, WriteSignal};

pub mod components;
pub mod utils;

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationData {
    pub content_map: HashMap<String, toml::Table>,
    pub language: ReadSignal<String>,
    pub set_language: WriteSignal<String>,
}

impl ApplicationData {
    pub fn new(content_map: HashMap<String, toml::Table>, language: String) -> Self {
        let (language, set_language) = create_signal(language);

        Self {
            content_map,
            language,
            set_language,
        }
    }
}
