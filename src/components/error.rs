use dioxus::prelude::*;
use crate::i18n::{Locale, translate};

#[component]
pub fn error_page() -> Element {
    let locale = use_context::<Signal<Locale>>();
    rsx! {
        section { class: "py-20",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-wrap -mx-4 mb-24 text-center",
                    {translate(locale(), "error.internal")}
                }
            }
        }
    }
}
