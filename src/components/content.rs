use crate::ApplicationData;
use leptos::{component, use_context, view, CollectView, IntoView, SignalGetUntracked};
use leptos_router::use_params_map;

#[component]
pub fn ContentDetailsView() -> impl IntoView {
    let params = use_params_map().get_untracked();

    let id = params.get("id");

    if id.is_none() {
        return view! {
            <div>
                <h1>{ "No id" }</h1>
            </div>
        };
    }

    let id = id.unwrap();

    let app_data = use_context::<ApplicationData>().expect("No context found!");
    let lang = app_data.language;
    let content = app_data.content_map.get(id);

    if content.is_none() {
        return view! {
            <div>
                <h1>"Content not found"</h1>
            </div>
        };
    }

    let content = content.unwrap().get(&lang).unwrap().clone();

    let title = content
        .get("title")
        .map(|v| v.as_str().unwrap_or("").to_owned());

    let links = content.get("links").map(|v| {
        v.as_table()
            .unwrap()
            .iter()
            .map(|(key, value)| {
                let url = value.as_str().unwrap().to_owned();
                match key.as_str() {
                    "github" => Some(view! { <a href=url>{"GH"}</a> }),
                    "itchio" => Some(view! { <a href=url>{"II"}</a> }),
                    _ => None,
                }
            })
            .collect_view()
    });

    view! {
        <div>
            <h1>{id}</h1>
            <h1>{title}</h1>
            {links}
        </div>
    }
}
