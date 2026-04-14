use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::stores::{get_stores, Store};
use dioxus::prelude::*;

pub fn ShopDirectory() -> Element {
    let mut search = use_signal(String::new);
    let mut active_floor = use_signal(|| 0u8);

    let stores = use_loader(|| get_stores())?;

    let q = search().to_lowercase();
    let search_results: Vec<Store> = if q.is_empty() {
        vec![]
    } else {
        stores
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&q))
            .take(5)
            .map(|s| (*s).clone())
            .collect()
    };

    let floor_btn_class = |floor: u8| {
        if active_floor() == floor {
            "px-6 py-2.5 text-xs font-bold tracking-wider text-white bg-dark rounded-full"
        } else {
            "px-6 py-2.5 text-xs font-bold tracking-wider text-muted hover:text-dark rounded-full transition-colors"
        }
    };

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::Map }

            // ─── Hero ───────────────────────────────────────────────
            section { class: "max-w-7xl mx-auto px-6 pt-16 pb-8",
                h1 { class: "text-4xl md:text-5xl font-extrabold text-dark leading-tight mb-4",
                    "Navigate Luxury."
                }
                p { class: "text-body leading-relaxed max-w-2xl",
                    "Explore three floors of premium outlet shopping. Locate your favorite boutiques, find exclusive dining, and plan your journey through FoxTown."
                }
            }

            // ─── Main content ───────────────────────────────────────
            section { class: "max-w-7xl mx-auto px-6 pb-16",
                div { class: "flex flex-col lg:flex-row gap-8",

                    // ── Sidebar ─────────────────────────────────────
                    div { class: "w-full lg:w-80 shrink-0 space-y-8",

                        // Store search
                        div {
                            h3 { class: "text-lg font-bold text-accent mb-4", "Find a Store" }
                            div { class: "flex border border-gray-200 rounded-lg overflow-hidden",
                                input {
                                    class: "flex-1 py-3 px-4 text-sm placeholder-muted focus:outline-none",
                                    r#type: "text",
                                    placeholder: "Search brands...",
                                    value: "{search}",
                                    oninput: move |e| search.set(e.value()),
                                }
                                button { class: "px-4 text-muted hover:text-dark transition-colors",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "16",
                                        height: "16",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        circle { cx: "11", cy: "11", r: "8" }
                                        line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                                    }
                                }
                            }

                            // Search results
                            if !search_results.is_empty() {
                                div { class: "mt-2 border border-gray-100 rounded-lg divide-y divide-gray-50",
                                    for store in search_results {
                                        div { class: "px-4 py-3 hover:bg-gray-50 cursor-pointer",
                                            p { class: "text-sm font-bold text-dark", "{store.name}" }
                                            if let Some(level) = store.level {
                                                p { class: "text-xs text-muted",
                                                    if let Some(ref num) = store.store_number {
                                                        "L{level} \u{2022} {num}"
                                                    } else {
                                                        "L{level}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Facilities legend
                        div {
                            h3 { class: "text-lg font-bold text-dark mb-4", "Facilities" }
                            div { class: "space-y-3",
                                FacilityItem { label: "Restrooms" }
                                FacilityItem { label: "Elevators" }
                                div { class: "flex items-center gap-3",
                                    div { class: "w-8 h-8 bg-gray-100 rounded-lg flex items-center justify-center text-muted text-xs", "\u{1F37D}" }
                                    div {
                                        p { class: "text-sm text-dark", "Food Court" }
                                        p { class: "text-xs text-accent", "Level 2" }
                                    }
                                }
                                FacilityItem { label: "Parking Access" }
                                FacilityItem { label: "First Aid" }
                            }
                        }
                    }

                    // ── Map area ────────────────────────────────────
                    div { class: "flex-1",

                        // Floor level selector
                        div { class: "flex items-center gap-2 mb-6 bg-gray-50 rounded-full p-1 w-fit",
                            for floor in 0u8..4 {
                                button {
                                    key: "floor-{floor}",
                                    class: floor_btn_class(floor),
                                    onclick: move |_| active_floor.set(floor),
                                    "Level {floor}"
                                }
                            }
                        }

                        // Map canvas
                        div { class: "relative bg-gray-50 rounded-2xl border border-gray-100 overflow-hidden",
                            // Floor plan image
                            img {
                                src: "/floor-plan.png",
                                class: "w-full object-cover",
                                alt: "Level {active_floor()} Floor Plan",
                            }

                            // Zoom controls
                            div { class: "absolute right-4 top-1/2 -translate-y-1/2 flex flex-col gap-2",
                                button { class: "w-10 h-10 bg-white rounded-lg shadow-md flex items-center justify-center text-dark hover:bg-gray-50",
                                    "+"
                                }
                                button { class: "w-10 h-10 bg-white rounded-lg shadow-md flex items-center justify-center text-dark hover:bg-gray-50",
                                    "\u{2212}"
                                }
                                button { class: "w-10 h-10 bg-white rounded-lg shadow-md flex items-center justify-center text-dark hover:bg-gray-50",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "16",
                                        height: "16",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        // Crosshair / location icon
                                        circle { cx: "12", cy: "12", r: "3" }
                                        path { d: "M12 2v4M12 18v4M2 12h4M18 12h4" }
                                    }
                                }
                            }

                            // Wing label
                            div { class: "absolute top-4 left-4",
                                span { class: "px-3 py-1.5 bg-dark/80 text-white text-xs font-bold tracking-wider rounded-full",
                                    "NORTH WING"
                                }
                            }
                        }
                    }
                }
            }

            Footer { dark: false }
        }
    }
}

#[component]
fn FacilityItem(label: &'static str) -> Element {
    rsx! {
        div { class: "flex items-center gap-3",
            div { class: "w-8 h-8 bg-gray-100 rounded-lg flex items-center justify-center",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "14",
                    height: "14",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    circle { cx: "12", cy: "12", r: "10" }
                }
            }
            p { class: "text-sm text-dark", "{label}" }
        }
    }
}
