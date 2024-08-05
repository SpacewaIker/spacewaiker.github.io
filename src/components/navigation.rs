use crate::i18n::{use_i18n, Locale};
use leptos::{component, document, view, window, IntoView, SignalGet};
use leptos_i18n::{t, Locale as _};
use leptos_router::{use_location, use_navigate, NavigateOptions, A};
use wasm_bindgen::JsCast;

const CV_URL: &str = "https://github.com/SpacewaIker/spacewaiker.github.io/blob/content/thibaut_baguette_cv.pdf";

fn expand_header(_: web_sys::MouseEvent) {
    let header = document()
        .get_elements_by_tag_name("header")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    header
        .style()
        .set_property("right", "0")
        .expect("Failed to set right");
}

fn collapse_header(_: web_sys::MouseEvent) {
    let query = window().match_media("(min-width: 768px)").unwrap().unwrap();
    if query.matches() {
        return;
    }

    let header = document()
        .get_elements_by_tag_name("header")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    header
        .style()
        .set_property("right", "-150vw")
        .expect("Failed to set right");
}

/// Header component
#[component]
pub fn Header() -> impl IntoView {
    let i18n = use_i18n();

    let navigate = use_navigate();
    let switch_lang = move |_| {
        let current = use_location().pathname.get();
        #[rustfmt::skip]
        let (lang, locale) = if current.starts_with("/fr") { ("en", Locale::en) } else { ("fr", Locale::fr) };
        let new = format!(
            "/{lang}{}",
            current.trim_start_matches("/en").trim_start_matches("/fr")
        );
        navigate(&new, NavigateOptions::default());
        i18n.set_locale(locale);
    };

    let lang = move || format!("lang: {}", i18n.get_locale().as_str());

    view! {
        <header class="text-yellow font-mono text-lg fixed w-[90vw] h-screen z-20 -right-[150vw] transition-[right] duration-300
                       md:text-center md:w-screen md:h-auto md:right-0 md:top:0">
            <button on:click=expand_header class="absolute h-16 w-16 rounded-full top-8 -left-[80vw] text-4xl bg-purple shadow-header md:hidden">
                <i class="nf nf-md-menu"></i>
            </button>
            <button on:click=collapse_header class="absolute top-12 left-4 text-4xl md:hidden">
                <i class="nf nf-md-close"></i>
            </button>
            <div class="bg-purple -z-[1] w-[130vw] h-[130vh] absolute -rotate-12 left-[15vw] -top-24 shadow-header
                        md:h-24 md:rotate-1 md:-left-[15vw] md:-top-8"></div>
            <nav class="flex flex-col items-end space-y-4 mr-12 mt-12 mb-12
                        md:inline-block md:*:inline-block md:*:mr-4 md:space-y-0 md:mr-0 md:mt-3 md:mb-0">
                <div>
                    "menu("
                    <div class="sliding-underline-yellow hover:cursor-pointer" on:click=switch_lang>{lang}</div>
                    ") {"
                </div>
                <A on:click=collapse_header href="home" class="sliding-underline-yellow">{t!(i18n, header.nav.home)}</A>
                <A on:click=collapse_header href="projects" class="sliding-underline-yellow">{t!(i18n, header.nav.projects)}</A>
                <A on:click=collapse_header href="experience" class="sliding-underline-yellow">{t!(i18n, header.nav.experience)}</A>
                <A on:click=collapse_header href="education" class="sliding-underline-yellow">{t!(i18n, header.nav.education)}</A>
                <div>"}"</div>
            </nav>
            <nav class="flex flex-col space-y-8 mt-4 absolute right-12 text-2xl *:sliding-underline-yellow-low
                        md:inline-block md:*:inline-block md:*:mr-4 md:space-y-0 md:right-4">
                <a href="https://www.linkedin.com/in/thibaut-baguette" title=t!(i18n, header.hover.linkedin) target="_blank"><i class="nf nf-fa-linkedin"></i></a>
                <a href="https://www.github.com/SpacewaIker" title=t!(i18n, header.hover.github) target="_blank"><i class="nf nf-fa-github"></i></a>
                <a href="https://spacewaiker.itch.io" title=t!(i18n, header.hover.itchio) target="_blank"><i class="nf nf-fa-itch_io"></i></a>
                <a class="font-paragraph font-black" href=CV_URL title=t!(i18n, header.hover.resume) target="_blank">CV</a>
                <a href="mailto:tb@thibautbaguette.com" title=t!(i18n, header.hover.email)><i class="nf nf-md-email_edit_outline"></i></a>
            </nav>
        </header>
    }
}

fn copy_email_action(_: web_sys::MouseEvent) {
    let el = document()
        .create_element("input")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    document()
        .body()
        .unwrap()
        .append_child(&el)
        .expect("Failed to append child");
    el.set_value("tb@thibautbaguette.com");
    el.select();
    document()
        .dyn_into::<web_sys::HtmlDocument>()
        .unwrap()
        .exec_command("copy")
        .expect("Failed to copy");
    el.remove();

    // show "copied" popup
    let popup = document()
        .get_element_by_id("email-copy-popup")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    popup
        .style()
        .set_property("z-index", "10")
        .expect("Failed to set z-index");
    popup
        .style()
        .set_property("opacity", "100")
        .expect("Failed to set opacity");
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        popup
            .style()
            .set_property("opacity", "0")
            .expect("Failed to set opacity");
        popup
            .style()
            .set_property("z-index", "-1")
            .expect("Failed to set z-index");
    }) as Box<dyn FnMut()>);
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            800,
        )
        .expect("Failed to set timeout");
    closure.forget();
}

/// Footer component
#[component]
pub fn Footer() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <footer class="sticky bottom-0 w-full justify-center text-beige text-lg font-line bg-darkpurple p-8 md:flex md:space-x-20">
            // "Website Links" column
            <div class="flex flex-col items-center text-center z-10">
                <h1 class="font-mono text-3xl font-bold my-6">{t!(i18n, footer.website_links.title)}</h1>
                <ul>
                    <li><a class="sliding-underline-beige" href="home">{t!(i18n, footer.website_links.home)}</a></li>
                    <li><a class="sliding-underline-beige" href="home#intro">{t!(i18n, footer.website_links.about_me)}</a></li>
                </ul>
            </div>

            // "Social" column
            <div class="flex flex-col items-center text-center z-10">
                <h1 class="font-mono text-3xl font-bold my-6">{t!(i18n, footer.social.title)}</h1>
                <div class="absolute top-32 p-2 rounded-md border-4 border-purple bg-darkpurple -z-[1] opacity-0 transition-opacity duration-300" id="email-copy-popup">
                    {t!(i18n, footer.social.popup)}
                </div>
                <p class="cursor-pointer" on:click=copy_email_action title=t!(i18n, footer.social.email)>
                    "Email"<br/>"tb@thibautbaguette.com"
                </p>
                <ul class="*:*:sliding-underline-beige">
                    <li><a href="https://www.github.com/SpacewaIker" title=t!(i18n, footer.social.github) target="_blank">"GitHub"</a></li>
                    <li><a href="https://spacewaiker.itch.io" title=t!(i18n, footer.social.itchio) target="_blank">"Itch.io"</a></li>
                    <li><a href="https://www.linkedin.com/in/thibaut-baguette" title=t!(i18n, footer.social.linkedin) target="_blank">LinkedIn</a></li>
                    <li><a href=CV_URL title=t!(i18n, footer.social.resume) target="_blank">CV</a></li>
                </ul>
            </div>

            // "About" column
            <div class="flex flex-col items-center text-center z-10">
                <h1 class="font-mono text-3xl font-bold my-6">{t!(i18n, footer.about.title)}</h1>
                <p>{t!(i18n, footer.about.credits)}<br/>"Thibaut Baguette"</p>
                <p class="mt-4">
                    <a class="sliding-underline-beige" href="https://www.github.com/SpacewaIker/spacewaiker.github.io" target="_blank">
                        {t!(i18n, footer.about.repo)}
                    </a>
                </p>
            </div>
        </footer>
    }
}
