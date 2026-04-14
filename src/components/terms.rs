use crate::Route;
use crate::i18n::{Locale, translate};
use dioxus::prelude::*;

#[component]
pub fn TermsPage() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let row_class = "px-6 py-4";
    let section_title = "text-xs uppercase font-semibold font-heading text-gray-400 tracking-widest mb-3";
    let body = "text-sm text-gray-700 leading-relaxed";

    rsx! {
        section { class: "max-w-3xl mx-auto px-6 py-10",
            Link {
                to: Route::Home {},
                class: "inline-flex items-center text-sm font-semibold font-heading text-gray-500 hover:text-gray-900 mb-8",
                {format!("← {}", translate(locale(), "common.home"))}
            }

            div { class: "mb-8",
                h1 { class: "text-3xl font-bold font-heading text-gray-900 mb-2", {translate(locale(), "terms.title")} }
                p { class: "text-sm text-gray-500", {translate(locale(), "terms.updated")} }
            }

            div { class: "bg-white border border-gray-200 rounded-lg divide-y divide-gray-100",
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "terms.s1.title")} }
                    p { class: "{body}", {translate(locale(), "terms.s1.body")} }
                }
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "terms.s2.title")} }
                    p { class: "{body}", {translate(locale(), "terms.s2.body")} }
                }
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "terms.s3.title")} }
                    p { class: "{body}", {translate(locale(), "terms.s3.body")} }
                }
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "terms.s4.title")} }
                    p { class: "{body}", {translate(locale(), "terms.s4.body")} }
                }
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "terms.s5.title")} }
                    p { class: "{body}", {translate(locale(), "terms.s5.body")} }
                }
            }
        }
    }
}
