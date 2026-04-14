use crate::Route;
use crate::i18n::{Locale, translate};
use dioxus::prelude::*;

#[component]
pub fn PrivacyPage() -> Element {
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
                h1 { class: "text-3xl font-bold font-heading text-gray-900 mb-2", {translate(locale(), "privacy.title")} }
                p { class: "text-sm text-gray-500", {translate(locale(), "privacy.updated")} }
            }

            div { class: "bg-white border border-gray-200 rounded-lg divide-y divide-gray-100",

                // Section 1
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "privacy.s1.title")} }
                    p { class: "{body}",
                        {translate(locale(), "privacy.s1.body")}
                    }
                }

                // Section 2
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "privacy.s2.title")} }
                    p { class: "{body}", {translate(locale(), "privacy.s2.body")} }
                }

                // Section 3
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "privacy.s3.title")} }
                    p { class: "{body}", {translate(locale(), "privacy.s3.body")} }
                }

                // Section 4
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "privacy.s4.title")} }
                    p { class: "{body}", {translate(locale(), "privacy.s4.body")} }
                }

                // Section 5
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "privacy.s5.title")} }
                    p { class: "{body}", {translate(locale(), "privacy.s5.body")} }
                }

                // Section 6
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "privacy.s6.title")} }
                    p { class: "{body}", {translate(locale(), "privacy.s6.body")} }
                }

                // Section 7
                div { class: "{row_class}",
                    p { class: "{section_title}", {translate(locale(), "privacy.s7.title")} }
                    p { class: "{body}", {translate(locale(), "privacy.s7.body")} }
                }
            }
        }
    }
}
