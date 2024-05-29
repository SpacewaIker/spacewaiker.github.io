use crate::{data_loading::get_content, i18n::use_i18n};
use leptos::{component, create_memo, view, Await, IntoView, SignalGet};
use leptos_i18n::Locale as _;
use leptos_router::{use_params_map, A};
use std::borrow::ToOwned;

mod content_parts;

#[allow(clippy::wildcard_imports)]
use content_parts::*;

/// Component for showing a detailed view of a piece of content
///
/// This is intended as a full-page view
#[component]
pub fn ContentDetailsView(directory: String) -> impl IntoView {
    let params = use_params_map();

    let id = move || params.get().get("id").unwrap().clone();

    view! {
        move ||
        <Await
            future=move || get_content(format!("{directory}/{}", id()))
            let:content
        >
            <ContentDetailsViewInner content=content.clone() />

            // required to make the footer angled
            <svg class="h-20 w-full relative -top-2" viewBox="0 0 100 100" preserveAspectRatio="none">
                <polyline class="fill-beige" points="0 0, 100 0, 100 60, 0 100"></polyline>
            </svg>
        </Await>
    }
}

#[component]
fn ContentDetailsViewInner(content: toml::Table) -> impl IntoView {
    let i18n = use_i18n();
    let lang = move || i18n.get_locale().as_str();

    let links = content.get("links").map(ToOwned::to_owned);
    let date = content.get("date").map(ToOwned::to_owned);
    let images = content.get("media").map(ToOwned::to_owned);

    let content = create_memo(move |_| content.get(lang()).unwrap().clone());

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
        <div class="bg-beige h-fit min-h-screen p-4 md:pt-24 md:p-10">
            <h1 class="font-title text-4xl font-bold underline text-darkpurple inline-block mb-4">{title}</h1>
            <ContentLinkIcons links=links />
            <ContentDate date=date />
            <ContentTags tags=move || content.get().get("tags").map(ToOwned::to_owned) />
            <div class="flex flex-col md:flex-row md:space-x-8">
                <div class="md:ml-8 basis-3/4">
                    <ContentResumeLines lines=move || content.get().get("resume_lines").map(ToOwned::to_owned) />
                    {body}
                </div>
                <div class="basis-1/4">
                    {icon}
                    <ContentImageGallery images=images.clone() show_all=true />
                </div>
            </div>
            <div class="mt-16">
                <ContentImageGalleryL images=images />
            </div>
        </div>
    }
}

/// Component for showing a summarized view of a piece of content
///
/// This is intended as a preview in a list
#[component]
pub fn ContentSummaryView(directory: String, id: String) -> impl IntoView {
    let id2 = id.clone();
    view! {
        <Await
            future=move || get_content(format!("{directory}/{id}"))
            let:content
        >
            <ContentSummaryViewInner id=&id2 content=content.clone() />
        </Await>
    }
}

#[component]
#[allow(clippy::needless_lifetimes)]
fn ContentSummaryViewInner<'a>(id: &'a str, content: toml::Table) -> impl IntoView {
    let i18n = use_i18n();
    let lang = move || i18n.get_locale().as_str();

    let links = content.get("links").map(ToOwned::to_owned);
    let date = content.get("date").map(ToOwned::to_owned);
    let images = content.get("media").map(ToOwned::to_owned);

    let content = create_memo(move |_| content.get(lang()).unwrap().clone());

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

    let summary_html =
        move || markdown::to_html(content.get().get("summary").unwrap().as_str().unwrap());
    let summary = view! { <div inner_html=summary_html class="font-paragraph text-darkpurple text-lg styled-body" /> };

    view! {
        <A href=format!("./{id}") class="block mt-28 mb-4 rounded-md hover:outline-purple hover:outline hover:outline-4">
            <div class="bg-beige p-0 md:p-4">
                <h1 class="font-title text-4xl font-bold underline text-darkpurple inline-block mb-4">{title}</h1>
                <ContentLinkIcons links=links />
                <ContentDate date=date />
                <ContentTags tags=move || content.get().get("tags").map(ToOwned::to_owned) />
                <div class="flex flex-col md:flex-row md:space-x-8">
                    <div class="md:ml-8 basis-3/4">
                        <ContentResumeLines lines=move || content.get().get("resume_lines").map(ToOwned::to_owned) />
                        {summary}
                    </div>
                    <div class="basis-1/4">
                        {icon}
                        <ContentImageGallery images=images show_all=false />
                    </div>
                </div>
            </div>
        </A>
    }.into_view()
}
