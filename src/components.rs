use leptos::{component, view, CollectView, IntoView, Signal, SignalGet};
use toml::Value;

mod content;
mod navigation;

pub use content::ContentDetailsView;
pub use navigation::{Footer, Header};

#[must_use]
#[component]
#[allow(clippy::needless_lifetimes)]
pub fn LinkIcons(#[prop(into)] links: Signal<Option<Value>>) -> impl IntoView {
    links.get().map(|links| {
        links
            .as_table()
            .unwrap()
            .iter()
            .map(|(key, value)| {
                let url = value.as_str().unwrap().to_owned();
                let icon = match key.as_str() {
                    "github" => view! { <i class="nf nf-fa-github"></i> },
                    "itchio" => view! { <i class="nf nf-fa-itch-io"></i> },
                    _ => view! { <i class="nf nf-fa-link"></i> },
                };
                view! {
                    <a href=url target="_blank" class="text-4xl text-darkpurle ml-4 sliding-underline-low -top-1">
                        { icon }
                    </a>
                }
            })
            .collect_view()
    })
}
