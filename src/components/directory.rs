use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::i18n::{Locale, translate, translate_fmt};
use crate::stores::{get_stores, Store};
use dioxus::prelude::*;

pub fn ShopDirectory() -> Element {
    let locale = use_context::<Signal<Locale>>();
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
                    {translate(locale(), "directory.title")}
                }
                p { class: "text-body leading-relaxed max-w-2xl",
                    {translate(locale(), "directory.subtitle")}
                }
            }

            // ─── Main content ───────────────────────────────────────
            section { class: "max-w-7xl mx-auto px-6 pb-16",
                div { class: "flex flex-col lg:flex-row gap-8",

                    // ── Sidebar ─────────────────────────────────────
                    div { class: "w-full lg:w-80 shrink-0 space-y-8",

                        // Store search
                        div {
                            h3 { class: "text-lg font-bold text-accent mb-4", {translate(locale(), "directory.find_store")} }
                            div { class: "flex border border-gray-200 rounded-lg overflow-hidden",
                                input {
                                    class: "flex-1 py-3 px-4 text-sm placeholder-muted focus:outline-none",
                                    r#type: "text",
                                    placeholder: {translate(locale(), "directory.search_placeholder")},
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
                                        div {
                                            class: "px-4 py-3 hover:bg-gray-50 cursor-pointer",
                                            onclick: move |_| {
                                                if let Some(level) = store.level {
                                                    active_floor.set(level);
                                                }
                                                search.set(String::new());
                                            },
                                            p { class: "text-sm font-bold text-dark", "{store.name}" }
                                            if let Some(level) = store.level {
                                                p { class: "text-xs text-muted",
                                                    if let Some(ref num) = store.store_number {
                                                        {translate_fmt(locale(), "directory.level_short_unit", &[("level", level.to_string()), ("unit", num.clone())])}
                                                    } else {
                                                        {translate_fmt(locale(), "directory.level_short", &[("level", level.to_string())])}
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
                            h3 { class: "text-lg font-bold text-dark mb-4", {translate(locale(), "directory.facilities")} }
                            div { class: "space-y-3",
                                FacilityItem { label: translate(locale(), "directory.restrooms") }
                                FacilityItem { label: translate(locale(), "directory.elevators") }
                                div { class: "flex items-center gap-3",
                                    div { class: "w-8 h-8 bg-gray-100 rounded-lg flex items-center justify-center text-muted text-xs", "\u{1F37D}" }
                                    div {
                                        p { class: "text-sm text-dark", {translate(locale(), "directory.food_court")} }
                                        p { class: "text-xs text-accent", {translate_fmt(locale(), "directory.level", &[("level", "2".to_string())])} }
                                    }
                                }
                                FacilityItem { label: translate(locale(), "directory.parking") }
                                FacilityItem { label: translate(locale(), "directory.first_aid") }
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
                                    onclick: move |_| {
                                        active_floor.set(floor);
                                    },
                                    {translate_fmt(locale(), "directory.floor_button", &[("level", floor.to_string())])}
                                }
                            }
                        }

                        DirectoryMap { active_floor, locale }
                    }
                }
            }

            Footer { dark: false }
        }
    }
}

#[component]
fn DirectoryMap(active_floor: Signal<u8>, locale: Signal<Locale>) -> Element {
    let mut zoom_level = use_signal(|| 1.0f32);
    let mut pan_offset = use_signal(|| (0.0f32, 0.0f32));
    let mut is_dragging = use_signal(|| false);
    let mut drag_origin = use_signal(|| (0.0f32, 0.0f32));
    let zoom_step = 0.2f32;
    let min_zoom = 0.6f32;
    let max_zoom = 2.4f32;

    use_effect(move || {
        let _ = active_floor();
        zoom_level.set(1.0);
        pan_offset.set((0.0, 0.0));
    });

    let floor_plan_src = match active_floor() {
        0 => asset!("/assets/fox_town/level_0.jpg"),
        1 => asset!("/assets/fox_town/level_1.jpg"),
        2 => asset!("/assets/fox_town/level_2.jpg"),
        _ => asset!("/assets/fox_town/level_3.jpg"),
    };

    rsx! {
        div {
            class: "relative bg-gray-50 rounded-2xl border border-gray-100 overflow-hidden",
            onmousedown: move |evt| {
                is_dragging.set(true);
                let coords = evt.client_coordinates();
                drag_origin.set((
                    coords.x as f32 - pan_offset().0,
                    coords.y as f32 - pan_offset().1,
                ));
            },
            onmousemove: move |evt| {
                if !is_dragging() {
                    return;
                }

                let coords = evt.client_coordinates();
                pan_offset.set((
                    coords.x as f32 - drag_origin().0,
                    coords.y as f32 - drag_origin().1,
                ));
            },
            onmouseup: move |_| {
                is_dragging.set(false);
            },
            onmouseleave: move |_| {
                is_dragging.set(false);
            },
            // Floor plan image
            img {
                src: floor_plan_src,
                class: if is_dragging() {
                    "w-full object-cover transition-transform duration-200 cursor-grabbing select-none"
                } else {
                    "w-full object-cover transition-transform duration-200 cursor-grab select-none"
                },
                alt: {translate_fmt(locale(), "directory.floor_plan_alt", &[("level", active_floor().to_string())])},
                draggable: false,
                style: format!(
                    "transform: translate({}px, {}px) scale({}); transform-origin: center center;",
                    pan_offset().0,
                    pan_offset().1,
                    zoom_level()
                ),
            }

            // Zoom controls
            div { class: "absolute right-4 top-1/2 -translate-y-1/2 flex flex-col gap-2",
                button {
                    class: "w-10 h-10 bg-white rounded-lg shadow-md flex items-center justify-center text-dark hover:bg-gray-50",
                    onclick: move |_| {
                        let next_zoom = (zoom_level() + zoom_step).min(max_zoom);
                        zoom_level.set(next_zoom);
                    },
                    "+"
                }
                button {
                    class: "w-10 h-10 bg-white rounded-lg shadow-md flex items-center justify-center text-dark hover:bg-gray-50",
                    onclick: move |_| {
                        let next_zoom = (zoom_level() - zoom_step).max(min_zoom);
                        zoom_level.set(next_zoom);
                    },
                    "\u{2212}"
                }
                button {
                    class: "w-10 h-10 bg-white rounded-lg shadow-md flex items-center justify-center text-dark hover:bg-gray-50",
                    onclick: move |_| {
                        zoom_level.set(1.0);
                        pan_offset.set((0.0, 0.0));
                    },
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
                    {translate(locale(), "directory.north_wing")}
                }
            }
        }
    }
}

#[component]
fn FacilityItem(label: String) -> Element {
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
