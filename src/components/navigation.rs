use crate::AppLanguage;
use leptos::{component, document, use_context, view, window, IntoView, SignalGet, SignalSet};
use leptos_router::A;
use wasm_bindgen::JsCast;

/// Header component
#[component]
pub fn Header() -> impl IntoView {
    let lang = use_context::<AppLanguage>().expect("No context found!").0;
    let switch_lang = move |_| {
        lang.set(if lang.get() == "fr" {
            "en".to_string()
        } else {
            "fr".to_string()
        });
    };

    #[rustfmt::skip]
    let links_titles = (
        move || { if lang.get() == "fr" { "Accueil();" } else { "Home();" } },
        move || { if lang.get() == "fr" { "Projets();" } else { "Projects();" } },
        move || { if lang.get() == "fr" { "Expérience();" } else { "Experience();" } },
        move || { if lang.get() == "fr" { "Éducation();" } else { "Education();" } },
    );

    #[rustfmt::skip]
    let icons_title = (
        move || { if lang.get() == "fr" { "Connecter avec moi!" } else { "Connect with me!" } },
        move || { if lang.get() == "fr" { "Voir mes projets!" } else { "Check out my projects!" } },
        move || { if lang.get() == "fr" { "Voir mes jeux!" } else { "Check out my games!" } },
        move || { if lang.get() == "fr" { "Obtenis mon CV!" } else { "Get my resume!" } },
        move || { if lang.get() == "fr" { "Envoyer un courriel!" } else { "Send me an email!" } },
    );

    view! {
        <header class="text-yellow font-mono text-lg fixed text-center w-screen z-20 top-0">
            <div class="bg-purple -z-[1] w-[130vw] h-24 absolute rotate-1 -left-[15vw] -top-8 shadow-header"></div>
            <nav class="inline-block *:inline-block *:mr-4 mt-4">
                <div>
                    "menu("
                    <div class="sliding-underline-yellow hover:cursor-pointer" on:click=switch_lang>{lang}</div>
                    ") {"
                </div>
                <A href="/" class="sliding-underline-yellow">{links_titles.0}</A>
                <A href="/projects" class="sliding-underline-yellow">{links_titles.1}</A>
                <A href="/experience" class="sliding-underline-yellow">{links_titles.2}</A>
                <A href="/education" class="sliding-underline-yellow">{links_titles.3}</A>
                "}"
            </nav>
            <nav class="inline-block *:inline-block *:mr-4 mt-4 absolute right-4 text-2xl">
                <a class="sliding-underline-yellow-low" href="https://www.linkedin.com/in/thibaut-baguette" title=icons_title.0 target="_blank"><i class="nf nf-fa-linkedin"></i></a>
                <a class="sliding-underline-yellow-low" href="https://www.github.com/SpacewaIker" title=icons_title.1 target="_blank"><i class="nf nf-fa-github"></i></a>
                <a class="sliding-underline-yellow-low" href="https://spacewaiker.itch.io" title=icons_title.2 target="_blank"><i class="nf nf-fa-itch_io"></i></a>
                <a class="sliding-underline-yellow-low font-paragraph font-black" href="./cv.pdf" title=icons_title.3 target="_blank">CV</a>
                <a class="sliding-underline-yellow-low" href="maito:thibaut.baguette@mail.mcgill.ca" title=icons_title.4><i class="nf nf-md-email_edit_outline"></i></a>
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
    el.set_value("thibaut.baguette@mail.mcgill.ca");
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
    let lang = use_context::<AppLanguage>().expect("No context found!").0;

    #[rustfmt::skip]
    let website_links = (
        move || { if lang.get() == "fr" { "Liens du site" } else { "Website Links" } },
        move || { if lang.get() == "fr" { "Accueil" } else { "Home" } },
        move || { if lang.get() == "fr" { "À propos de moi" } else { "About me" } },
        move || { if lang.get() == "fr" { "Chronologie" } else { "Timeline" } },
    );

    #[rustfmt::skip]
    let copy_email = move || { if lang.get() == "fr" { "Courriel copié!" } else { "Email copied!" } };

    #[rustfmt::skip]
    let about = (
        move || { if lang.get() == "fr" { "À propos" } else { "About" } },
        move || { if lang.get() == "fr" { "Site web conçu et construit par" } else { "Website designed and built by" } },
        move || { if lang.get() == "fr" { "Visiter le dépôt du site web" } else { "Visit Website Repository" } },
    );

    #[rustfmt::skip]
    let links_titles = (
        move || { if lang.get() == "fr" { "Copier mon adresse courriel!" } else { "Copy my email address!" } },
        move || { if lang.get() == "fr" { "Voir mes projets!" } else { "Check out my projects!" } },
        move || { if lang.get() == "fr" { "Voir mes jeux!" } else { "Check out my games!" } },
        move || { if lang.get() == "fr" { "Connecter avec moi!" } else { "Connect with me!" } },
        move || { if lang.get() == "fr" { "Obtenir mon CV!" } else { "Get my resume!" } },
    );

    view! {
        <footer class="sticky bottom-0 w-full flex justify-center text-beige text-lg space-x-20 font-line bg-darkpurple p-8">
            // "Website Links" column
            <div class="flex flex-col items-center text-center z-10">
                <h1 class="font-mono text-3xl font-bold my-6">{website_links.0}</h1>
                <ul>
                    <li><a class="sliding-underline-beige" href="/">{website_links.1}</a></li>
                    <li><a class="sliding-underline-beige" href="/#intro-screen">{website_links.2}</a></li>
                    <li><a class="sliding-underline-beige" href="/#timeline-screen">{website_links.3}</a></li>
                </ul>
            </div>

            // "Social" column
            <div class="flex flex-col items-center text-center z-10">
                <h1 class="font-mono text-3xl font-bold my-6">"Social"</h1>
                <div class="absolute top-32 p-2 rounded-md border-4 border-purple bg-darkpurple -z-[1] opacity-0 transition-opacity duration-300" id="email-copy-popup">
                    {copy_email}
                </div>
                <p class="cursor-pointer" on:click=copy_email_action title=links_titles.0>
                    "Email"<br/>"thibaut.baguette@mail.mcgill.ca"
                </p>
                <ul>
                    <li><a class="sliding-underline-beige" href="https://www.github.com/SpacewaIker" title=links_titles.1 target="_blank">"GitHub"</a></li>
                    <li><a class="sliding-underline-beige" href="https://spacewaiker.itch.io" title=links_titles.2 target="_blank">"Itch.io"</a></li>
                    <li><a class="sliding-underline-beige" href="https://www.linkedin.com/in/thibaut-baguette" title=links_titles.3 target="_blank">LinkedIn</a></li>
                    <li><a class="sliding-underline-beige" href="/cv.pdf" title=links_titles.4>CV</a></li>
                </ul>
            </div>

            // "About" column
            <div class="flex flex-col items-center text-center z-10">
                <h1 class="font-mono text-3xl font-bold my-6">{about.0}</h1>
                <p>{about.1}<br/>"Thibaut Baguette"</p>
                <p class="mt-4">
                    <a class="sliding-underline-beige" href="https://www.github.com/SpacewaIker/spacewaiker.github.io" target="_blank">{about.2}</a>
                </p>
            </div>
        </footer>
    }
}
