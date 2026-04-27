use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::i18n::{translate, translate_fmt, Locale};
use crate::stores::{get_stores, slugify, Category, Store};
use crate::Route;
use dioxus::prelude::*;

fn floor_marker_classes(level: u8) -> &'static str {
    match level {
        0 => "bg-yellow-400 border-yellow-700 text-yellow-900",
        1 => "bg-red-500 border-red-800 text-white",
        2 => "bg-blue-500 border-blue-800 text-white",
        _ => "bg-green-500 border-green-800 text-white",
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

fn store_matches(store: &Store, search_lc: &str, category: Option<&Category>) -> bool {
    let matches_search = search_lc.is_empty() || store.name.to_lowercase().contains(search_lc);
    let matches_cat = category.is_none_or(|c| store.category == *c);
    matches_search && matches_cat
}

pub fn ShopDirectory() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut search = use_signal(String::new);
    let mut active_floor = use_signal(|| 0u8);
    let mut category_filter = use_signal(|| None::<Category>);

    let stores = use_loader(|| get_stores())?;

    let q = search().to_lowercase();
    let cat = category_filter();
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

    let stores_owned: Vec<Store> = stores.iter().map(|s| (*s).clone()).collect();

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
                                        {
                                            let level_for_click = store.level;
                                            rsx! {
                                                div {
                                                    class: "px-4 py-3 hover:bg-gray-50 cursor-pointer",
                                                    onclick: move |_| {
                                                        if let Some(level) = level_for_click {
                                                            active_floor.set(level);
                                                        }
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
                            }
                        }

                        // Category filter
                        div {
                            h3 { class: "text-lg font-bold text-dark mb-4",
                                {translate(locale(), "directory.category_filter")}
                            }
                            select {
                                class: "w-full py-3 px-4 text-sm border border-gray-200 rounded-lg bg-white focus:outline-none",
                                onchange: move |e| {
                                    let v = e.value();
                                    if v.is_empty() {
                                        category_filter.set(None);
                                    } else {
                                        let next = Category::all().into_iter().find(|c| c.key() == v);
                                        category_filter.set(next);
                                    }
                                },
                                option { value: "", {translate(locale(), "directory.all_categories")} }
                                for c in Category::all() {
                                    option {
                                        key: "{c.key()}",
                                        value: "{c.key()}",
                                        selected: category_filter().as_ref() == Some(&c),
                                        {translate(locale(), category_label_key(&c))}
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

                        DirectoryMap {
                            active_floor,
                            locale,
                            stores: stores_owned,
                            search_lc: q,
                            category: cat,
                        }
                    }
                }
            }

            Footer { dark: false }
        }
    }
}

#[component]
fn DirectoryMap(
    active_floor: Signal<u8>,
    locale: Signal<Locale>,
    stores: Vec<Store>,
    search_lc: String,
    category: Option<Category>,
) -> Element {
    let mut zoom_level = use_signal(|| 1.0f32);
    let mut pan_offset = use_signal(|| (0.0f32, 0.0f32));
    let mut is_dragging = use_signal(|| false);
    let mut drag_origin = use_signal(|| (0.0f32, 0.0f32));
    let mut pinch_start_distance = use_signal(|| None::<f32>);
    let mut pinch_start_zoom = use_signal(|| 1.0f32);
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

    let nav = use_navigator();
    let any_filter_active = !search_lc.is_empty() || category.is_some();

    let visible_markers: Vec<Store> = stores
        .iter()
        .filter(|s| s.level == Some(active_floor()) && s.map_x.is_some() && s.map_y.is_some())
        .cloned()
        .collect();

    let transform_style = format!(
        "transform: translate({}px, {}px) scale({}); transform-origin: center center; touch-action: none; will-change: transform;",
        pan_offset().0,
        pan_offset().1,
        zoom_level()
    );

    rsx! {
        div {
            class: "relative bg-gray-50 rounded-2xl border border-gray-100 overflow-hidden",
            // Transformable wrapper holds image + markers so they pan/zoom together
            div {
                class: if is_dragging() {
                    "relative cursor-grabbing select-none"
                } else {
                    "relative cursor-grab select-none"
                },
                style: "{transform_style}",
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
                onwheel: move |evt| {
                    evt.prevent_default();
                    let delta_y = evt.delta().strip_units().y as f32;
                    if delta_y.abs() <= f32::EPSILON {
                        return;
                    }
                    let direction = if delta_y < 0.0 { 1.0 } else { -1.0 };
                    let wheel_multiplier = (delta_y.abs() / 120.0).clamp(0.5, 4.0);
                    let next_zoom =
                        (zoom_level() + direction * zoom_step * wheel_multiplier).clamp(min_zoom, max_zoom);
                    zoom_level.set(next_zoom);
                },
                ontouchstart: move |evt| {
                    evt.prevent_default();
                    let touches = evt.touches();
                    if touches.len() >= 2 {
                        is_dragging.set(false);
                        let p1 = touches[0].client_coordinates();
                        let p2 = touches[1].client_coordinates();
                        let dx = (p1.x - p2.x) as f32;
                        let dy = (p1.y - p2.y) as f32;
                        let distance = (dx * dx + dy * dy).sqrt();
                        pinch_start_distance.set(Some(distance));
                        pinch_start_zoom.set(zoom_level());
                    } else if let Some(touch) = touches.first() {
                        is_dragging.set(true);
                        pinch_start_distance.set(None);
                        let coords = touch.client_coordinates();
                        drag_origin.set((
                            coords.x as f32 - pan_offset().0,
                            coords.y as f32 - pan_offset().1,
                        ));
                    }
                },
                ontouchmove: move |evt| {
                    evt.prevent_default();
                    let touches = evt.touches();
                    if touches.len() >= 2 {
                        is_dragging.set(false);
                        let p1 = touches[0].client_coordinates();
                        let p2 = touches[1].client_coordinates();
                        let dx = (p1.x - p2.x) as f32;
                        let dy = (p1.y - p2.y) as f32;
                        let distance = (dx * dx + dy * dy).sqrt();
                        if let Some(start_distance) = pinch_start_distance() {
                            if start_distance > f32::EPSILON {
                                let scale = distance / start_distance;
                                let next_zoom = (pinch_start_zoom() * scale).clamp(min_zoom, max_zoom);
                                zoom_level.set(next_zoom);
                            }
                        } else {
                            pinch_start_distance.set(Some(distance));
                            pinch_start_zoom.set(zoom_level());
                        }
                    } else if touches.len() == 1 && is_dragging() {
                        let coords = touches[0].client_coordinates();
                        pan_offset.set((
                            coords.x as f32 - drag_origin().0,
                            coords.y as f32 - drag_origin().1,
                        ));
                    }
                },
                ontouchend: move |evt| {
                    let touches = evt.touches();
                    if touches.len() < 2 {
                        pinch_start_distance.set(None);
                    }
                    if touches.is_empty() {
                        is_dragging.set(false);
                    } else if touches.len() == 1 {
                        let coords = touches[0].client_coordinates();
                        drag_origin.set((
                            coords.x as f32 - pan_offset().0,
                            coords.y as f32 - pan_offset().1,
                        ));
                        is_dragging.set(true);
                    }
                },

                // Floor plan image
                img {
                    src: floor_plan_src,
                    class: "w-full h-auto block object-contain transition-transform duration-200",
                    alt: {translate_fmt(locale(), "directory.floor_plan_alt", &[("level", active_floor().to_string())])},
                    draggable: false,
                }

                // Marker overlay
                div { class: "absolute inset-0 pointer-events-none",
                    for store in visible_markers.clone() {
                        {
                            let x = store.map_x.unwrap_or(0.0);
                            let y = store.map_y.unwrap_or(0.0);
                            let level = active_floor();
                            let name_for_nav = store.name.clone();
                            let slug = slugify(&store.name);
                            let unit = store.store_number.clone().unwrap_or_default();
                            let matches = store_matches(&store, &search_lc, category.as_ref());
                            let dimmed = any_filter_active && !matches;
                            let highlighted = any_filter_active && matches;
                            let dim_class = if dimmed { "opacity-20 grayscale" } else { "opacity-100" };
                            let pulse_class = if highlighted { "ring-2 ring-accent ring-offset-1 animate-pulse" } else { "" };
                            let color = floor_marker_classes(level);
                            rsx! {
                                button {
                                    key: "marker-{slug}",
                                    class: "pointer-events-auto absolute -translate-x-1/2 -translate-y-1/2 rounded-full border-2 px-1.5 py-0.5 text-[10px] font-bold shadow {color} {dim_class} {pulse_class} hover:scale-125 transition-transform",
                                    style: "left: {x}%; top: {y}%;",
                                    title: "{name_for_nav}",
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                        let _ = nav.push(Route::Store { name: slug.clone() });
                                    },
                                    onmousedown: move |evt| { evt.stop_propagation(); },
                                    ontouchstart: move |evt| { evt.stop_propagation(); },
                                    "{unit}"
                                }
                            }
                        }
                    }
                }
            }

            // Zoom controls (outside the transform)
            div { class: "absolute right-4 top-1/2 -translate-y-1/2 flex flex-col gap-2 z-10",
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
