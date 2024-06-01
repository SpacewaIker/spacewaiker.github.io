use leptos::{component, create_memo, view, Await, CollectView, IntoView, SignalGet};
use leptos_i18n::Locale as _;
use rand::{seq::SliceRandom, thread_rng};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{data_loading::get_content, i18n::use_i18n};

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
    let i18n = use_i18n();
    let lang = move || i18n.get_locale().as_str();

    let words = content
        .get("word_soup")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    let content = create_memo(move |_| content.get(lang()).unwrap().clone());

    let intro_html =
        move || markdown::to_html(content.get().get("intro").unwrap().as_str().unwrap());

    let body_html = move || markdown::to_html(content.get().get("body").unwrap().as_str().unwrap());

    // add scroll event listener to change name colour and position
    let name_event = wasm_bindgen::closure::Closure::wrap(Box::new(name_event) as Box<dyn FnMut()>);
    leptos::window()
        .dyn_into::<web_sys::EventTarget>()
        .unwrap()
        .add_event_listener_with_callback("scroll", name_event.as_ref().unchecked_ref())
        .unwrap();
    name_event.forget();

    view! {
        <div>
            <div class="bg-beige h-fit min-h-screen overflow-x-hidden">
                // title screen
                <div class="absolute w-full h-[120vh]">
                    <WordSoup words=&words />
                </div>

                <div class="relative z-20 font-title text-[23vw] text-[transparent] mix-blend-color-dodge title-text-stroke">
                    <div id="first-name" class="absolute left-1/2 -translate-x-1/2 top-[20vh] md:top-0">Thibaut</div>
                    <div id="last-name"  class="absolute left-1/2 -translate-x-1/2 top-[50vh] md:top-[45vh]">Baguette</div>
                </div>

                <div class="w-full h-screen">
                </div>

                <div class="relative w-full h-40 z-10">
                    <svg class="w-full h-[175%] fill-purple" viewBox="0 0 100 100" preserveAspectRatio="none">
                        <polyline points="0 30,100 0,100 70,0 100"></polyline>
                    </svg>
                </div>

                // intro screen
                <div class="bg-darkgray text-beige relative w-full flex flex-col md:flex-row items-center"
                     style="padding-top: calc(15rem + 12vw);">
                    <div class="relative overflow-hidden h-[80vw] w-[80vw] rounded-[30%] rotate-12
                                md:-left-16 -top-10 md:h-[40vw] md:w-[40vw]">
                        <img class="relative -top-[6%] -left-[6%] h-[110%] w-[110%] max-w-none -rotate-12"
                             src="https://thibautbaguette.com/img/profile_picture.jpg" />
                    </div>
                    <div inner_html=intro_html class="px-4 w-full md:w-1/2 right-20 font-paragraph text-2xl styled-body leading-relaxed" />
                </div>

                <div class="relative -top-2 bg-darkgray text-beige w-full font-paragraph text-xl styled-body styled-body-cols
                            py-10 px-4 leading-relaxed space-y-8 md:px-24"
                     inner_html=body_html>
                </div>
            </div>

            // required to make the footer angled
            <svg class="h-20 w-full relative -top-4" viewBox="0 0 100 100" preserveAspectRatio="none">
                <polyline class="fill-darkgray" points="0 0, 100 0, 100 60, 0 100"></polyline>
            </svg>
        </div>
    }
}

#[component]
#[allow(clippy::needless_lifetimes)]
fn WordSoup<'a>(words: &'a [String]) -> impl IntoView {
    let mut rng = thread_rng();

    let mut all_words = Vec::new();
    for _ in 0..10 {
        all_words.extend_from_slice(words);
    }
    all_words.shuffle(&mut rng);

    let list_items = all_words
        .into_iter()
        .map(|word| view! { <li class="inline-block">{ word }</li> })
        .collect_view();

    view! {
        <div class="w-full h-full overflow-hidden font-mono text-darkgray tracking-[0.3em] blur-[1px] opacity-20 select-none">
            <ul class="rotate-12 list-none w-[130%] h-[130%] relative fixed-center space-x-8 space-y-4">
                {list_items}
            </ul>
        </div>
    }
}

/// Change size and position of name
#[allow(clippy::cast_lossless)]
#[allow(clippy::similar_names)]
fn name_event() {
    let window = leptos::window();

    // check if path is home, otherwise can't get the first-name and last-name elements
    let href = window.location().href().unwrap();
    if !href.ends_with("/home") {
        return;
    }

    let document = leptos::document();

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
    let end_scroll = window.inner_height().unwrap().as_f64().unwrap() * 0.9;
    let scroll_amount = f64::min(scroll_top, end_scroll);
    let t = scroll_amount / end_scroll;

    let window_width = window.inner_width().unwrap().as_f64().unwrap();
    let window_height = window.inner_height().unwrap().as_f64().unwrap();
    let rem = window
        .get_computed_style(&leptos::document().document_element().unwrap())
        .unwrap()
        .unwrap()
        .get_property_value("font-size")
        .unwrap()
        .replace("px", "")
        .parse::<f64>()
        .unwrap();

    // size
    let size_start = 0.23 * window_width;
    let size_end = 0.12 * window_width;
    let size = (size_end - size_start).mul_add(t, size_start);

    first_name
        .style()
        .set_property("font-size", &format!("{size}px"))
        .unwrap();
    last_name
        .style()
        .set_property("font-size", &format!("{size}px"))
        .unwrap();

    // y position
    let query = window.match_media("(min-width: 768px)").unwrap().unwrap();
    let (y_start_first, y_start_last) = if query.matches() {
        // md
        (0.0, 0.45 * window_height)
    } else {
        // sm
        (0.20 * window_height, 0.50 * window_height)
    };
    let y_end = 17.5f64.mul_add(rem, window_height); // 100vh + 10rem * 175% (separator height)
    let y_first = (y_end - y_start_first).mul_add(t, y_start_first);
    let y_last = (y_end - y_start_last).mul_add(t, y_start_last);

    first_name
        .style()
        .set_property("top", &format!("{y_first}px"))
        .unwrap();
    last_name
        .style()
        .set_property("top", &format!("{y_last}px"))
        .unwrap();

    // x position
    let x_start = -50.0f64;
    let x_end_first = -115.0f64;
    let x_end_last = -5.0f64;
    let x_first = (x_end_first - x_start).mul_add(t, x_start);
    let x_last = (x_end_last - x_start).mul_add(t, x_start);

    first_name
        .style()
        .set_property("transform", &format!("translateX({x_first}%)"))
        .unwrap();
    last_name
        .style()
        .set_property("transform", &format!("translateX({x_last}%)"))
        .unwrap();
}
