use crate::auth::Role;
use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::auth::AuthState;
use crate::i18n::{translate, translate_fmt, Locale};
use crate::services::visits::{
    get_hourly_affluence, get_hourly_web_affluence, get_visit_stats, HourlyAffluence,
};
use crate::Route;
use dioxus::prelude::*;

fn format_hour(hour: u8) -> String {
    format!("{hour:02}h")
}

pub fn VisitsStatsPage() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();
    let nav = use_navigator();

    let mut loading = use_signal(|| true);
    let mut error = use_signal(String::new);
    let mut daily_visits = use_signal(|| 0_i64);
    let mut monthly_visits = use_signal(|| 0_i64);
    let mut yearly_visits = use_signal(|| 0_i64);
    let mut histogram = use_signal(Vec::<HourlyAffluence>::new);
    let mut web_histogram = use_signal(Vec::<HourlyAffluence>::new);

    use_effect(move || {
        if matches!(auth(), AuthState::LoggedOut) {
            let _ = nav.replace(Route::Login {});
            return;
        }
        if matches!(auth(), AuthState::LoggedIn(user) if user.role != Role::Admin) {
            let _ = nav.replace(Route::Rewards {});
        }
    });

    use_effect(move || {
        let is_admin = matches!(auth(), AuthState::LoggedIn(user) if user.role == Role::Admin);
        if !is_admin || !loading() {
            return;
        }
        spawn(async move {
            error.set(String::new());
            let stats_result = get_visit_stats().await;
            let histogram_result = get_hourly_affluence().await;
            let web_histogram_result = get_hourly_web_affluence().await;

            match (stats_result, histogram_result, web_histogram_result) {
                (Ok(stats), Ok(hourly), Ok(web_hourly)) => {
                    daily_visits.set(stats.daily);
                    monthly_visits.set(stats.monthly);
                    yearly_visits.set(stats.yearly);
                    histogram.set(hourly);
                    web_histogram.set(web_hourly);
                }
                (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => {
                    error.set(format!("Impossible de charger les statistiques: {e}"));
                }
            }
            loading.set(false);
        });
    });

    let hist = histogram();
    let peak = hist.iter().map(|h| h.visits).max().unwrap_or(0);
    let peak_hour = hist
        .iter()
        .max_by_key(|h| h.visits)
        .map(|h| format_hour(h.hour))
        .unwrap_or_else(|| "--".to_string());
    let web_hist = web_histogram();
    let web_peak = web_hist.iter().map(|h| h.visits).max().unwrap_or(0);
    let web_peak_hour = web_hist
        .iter()
        .max_by_key(|h| h.visits)
        .map(|h| format_hour(h.hour))
        .unwrap_or_else(|| "--".to_string());

    rsx! {
        div { class: "min-h-screen flex flex-col bg-gray-50 font-heading",
            Nav { active: NavPage::Rewards }

            section { class: "max-w-5xl mx-auto w-full px-6 py-12 flex-1",
                div { class: "flex flex-col md:flex-row md:items-center md:justify-between gap-4 mb-8",
                    div {
                        p { class: "text-xs font-bold tracking-widest text-accent mb-2", {translate(locale(), "visits.admin.badge")} }
                        h1 { class: "text-3xl md:text-4xl font-extrabold text-dark", {translate(locale(), "visits.page.title")} }
                        p { class: "text-sm text-muted mt-2",
                            {translate(locale(), "visits.page.subtitle")}
                        }
                    }
                }

                if loading() {
                    p { class: "text-sm text-muted", {translate(locale(), "visits.page.loading")} }
                } else if !error().is_empty() {
                    p { class: "text-sm text-red-600", "{error()}" }
                } else {
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-10",
                        div { class: "rounded-2xl border border-gray-200 bg-gradient-to-br from-white to-gray-50 p-5 shadow-sm",
                            p { class: "text-[11px] uppercase tracking-[0.22em] text-gray-500 text-center mb-2", {translate(locale(), "visits.card.daily")} }
                            p { class: "text-4xl md:text-[2.4rem] leading-none font-extrabold text-dark text-center", "{daily_visits()}" }
                        }
                        div { class: "rounded-2xl border border-gray-200 bg-gradient-to-br from-white to-gray-50 p-5 shadow-sm",
                            p { class: "text-[11px] uppercase tracking-[0.22em] text-gray-500 text-center mb-2", {translate(locale(), "visits.card.monthly")} }
                            p { class: "text-4xl md:text-[2.4rem] leading-none font-extrabold text-dark text-center", "{monthly_visits()}" }
                        }
                        div { class: "rounded-2xl border border-gray-200 bg-gradient-to-br from-white to-gray-50 p-5 shadow-sm",
                            p { class: "text-[11px] uppercase tracking-[0.22em] text-gray-500 text-center mb-2", {translate(locale(), "visits.card.yearly")} }
                            p { class: "text-4xl md:text-[2.4rem] leading-none font-extrabold text-dark text-center", "{yearly_visits()}" }
                        }
                    }

                    div { class: "rounded-2xl border border-gray-200 bg-white p-6 shadow-sm",
                        div { class: "flex flex-col md:flex-row md:items-center md:justify-between gap-2 mb-5",
                            h2 { class: "text-xl font-extrabold text-dark", {translate(locale(), "visits.histogram.title")} }
                            p { class: "text-xs tracking-wider text-gray-500 uppercase",
                                {translate_fmt(locale(), "visits.histogram.peak", &[("hour", peak_hour)])}
                            }
                        }
                        p { class: "text-sm text-gray-600 mb-4",
                            {translate(locale(), "visits.histogram.opening_hours")}
                        }

                        div { class: "h-72 border border-gray-100 rounded-xl p-4 bg-gray-50",
                            div { class: "h-full flex items-end gap-1.5",
                                for bucket in hist.iter() {
                                    {
                                        let height_pct = if peak <= 0 {
                                            4.0_f32
                                        } else {
                                            ((bucket.visits as f32 / peak as f32) * 100.0).max(4.0)
                                        };
                                        rsx! {
                                            div {
                                                class: "flex-1 flex flex-col items-center justify-end min-w-0",
                                                title: {
                                                    translate_fmt(
                                                        locale(),
                                                        "visits.histogram.tooltip",
                                                        &[("hour", format_hour(bucket.hour)), ("count", bucket.visits.to_string())],
                                                    )
                                                },
                                                div {
                                                    class: "w-full rounded-t bg-accent/80 hover:bg-accent transition-colors",
                                                    style: "height: {height_pct}%; min-height: 6px;",
                                                }
                                                span { class: "mt-2 text-[10px] text-gray-500", "{format_hour(bucket.hour)}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "rounded-2xl border border-gray-200 bg-white p-6 shadow-sm mt-8",
                        div { class: "flex flex-col md:flex-row md:items-center md:justify-between gap-2 mb-5",
                            h2 { class: "text-xl font-extrabold text-dark", {translate(locale(), "visits.web_histogram.title")} }
                            p { class: "text-xs tracking-wider text-gray-500 uppercase",
                                {translate_fmt(locale(), "visits.web_histogram.peak", &[("hour", web_peak_hour)])}
                            }
                        }
                        p { class: "text-sm text-gray-600 mb-4",
                            {translate(locale(), "visits.web_histogram.subtitle")}
                        }

                        div { class: "h-72 border border-gray-100 rounded-xl p-4 bg-gray-50 overflow-x-auto",
                            div { class: "h-full flex items-end gap-1.5 min-w-[980px]",
                                for bucket in web_hist.iter() {
                                    {
                                        let height_pct = if web_peak <= 0 {
                                            4.0_f32
                                        } else {
                                            ((bucket.visits as f32 / web_peak as f32) * 100.0).max(4.0)
                                        };
                                        rsx! {
                                            div {
                                                class: "w-9 flex-shrink-0 flex flex-col items-center justify-end",
                                                title: {
                                                    translate_fmt(
                                                        locale(),
                                                        "visits.histogram.tooltip",
                                                        &[("hour", format_hour(bucket.hour)), ("count", bucket.visits.to_string())],
                                                    )
                                                },
                                                div {
                                                    class: "w-full rounded-t bg-dark/80 hover:bg-dark transition-colors",
                                                    style: "height: {height_pct}%; min-height: 6px;",
                                                }
                                                span { class: "mt-2 text-[10px] text-gray-500", "{format_hour(bucket.hour)}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Footer { dark: true }
        }
    }
}
