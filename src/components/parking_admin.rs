use crate::auth::Role;
use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::auth::AuthState;
use crate::i18n::{translate, translate_fmt, Locale};
use crate::services::game::delay_ms;
use crate::services::parking::{get_parking_snapshot, refresh_parking, ParkingSnapshot, ParkingZoneStatus};
use crate::Route;
use dioxus::prelude::*;

fn fill_percent(occupied: u32, capacity: u32) -> u32 {
    if capacity == 0 {
        return 0;
    }
    ((occupied as f32 / capacity as f32) * 100.0).round() as u32
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
                    p { class: "text-xs text-gray-500 mt-1",
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
                        p { class: "text-xs text-gray-500 mt-1",
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

pub fn ParkingAdminPage() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();
    let nav = use_navigator();
    let mut snapshot = use_signal(|| None::<ParkingSnapshot>);

    use_effect(move || {
        if matches!(auth(), AuthState::LoggedOut) {
            let _ = nav.replace(Route::Login {});
            return;
        }
        if matches!(auth(), AuthState::LoggedIn(user) if user.role != Role::Admin) {
            let _ = nav.replace(Route::Home {});
        }
    });

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

    let zones = snapshot().map(|s| s.zones).unwrap_or_default();

    rsx! {
        div { class: "min-h-screen bg-gray-50 font-heading",
            Nav { active: NavPage::None }
            main { class: "max-w-7xl mx-auto px-6 py-10",
                section { class: "mt-2",
                    div { class: "flex items-center justify-between mb-4",
                        h2 { class: "text-2xl font-extrabold text-dark", {translate(locale(), "parking.admin.title")} }
                        div { class: "flex items-center gap-3",
                            span { class: "text-xs uppercase tracking-widest text-accent font-bold",
                                {translate(locale(), "parking.admin.badge")}
                            }
                            button {
                                onclick: move |_| {
                                    spawn(async move {
                                        if let Ok(snap) = refresh_parking().await {
                                            snapshot.set(Some(snap));
                                        }
                                    });
                                },
                                class: "px-3 py-1 bg-accent hover:bg-accent-dark text-white font-semibold rounded text-sm transition",
                                "🔄 Refresh"
                            }
                        }
                    }
                    p { class: "text-sm text-gray-600 mb-6",
                        {translate(locale(), "parking.admin.subtitle")}
                    }
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-5",
                        for zone in zones {
                            ZoneGauge { zone }
                        }
                    }
                }
            }
            Footer {}
        }
    }
}
