use std::collections::HashMap;

use leptos::{component, provide_context, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::{Route, Router, Routes};
#[allow(clippy::wildcard_imports)]
use portfolio::components::*;
use portfolio::utils::reroot;
use portfolio::{ApplicationData, PageData};

#[component]
fn App(
    app_data: ApplicationData,
    projects: PageData,
    experience: PageData,
    education: PageData,
) -> impl IntoView {
    provide_meta_context();
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
    let (app_data, projects, experience, education) = load_data();

    console_error_panic_hook::set_once();

    leptos::mount_to_body(
        || view! { <App app_data=app_data projects=projects experience=experience education=education /> },
    );
}

fn load_data() -> (ApplicationData, PageData, PageData, PageData) {
    // get and parse content
    let mut content_map = HashMap::new();

    let content_file = include_str!("../content/projects/test_content1.toml");
    let content = toml::from_str::<toml::Table>(content_file).expect("Unable to parse TOML");
    let content = reroot(content);

    content_map.insert("test_content1".to_string(), content.clone());
    content_map.insert("test_content2".to_string(), content.clone());
    content_map.insert("test_content3".to_string(), content.clone());

    let projects = vec![
        String::from("test_content1"),
        String::from("test_content2"),
        String::from("test_content3"),
    ];

    let projects_index_file = include_str!("../content/projects/index.toml");
    let projects_index =
        toml::from_str::<toml::Table>(projects_index_file).expect("Unable to parse TOML");
    let projects_index = reroot(projects_index);

    let (language, set_language) = leptos::create_signal("en".to_string());

    (
        ApplicationData {
            content_map,
            language,
            set_language,
        },
        PageData {
            index: projects_index,
            ids: projects,
        },
        PageData {
            index: toml::Table::new(),
            ids: Vec::new(),
        },
        PageData {
            index: toml::Table::new(),
            ids: Vec::new(),
        },
    )
}
