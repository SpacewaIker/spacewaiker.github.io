use leptos::{component, create_memo, use_context, view, Await, IntoView, SignalGet};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{data_loading::get_content, AppLanguage};

/// Home page of the website
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Await
            future=|| get_content(String::from("homepage"))
            let:content
        >
            <HomePageInner content=content.clone() />
        </Await>
    }
}

#[component]
fn HomePageInner(content: toml::Table) -> impl IntoView {
    let lang = use_context::<AppLanguage>().expect("No context found!").0;

    let content = create_memo(move |_| content.get(&lang.get()).unwrap().clone());

    let greeting = move || {
        content
            .get()
            .get("greeting")
            .map(|v| v.as_str().unwrap_or("").to_owned())
    };

    let intro_html =
        move || markdown::to_html(content.get().get("intro").unwrap().as_str().unwrap());

    // add scroll event listener to change name colour and position
    let name_event = wasm_bindgen::closure::Closure::wrap(Box::new(name_event) as Box<dyn FnMut()>);
    leptos::window()
        .dyn_into::<web_sys::EventTarget>()
        .unwrap()
        .add_event_listener_with_callback("scroll", name_event.as_ref().unchecked_ref())
        .unwrap();
    name_event.forget();

    view! {
        <div class="bg-beige h-fit min-h-screen">
            // title screen
            <div class="relative w-full h-screen">
                // name container
                <div class="absolute w-full text-center left-0 bottom-0 text-darkpurple">
                    <h2 class="absolute font-serif text-4xl top-[calc(-120px-12vw)] left-10">{greeting}</h2>
                    <h1 id="first-name" class="absolute inline-block bottom-0 text-[12.5vw] font-title font-extralight left-0 z-20">Thibaut</h1>
                    <h1 id="last-name" class="absolute inline-block bottom-0 text-[12.5vw] font-title font-extralight right-0 z-20">Baguette</h1>
                </div>
            </div>

            <div class="relative w-full h-40 z-10">
                <svg class="w-full h-[175%] fill-purple" viewBox="0 0 100 100" preserveAspectRatio="none">
                    <polyline points="0 30,100 0,100 70,0 100"></polyline>
                </svg>
            </div>

            // intro screen
            <div class="bg-darkgray text-beige relative w-full h-screen shadow-header ">
                <div class="absolute overflow-hidden -left-10 top-16 h-[40vw] w-[40vw] rounded-r-[35%] rotate-12">
                    // <img class="-rotate-12" src="https://github.com/SpacewaIker/portfolio-v2/blob/content/media/profile_picture.jpg?raw=true" />
                    <img class="h-[110%] w-[110%] max-w-none -rotate-12" src="https://thibautbaguette.com/img/profile_picture.jpg" />
                </div>
                <div inner_html=intro_html class="absolute w-1/2 top-[50vh] right-20 font-paragraph text-2xl styled-body" />
            </div>

            <div class="h-screen">
            </div>
        </div>
    }
}

/// Change name colour and position of the name
#[allow(clippy::cast_lossless)]
#[allow(clippy::similar_names)]
fn name_event() {
    let document = leptos::document();
    let window = leptos::window();

    let first_name = document
        .get_element_by_id("first-name")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let last_name = document
        .get_element_by_id("last-name")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    let scroll_top = window.page_y_offset().unwrap();
    let end_scroll = window.inner_height().unwrap().as_f64().unwrap() * 0.7;
    let scroll_amount = if scroll_top < end_scroll {
        scroll_top
    } else {
        end_scroll
    };
    let t = scroll_amount / end_scroll;

    // colour
    // from rgb(41 0 41) to rgb(97 40 255)
    let max_r = 97.0f64;
    let min_r = 41.0f64;
    let max_g = 40.0f64;
    let min_g = 0.0f64;
    let max_b = 255.0f64;
    let min_b = 41.0f64;

    let red = (max_r - min_r).mul_add(t, min_r);
    let green = (max_g - min_g).mul_add(t, min_g);
    let blue = (max_b - min_b).mul_add(t, min_b);
    let colour = format!("rgb({red} {green} {blue})");

    first_name.style().set_property("color", &colour).unwrap();
    last_name.style().set_property("color", &colour).unwrap();

    // position
    let window_width = window.inner_width().unwrap().as_f64().unwrap();
    let window_height = window.inner_height().unwrap().as_f64().unwrap();

    let max_lnc = (window_width - last_name.offset_width() as f64) * 0.15;
    let min_lnc = 0.0;
    let max_fnc = (window_width - first_name.offset_width() as f64) * 0.95;
    let min_fnc = 0.0;

    let max_lnv = window_height.mul_add(0.25, window_width * 0.25);
    let min_lnv = 0.0;
    let max_fnv = window_height.mul_add(0.25, window_width * 0.125);
    let min_fnv = 0.0;

    let last_name_centering = (max_lnc - min_lnc).mul_add(t, min_lnc);
    let first_name_centering = (max_fnc - min_fnc).mul_add(t, min_fnc);
    let last_name_vertical = -(max_lnv - min_lnv).mul_add(t, min_lnv);
    let first_name_vertical = -(max_fnv - min_fnv).mul_add(t, min_fnv);

    last_name
        .style()
        .set_property("right", &format!("{last_name_centering}px"))
        .unwrap();
    first_name
        .style()
        .set_property("left", &format!("{first_name_centering}px"))
        .unwrap();
    last_name
        .style()
        .set_property("bottom", &format!("{last_name_vertical}px"))
        .unwrap();
    first_name
        .style()
        .set_property("bottom", &format!("{first_name_vertical}px"))
        .unwrap();
}
