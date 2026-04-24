use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::i18n::{Locale, translate, translate_fmt};
use crate::services::game::delay_ms;
use crate::services::parking::{get_parking_snapshot, ParkingSnapshot, ParkingZoneStatus};
use crate::services::visits::{get_today_physical_recommendation, register_visit, VisitRecommendation};
use dioxus::prelude::*;

fn fill_percent(occupied: u32, capacity: u32) -> u32 {
    if capacity == 0 {
        return 0;
    }
    ((occupied as f32 / capacity as f32) * 100.0).round() as u32
}

fn visitor_session_id() -> String {
    #[cfg(target_family = "wasm")]
    {
        const KEY: &str = "visitor_session_id";
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            if let Ok(Some(existing)) = storage.get_item(KEY) {
                if !existing.is_empty() {
                    return existing;
                }
            }
            let generated = js_sys::Date::now().to_string();
            let _ = storage.set_item(KEY, &generated);
            return generated;
        }
    }
    chrono::Utc::now().timestamp_millis().to_string()
}

#[component]
fn RingGauge(percent: u32, size: u32) -> Element {
    let pct = percent.min(100);
    // The SVG viewBox is fixed to 100x100 with r=42, so math must use the same radius.
    let stroke_width = 10;
    let radius = 42.0_f32;
    let circumference = 2.0 * std::f32::consts::PI * radius;
    let dash_offset = circumference * (1.0 - pct as f32 / 100.0);
    let ring_color = if pct >= 90 {
        "text-red-500"
    } else if pct >= 70 {
        "text-amber-500"
    } else {
        "text-emerald-500"
    };

    rsx! {
        div {
            class: "relative flex items-center justify-center",
            style: "width: {size}px; height: {size}px;",
            svg {
                view_box: "0 0 100 100",
                class: "w-full h-full -rotate-90",
                circle {
                    cx: "50",
                    cy: "50",
                    r: "42",
                    fill: "none",
                    stroke: "rgb(229 231 235)",
                    stroke_width: "{stroke_width}",
                }
                circle {
                    cx: "50",
                    cy: "50",
                    r: "42",
                    fill: "none",
                    stroke: "currentColor",
                    class: "{ring_color}",
                    stroke_width: "{stroke_width}",
                    stroke_linecap: "round",
                    stroke_dasharray: "{circumference}",
                    stroke_dashoffset: "{dash_offset}",
                    style: "transition: stroke-dashoffset 250ms ease;",
                }
            }
            span { class: "absolute text-sm font-black text-dark", "{pct}%" }
        }
    }
}

#[component]
fn SpeedGauge(percent: u32, width: u32, height: u32) -> Element {
    let pct = percent.min(100);
    let needle_angle = 180.0_f32 - (pct as f32 * 180.0 / 100.0);
    let needle_len = 33.0_f32;
    let needle_rad = needle_angle.to_radians();
    let needle_x = 60.0 + needle_len * needle_rad.cos();
    let needle_y = 60.0 - needle_len * needle_rad.sin();
    let mut ticks = Vec::new();
    for i in 0..=20 {
        let a = 180.0_f32 - (i as f32 * 9.0);
        let rad = a.to_radians();
        let outer = 49.0_f32;
        let inner = if i % 5 == 0 { 42.0_f32 } else { 45.0_f32 };
        let x1 = 60.0 + outer * rad.cos();
        let y1 = 60.0 - outer * rad.sin();
        let x2 = 60.0 + inner * rad.cos();
        let y2 = 60.0 - inner * rad.sin();
        ticks.push((x1, y1, x2, y2));
    }

    rsx! {
        div {
            class: "relative flex items-center justify-center",
            style: "width: {width}px; height: {height}px;",
            svg {
                view_box: "0 0 120 70",
                class: "w-full h-full",
                path {
                    d: "M 10 60 A 50 50 0 0 1 110 60",
                    fill: "none",
                    stroke: "rgb(82 82 91)",
                    stroke_width: "18",
                }
                path {
                    d: "M 10 60 A 50 50 0 0 1 110 60",
                    fill: "none",
                    stroke: "rgb(228 228 231)",
                    stroke_width: "14",
                    stroke_linecap: "butt",
                }
                path {
                    d: "M 18 60 A 42 42 0 0 1 38 23",
                    fill: "none",
                    stroke: "rgb(101 163 13)",
                    stroke_width: "14",
                    stroke_linecap: "butt",
                }
                path {
                    d: "M 38 23 A 42 42 0 0 1 82 23",
                    stroke: "rgb(217 119 6)",
                    stroke_width: "14",
                    fill: "none",
                }
                path {
                    d: "M 82 23 A 42 42 0 0 1 102 60",
                    fill: "none",
                    stroke: "rgb(185 28 28)",
                    stroke_width: "14",
                    stroke_linecap: "butt",
                }
                for (x1, y1, x2, y2) in ticks {
                    line {
                        x1: "{x1}",
                        y1: "{y1}",
                        x2: "{x2}",
                        y2: "{y2}",
                        stroke: "rgb(31 41 55)",
                        stroke_width: "1.8",
                        stroke_linecap: "round",
                    }
                }
                line {
                    x1: "60",
                    y1: "60",
                    x2: "{needle_x}",
                    y2: "{needle_y}",
                    stroke: "rgb(17 24 39)",
                    stroke_width: "3",
                    stroke_linecap: "round",
                }
                circle {
                    cx: "60",
                    cy: "60",
                    r: "6.5",
                    fill: "rgb(17 24 39)",
                }
                circle {
                    cx: "60",
                    cy: "60",
                    r: "2",
                    fill: "rgb(161 161 170)",
                }
            }
        }
    }
}

#[component]
fn GaugeCard(title: String, occupied: u32, capacity: u32, subtitle: String) -> Element {
    let locale = use_context::<Signal<Locale>>();
    let percent = fill_percent(occupied, capacity);
    let available = capacity.saturating_sub(occupied);
    rsx! {
        div { class: "rounded-2xl border border-gray-200 bg-white p-6 shadow-sm",
            div { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
                div { class: "min-w-0 md:flex-1",
                    p { class: "text-sm font-semibold tracking-widest text-gray-500 uppercase mb-2", "{title}" }
                    p { class: "text-sm text-gray-600 mt-1",
                        {translate_fmt(
                            locale(),
                            "parking.card.occupied_count",
                            &[("occupied", occupied.to_string()), ("capacity", capacity.to_string())],
                        )}
                    }
                    p { class: "text-sm text-gray-600",
                        {translate_fmt(
                            locale(),
                            "parking.card.available_count",
                            &[("count", available.to_string())],
                        )}
                    }
                    p { class: "text-xs text-gray-500 mt-3", "{subtitle}" }
                }
                div { class: "flex justify-center md:justify-end md:pl-4",
                    RingGauge { percent: percent, size: 140 }
                }
            }
        }
    }
}

#[component]
fn ZoneGauge(zone: ParkingZoneStatus) -> Element {
    let locale = use_context::<Signal<Locale>>();
    let fill = fill_percent(zone.occupied, zone.capacity);
    let ev_fill = fill_percent(zone.ev_occupied, zone.ev_capacity);
    rsx! {
        div { class: "rounded-2xl border border-gray-200 bg-white p-5 shadow-sm",
            div { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
                div { class: "min-w-0 md:flex-1",
                    h3 { class: "text-lg font-bold text-dark mb-4", "{zone.name}" }

                    p { class: "text-xs uppercase tracking-widest text-gray-500 mb-1", {translate(locale(), "parking.zone.standard_spaces")} }
                    p { class: "text-sm text-gray-700",
                        {translate_fmt(
                            locale(),
                            "parking.zone.occupied_percent",
                            &[("occupied", zone.occupied.to_string()), ("capacity", zone.capacity.to_string()), ("percent", fill.to_string())],
                        )}
                    }
                    p {
                        class: "text-xs text-gray-500 mt-1",
                        {translate_fmt(
                            locale(),
                            "parking.card.available_count",
                            &[("count", zone.capacity.saturating_sub(zone.occupied).to_string())],
                        )}
                    }
                }
                div { class: "flex justify-center md:justify-end md:pl-4",
                    SpeedGauge { percent: fill, width: 336, height: 156 }
                }
            }

            if zone.ev_capacity > 0 {
                div { class: "mt-2 flex flex-col gap-3 md:flex-row md:items-center md:justify-between",
                    div { class: "min-w-0 md:flex-1",
                        p { class: "text-xs uppercase tracking-widest text-gray-500 mb-1", {translate(locale(), "parking.zone.ev_stations")} }
                        p { class: "text-sm text-gray-700",
                            {translate_fmt(
                                locale(),
                                "parking.zone.occupied_percent",
                                &[("occupied", zone.ev_occupied.to_string()), ("capacity", zone.ev_capacity.to_string()), ("percent", ev_fill.to_string())],
                            )}
                        }
                        p {
                            class: "text-xs text-gray-500 mt-1",
                            {translate_fmt(
                                locale(),
                                "parking.zone.ev_available_count",
                                &[("count", zone.ev_capacity.saturating_sub(zone.ev_occupied).to_string())],
                            )}
                        }
                    }
                    div { class: "flex justify-center md:justify-end md:pl-4",
                        SpeedGauge { percent: ev_fill, width: 336, height: 156 }
                    }
                }
            }
        }
    }
}

pub fn ParkingPage() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut snapshot = use_signal(|| None::<ParkingSnapshot>);
    let mut recommendation = use_signal(|| None::<VisitRecommendation>);

    use_effect(move || {
        spawn(async move {
            loop {
                if let Ok(s) = get_parking_snapshot().await {
                    snapshot.set(Some(s));
                }
                delay_ms(10_000).await;
            }
        });
    });

    use_effect(move || {
        spawn(async move {
            let _ = register_visit("/parking".to_string(), visitor_session_id()).await;
        });
    });

    use_effect(move || {
        spawn(async move {
            if let Ok(rec) = get_today_physical_recommendation().await {
                recommendation.set(Some(rec));
            }
        });
    });

    let zones = snapshot().map(|s| s.zones).unwrap_or_default();
    let occupied_total: u32 = zones.iter().map(|z| z.occupied).sum();
    let capacity_total: u32 = zones.iter().map(|z| z.capacity).sum();
    let ev_occupied_total: u32 = zones.iter().map(|z| z.ev_occupied).sum();
    let ev_capacity_total: u32 = zones.iter().map(|z| z.ev_capacity).sum();

    rsx! {
        div { class: "min-h-screen bg-gray-50 font-heading",
            Nav { active: NavPage::Parking }

            main { class: "max-w-7xl mx-auto px-6 py-10",
                h1 { class: "text-4xl md:text-5xl font-black text-dark mb-3",
                    {translate(locale(), "parking.title")}
                }
                p { class: "text-gray-600 mb-8 max-w-3xl",
                    {translate(locale(), "parking.subtitle")}
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    GaugeCard {
                        title: translate(locale(), "parking.card.global_occupancy"),
                        occupied: occupied_total,
                        capacity: capacity_total,
                        subtitle: translate(locale(), "parking.card.visitor_subtitle"),
                    }
                    GaugeCard {
                        title: translate(locale(), "parking.card.ev"),
                        occupied: ev_occupied_total,
                        capacity: ev_capacity_total,
                        subtitle: translate(locale(), "parking.card.ev_subtitle"),
                    }
                }

                if let Some(rec) = recommendation() {
                    div { class: "mt-8 rounded-2xl border border-gray-200 bg-white p-5 shadow-sm",
                        p { class: "text-xs uppercase tracking-widest text-gray-500 mb-2",
                            {translate(locale(), "parking.reco.title")}
                        }
                        p { class: "text-sm text-gray-700",
                            {translate_fmt(
                                locale(),
                                "parking.reco.best_slots",
                                &[("slots", rec.best_slots.join(", "))],
                            )}
                        }
                        p { class: "text-sm text-gray-700 mt-1",
                            {translate_fmt(
                                locale(),
                                "parking.reco.avoid_slots",
                                &[("slots", rec.avoid_slots.join(", "))],
                            )}
                        }
                    }
                }

            }

            Footer {}
        }
    }
}

