use crate::components::directory::DirectoryMap;
use crate::stores::{get_store_local, get_stores, slugify, Store};
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
pub fn StorePage(name: String) -> Element {
    let locale = use_context::<Signal<Locale>>();
    let Some(store) = get_store_local(&name) else {
        return rsx! {
            section { class: "max-w-3xl mx-auto px-6 py-10",
                Link {
                    to: Route::Map {},
                    class: "inline-flex items-center text-sm font-semibold font-heading text-gray-500 hover:text-gray-900 mb-8",
                    {translate(locale(), "store.back_directory")}
                }
                div { class: "bg-white border border-gray-200 rounded-lg px-6 py-8",
                    h1 { class: "text-2xl font-bold font-heading text-gray-900 mb-2",
                        {translate(locale(), "store.not_found_title")}
                    }
                    p { class: "text-sm text-gray-600",
                        {translate(locale(), "store.not_found_body")}
                    }
                }
            }
        };
    };

    let Store {
        name,
        category,
        store_number,
        level,
        phone,
        website,
        icon_path,
        ..
    } = store;

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
            div { class: "mb-8 flex items-start gap-5",
                if let Some(ref src) = icon_path {
                    div { class: "h-20 w-20 shrink-0 rounded-lg border border-gray-200 bg-white overflow-hidden flex items-center justify-center",
                        img {
                            src: "{src}",
                            class: "max-h-full max-w-full object-contain p-2",
                            alt: "{name}",
                        }
                    }
                }
                div { class: "flex-1 min-w-0",
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
                        {translate(locale(), category.label_key())}
                    }
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

            // Map: same component as the directory page, with this store highlighted.
            // `key` is bound to the store name so navigating between stores via the
            // map remounts StoreMap with a fresh active_floor signal.
            div { class: "bg-white border border-gray-200 rounded-lg overflow-hidden",
                h2 { class: "px-6 py-4 text-xs uppercase font-semibold font-heading text-gray-400 tracking-widest border-b border-gray-100",
                    {translate(locale(), "store.map_title")}
                }
                div { class: "p-4",
                    StoreMap { key: "{name}", name: name.clone(), level }
                }
            }
        }
    }
}

#[component]
fn StoreMap(name: String, level: Option<u8>) -> Element {
    let locale = use_context::<Signal<Locale>>();
    let active_floor = use_signal(|| level.unwrap_or(0));
    let stores = use_loader(|| get_stores())?;

    // If the current store has no coordinates yet, embedding the map would
    // dim every other marker without showing the highlighted one — confusing.
    // Show a "not placed" message instead.
    let slug = slugify(&name);
    let current_placed = stores
        .iter()
        .any(|s| slugify(&s.name) == slug && s.map_x.is_some() && s.map_y.is_some());
    if !current_placed {
        return rsx! {
            div { class: "py-8 text-sm text-gray-500 text-center",
                {translate(locale(), "store.map_not_placed")}
            }
        };
    }

    let stores_owned: Vec<Store> = stores.iter().map(|s| (*s).clone()).collect();

    rsx! {
        DirectoryMap {
            active_floor,
            locale,
            stores: stores_owned,
            search_lc: String::new(),
            category: None,
            highlight_slug: Some(slug),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{level_badge_class, level_label};

    #[test]
    fn level_badge_class_matches_floor_palette() {
        assert_eq!(level_badge_class(0), "bg-yellow-100 text-yellow-700 border border-yellow-200");
        assert_eq!(level_badge_class(1), "bg-red-100 text-red-700 border border-red-200");
        assert_eq!(level_badge_class(2), "bg-blue-100 text-blue-700 border border-blue-200");
        assert_eq!(level_badge_class(3), "bg-green-100 text-green-700 border border-green-200");
        assert_eq!(level_badge_class(99), "bg-green-100 text-green-700 border border-green-200");
    }

    #[test]
    fn level_label_maps_to_expected_translation_keys() {
        assert_eq!(level_label(0), "store.level.0");
        assert_eq!(level_label(1), "store.level.1");
        assert_eq!(level_label(2), "store.level.2");
        assert_eq!(level_label(3), "store.level.3");
        assert_eq!(level_label(99), "store.level.3");
    }
}
