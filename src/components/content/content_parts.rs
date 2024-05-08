use leptos::{component, view, CollectView, IntoView, Signal, SignalGet};
use toml::Value;

use crate::utils::format_date;

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

#[component]
pub fn ContentImageGallery(#[prop(into)] images: Signal<Option<Value>>) -> impl IntoView {
    images.get().map(|images| {
        let images = images
            .as_array()
            .unwrap()
            .iter()
            .map(|image| {
                view! {
                    <div class="py-2 has-[:focus]:fixed has-[:focus]:w-screen has-[:focus]:h-screen has-[:focus]:top-0 has-[:focus]:left-0
                                has-[:focus]:backdrop-blur-sm has-[:focus]:backdrop-brightness-50">
                        <button class="focus:fixed focus:fixed-center group">
                            <img src=image.as_str().unwrap().to_owned()
                                class="rounded-xl hover:outline hover:outline-purple hover:shadow-lg group-focus:hover:outline-none
                                       group-focus:max-w-[90vw] group-focus:max-h-[90vh]"/>
                        </button>
                    </div>
                }
            })
            .collect_view();

        view! { <div>{ images }</div> }
    })
}

#[component]
pub fn ContentImageGalleryL(#[prop(into)] images: Signal<Option<Value>>) -> impl IntoView {
    images.get().map(|images| {
        let images = images
            .as_array()
            .unwrap()
            .iter()
            .map(|image| view! { <img src=image.as_str().unwrap().to_owned() class="rounded-xl shadow-lg"/> })
            .collect_view();

        view! { <div class="space-y-4">{ images }</div> }
    })
}
