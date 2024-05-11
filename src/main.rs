use std::collections::HashMap;

use leptos::{component, provide_context, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::{Route, Router, Routes};
#[allow(clippy::wildcard_imports)]
use portfolio::components::*;
use portfolio::data_loading::load_data;
use portfolio::{ApplicationData, PageData};

#[component]
fn App(
    data: (
        HashMap<String, toml::Table>,
        String,
        PageData,
        PageData,
        PageData,
    ),
) -> impl IntoView {
    let (content_map, language, projects, experience, education) = data;

    provide_meta_context();

    let (language, set_language) = leptos::create_signal(language);
    let app_data = ApplicationData {
        content_map,
        language,
        set_language,
    };
    provide_context(app_data);

    view! {
        <Stylesheet id="leptos" href="/dist/output.css" />
        <Router>
            <Header />
            <div class="relative z-10">
                <Routes>
                    <Route path="/" view=|| view! { <h1 class="text-2xl mt-20 h-screen">"Home"</h1> } />
                    <Route path="/projects" view=move || view! { <ContentListingPage page_data=&projects /> } />
                    <Route path="/experience" view=move || view! { <ContentListingPage page_data=&experience /> } />
                    <Route path="/education" view=move || view! { <ContentListingPage page_data=&education /> } />
                    <Route path="/content/:id" view=ContentDetailsView />
                    <Route path="*" view=NotFound />
                </Routes>

                // required to make the footer angled
                <svg class="h-20 w-full relative -top-2" viewBox="0 0 100 100" preserveAspectRatio="none">
                    <polyline class="fill-beige" points="0 0, 100 0, 100 60, 0 100"></polyline>
                </svg>
            </div>
            <Footer />
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::spawn_local(async {
        let data = load_data().await;

        leptos::mount_to_body(|| view! { <App data=data /> });
    });
}
