use leptos::{component, view, CollectView, IntoView, Signal, SignalGet};
use toml::Value;

use crate::utils::format_date;

/// Component for the "tags" in a piece of content
#[component]
pub fn ContentTags(#[prop(into)] tags: Signal<Option<Value>>) -> impl IntoView {
    let list_items = move || {
        tags.get().map(|v| {
            v.as_array()
                .unwrap()
                .iter()
                .map(|tag| view! { <li class="inline-block">{ tag.as_str().unwrap().to_owned() }</li> })
                .collect_view()
        })
    };

    let items_exist = move || tags.get().is_some();

    move || {
        items_exist().then(|| view! { <ul class="font-mono text-purple text-lg mb-2 space-x-6">{ list_items() }</ul> })
    }
}

/// Component for the date in a piece of content
///
/// The date is constant once loaded, but the language can change
#[component]
pub fn ContentDate(date: Option<Value>, #[prop(into)] lang: Signal<String>) -> impl IntoView {
    date.map(|v| {
        let date = v.as_table().unwrap();
        let start_date = date.get("start").cloned();
        let end_date = date.get("end").cloned();

        let start_date = move || {
            start_date
                .clone()
                .map(|v| format_date(v.as_datetime().unwrap(), &lang.get()))
        };
        let end_date = move || {
            end_date
                .clone()
                .map(|v| format!(" - {}", format_date(v.as_datetime().unwrap(), &lang.get())))
        };

        view! { <span class="font-mono text-lg float-right">{ start_date }{ end_date }</span> }
    })
}

/// Component for the "lines" in a piece of content
#[component]
pub fn ContentResumeLines(#[prop(into)] lines: Signal<Option<Value>>) -> impl IntoView {
    let list_items = move || {
        lines.get().map(|v| {
            v.as_array()
                .unwrap()
                .iter()
                .map(|line| view! { <li>{ line.as_str().unwrap().to_owned() }</li> })
                .collect_view()
        })
    };

    let items_exist = move || lines.get().is_some();

    move || {
        items_exist().then(|| view! { <ul class="font-line text-darkpurple text-lg list-arrow mb-4">{ list_items }</ul> })
    }
}

/// Component for the images in a piece of content
///
/// The images are rendered in a button that gets maximized when clicked
#[component]
pub fn ContentImageGallery(images: Option<Value>, show_all: bool) -> impl IntoView {
    images.map(|images| {
        let images = images
            .as_array()
            .unwrap()
            .iter()
            .map(|image| {
                view! {
                    <div class="py-2 has-[:focus]:fixed has-[:focus]:w-screen has-[:focus]:h-screen has-[:focus]:top-0 has-[:focus]:left-0
                                has-[:focus]:backdrop-blur-sm has-[:focus]:backdrop-brightness-50 has-[:focus]:z-50">
                        <button class="focus:fixed focus:fixed-center group">
                            <img src=image.as_str().unwrap().to_owned()
                                class="rounded-xl hover:outline hover:outline-purple hover:shadow-lg group-focus:hover:outline-none
                                       group-focus:max-w-[90vw] group-focus:max-h-[90vh]"/>
                        </button>
                    </div>
                }
            });

        let images = if show_all {
            images.collect_view()
        } else {
            images.take(1).collect_view()
        };

        view! { <div>{ images }</div> }
    })
}

/// Component for the images in a piece of content
///
/// The images are rendered in full width
#[component]
pub fn ContentImageGalleryL(images: Option<Value>) -> impl IntoView {
    images.map(|images| {
        let images = images
            .as_array()
            .unwrap()
            .iter()
            .map(|image| view! { <img src=image.as_str().unwrap().to_owned() class="rounded-xl shadow-lg"/> })
            .collect_view();

        view! { <div class="space-y-4">{ images }</div> }
    })
}

/// Component for showing links as icons
#[must_use]
#[component]
pub fn ContentLinkIcons(links: Option<Value>) -> impl IntoView {
    links.map(|links| {
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
