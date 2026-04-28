use crate::auth::Role;
use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::auth::{read_token, AuthState};
use crate::i18n::{translate, translate_fmt, Locale};
use crate::stores::{get_stores, set_store_position, slugify, Category, Store};
use dioxus::prelude::*;

fn floor_color_classes(level: u8) -> &'static str {
    match level {
        0 => "bg-yellow-400 border-yellow-700 text-yellow-900",
        1 => "bg-red-500 border-red-800 text-white",
        2 => "bg-blue-500 border-blue-800 text-white",
        _ => "bg-green-500 border-green-800 text-white",
    }
}

fn floor_plan_src(level: u8) -> Asset {
    match level {
        0 => asset!("/assets/fox_town/level_0.jpg"),
        1 => asset!("/assets/fox_town/level_1.jpg"),
        2 => asset!("/assets/fox_town/level_2.jpg"),
        _ => asset!("/assets/fox_town/level_3.jpg"),
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

pub fn MapEditorPage() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();

    let is_admin = matches!(auth(), AuthState::LoggedIn(ref u) if u.role == Role::Admin);

    let mut stores = use_signal(Vec::<Store>::new);
    let mut active_level = use_signal(|| 0u8);
    let mut selected_slug = use_signal(|| None::<String>);
    let mut error = use_signal(String::new);
    let mut saving = use_signal(|| false);
    let mut img_handle = use_signal(|| None::<std::rc::Rc<MountedData>>);
    let mut query = use_signal(String::new);
    let mut hide_placed = use_signal(|| false);
    // Whether the store list for the active level is currently shown.
    // Clicking the active floor button toggles this; clicking a different
    // floor switches level and forces the list visible.
    let mut list_visible = use_signal(|| true);

    // Initial load
    use_effect(move || {
        if !is_admin {
            return;
        }
        spawn(async move {
            match get_stores().await {
                Ok(list) => stores.set(list),
                Err(e) => error.set(e.to_string()),
            }
        });
    });

    if !is_admin {
        return rsx! {
            div { class: "min-h-screen flex flex-col bg-white font-heading",
                Nav { active: NavPage::None }
                section { class: "max-w-3xl mx-auto px-6 py-16 flex-1",
                    h1 { class: "text-3xl font-extrabold text-dark mb-4",
                        {translate(locale(), "map_editor.title")}
                    }
                    p { class: "text-sm text-muted",
                        {translate(locale(), "map_editor.admin_only")}
                    }
                }
                Footer { dark: false, stick_to_bottom: false }
            }
        };
    }

    let stores_snapshot = stores();
    let stores_for_level: Vec<Store> = stores_snapshot
        .iter()
        .filter(|s| s.level == Some(active_level()))
        .cloned()
        .collect();
    let placed_for_level: Vec<Store> = stores_for_level
        .iter()
        .filter(|s| s.map_x.is_some() && s.map_y.is_some())
        .cloned()
        .collect();
    let placed_count = placed_for_level.len();
    let level_total = stores_for_level.len();

    // Filter by search query and "hide placed" toggle, then sort:
    // unplaced first, then alphabetical.
    let q = query().trim().to_lowercase();
    let hide = hide_placed();
    let mut listed: Vec<Store> = stores_for_level
        .iter()
        .filter(|s| {
            if hide && s.map_x.is_some() {
                return false;
            }
            if q.is_empty() {
                return true;
            }
            let in_name = s.name.to_lowercase().contains(&q);
            let in_unit = s
                .store_number
                .as_deref()
                .map(|u| u.to_lowercase().contains(&q))
                .unwrap_or(false);
            in_name || in_unit
        })
        .cloned()
        .collect();
    listed.sort_by(|a, b| {
        let a_placed = a.map_x.is_some();
        let b_placed = b.map_x.is_some();
        a_placed.cmp(&b_placed).then(a.name.cmp(&b.name))
    });

    let selected_store = selected_slug()
        .as_ref()
        .and_then(|slug| stores().into_iter().find(|s| slugify(&s.name) == *slug));

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::None }

            section { class: "max-w-7xl mx-auto w-full px-6 py-10 flex-1",
                div { class: "mb-6",
                    p { class: "text-xs font-bold tracking-widest text-accent mb-2", "ADMIN AREA" }
                    h1 { class: "text-3xl md:text-4xl font-extrabold text-dark mb-2",
                        {translate(locale(), "map_editor.title")}
                    }
                    p { class: "text-sm text-muted",
                        {translate(locale(), "map_editor.subtitle")}
                    }
                }

                // Floor selector with placed-count badges
                div { class: "flex flex-wrap items-center gap-2 mb-6",
                    for floor in 0u8..4u8 {
                        {
                            let total = stores_snapshot.iter().filter(|s| s.level == Some(floor)).count();
                            let placed = stores_snapshot.iter()
                                .filter(|s| s.level == Some(floor) && s.map_x.is_some())
                                .count();
                            let is_active = active_level() == floor;
                            rsx! {
                                button {
                                    key: "lvl-{floor}",
                                    class: if is_active {
                                        "px-5 py-2 rounded-full text-xs font-bold tracking-wider bg-dark text-white"
                                    } else {
                                        "px-5 py-2 rounded-full text-xs font-bold tracking-wider bg-gray-100 text-muted hover:bg-gray-200 transition-colors"
                                    },
                                    onclick: move |_| {
                                        if active_level() == floor {
                                            list_visible.set(!list_visible());
                                        } else {
                                            active_level.set(floor);
                                            list_visible.set(true);
                                            selected_slug.set(None);
                                        }
                                    },
                                    {translate_fmt(locale(), "directory.floor_button", &[("level", floor.to_string())])}
                                    span { class: "ml-2 opacity-80", " {placed}/{total}" }
                                }
                            }
                        }
                    }
                }

                // Error banner
                if !error().is_empty() {
                    div { class: "mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-sm text-red-700",
                        "{error}"
                    }
                }

                // Placement instruction
                if let Some(s) = selected_store.clone() {
                    div { class: "mb-4 p-3 bg-accent/10 border border-accent/30 rounded-lg flex items-center justify-between gap-3",
                        p { class: "text-sm text-dark",
                            {translate_fmt(locale(), "map_editor.click_to_place", &[("store", s.name.clone())])}
                        }
                        button {
                            class: "text-xs font-bold tracking-wider text-muted hover:text-dark",
                            onclick: move |_| selected_slug.set(None),
                            {translate(locale(), "common.cancel")}
                        }
                    }
                }

                // Layout: list + map. Sidebar is hidden when list is collapsed,
                // and the map expands to fill the row.
                div { class: if list_visible() {
                        "grid grid-cols-1 lg:grid-cols-[320px_1fr] gap-6"
                    } else {
                        "grid grid-cols-1 gap-6"
                    },

                    if list_visible() {
                        // Sidebar: store list for active level
                        div { class: "border border-gray-100 rounded-xl overflow-hidden flex flex-col max-h-[700px]",
                            div { class: "px-4 py-3 bg-gray-50 border-b border-gray-100 flex flex-col gap-2",
                                div { class: "flex items-center justify-between gap-2",
                                    p { class: "text-xs font-bold tracking-widest text-dark",
                                        {translate_fmt(locale(), "map_editor.placed_count", &[
                                            ("placed", placed_count.to_string()),
                                            ("total", level_total.to_string()),
                                        ])}
                                    }
                                    label { class: "flex items-center gap-1.5 text-[11px] font-bold tracking-wider text-muted cursor-pointer select-none",
                                        input {
                                            r#type: "checkbox",
                                            class: "accent-dark",
                                            checked: hide_placed(),
                                            onchange: move |evt| hide_placed.set(evt.checked()),
                                        }
                                        "UNPLACED ONLY"
                                    }
                                }
                                div { class: "relative",
                                    input {
                                        r#type: "text",
                                        value: "{query}",
                                        placeholder: "Search store or unit…",
                                        class: "w-full px-3 py-1.5 pr-7 text-sm border border-gray-200 rounded-md bg-white focus:outline-none focus:border-dark",
                                        oninput: move |evt| query.set(evt.value()),
                                    }
                                    if !query().is_empty() {
                                        button {
                                            class: "absolute right-2 top-1/2 -translate-y-1/2 text-muted hover:text-dark text-sm",
                                            onclick: move |_| query.set(String::new()),
                                            "×"
                                        }
                                    }
                                }
                            }
                            div { class: "overflow-y-auto divide-y divide-gray-50",
                                if listed.is_empty() {
                                    p { class: "px-4 py-8 text-sm text-muted text-center",
                                        if stores_for_level.is_empty() {
                                            {translate(locale(), "map_editor.no_stores_for_level")}
                                        } else {
                                            "No stores match your filters."
                                        }
                                    }
                                } else {
                                    for store in listed.clone() {
                                        {
                                            let slug = slugify(&store.name);
                                            let is_selected = selected_slug() == Some(slug.clone());
                                            let placed = store.map_x.is_some() && store.map_y.is_some();
                                            let cat_label = translate(locale(), category_label_key(&store.category));
                                            let unit = store.store_number.clone().unwrap_or_default();
                                            let slug_for_select = slug.clone();
                                            let slug_for_clear = slug.clone();
                                            let saving_clear = saving;
                                            rsx! {
                                                div {
                                                    key: "{slug}",
                                                    class: if is_selected {
                                                        "px-3 py-2 bg-accent/10"
                                                    } else {
                                                        "px-3 py-2 hover:bg-gray-50"
                                                    },
                                                    div { class: "flex items-start justify-between gap-2",
                                                        div { class: "min-w-0 flex-1",
                                                            p { class: "text-sm font-bold text-dark truncate", "{store.name}" }
                                                            p { class: "text-xs text-muted truncate", "{cat_label} • {unit}" }
                                                        }
                                                        if placed {
                                                            span { class: "text-[10px] font-bold tracking-wider text-green-700 bg-green-50 px-2 py-0.5 rounded",
                                                                {translate(locale(), "map_editor.placed_badge")}
                                                            }
                                                        }
                                                    }
                                                    div { class: "mt-2 flex items-center gap-2",
                                                        button {
                                                            class: "px-3 py-1 rounded text-[11px] font-bold tracking-wider bg-dark text-white hover:bg-gray-800",
                                                            onclick: move |_| {
                                                                selected_slug.set(Some(slug_for_select.clone()));
                                                            },
                                                            if placed {
                                                                {translate(locale(), "map_editor.move_btn")}
                                                            } else {
                                                                {translate(locale(), "map_editor.place_btn")}
                                                            }
                                                        }
                                                        if placed {
                                                            button {
                                                                class: "px-3 py-1 rounded text-[11px] font-bold tracking-wider text-red-700 hover:bg-red-50",
                                                                disabled: saving_clear(),
                                                                onclick: move |_| {
                                                                    let slug = slug_for_clear.clone();
                                                                    let level = active_level();
                                                                    let mut saving = saving_clear;
                                                                    let mut error = error;
                                                                    let mut stores = stores;
                                                                    let mut selected_slug = selected_slug;
                                                                    spawn(async move {
                                                                        let Some(token) = read_token() else {
                                                                            error.set("Missing auth token".to_string());
                                                                            return;
                                                                        };
                                                                        saving.set(true);
                                                                        error.set(String::new());
                                                                        match set_store_position(token, slug.clone(), Some(level), None, None).await {
                                                                            Ok(updated) => {
                                                                                let mut list = stores();
                                                                                if let Some(s) = list.iter_mut().find(|s| slugify(&s.name) == slug) {
                                                                                    *s = updated;
                                                                                }
                                                                                stores.set(list);
                                                                                if selected_slug() == Some(slug) {
                                                                                    selected_slug.set(None);
                                                                                }
                                                                            }
                                                                            Err(e) => error.set(e.to_string()),
                                                                        }
                                                                        saving.set(false);
                                                                    });
                                                                },
                                                                {translate(locale(), "map_editor.remove_btn")}
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
                    }

                    // Map area
                    div { class: "border border-gray-100 rounded-xl overflow-hidden bg-gray-50",
                        div {
                            class: "relative w-full select-none",
                            style: if selected_store.is_some() { "cursor: crosshair;" } else { "cursor: default;" },
                            img {
                                src: floor_plan_src(active_level()),
                                class: "w-full h-auto block",
                                alt: translate_fmt(locale(), "directory.floor_plan_alt", &[("level", active_level().to_string())]),
                                draggable: false,
                                onmounted: move |evt| {
                                    img_handle.set(Some(evt.data()));
                                },
                                onclick: move |evt| {
                                    let Some(slug) = selected_slug() else { return };
                                    let Some(handle) = img_handle.read().clone() else { return };
                                    let coords = evt.element_coordinates();
                                    let cx = coords.x;
                                    let cy = coords.y;
                                    let level = active_level();
                                    let mut saving = saving;
                                    let mut error = error;
                                    let mut stores = stores;
                                    let mut selected_slug = selected_slug;
                                    spawn(async move {
                                        let Ok(rect) = handle.get_client_rect().await else {
                                            error.set("Could not measure floor plan".to_string());
                                            return;
                                        };
                                        let (w, h) = (rect.size.width, rect.size.height);
                                        if w <= 0.0 || h <= 0.0 {
                                            return;
                                        }
                                        let x_pct = ((cx / w).clamp(0.0, 1.0) * 100.0) as f32;
                                        let y_pct = ((cy / h).clamp(0.0, 1.0) * 100.0) as f32;
                                        let Some(token) = read_token() else {
                                            error.set("Missing auth token".to_string());
                                            return;
                                        };
                                        saving.set(true);
                                        error.set(String::new());
                                        match set_store_position(token, slug.clone(), Some(level), Some(x_pct), Some(y_pct)).await {
                                            Ok(updated) => {
                                                let mut list = stores();
                                                if let Some(s) = list.iter_mut().find(|s| slugify(&s.name) == slug) {
                                                    *s = updated;
                                                }
                                                stores.set(list);
                                                selected_slug.set(None);
                                            }
                                            Err(e) => error.set(e.to_string()),
                                        }
                                        saving.set(false);
                                    });
                                },
                            }

                            // Existing markers for this level
                            for store in placed_for_level.clone() {
                                {
                                    let x = store.map_x.unwrap_or(0.0);
                                    let y = store.map_y.unwrap_or(0.0);
                                    let unit = store.store_number.clone().unwrap_or_default();
                                    let icon = store.icon_path.clone();
                                    let name = store.name.clone();
                                    let level = active_level();
                                    let slug = slugify(&store.name);
                                    let is_selected_marker = selected_slug() == Some(slug.clone());
                                    let outline = if is_selected_marker {
                                        "ring-2 ring-accent ring-offset-2"
                                    } else {
                                        ""
                                    };
                                    let color = floor_color_classes(level);
                                    rsx! {
                                        button {
                                            key: "marker-{slug}",
                                            class: "absolute w-8 h-8 rounded-full border-2 bg-white overflow-hidden flex items-center justify-center shadow {color} {outline}",
                                            style: "left: calc({x}% - 16px); top: calc({y}% - 16px);",
                                            title: "{name}",
                                            onclick: move |evt| {
                                                evt.stop_propagation();
                                                selected_slug.set(Some(slug.clone()));
                                            },
                                            if let Some(ref src) = icon {
                                                img {
                                                    src: "{src}",
                                                    class: "max-h-full max-w-full object-contain",
                                                    alt: "{name}",
                                                    draggable: false,
                                                }
                                            } else {
                                                span { class: "text-[10px] font-bold leading-none", "{unit}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Footer { dark: false, stick_to_bottom: false }
        }
    }
}
