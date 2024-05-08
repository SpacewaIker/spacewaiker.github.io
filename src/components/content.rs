use crate::{components::NotFound, ApplicationData};
use leptos::{component, create_memo, use_context, view, IntoView, SignalGet, SignalGetUntracked};
use leptos_router::use_params_map;
use std::borrow::ToOwned;

mod content_parts;

#[allow(clippy::wildcard_imports)]
use content_parts::*;

/// Component for showing a detailed view of a piece of content
///
/// This is intended as a full-page view
#[component]
pub fn ContentDetailsView() -> impl IntoView {
    let params = use_params_map().get_untracked();

    let id = params.get("id");

    if id.is_none() {
        return NotFound().into_view();
    }

    let id = id.unwrap();

    let app_data = use_context::<ApplicationData>().expect("No context found!");
    let lang = app_data.language;
    let content = app_data.content_map.get(id).cloned();

    if content.is_none() {
        return NotFound().into_view();
    }

    let content = create_memo(move |_| content.as_ref().unwrap().get(&lang.get()).unwrap().clone());

    let title = move || {
        content
            .get()
            .get("title")
            .map(|v| v.as_str().unwrap_or("").to_owned())
    };

    let icon = move || {
        content.get()
        .get("icon")
        .map(|v| view! { <img src=v.as_str().unwrap().to_owned() class="relative w-1/2 left-1/4 -top-8 mb-4" /> })
    };

    let body_html = move || markdown::to_html(content.get().get("body").unwrap().as_str().unwrap());
    let body = view! { <div inner_html=body_html class="font-paragraph text-darkpurple text-lg styled-body" /> };

    view! {
        <div class="bg-beige h-fit min-h-screen p-10 pt-20">
            <h1 class="font-title text-4xl font-bold underline text-darkpurple inline-block mb-4">{title}</h1>
            <ContentLinkIcons links=move || content.get().get("links").map(ToOwned::to_owned) />
            <ContentDate date=content.get().get("date").map(ToOwned::to_owned) lang=lang />
            <ContentTags tags=move || content.get().get("tags").map(ToOwned::to_owned) />
            <div class="flex flex-row space-x-8">
                <div class="ml-8 basis-3/4">
                    <ContentResumeLines lines=move || content.get().get("resume_lines").map(ToOwned::to_owned) />
                    {body}
                </div>
                <div class="basis-1/4">
                    {icon}
                    <ContentImageGallery images=move || content.get().get("media").map(ToOwned::to_owned) />
                </div>
            </div>
            <div class="mt-16">
                <ContentImageGalleryL images=move || content.get().get("media").map(ToOwned::to_owned) />
            </div>
        </div>
    }.into_view()
}
