use std::collections::HashMap;
use yew::prelude::*;

pub mod components;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ApplicationData {
    pub content_map: HashMap<String, toml::Table>,
    pub language: String,
}
