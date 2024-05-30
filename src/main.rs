use leptos::{component, view, IntoView, SignalGetUntracked};
use leptos_meta::Stylesheet;
use leptos_router::{use_params_map, Outlet, Redirect, Route, Router, Routes};
#[allow(clippy::wildcard_imports)]
use portfolio::components::*;
use portfolio::i18n::{provide_i18n_context, use_i18n, Locale};

/// Main application component
#[component]
fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();

    provide_i18n_context();

    view! {
        <Stylesheet id="leptos" href="/dist/output.css" />
        <Router>
            <Routes>
                <Route path="/:lang" view=WebsiteOutlet >
                    <Route path="/home" view=HomePage />
                    <Route path="/projects" view=move || view! { <ContentListingPage directory=String::from("projects") /> } />
                    <Route path="/experience" view=move || view! { <ContentListingPage directory=String::from("experience") /> } />
                    <Route path="/education" view=move || view! { <ContentListingPage directory=String::from("education") /> } />
                    <Route path="/projects/:id" view=move || view! { <ContentDetailsView directory=String::from("projects") /> } />
                    <Route path="/experience/:id" view=move || view! { <ContentDetailsView directory=String::from("experience") /> } />
                    <Route path="/education/:id" view=move || view! { <ContentDetailsView directory=String::from("education") /> } />
                    <Route path="*" view=|| view! { <Redirect path="home" /> } />
                </Route>
                <Route path="*" view=|| view! { <Redirect path="/en/home" /> } />
            </Routes>
        </Router>
    }
}

/// Outlet for the website
#[component]
fn WebsiteOutlet() -> impl IntoView {
    let params = use_params_map().get_untracked();
    let lang = params.get("lang").unwrap();
    let i18n = use_i18n();

    match lang.as_str() {
        "fr" => i18n.set_locale(Locale::fr),
        "en" => i18n.set_locale(Locale::en),
        _ => return view! { <Redirect path="/en/home" /> }.into_view(),
    };

    view! {
        <Header />
        <div class="relative z-10">
            <Outlet />
        </div>
        <Footer />
    }
    .into()
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(move || view! { <App /> });
}
