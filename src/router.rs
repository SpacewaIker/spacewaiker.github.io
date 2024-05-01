use portfolio::components::ContentDetailsView;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/content/:id")]
    Content { id: String },
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { "Home" },
        Route::Content { id } => html! { <ContentDetailsView id={id} /> },
    }
}
