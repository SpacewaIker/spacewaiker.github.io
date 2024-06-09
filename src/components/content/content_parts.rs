use leptos::{component, view, CollectView, IntoView, Signal, SignalGet};
use leptos_i18n::Locale as _;
use toml::Value;

use crate::{i18n::use_i18n, utils::format_date};

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
        items_exist().then(|| view! { <ul class="font-mono text-purple text-lg py-4 md:py-0 mb-2 inline-block md:block *:mr-6">{ list_items() }</ul> })
    }
}

/// Component for the date in a piece of content
///
/// The date is constant once loaded, but the language can change
#[component]
pub fn ContentDate(date: Option<Value>) -> impl IntoView {
    let i18n = use_i18n();
    let lang = move || i18n.get_locale().as_str();

    date.map(|v| {
        let date = v.as_table().unwrap();
        let start_date = date.get("start").cloned();
        let end_date = date.get("end").cloned();

        let start_date = move || {
            start_date
                .clone()
                .map(|v| format_date(v.as_datetime().unwrap(), lang()))
        };
        let end_date = move || {
            end_date
                .clone()
                .map(|v| format!(" - {}", format_date(v.as_datetime().unwrap(), lang())))
        };

        view! { <span class="font-mono text-lg float-right block md:inline py-4 md:py-0">{ start_date }{ end_date }</span> }
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
                .map(|line| {
                    let line_html = markdown::to_html(line.as_str().unwrap())
                        .replace("<p>", "")
                        .replace("</p>", "");
                    view! { <li inner_html=line_html></li> }
                })
                .collect_view()
        })
    };

    let items_exist = move || lines.get().is_some();

    move || {
        items_exist().then(|| view! { <ul class="font-line text-darkpurple text-lg list-arrow mb-4">{ list_items }</ul> })
    }
}

fn image_path(image: &str) -> String {
    if image.starts_with("http") {
        // If the image is already a URL, return it as is
        image.to_owned()
    } else {
        format!(
            "https://github.com/SpacewaIker/spacewaiker.github.io/blob/content/media/{image}?raw=true"
        )
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
                let mut div_class = String::from("py-2");
                let mut button_class = String::new();
                let mut img_class = String::from("rounded-xl");

                if show_all {
                    div_class += " has-[:focus]:fixed has-[:focus]:w-screen has-[:focus]:h-screen has-[:focus]:top-0 has-[:focus]:left-0 has-[:focus]:backdrop-blur-sm has-[:focus]:backdrop-brightness-50 has-[:focus]:z-50";
                    button_class += " focus:fixed focus:fixed-center group focus:mt-10";
                    img_class += " hover:outline hover:outline-purple hover:shadow-lg group-focus:hover:outline-none group-focus:max-w-[90vw] group-focus:max-h-[90vh]";
                }

                view! {
                    <div class=div_class>
                        <button class=button_class>
                            <img src=image_path(image.as_str().unwrap()) class=img_class/>
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
