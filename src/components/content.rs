use crate::ApplicationData;
use toml::Value;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub id: String,
}

#[function_component(ContentDetailsView)]
pub fn content_details_view(props: &Props) -> Html {
    let app_data = use_context::<ApplicationData>().expect("No context found!");
    let lang = app_data.language;
    let content = app_data.content_map.get(&props.id);

    if content.is_none() {
        return html! {
            <div>
                <h1>{ "Content not found" }</h1>
            </div>
        };
    }

    let content = content.unwrap().get(&lang).unwrap().clone();

    let links = content.get("links").map(|v| {
        v.as_table()
            .unwrap()
            .iter()
            .map(|(key, value)| match key.as_str() {
                "github" => html! { <a href={value.as_str()}>{"GH"}</a> },
                "itchio" => html! { <a href={value.as_str()}>{"II"}</a> },
                _ => Html::default(),
            })
            .collect::<Vec<Html>>()
    });

    html! {
        <div>
            if let Some(Value::String(title)) = content.get("title") { <h1>{ title }</h1> }
            if let Some(links) = links {{ links }}
            <div>{"hello"}</div>
        </div>
    }
}
