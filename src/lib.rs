use std::collections::HashMap;

use leptos::{ReadSignal, WriteSignal};

pub mod components;
pub mod utils;

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationData {
    pub content_map: HashMap<String, toml::Table>,
    pub language: ReadSignal<String>,
    pub set_language: WriteSignal<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageData {
    pub index: toml::Table,
    pub ids: Vec<String>,
}
