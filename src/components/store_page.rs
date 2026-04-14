use crate::stores::{get_store, Store};
use crate::i18n::{Locale, translate, translate_fmt};
use crate::Route;
use dioxus::prelude::*;

fn level_badge_class(level: u8) -> &'static str {
    match level {
        0 => "bg-yellow-100 text-yellow-700 border border-yellow-200",
        1 => "bg-red-100 text-red-700 border border-red-200",
        2 => "bg-blue-100 text-blue-700 border border-blue-200",
        _ => "bg-green-100 text-green-700 border border-green-200",
    }
}

fn level_label(level: u8) -> &'static str {
    match level {
        0 => "store.level.0",
        1 => "store.level.1",
        2 => "store.level.2",
        _ => "store.level.3",
    }
}

#[component]
pub fn StorePage(name: ReadSignal<String>) -> Element {
    let locale = use_context::<Signal<Locale>>();
    let store = use_loader(move || get_store(name()))?;

    let Store {
        name,
        category,
        store_number,
        level,
        phone,
        website,
        ..
    } = store();

    let row_class = "px-6 py-4 flex items-center justify-between gap-4";
    let label_class = "text-sm font-semibold font-heading text-gray-500 shrink-0";
    let value_class = "text-sm text-gray-900 text-right";

    rsx! {
        section { class: "max-w-3xl mx-auto px-6 py-10",

            Link {
                to: Route::Map {},
                class: "inline-flex items-center text-sm font-semibold font-heading text-gray-500 hover:text-gray-900 mb-8",
                {translate(locale(), "store.back_directory")}
            }

            // Header
            div { class: "mb-8",
                div { class: "flex flex-wrap items-center gap-3 mb-3",
                    h1 { class: "text-3xl font-bold font-heading text-gray-900", "{name}" }
                    if let Some(lvl) = level {
                        span {
                            class: "text-sm font-semibold px-3 py-1 rounded-full border {level_badge_class(lvl)}",
                            {translate_fmt(locale(), "store.floor_badge", &[("level", lvl.to_string())])}
                        }
                    }
                }
                span { class: "inline-block text-sm px-3 py-1 rounded-full bg-gray-100 text-gray-600",
                    "{category.label()}"
                }
            }

            // Info card
            div { class: "bg-white border border-gray-200 rounded-lg divide-y divide-gray-100 mb-6",
                h2 { class: "px-6 py-4 text-xs uppercase font-semibold font-heading text-gray-400 tracking-widest",
                    {translate(locale(), "store.info_title")}
                }

                if let Some(lvl) = level {
                    div { class: "{row_class}",
                        span { class: "{label_class}", {translate(locale(), "store.floor")} }
                        span { class: "{value_class}", {translate(locale(), level_label(lvl))} }
                    }
                }

                if let Some(num) = store_number {
                    div { class: "{row_class}",
                        span { class: "{label_class}", {translate(locale(), "store.store_number")} }
                        span { class: "{value_class}", "#{num}" }
                    }
                }

                if let Some(p) = phone {
                    div { class: "{row_class}",
                        span { class: "{label_class}", {translate(locale(), "store.phone")} }
                        a {
                            class: "text-sm text-blue-600 hover:text-blue-800 hover:underline",
                            href: "tel:{p}",
                            "{p}"
                        }
                    }
                }

                if let Some(w) = website {
                    div { class: "{row_class}",
                        span { class: "{label_class}", {translate(locale(), "store.website")} }
                        a {
                            class: "text-sm font-semibold text-blue-600 hover:text-blue-800 hover:underline",
                            href: "{w}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            {translate(locale(), "store.visit_website")}
                        }
                    }
                }
            }

            // Map placeholder
            div { class: "bg-white border border-gray-200 rounded-lg overflow-hidden",
                h2 { class: "px-6 py-4 text-xs uppercase font-semibold font-heading text-gray-400 tracking-widest border-b border-gray-100",
                    {translate(locale(), "store.map_title")}
                }
                // TODO: render interactive floor plan highlighting this store's position on its level
                div { class: "flex items-center justify-center h-48 bg-gray-50 text-sm text-gray-400 font-semibold font-heading",
                    {translate(locale(), "store.map_coming")}
                }
            }
        }
    }
}
