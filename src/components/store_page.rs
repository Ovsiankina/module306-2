use crate::stores::{Category, Store, get_store_local};
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

fn category_label_key(category: &Category) -> &'static str {
    match category {
        Category::HighFashion => "home.category.luxury_fashion",
        Category::LadiesMenswear => "home.category.fashion",
        Category::Casualwear => "home.category.casualwear",
        Category::SportswearEquipment => "home.category.sport_performance",
        Category::Childrenswear => "home.category.kidswear",
        Category::Footwear => "home.category.footwear",
        Category::Underwear => "home.category.underwear",
        Category::WatchesJewellery => "home.category.luxury_heritage",
        Category::Accessories => "home.category.accessories",
        Category::Electronics => "home.category.electronics",
        Category::Beauty => "home.category.beauty",
        Category::Home => "home.category.home_lifestyle",
        Category::FoodDrinks => "home.category.food_drinks",
        Category::Services => "home.category.services",
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
                    {translate(locale(), category_label_key(&category))}
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
