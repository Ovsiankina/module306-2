use crate::stores::{get_stores, Category, Store};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
enum SortOrder {
    #[default]
    NameAZ,
    NameZA,
    LevelAsc,
    LevelDesc,
}

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
        0 => "Floor 0",
        1 => "Floor 1",
        2 => "Floor 2",
        _ => "Floor 3",
    }
}

pub fn ShopDirectory() -> Element {
    let mut name_query = use_signal(|| String::new());
    let mut cat_filter = use_signal(|| String::new());
    let mut lvl_filter = use_signal(|| String::new());
    let mut sort_order = use_signal(SortOrder::default);

    let stores = use_loader(|| get_stores())?;

    let q = name_query().to_lowercase();
    let ck = cat_filter();
    let lv = lvl_filter();

    let mut filtered: Vec<Store> = stores
        .iter()
        .filter(|s| {
            (q.is_empty() || s.name.to_lowercase().contains(&q))
                && (ck.is_empty() || s.category.key() == ck.as_str())
                && (lv.is_empty() || s.level.map(|l| l.to_string()).as_deref() == Some(lv.as_str()))
        })
        .map(|s| (*s).clone())
        .collect();

    match sort_order() {
        SortOrder::NameAZ => filtered.sort_by(|a, b| a.name.cmp(&b.name)),
        SortOrder::NameZA => filtered.sort_by(|a, b| b.name.cmp(&a.name)),
        SortOrder::LevelAsc => filtered.sort_by(|a, b| a.level.cmp(&b.level).then(a.name.cmp(&b.name))),
        SortOrder::LevelDesc => filtered.sort_by(|a, b| b.level.cmp(&a.level).then(a.name.cmp(&b.name))),
    }

    let count = filtered.len();
    let count_label = if count == 1 { "store" } else { "stores" };

    let select_class = "py-3 px-4 text-xs uppercase font-semibold font-heading bg-gray-50 border border-gray-200 rounded-md focus:ring-blue-300 focus:border-blue-300 focus:outline-none cursor-pointer";

    rsx! {
        section { class: "max-w-7xl mx-auto px-6 py-10",
            div { class: "mb-6",
                h1 { class: "text-3xl font-bold font-heading mb-1", "Shop Directory" }
                p { class: "text-sm text-gray-500", "{count} {count_label} found" }
            }

            // Filter bar
            div { class: "flex flex-wrap gap-3 mb-8",
                input {
                    class: "flex-1 min-w-48 py-3 px-4 text-xs uppercase font-semibold font-heading bg-gray-50 border border-gray-200 rounded-md placeholder-gray-400 focus:ring-blue-300 focus:border-blue-300 focus:outline-none",
                    r#type: "text",
                    placeholder: "Search by name...",
                    oninput: move |e| name_query.set(e.value()),
                }
                select {
                    class: "{select_class}",
                    onchange: move |e| cat_filter.set(e.value()),
                    option { value: "", "All categories" }
                    for cat in Category::all() {
                        option { value: "{cat.key()}", "{cat.label()}" }
                    }
                }
                select {
                    class: "{select_class}",
                    onchange: move |e| lvl_filter.set(e.value()),
                    option { value: "", "All floors" }
                    option { value: "0", "Floor 0 (Yellow)" }
                    option { value: "1", "Floor 1 (Red)" }
                    option { value: "2", "Floor 2 (Blue)" }
                    option { value: "3", "Floor 3 (Green)" }
                }
                select {
                    class: "{select_class}",
                    onchange: move |e| {
                        sort_order.set(match e.value().as_str() {
                            "name_za"    => SortOrder::NameZA,
                            "level_asc"  => SortOrder::LevelAsc,
                            "level_desc" => SortOrder::LevelDesc,
                            _            => SortOrder::NameAZ,
                        });
                    },
                    option { value: "name_az",    "Name A \u{2192} Z" }
                    option { value: "name_za",    "Name Z \u{2192} A" }
                    option { value: "level_asc",  "Floor 0 \u{2192} 3" }
                    option { value: "level_desc", "Floor 3 \u{2192} 0" }
                }
            }

            // Results
            if filtered.is_empty() {
                div { class: "text-center py-20 text-gray-400 font-semibold font-heading",
                    "No stores match your filters."
                }
            } else {
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4",
                    for store in filtered {
                        div {
                            key: "{store.name}",
                            class: "bg-white border border-gray-200 rounded-lg p-4 flex flex-col gap-2 hover:shadow-md transition-shadow",

                            // Name + floor badge
                            div { class: "flex items-start justify-between gap-2",
                                h3 { class: "font-bold font-heading text-gray-900 leading-tight", "{store.name}" }
                                if let Some(level) = store.level {
                                    span {
                                        class: "shrink-0 text-xs font-semibold px-2 py-0.5 rounded-full {level_badge_class(level)}",
                                        "{level_label(level)}"
                                    }
                                }
                            }

                            // Category badge
                            span { class: "text-xs px-2 py-0.5 rounded-full bg-gray-100 text-gray-600 self-start",
                                "{store.category.label()}"
                            }

                            // Store number
                            if let Some(ref num) = store.store_number {
                                p { class: "text-xs text-gray-400", "Store #{num}" }
                            }

                            // Contact / link
                            div { class: "mt-auto pt-1 flex flex-col gap-1",
                                if let Some(phone) = store.phone {
                                    p { class: "text-xs text-gray-500", "{phone}" }
                                }
                                if let Some(website) = store.website {
                                    a {
                                        class: "text-xs font-semibold text-blue-600 hover:text-blue-800 hover:underline self-start",
                                        href: "{website}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "Visit website \u{2192}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
