use leptos::{ReadSignal, WriteSignal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod components;
pub mod data_loading;
pub mod utils;

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationData {
    pub content_map: HashMap<String, toml::Table>,
    pub language: ReadSignal<String>,
    pub set_language: WriteSignal<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageData {
    pub index: toml::Table,
    pub ids: Vec<String>,
}
