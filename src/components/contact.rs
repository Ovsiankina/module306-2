use crate::Route;
use crate::i18n::{Locale, translate};
use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let row_class = "px-6 py-4 flex items-center justify-between gap-4";
    let label_class = "text-sm font-semibold font-heading text-gray-500 shrink-0";
    let value_class = "text-sm text-gray-900 text-right";
    let section_title = "px-6 py-4 text-xs uppercase font-semibold font-heading text-gray-400 tracking-widest";

    rsx! {
        section { class: "max-w-3xl mx-auto px-6 py-10",

            Link {
                to: Route::Home {},
                class: "inline-flex items-center text-sm font-semibold font-heading text-gray-500 hover:text-gray-900 mb-8",
                {format!("← {}", translate(locale(), "common.home"))}
            }

            div { class: "mb-8",
                h1 { class: "text-3xl font-bold font-heading text-gray-900", {translate(locale(), "contact.title")} }
                p { class: "text-sm text-gray-500 mt-1",
                    {translate(locale(), "contact.subtitle")}
                }
            }

            // Contact info card
            div { class: "bg-white border border-gray-200 rounded-lg divide-y divide-gray-100 mb-6",
                h2 { class: "{section_title}", {translate(locale(), "contact.company")} }

                div { class: "{row_class}",
                    span { class: "{label_class}", {translate(locale(), "contact.address")} }
                    span { class: "{value_class}",
                        "Via Angelo Maspoli 18"
                        br {}
                        "CH \u{2014} 6850 Mendrisio"
                    }
                }

                div { class: "{row_class}",
                    span { class: "{label_class}", {translate(locale(), "contact.email")} }
                    a {
                        class: "text-sm text-blue-600 hover:text-blue-800 hover:underline",
                        href: "mailto:info@foxtown.ch",
                        "info@foxtown.ch"
                    }
                }

                div { class: "{row_class}",
                    span { class: "{label_class}", {translate(locale(), "contact.telephone")} }
                    a {
                        class: "text-sm text-blue-600 hover:text-blue-800 hover:underline",
                        href: "tel:+41848828888",
                        "+41 848 828 888"
                    }
                }
            }

            // Opening hours card
            div { class: "bg-white border border-gray-200 rounded-lg divide-y divide-gray-100",
                h2 { class: "{section_title}", {translate(locale(), "contact.opening_hours")} }

                div { class: "{row_class}",
                    span { class: "{label_class}", {translate(locale(), "contact.days")} }
                    span { class: "{value_class}", {translate(locale(), "contact.days_value")} }
                }

                div { class: "{row_class}",
                    span { class: "{label_class}", {translate(locale(), "contact.hours")} }
                    span { class: "{value_class}", {translate(locale(), "contact.hours_value")} }
                }

                div { class: "px-6 py-4",
                    span { class: "{label_class} block mb-2", {translate(locale(), "contact.closing_days")} }
                    ul { class: "list-disc list-inside space-y-1 text-sm text-gray-700",
                        li { {translate(locale(), "contact.close_1")} }
                        li { {translate(locale(), "contact.close_2")} }
                        li { {translate(locale(), "contact.close_3")} }
                        li { {translate(locale(), "contact.close_4")} }
                    }
                }
            }
        }
    }
}
