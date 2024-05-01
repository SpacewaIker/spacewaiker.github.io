mod router;

use std::collections::HashMap;

use portfolio::ApplicationData;
use router::{switch, Route};
use stylist::yew::use_style;
use toml::Table;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn App(data: &ApplicationData) -> Html {
    let style = use_style!(
        r"
            height: 100%;
            width: 100%;
        "
    );

    html! {
        <div class={style}>
            <ContextProvider<ApplicationData> context={data.clone()}>
                <BrowserRouter>
                    <Switch<Route> render={ switch } />
                </BrowserRouter>
            </ContextProvider<ApplicationData>>
        </div>
    }
}

fn main() {
    // get and parse content
    let mut content_map = HashMap::new();

    let file = include_str!("../content/projects/test_content1.toml");
    let content = toml::from_str::<toml::Table>(file).expect("Unable to parse TOML");
    content_map.insert("test_content1".to_string(), reroot(content));

    let app_data = ApplicationData {
        content_map,
        language: String::from("en"),
    };

    yew::Renderer::<App>::with_props(app_data).render();
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
