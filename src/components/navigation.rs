use crate::ApplicationData;
use leptos::{component, document, use_context, view, window, IntoView, SignalGet, SignalSet};
use leptos_router::A;
use wasm_bindgen::JsCast;

#[component]
pub fn Header() -> impl IntoView {
    let app_data = use_context::<ApplicationData>().expect("No context found");
    let lang = app_data.language;
    let switch_lang = move |_| {
        app_data.set_language.set(if lang.get() == "fr" {
            "en".to_string()
        } else {
            "fr".to_string()
        });
    };

    view! {
        <header class="text-yellow font-mono text-lg fixed text-center w-screen z-10 top-0">
            <svg class="-z-[1] h-28 w-full absolute" viewBox="0 0 100 150" preserveAspectRatio="none">
                <polyline class="fill-purple drop-shadow-header" points="0 0, 100 0, 100 100, 0 70"></polyline>
            </svg>
            <nav class="inline-block *:inline-block *:mr-4 mt-4">
                <div>
                    "menu("
                    <div class="sliding-underline-yellow hover:cursor-pointer" on:click=switch_lang>{lang}</div>
                    ") {"
                </div>
                <A href="/" class="sliding-underline-yellow">{move || if lang.get() == "fr" {"Accueil();"} else {"Home();"}}</A>
                <A href="/projects" class="sliding-underline-yellow">{move || if lang.get() == "fr" {"Projets();"} else {"Projects();"}}</A>
                <A href="/experience" class="sliding-underline-yellow">{move || if lang.get() == "fr" {"Expérience();"} else {"Experience();"}}</A>
                <A href="/education" class="sliding-underline-yellow">{move || if lang.get() == "fr" {"Éducation();"} else {"Education();"}}</A>
                "}"
            </nav>
            <nav class="inline-block *:inline-block *:mr-4 mt-4 absolute right-4 text-2xl">
                <a class="sliding-underline-yellow-low" href="https://www.linkedin.com/in/thibaut-baguette" target="_blank"><i class="nf nf-fa-linkedin"></i></a>
                <a class="sliding-underline-yellow-low" href="https://www.github.com/SpacewaIker" target="_blank"><i class="nf nf-fa-github"></i></a>
                <a class="sliding-underline-yellow-low" href="https://spacewaiker.itch.io" target="_blank"><i class="nf nf-fa-itch_io"></i></a>
                <a class="sliding-underline-yellow-low font-paragraph font-black" href="./cv.pdf" target="_blank">CV</a>
                <a class="sliding-underline-yellow-low" href="maito:thibaut.baguette@mail.mcgill.ca"><i class="nf nf-md-email_edit_outline"></i></a>
            </nav>
        </header>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    let app_data = use_context::<ApplicationData>().expect("No context found");
    let lang = app_data.language;

    let copy_email = move |_| {
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
        el.set_value("thibaut.baguette@mail.mcgill.ca");
        el.select();
        document()
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap()
            .exec_command("copy")
            .expect("Failed to copy");
        el.remove();

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
    };

    view! {
        <footer class="absolute w-full flex justify-center text-beige text-lg space-x-20 font-line bg-beige">
            <svg class="absolute fill-darkpurple w-full h-[28rem]" viewBox="0 0 100 120" preserveAspectRatio="none">
                <polyline class="drop-shadow-footer" points="0 40, 100 20, 100 120, 0 120"></polyline>
            </svg>
            <div class="flex flex-col items-center text-center z-10 mt-40">
                <h1 class="font-mono text-3xl font-bold my-6">{move || if lang.get() == "fr" {"Liens du site"} else {"Website Links"}}</h1>
                <ul>
                    <li><a class="sliding-underline-beige" href="/">{move || if lang.get() == "fr" {"Accueil"} else {"Home"}}</a></li>
                    <li><a class="sliding-underline-beige" href="/#intro-screen">{move || if lang.get() == "fr" {"À propos de moi"} else {"About me"}}</a></li>
                    <li><a class="sliding-underline-beige" href="/#timeline-screen">{move || if lang.get() == "fr" {"Chronologie"} else {"Timeline"}}</a></li>
                </ul>
            </div>
            <div class="flex flex-col items-center text-center z-10 mt-40">
                <h1 class="font-mono text-3xl font-bold my-6">"Social"</h1>
                <div class="absolute top-32 p-2 rounded-md border-4 border-purple bg-darkpurple -z-[1] opacity-0 transition-opacity duration-300" id="email-copy-popup">
                    {move || if lang.get() == "fr" {"Courriel copié !"} else {"Email Copied!"}}
                </div>
                <p class="cursor-pointer" on:click=copy_email>
                    "Email"<br/>"thibaut.baguette@mail.mcgill.ca"
                </p>
                <ul>
                    <li><a class="sliding-underline-beige" href="https://www.github.com/SpacewaIker" target="_blank">"GitHub"</a></li>
                    <li><a class="sliding-underline-beige" href="https://spacewaiker.itch.io" target="_blank">"Itch.io"</a></li>
                    <li><a class="sliding-underline-beige" href="https://www.linkedin.com/in/thibaut-baguette" target="_blank">LinkedIn</a></li>
                    <li><a class="sliding-underline-beige" href="/cv.pdf">CV</a></li>
                </ul>
            </div>
            <div class="flex flex-col items-center text-center z-10 mt-40">
                <h1 class="font-mono text-3xl font-bold my-6">{move || if lang.get() == "fr" {"À propos"} else {"About"}}</h1>
                <p>
                    {move || if lang.get() == "fr" {"Site web conçu et construit par"} else {"Website designed and built by"}}
                    <br/>"Thibaut Baguette"
                </p>
                <p class="mt-4">
                    <a class="sliding-underline-beige" href="https://www.github.com/SpacewaIker/spacewaiker.github.io" target="_blank">
                        {move|| if lang.get() == "fr" {"Visiter le dépôt du site web"} else {"Visit Website Repository"}}
                    </a>
                </p>
            </div>
        </footer>
    }
}
