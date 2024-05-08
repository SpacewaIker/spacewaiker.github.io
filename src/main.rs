use std::collections::HashMap;

use leptos::{component, provide_context, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::{Route, Router, Routes};
use portfolio::components::{ContentDetailsView, Footer, Header};
use portfolio::ApplicationData;
use toml::Table;

#[component]
fn App(data: ApplicationData) -> impl IntoView {
    provide_meta_context();
    provide_context(data);

    view! {
        <Stylesheet id="leptos" href="/dist/output.css" />
        <Router>
            // navbar
            <Header />
            <Routes>
                <Route path="/" view=|| view! { <h1 class="text-2xl mt-20 h-screen">"Home"</h1> } />
                <Route path="/projects" view=|| view! { <h1 class="text-2xl mt-20">"projects"</h1> } />
                <Route path="/experience" view=|| view! { <h1 class="text-2xl mt-20">"experience"</h1> } />
                <Route path="/education" view=|| view! { <h1 class="text-2xl mt-20">"education"</h1> } />
                <Route path="/content/:id" view=ContentDetailsView />
            </Routes>
            <Footer />
        </Router>
    }
}

fn main() {
    // get and parse content
    let mut content_map = HashMap::new();

    let file = include_str!("../content/projects/test_content1.toml");
    let content = toml::from_str::<toml::Table>(file).expect("Unable to parse TOML");
    content_map.insert("test_content1".to_string(), reroot(content));

    let app_data = ApplicationData::new(content_map, String::from("en"));

    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| view! { <App data=app_data/> });
}

/// Reroots the table to bring the language keys to the top level
fn reroot(table: Table) -> Table {
    let mut table_en = Table::new();
    let mut table_fr = Table::new();

    for (key, value) in table {
        if let toml::Value::Table(table) = value {
            if table.contains_key("en") && table.contains_key("fr") {
                table_en.insert(key.clone(), table.get("en").unwrap().clone());
                table_fr.insert(key.clone(), table.get("fr").unwrap().clone());
                continue;
            }
            table_en.insert(key.clone(), toml::Value::Table(table.clone()));
            table_fr.insert(key.clone(), toml::Value::Table(table.clone()));
        } else {
            table_en.insert(key.clone(), value.clone());
            table_fr.insert(key.clone(), value.clone());
        }
    }

    let mut new_table = Table::new();
    new_table.insert("en".to_string(), toml::Value::Table(table_en));
    new_table.insert("fr".to_string(), toml::Value::Table(table_fr));
    new_table
}
