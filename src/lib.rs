use std::collections::HashMap;

pub mod components;

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationData {
    pub content_map: HashMap<String, toml::Table>,
    pub language: String,
}
