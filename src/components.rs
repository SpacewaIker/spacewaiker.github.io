use leptos::{component, view, IntoView};

mod content;
mod navigation;
mod page;

pub use content::ContentDetailsView;
pub use navigation::{Footer, Header};
pub use page::ContentListingPage;

/// Page for a wrong route
#[must_use]
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="h-screen bg-beige">
            <h1 class="pt-20 font-title text-2xl">"404"</h1>
        </div>
    }
}
