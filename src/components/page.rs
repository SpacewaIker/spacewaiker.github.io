use crate::{components::content::ContentSummaryView, ApplicationData, PageData};
use itertools::Itertools;
use leptos::{component, create_memo, use_context, view, CollectView, IntoView, SignalGet};

/// Comtonent for the main pages of the site
#[component]
#[allow(clippy::needless_lifetimes)]
#[allow(clippy::module_name_repetitions)]
pub fn ContentListingPage<'a>(page_data: &'a PageData) -> impl IntoView {
    let app_data = use_context::<ApplicationData>().expect("No context found");
    let lang = app_data.language;
    let index = page_data.index.clone();
    let index = create_memo(move |_| index.get(&lang.get()).unwrap().clone());
    let ids = page_data.ids.clone();

    let separator = || {
        view! {
            <svg class="absolute h-20 w-full left-0 fill-purple" viewBox="0 0 100 100" preserveAspectRatio="none">
                <polyline points="0 30, 100 0, 100 70, 0 100"></polyline>
            </svg>
        }
    };

    #[allow(unstable_name_collisions)]
    let projects = ids
        .iter()
        .map(|id| view! { <div class="mt-10"><ContentSummaryView id=id /></div> }.into_view())
        .intersperse_with(|| separator().into_view())
        .collect_view();

    let title = move || {
        index
            .get()
            .get("title")
            .map(|v| v.as_str().unwrap_or("").to_owned())
    };

    let body_html = move || markdown::to_html(index.get().get("body").unwrap().as_str().unwrap());

    view! {
        <div class="bg-beige h-fit min-h-screen p-10 pt-20">
            <div class="h-[80vh] text-darkpurple">
                <h1 class="text-6xl font-serif my-8">{title}</h1>
                <div inner_html=body_html class="font-paragraph text-darkpurple text-xl styled-body" />
            </div>
            {separator()}
            {projects}
        </div>
    }
}
