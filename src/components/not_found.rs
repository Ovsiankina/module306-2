use crate::Route;
use crate::i18n::{Locale, translate};
use dioxus::prelude::*;

#[component]
pub fn NotFoundPage(segments: Vec<String>) -> Element {
    let locale = use_context::<Signal<Locale>>();

    rsx! {
        section { class: "max-w-3xl mx-auto px-6 py-10",

            Link {
                to: Route::Home {},
                class: "inline-flex items-center text-sm font-semibold font-heading text-gray-500 hover:text-gray-900 mb-8",
                {format!("← {}", translate(locale(), "common.home"))}
            }

            div { class: "bg-white border border-gray-200 rounded-lg divide-y divide-gray-100",
                div { class: "px-6 py-16 flex flex-col items-center text-center",
                    p { class: "text-8xl font-extrabold font-heading text-gray-100 mb-4", "404" }
                    h1 { class: "text-2xl font-bold font-heading text-gray-900 mb-2",
                        {translate(locale(), "not_found.title")}
                    }
                    p { class: "text-sm text-gray-500 mb-8 max-w-sm",
                        {translate(locale(), "not_found.subtitle")}
                    }
                    Link {
                        to: Route::Home {},
                        class: "inline-flex items-center px-6 py-3 bg-gray-900 text-white text-xs font-semibold font-heading tracking-widest hover:bg-gray-700 transition-colors rounded",
                        {translate(locale(), "not_found.button")}
                    }
                }
            }
        }
    }
}
