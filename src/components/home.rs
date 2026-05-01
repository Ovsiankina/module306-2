use crate::components::footer::Footer;
use crate::components::landing::LandingSection;
use crate::components::nav::{Nav, NavPage};
use crate::i18n::{Locale, translate, translate_fmt};
use crate::services::game::{
    delay_ms, format_daily_prize_reset_countdown_hms, get_daily_prize_pool_snapshot,
    DailyPrizePoolSnapshot,
};
use crate::services::parking::get_parking_snapshot;
use crate::services::vouchers::list_recent_vouchers;
use crate::stores::{search_stores, slugify, Category, Store};
use crate::Route;
use chrono::Utc;
use dioxus::prelude::*;

/// Reporte l'exécution sur la file JS (`setTimeout(0)`), hors de l'exécuteur
/// `wasm-bindgen-futures` — évite les panics `RefCell already borrowed` lors
/// des `Signal::set` déclenchés depuis des gestionnaires de clic / hydratation.
#[cfg(target_family = "wasm")]
fn defer_after_paint(f: impl FnOnce() + 'static) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let Some(window) = web_sys::window() else {
        return;
    };
    let mut f = Some(f);
    let closure = Closure::wrap(Box::new(move || {
        if let Some(done) = f.take() {
            done();
        }
    }) as Box<dyn FnMut()>);
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        0,
    );
    closure.forget();
}

#[cfg(not(target_family = "wasm"))]
fn defer_after_paint(f: impl FnOnce() + 'static) {
    f();
}

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
    fn label_key(&self) -> &'static str {
        match self {
            Self::All => "home.filter.all",
            Self::Luxury => "home.filter.luxury",
            Self::FashionAccessories => "home.filter.fashion",
            Self::SportOutdoor => "home.filter.sport",
            Self::HomeLifestyle => "home.filter.home",
            Self::Kids => "home.filter.kids",
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

#[component]
fn StoreCardImage(name: String, icon_path: Option<String>) -> Element {
    let mut image_failed = use_signal(|| false);
    let resolved = icon_path.filter(|p| !p.trim().is_empty());
    let resolved_value = resolved.as_deref().unwrap_or_default();

    rsx! {
        div { class: "h-40 w-full bg-gray-100 rounded-lg mb-4 overflow-hidden flex items-center justify-center",
            if image_failed() || resolved.is_none() {
                span { class: "text-2xl font-extrabold text-gray-300 tracking-wider group-hover:text-accent transition-colors",
                    "{name}"
                }
            } else {
                img {
                    src: "{resolved_value}",
                    class: "w-full h-full object-cover",
                    alt: "{name}",
                    onerror: move |_| image_failed.set(true),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FilterGroup, category_label};
    use crate::stores::Category;

    #[test]
    fn filter_group_label_keys_are_stable_and_complete() {
        assert!(FilterGroup::all() == &[
            FilterGroup::All,
            FilterGroup::Luxury,
            FilterGroup::FashionAccessories,
            FilterGroup::SportOutdoor,
            FilterGroup::HomeLifestyle,
            FilterGroup::Kids,
        ]);
        assert_eq!(FilterGroup::All.label_key(), "home.filter.all");
        assert_eq!(FilterGroup::Kids.label_key(), "home.filter.kids");
    }

    #[test]
    fn filter_group_matching_rules_cover_expected_categories() {
        assert!(FilterGroup::All.matches(&Category::Services));
        assert!(FilterGroup::Luxury.matches(&Category::HighFashion));
        assert!(FilterGroup::Luxury.matches(&Category::WatchesJewellery));
        assert!(!FilterGroup::Luxury.matches(&Category::SportswearEquipment));

        assert!(FilterGroup::FashionAccessories.matches(&Category::Accessories));
        assert!(FilterGroup::FashionAccessories.matches(&Category::Footwear));
        assert!(!FilterGroup::FashionAccessories.matches(&Category::Beauty));

        assert!(FilterGroup::HomeLifestyle.matches(&Category::Home));
        assert!(FilterGroup::HomeLifestyle.matches(&Category::Electronics));
        assert!(!FilterGroup::HomeLifestyle.matches(&Category::Childrenswear));
    }

    #[test]
    fn category_label_maps_to_expected_translation_keys() {
        assert_eq!(category_label(&Category::HighFashion), "home.category.luxury_fashion");
        assert_eq!(category_label(&Category::Accessories), "home.category.accessories");
        assert_eq!(category_label(&Category::Services), "home.category.services");
    }
}

fn category_label(cat: &Category) -> &'static str {
    match cat {
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

fn ticker_relative_time(locale: Locale, created_at: &str) -> String {
    let Ok(created) = chrono::DateTime::parse_from_rfc3339(created_at) else {
        return translate(locale, "home.ticker.ago.now");
    };
    let now = chrono::Utc::now();
    let delta = now.signed_duration_since(created.with_timezone(&chrono::Utc));

    if delta.num_minutes() <= 0 {
        return translate(locale, "home.ticker.ago.now");
    }
    if delta.num_minutes() < 60 {
        return translate_fmt(
            locale,
            "home.ticker.ago.minutes",
            &[("n", delta.num_minutes().to_string())],
        );
    }
    if delta.num_hours() < 24 {
        return translate_fmt(
            locale,
            "home.ticker.ago.hours",
            &[("n", delta.num_hours().to_string())],
        );
    }
    translate_fmt(
        locale,
        "home.ticker.ago.days",
        &[("n", delta.num_days().to_string())],
    )
}

fn format_remaining_to_timestamp_hms(target_rfc3339: &str) -> Option<String> {
    let target = chrono::DateTime::parse_from_rfc3339(target_rfc3339)
        .ok()?
        .with_timezone(&Utc);
    let secs = target.signed_duration_since(Utc::now()).num_seconds().max(0) as u64;
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    Some(format!("{h:02}:{m:02}:{s:02}"))
}

#[component]
fn ParkingWidgetBar() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut available_parking = use_signal(|| 0u32);
    let nav = use_navigator();

    use_effect(move || {
        spawn(async move {
            loop {
                if let Ok(snapshot) = get_parking_snapshot().await {
                    let total_capacity: u32 = snapshot.zones.iter().map(|z| z.capacity).sum();
                    let total_occupied: u32 = snapshot.zones.iter().map(|z| z.occupied).sum();
                    available_parking.set(total_capacity.saturating_sub(total_occupied));
                }
                delay_ms(30_000).await;
            }
        });
    });

    rsx! {
        button {
            r#type: "button",
            onclick: move |_| { let _ = nav.push(Route::Parking {}); },
            class: "shrink-0 px-3 py-1.5 rounded-lg hover:bg-gray-800 transition-colors text-right cursor-pointer",
            title: {translate(locale(), "nav.parking")},
            "aria-label": {translate(locale(), "nav.parking")},
            style: "min-width: 7rem;",
            span {
                class: "text-xs font-bold tracking-widest text-accent uppercase block",
                {translate(locale(), "nav.parking")}
            }
            p {
                class: "text-sm font-black text-white",
                style: "margin: 0; margin-top: 2px;",
                "{available_parking()} "
                span { class: "text-xs text-gray-400", {translate(locale(), "parking.spots")} }
            }
        }
    }
}

#[component]
pub(crate) fn HomeWinnersTickerBar() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut pool = use_signal(|| None::<DailyPrizePoolSnapshot>);
    let mut winners = use_signal(Vec::<(String, String)>::new);
    let mut reset_hms = use_signal(|| format_daily_prize_reset_countdown_hms());
    let mut cooldown_hms = use_signal(|| None::<String>);
    let mut parking_stats = use_signal(|| None::<(u32, u32, u32, u32)>);

    use_effect(move || {
        let locale_sig = locale;
        spawn(async move {
            loop {
                let loc = locale_sig();
                if let Ok(s) = get_daily_prize_pool_snapshot().await {
                    pool.set(Some(s));
                }
                match list_recent_vouchers(10).await {
                    Ok(list) => {
                        let rows: Vec<(String, String)> = list
                            .into_iter()
                            .map(|v| {
                                (
                                    v.display_name.clone(),
                                    ticker_relative_time(loc, &v.created_at),
                                )
                            })
                            .collect();
                        winners.set(rows);
                    }
                    Err(_) => {}
                }
                if let Ok(snapshot) = get_parking_snapshot().await {
                    let occupied_total: u32 = snapshot.zones.iter().map(|z| z.occupied).sum();
                    let capacity_total: u32 = snapshot.zones.iter().map(|z| z.capacity).sum();
                    let ev_occupied_total: u32 = snapshot.zones.iter().map(|z| z.ev_occupied).sum();
                    let ev_capacity_total: u32 = snapshot.zones.iter().map(|z| z.ev_capacity).sum();
                    parking_stats.set(Some((
                        occupied_total,
                        capacity_total,
                        ev_occupied_total,
                        ev_capacity_total,
                    )));
                }
                delay_ms(15_000).await;
            }
        });
    });

    use_effect(move || {
        spawn(async move {
            loop {
                reset_hms.set(format_daily_prize_reset_countdown_hms());
                let cooldown_next = pool().and_then(|s| {
                    s.cooldown_until_utc
                        .as_deref()
                        .and_then(format_remaining_to_timestamp_hms)
                });
                cooldown_hms.set(cooldown_next);
                delay_ms(1_000).await;
            }
        });
    });

    let items = winners();
    let (current_str, max_str, cooldown_active) = match pool() {
        Some(s) => {
            let displayed_current = if s.cooldown_active { s.max } else { s.distributed };
            (
                displayed_current.to_string(),
                s.max.to_string(),
                s.cooldown_active,
            )
        }
        None => ("—".to_string(), "10".to_string(), false),
    };
    let right_hms = cooldown_hms().unwrap_or_else(|| reset_hms());
    let (global_available, ev_available) = match parking_stats() {
        Some((occupied_total, capacity_total, ev_occupied_total, ev_capacity_total)) => {
            (
                capacity_total.saturating_sub(occupied_total),
                ev_capacity_total.saturating_sub(ev_occupied_total),
            )
        }
        None => (0, 0),
    };

    rsx! {
        div {
            class: "sticky z-10 w-full h-16 max-h-16 overflow-hidden bg-dark text-white font-heading shadow-sm",
            style: "top: 4rem;",
            div {
                class: "max-w-7xl mx-auto px-6 h-full flex items-center justify-between gap-3",
                style: "min-height: 0;",
                div {
                    class: "shrink-0 flex flex-col justify-center gap-1",
                    style: "line-height: 1;",
                    span { class: "text-xs font-bold tracking-widest text-accent uppercase",
                        {translate(locale(), "home.ticker.prizes_label")}
                    }
                    span {
                        class: "text-sm font-black text-white",
                        style: "font-variant-numeric: tabular-nums;",
                        {translate_fmt(
                            locale(),
                            "home.ticker.prizes_ratio",
                            &[("current", current_str), ("max", max_str)],
                        )}
                    }
                }
                div {
                    class: "flex-1 overflow-hidden flex items-center",
                    style: "min-width: 0; min-height: 0;",
                    if items.is_empty() {
                        p { class: "text-xs text-gray-400 truncate w-full",
                            {translate(locale(), "home.ticker.empty")}
                        }
                    } else {
                        div {
                            class: "overflow-hidden w-full",
                            style: "min-height: 0;",
                            div { class: "ft-home-ticker-track",
                                for pass in 0..2u8 {
                                    for (i, (name, time)) in items.iter().enumerate() {
                                        div {
                                            key: "{pass}-{i}",
                                            class: "flex items-center gap-2 shrink-0",
                                            style: "white-space: nowrap;",
                                            p { class: "text-xs font-bold text-white", "{name}" }
                                            span { class: "text-xs text-gray-400 shrink-0", "{time}" }
                                            span { class: "text-xs text-gray-400",
                                                {translate(locale(), "home.ticker.sep")}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "shrink-0 flex items-center gap-4",
                    style: "margin-right: 0;",
                    div {
                        class: "flex flex-col justify-center items-end gap-1 text-right",
                        style: "line-height: 1; min-width: 9rem;",
                        span { class: "text-xs font-bold tracking-widest text-accent uppercase",
                            if cooldown_active {
                                {translate(locale(), "home.ticker.quota_full_label")}
                            } else {
                                {translate(locale(), "home.ticker.reset_in_label")}
                            }
                        }
                        p {
                            class: "mt-0 text-lg font-bold text-white",
                            style: "font-family: var(--font-mono), ui-monospace, monospace; font-variant-numeric: tabular-nums; line-height: 1.1; letter-spacing: -0.025em;",
                            "{right_hms}"
                        }
                    }
                    ParkingWidgetBar {}
                }
                div { class: "hidden md:flex h-16 max-h-16 shrink-0 items-center gap-2 rounded-lg border border-black bg-gray-900/70 px-2 py-1 self-center overflow-hidden",
                    img {
                        src: asset!("/assets/parking_icons/icons8-parking-100.png"),
                        alt: "Parking",
                        class: "h-4 w-4 shrink-0 object-contain",
                        style: "width: 60px; filter: brightness(0) invert(1);",
                    }
                    p { class: "text-[11px] font-semibold text-white whitespace-nowrap leading-4 flex flex-col justify-center items-start",
                        span { class: "text-[10px] font-bold tracking-widest uppercase text-accent mr-2", "Disponible" }
                        "{global_available} places · {ev_available} bornes"
                    }
                }
            }
            div { class: "md:hidden max-w-7xl mx-auto px-6 pb-2",
                div { class: "h-16 max-h-16 flex items-center gap-2 rounded-lg border border-black bg-gray-900/70 px-2 py-1 self-center overflow-hidden",
                    img {
                        src: asset!("/assets/parking_icons/icons8-parking-100.png"),
                        alt: "Parking",
                        class: "h-4 w-4 shrink-0 object-contain",
                        style: "width: 60px; filter: brightness(0) invert(1);",
                    }
                    p { class: "text-[11px] font-semibold text-white whitespace-nowrap leading-4 flex flex-col justify-center items-start",
                        span { class: "text-[10px] font-bold tracking-widest uppercase text-accent mr-2", "Disponible" }
                        "{global_available} places · {ev_available} bornes"
                    }
                }
            }
        }
    }
}

pub(crate) fn Home() -> Element {
    let locale = use_context::<Signal<Locale>>();

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::None }
            GamePromoModal {}

            LandingSection {}

            div { class: "mt-auto",
                // ─── Newsletter section ─────────────────────────────────
                section { class: "bg-dark",
                    div { class: "max-w-7xl mx-auto px-6 py-16 flex flex-col lg:flex-row items-center gap-12",
                        div { class: "flex-1",
                            h2 { class: "text-3xl md:text-4xl font-extrabold text-white leading-tight mb-4",
                                {translate(locale(), "home.newsletter.title")}
                            }
                            p { class: "text-muted leading-relaxed mb-8 max-w-md",
                                {translate(locale(), "home.newsletter.subtitle")}
                            }
                            div { class: "flex max-w-md",
                                input {
                                    class: "flex-1 py-3.5 px-4 text-sm bg-gray-800 border border-gray-700 rounded-l-lg placeholder-surface text-white focus:ring-accent focus:border-accent focus:outline-none",
                                    r#type: "email",
                                    placeholder: {translate(locale(), "home.newsletter.placeholder")},
                                }
                                button { class: "px-6 py-3.5 text-xs font-bold tracking-wider text-white bg-accent hover:bg-amber-600 rounded-r-lg transition-colors",
                                    {translate(locale(), "home.newsletter.button")}
                                }
                            }
                        }
                        // Decorative editorial image
                        div { class: "w-full lg:w-80 h-64 rounded-lg overflow-hidden",
                            img { src: "/editorial-fashion.png", class: "w-full h-full object-cover", alt: "" }
                        }
                    }
                }

                Footer { dark: false, stick_to_bottom: false }
            }
        }
    }
}

/// Logique localStorage (navigateur uniquement). Le serveur ne doit pas décider
/// d'afficher la modale à l'init : même état `false` partout évite les erreurs
/// d'hydratation (`hydrate_node` / attributs indéfinis).
fn wasm_game_promo_should_open_and_mark_seen() -> bool {
    const KEY: &str = "game_promo_last_seen_at_ms";
    const COOLDOWN_MS: f64 = 24.0 * 60.0 * 60.0 * 1000.0;

    #[cfg(target_family = "wasm")]
    {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let now = js_sys::Date::now();
            let should_show = match storage.get_item(KEY).ok().flatten() {
                None => true,
                Some(raw) => raw
                    .parse::<f64>()
                    .map(|last_seen| now - last_seen >= COOLDOWN_MS)
                    .unwrap_or(true),
            };
            if should_show {
                let _ = storage.set_item(KEY, &now.to_string());
            }
            return should_show;
        }
    }

    false
}

#[component]
fn GamePromoModal() -> Element {
    let mut is_open = use_signal(|| false);
    let nav = use_navigator();

    use_effect(move || {
        #[cfg(target_family = "wasm")]
        if wasm_game_promo_should_open_and_mark_seen() {
            defer_after_paint(move || is_open.set(true));
        }
    });

    rsx! {
        if is_open() {
            div {
                class: "fixed inset-0 flex items-center justify-center p-4 bg-black/55",
                style: "position: fixed; inset: 0; z-index: 9999;",
                onclick: move |_| {
                    defer_after_paint(move || is_open.set(false));
                },

                div {
                    class: "relative isolate w-full max-w-lg rounded-2xl bg-white border border-gray-200 shadow-2xl overflow-hidden",
                    onclick: move |e| e.stop_propagation(),

                    div { class: "py-6 px-6 md:p-8",
                        div { class: "mb-4 flex justify-center",
                            img {
                                src: asset!("/assets/fox_icon.svg"),
                                alt: "FoxTown game",
                                class: "h-24 w-24 ft-fox-pulse object-contain",
                            }
                        }

                        p { class: "text-xs font-bold tracking-[0.22em] uppercase text-accent text-center mb-2",
                            "New game"
                        }
                        p { class: "text-sm md:text-base text-body text-center mb-6",
                            "Play the FoxTown rewards game now and claim exclusive discounts in just a few clicks."
                        }

                        div { class: "flex flex-col sm:flex-row gap-3",
                            button {
                                r#type: "button",
                                class: "flex-1 py-3 px-4 rounded-lg bg-accent text-white font-bold tracking-wide hover:bg-amber-600 transition-colors",
                                onclick: move |_| {
                                    defer_after_paint(move || {
                                        is_open.set(false);
                                        nav.push(Route::Rewards {});
                                    });
                                },
                                "Play now"
                            }
                            button {
                                r#type: "button",
                                class: "flex-1 py-3 px-4 rounded-lg bg-gray-100 text-dark font-semibold hover:bg-gray-200 transition-colors",
                                onclick: move |_| {
                                    defer_after_paint(move || is_open.set(false));
                                },
                                "Maybe later"
                            }
                        }
                    }

                    button {
                        class: "pointer-events-auto absolute top-[12px] right-[12px] z-20 flex h-8 w-8 shrink-0 cursor-pointer items-center justify-center rounded-full border-0 bg-gray-100 p-0 text-base font-bold leading-none text-gray-600 shadow-sm hover:bg-gray-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-accent",
                        r#type: "button",
                        aria_label: "Close game promotion",
                        style: "margin: 0;",
                        onclick: move |e| {
                            e.stop_propagation();
                            defer_after_paint(move || is_open.set(false));
                        },
                        "×"
                    }
                }
            }
        }
    }
}

pub(crate) fn StoresPage() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut search = use_signal(String::new);
    let mut filter = use_signal(FilterGroup::default);
    let stores = use_loader(|| search_stores(String::new()))?;
    let mut queried_stores = use_signal(Vec::<Store>::new);
    let mut query_seq = use_signal(|| 0u64);

    let active_filter = filter();
    let source_rows: Vec<Store> = if search().trim().is_empty() {
        stores.iter().map(|s| (*s).clone()).collect()
    } else {
        queried_stores()
    };

    let filtered: Vec<Store> = source_rows
        .into_iter()
        .filter(|s| {
            let has_icon = s
                .icon_path
                .as_deref()
                .map(|p| !p.trim().is_empty())
                .unwrap_or(false);
            has_icon && active_filter.matches(&s.category)
        })
        .collect();

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::Stores }

            // ─── Hero section ───────────────────────────────────────
            section { class: "max-w-7xl mx-auto px-6 pt-16 pb-12",
                div { class: "max-w-2xl",
                    p { class: "text-sm font-semibold tracking-widest text-accent mb-4",
                        {translate(locale(), "home.directory")}
                    }
                    h1 { class: "text-4xl md:text-5xl font-extrabold text-dark leading-tight mb-6",
                        {translate(locale(), "home.title")}
                    }
                    p { class: "text-body leading-relaxed mb-8",
                        {translate(locale(), "home.subtitle")}
                    }
                }

                // Search bar
                div { class: "flex max-w-lg",
                    div { class: "flex-1 relative",
                        input {
                            class: "w-full py-3.5 pl-4 pr-12 text-sm border border-gray-200 rounded-l-lg placeholder-muted focus:ring-accent focus:border-accent focus:outline-none",
                            r#type: "text",
                            placeholder: {translate(locale(), "home.search_placeholder")},
                            value: "{search}",
                            oninput: move |e| {
                                let value = e.value();
                                search.set(value.clone());
                                let seq = query_seq() + 1;
                                query_seq.set(seq);
                                spawn(async move {
                                    let rows = search_stores(value).await.unwrap_or_default();
                                    if query_seq() == seq {
                                        queried_stores.set(rows);
                                    }
                                });
                            },
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
                                key: "{group.label_key()}",
                                class: if filter() == group {
                                    "shrink-0 px-5 py-2.5 text-xs font-bold tracking-wider rounded-full bg-dark text-white"
                                } else {
                                    "shrink-0 px-5 py-2.5 text-xs font-bold tracking-wider rounded-full text-body hover:bg-gray-100 transition-colors"
                                },
                                onclick: move |_| filter.set(group),
                                {translate(locale(), group.label_key())}
                            }
                        }
                    }
                }
            }

            // ─── Store grid ─────────────────────────────────────────
            section { class: "w-full",
                div { class: "max-w-7xl mx-auto px-6 py-12",
                if filtered.is_empty() {
                    div { class: "text-center py-20 text-muted font-semibold",
                        {translate(locale(), "home.empty")}
                    }
                } else {
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6",
                        for (idx, store) in filtered.into_iter().enumerate() {
                            Link {
                                key: "{idx}-{slugify(&store.name)}",
                                to: Route::Store { name: slugify(&store.name) },
                                class: "group block",

                                StoreCardImage {
                                    name: store.name.clone(),
                                    icon_path: store.icon_path.clone(),
                                }

                                // Info
                                h3 { class: "text-sm font-bold text-dark tracking-wide mb-1",
                                    "{store.name}"
                                }
                                p { class: "text-xs font-semibold text-accent tracking-wider mb-1",
                                    {translate(locale(), category_label(&store.category))}
                                }
                                if let Some(level) = store.level {
                                    p { class: "text-xs text-muted tracking-wider",
                                        if let Some(ref num) = store.store_number {
                                            {translate_fmt(locale(), "home.level_unit", &[("level", level.to_string()), ("unit", num.clone())])}
                                        } else {
                                            {translate_fmt(locale(), "home.level_only", &[("level", level.to_string())])}
                                        }
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
                            {translate(locale(), "home.newsletter.title")}
                        }
                        p { class: "text-muted leading-relaxed mb-8 max-w-md",
                            {translate(locale(), "home.newsletter.subtitle")}
                        }
                        div { class: "flex max-w-md",
                            input {
                                class: "flex-1 py-3.5 px-4 text-sm bg-gray-800 border border-gray-700 rounded-l-lg placeholder-surface text-white focus:ring-accent focus:border-accent focus:outline-none",
                                r#type: "email",
                                placeholder: {translate(locale(), "home.newsletter.placeholder")},
                            }
                            button { class: "px-6 py-3.5 text-xs font-bold tracking-wider text-white bg-accent hover:bg-amber-600 rounded-r-lg transition-colors",
                                {translate(locale(), "home.newsletter.button")}
                            }
                        }
                    }
                    // Decorative editorial image
                    div { class: "w-full lg:w-60 h-48 rounded-lg overflow-hidden",
                        img { src: "/editorial-fashion.png", class: "w-full h-full object-cover", alt: "" }
                    }
                }
            }

            Footer { dark: false, stick_to_bottom: false }
        }
    }
}
