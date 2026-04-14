use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::stores::{get_stores, slugify, Category, Store};
use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
enum FilterGroup {
    #[default]
    All,
    Luxury,
    FashionAccessories,
    SportOutdoor,
    HomeLifestyle,
    Kids,
}

impl FilterGroup {
    fn label(&self) -> &'static str {
        match self {
            Self::All => "ALL BRANDS",
            Self::Luxury => "LUXURY",
            Self::FashionAccessories => "FASHION & ACCESSORIES",
            Self::SportOutdoor => "SPORT & OUTDOOR",
            Self::HomeLifestyle => "HOME & LIFESTYLE",
            Self::Kids => "KIDS",
        }
    }

    fn matches(&self, cat: &Category) -> bool {
        match self {
            Self::All => true,
            Self::Luxury => matches!(cat, Category::HighFashion | Category::WatchesJewellery),
            Self::FashionAccessories => matches!(
                cat,
                Category::LadiesMenswear
                    | Category::Casualwear
                    | Category::Accessories
                    | Category::Footwear
                    | Category::Underwear
            ),
            Self::SportOutdoor => matches!(cat, Category::SportswearEquipment),
            Self::HomeLifestyle => {
                matches!(cat, Category::Home | Category::Electronics | Category::Beauty)
            }
            Self::Kids => matches!(cat, Category::Childrenswear),
        }
    }

    fn all() -> &'static [FilterGroup] {
        &[
            Self::All,
            Self::Luxury,
            Self::FashionAccessories,
            Self::SportOutdoor,
            Self::HomeLifestyle,
            Self::Kids,
        ]
    }
}

fn brand_image(name: &str) -> Option<&'static str> {
    match name.to_lowercase().as_str() {
        n if n.contains("gucci") => Some("/brands/gucci.png"),
        n if n.contains("prada") => Some("/brands/prada.png"),
        n if n.contains("armani") => Some("/brands/armani.png"),
        n if n.contains("burberry") => Some("/brands/burberry.png"),
        n if n.contains("nike") => Some("/brands/nike.png"),
        n if n.contains("adidas") => Some("/brands/adidas.png"),
        n if n.contains("dolce") || n.contains("gabbana") => Some("/brands/dolce-gabbana.png"),
        n if n.contains("valentino") => Some("/brands/valentino.png"),
        _ => None,
    }
}

fn category_label(cat: &Category) -> &'static str {
    match cat {
        Category::HighFashion => "LUXURY FASHION",
        Category::LadiesMenswear => "FASHION",
        Category::Casualwear => "CASUALWEAR",
        Category::SportswearEquipment => "SPORT & PERFORMANCE",
        Category::Childrenswear => "KIDSWEAR",
        Category::Footwear => "FOOTWEAR",
        Category::Underwear => "UNDERWEAR",
        Category::WatchesJewellery => "LUXURY HERITAGE",
        Category::Accessories => "ACCESSORIES",
        Category::Electronics => "ELECTRONICS",
        Category::Beauty => "BEAUTY",
        Category::Home => "HOME & LIFESTYLE",
        Category::FoodDrinks => "FOOD & DRINKS",
        Category::Services => "SERVICES",
    }
}

pub(crate) fn Home() -> Element {
    let mut search = use_signal(String::new);
    let mut filter = use_signal(FilterGroup::default);

    let stores = use_loader(|| get_stores())?;

    let q = search().to_lowercase();
    let fg = filter();

    let filtered: Vec<Store> = stores
        .iter()
        .filter(|s| {
            (q.is_empty() || s.name.to_lowercase().contains(&q)) && fg.matches(&s.category)
        })
        .map(|s| (*s).clone())
        .collect();

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::Stores }

            // ─── Hero section ───────────────────────────────────────
            section { class: "max-w-7xl mx-auto px-6 pt-16 pb-12",
                div { class: "max-w-2xl",
                    p { class: "text-sm font-semibold tracking-widest text-accent mb-4",
                        "STORE DIRECTORY"
                    }
                    h1 { class: "text-4xl md:text-5xl font-extrabold text-dark leading-tight mb-6",
                        "THE ARCHIVE OF\nEXCELLENCE."
                    }
                    p { class: "text-body leading-relaxed mb-8",
                        "Discover over 160 stores from the most prestigious international brands with discounts from 30% to 70% all year round."
                    }
                }

                // Search bar
                div { class: "flex max-w-lg",
                    div { class: "flex-1 relative",
                        input {
                            class: "w-full py-3.5 pl-4 pr-12 text-sm border border-gray-200 rounded-l-lg placeholder-muted focus:ring-accent focus:border-accent focus:outline-none",
                            r#type: "text",
                            placeholder: "FIND A BRAND...",
                            value: "{search}",
                            oninput: move |e| search.set(e.value()),
                        }
                    }
                    button { class: "px-5 bg-dark text-white rounded-r-lg hover:bg-gray-700 transition-colors",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "18",
                            height: "18",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "11", cy: "11", r: "8" }
                            line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                        }
                    }
                }
            }

            // ─── Filter bar ─────────────────────────────────────────
            section { class: "border-y border-gray-100",
                div { class: "max-w-7xl mx-auto px-6",
                    div { class: "flex gap-1 overflow-x-auto py-3 -mx-1",
                        for &group in FilterGroup::all() {
                            button {
                                key: "{group.label()}",
                                class: if filter() == group {
                                    "shrink-0 px-5 py-2.5 text-xs font-bold tracking-wider rounded-full bg-dark text-white"
                                } else {
                                    "shrink-0 px-5 py-2.5 text-xs font-bold tracking-wider rounded-full text-body hover:bg-gray-100 transition-colors"
                                },
                                onclick: move |_| filter.set(group),
                                "{group.label()}"
                            }
                        }
                    }
                }
            }

            // ─── Store grid ─────────────────────────────────────────
            section { class: "max-w-7xl mx-auto px-6 py-12",
                if filtered.is_empty() {
                    div { class: "text-center py-20 text-muted font-semibold",
                        "No stores match your search."
                    }
                } else {
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6",
                        for store in filtered {
                            Link {
                                key: "{store.name}",
                                to: Route::Store { name: slugify(&store.name) },
                                class: "group block",

                                // Store image
                                div { class: "h-40 w-full bg-gray-100 rounded-lg mb-4 overflow-hidden flex items-center justify-center",
                                    if let Some(img) = brand_image(&store.name) {
                                        img { src: "{img}", class: "w-full h-full object-cover", alt: "{store.name}" }
                                    } else {
                                        span { class: "text-2xl font-extrabold text-gray-300 tracking-wider group-hover:text-accent transition-colors",
                                            "{store.name}"
                                        }
                                    }
                                }

                                // Info
                                h3 { class: "text-sm font-bold text-dark tracking-wide mb-1",
                                    "{store.name}"
                                }
                                p { class: "text-xs font-semibold text-accent tracking-wider mb-1",
                                    "{category_label(&store.category)}"
                                }
                                if let Some(level) = store.level {
                                    p { class: "text-xs text-muted tracking-wider",
                                        if let Some(ref num) = store.store_number {
                                            "LEVEL {level} \u{2022} UNIT {num}"
                                        } else {
                                            "LEVEL {level}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ─── Newsletter section ─────────────────────────────────
            section { class: "bg-dark",
                div { class: "max-w-7xl mx-auto px-6 py-16 flex flex-col lg:flex-row items-center gap-12",
                    div { class: "flex-1",
                        h2 { class: "text-3xl md:text-4xl font-extrabold text-white leading-tight mb-4",
                            "NEVER MISS AN\nEXCLUSIVE DROP."
                        }
                        p { class: "text-muted leading-relaxed mb-8 max-w-md",
                            "Join our private list to receive early access to seasonal sales, limited events, and new store openings at FoxTown."
                        }
                        div { class: "flex max-w-md",
                            input {
                                class: "flex-1 py-3.5 px-4 text-sm bg-gray-800 border border-gray-700 rounded-l-lg placeholder-surface text-white focus:ring-accent focus:border-accent focus:outline-none",
                                r#type: "email",
                                placeholder: "YOUR EMAIL ADDRESS",
                            }
                            button { class: "px-6 py-3.5 text-xs font-bold tracking-wider text-white bg-accent hover:bg-amber-600 rounded-r-lg transition-colors",
                                "SUBSCRIBE"
                            }
                        }
                    }
                    // Decorative editorial image
                    div { class: "w-full lg:w-80 h-64 rounded-lg overflow-hidden",
                        img { src: "/editorial-fashion.png", class: "w-full h-full object-cover", alt: "" }
                    }
                }
            }

            Footer { dark: false }
        }
    }
}
