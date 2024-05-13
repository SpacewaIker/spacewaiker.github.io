use crate::{
    components::content::ContentSummaryView,
    data_loading::{get_content, get_directory_items},
    AppLanguage,
};
use itertools::Itertools;
use leptos::{component, create_memo, use_context, view, Await, CollectView, IntoView, SignalGet};

/// Comtonent for the main pages of the site
///
/// `directory` is the name of the directory to get the content from
#[component]
#[allow(clippy::module_name_repetitions)]
pub fn ContentListingPage(directory: String) -> impl IntoView {
    let index_directory = directory.clone();
    let ids_directory = directory.clone();
    view! {
        <div class="bg-beige h-fit min-h-screen p-10 pt-20">
            <Await
                future=move || get_content(index_directory.clone())
                let:index
            >
                <ContentListingPageIndex index=index.clone() />
            </Await>

            <svg class="absolute h-36 w-full top-[70vh] left-0 fill-purple" viewBox="0 0 100 100" preserveAspectRatio="none">
                <polyline points="0 30, 100 0, 100 70, 0 100"></polyline>
            </svg>

            <Await
                future=move || get_directory_items(ids_directory.clone())
                let:ids
            >
                <ContentListingPageList directory=&directory ids=ids />
            </Await>
        </div>
    }
}

/// Index part of a content listing page
#[component]
fn ContentListingPageIndex(index: toml::Table) -> impl IntoView {
    let lang = use_context::<AppLanguage>().expect("No context found!").0;

    let index = create_memo(move |_| index.get(&lang.get()).unwrap().clone());

    let title = move || {
        index
            .get()
            .get("title")
            .map(|v| v.as_str().unwrap_or("").to_owned())
    };

    let body_html = move || markdown::to_html(index.get().get("body").unwrap().as_str().unwrap());

    view! {
        <div class="h-[70vh] text-darkpurple">
            <h1 class="text-6xl font-serif my-8">{title}</h1>
            <div inner_html=body_html class="font-paragraph text-darkpurple text-xl styled-body" />
        </div>
    }
}

/// List part of a content listing page
#[component]
#[allow(clippy::needless_lifetimes)]
fn ContentListingPageList<'a>(directory: &'a str, ids: &'a [String]) -> impl IntoView {
    let separator = || {
        view! {
            <svg class="absolute h-20 w-full left-0 fill-purple" viewBox="0 0 100 100" preserveAspectRatio="none">
                <polyline points="0 30, 100 0, 100 70, 0 100"></polyline>
            </svg>
        }
    };

    #[allow(unstable_name_collisions)]
    ids
        .iter()
        .map(|id| {
            view! { <div class="mt-10"><ContentSummaryView directory=directory.to_owned() id=id.to_owned() /></div> }
                .into_view()
        })
        .intersperse_with(|| separator().into_view())
        .collect_view()
}
