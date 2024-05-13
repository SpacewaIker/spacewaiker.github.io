use leptos::{component, create_rw_signal, provide_context, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::{Route, Router, Routes};
#[allow(clippy::wildcard_imports)]
use portfolio::components::*;
use portfolio::AppLanguage;

#[component]
fn App(lang: AppLanguage) -> impl IntoView {
    provide_meta_context();

    provide_context(lang);

    view! {
        <Stylesheet id="leptos" href="/dist/output.css" />
        <Router>
            <Header />
            <div class="relative z-10">
                <Routes>
                    <Route path="/" view=|| view! { <h1 class="text-2xl mt-20 h-screen">"Home"</h1> } />
                    <Route path="/projects" view=move || view! { <ContentListingPage directory=String::from("projects") /> } />
                    <Route path="/experience" view=move || view! { <ContentListingPage directory=String::from("experience") /> } />
                    <Route path="/education" view=move || view! { <ContentListingPage directory=String::from("education") /> } />
                    <Route path="/projects/:id" view=move || view! { <ContentDetailsView directory=String::from("projects") /> } />
                    <Route path="/experience/:id" view=move || view! { <ContentDetailsView directory=String::from("experience") /> } />
                    <Route path="/education/:id" view=move || view! { <ContentDetailsView directory=String::from("education") /> } />
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

    let lang = AppLanguage(create_rw_signal(String::from("en")));

    leptos::mount_to_body(move || view! { <App lang=lang /> });
}
