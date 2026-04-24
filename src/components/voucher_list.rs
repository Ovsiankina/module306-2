use crate::auth::Role;
use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::auth::{read_token, AuthState};
use crate::i18n::{translate, Locale};
use crate::services::visits::get_visit_stats;
use crate::services::vouchers::{list_active_vouchers, VoucherAdminSummary};
use crate::Route;
use dioxus::prelude::*;

pub fn VoucherListPage() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();
    let nav = use_navigator();

    let mut vouchers = use_signal(Vec::<VoucherAdminSummary>::new);
    let mut loading = use_signal(|| false);
    let mut loaded = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut daily_visits = use_signal(|| 0_i64);
    let mut monthly_visits = use_signal(|| 0_i64);
    let mut yearly_visits = use_signal(|| 0_i64);

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
        if !is_admin || loaded() || loading() {
            return;
        }

        loaded.set(true);
        loading.set(true);
        error.set(String::new());

        spawn(async move {
            let Some(token) = read_token() else {
                loading.set(false);
                error.set("Cannot load vouchers: missing auth token.".to_string());
                return;
            };

            match list_active_vouchers(token).await {
                Ok(items) => vouchers.set(items),
                Err(err) => error.set(format!("Cannot load vouchers: {err}")),
            }
            if let Ok(stats) = get_visit_stats().await {
                daily_visits.set(stats.daily);
                monthly_visits.set(stats.monthly);
                yearly_visits.set(stats.yearly);
            }
            loading.set(false);
        });
    });

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::Rewards }

            section { class: "max-w-7xl mx-auto w-full px-6 py-16 flex-1",
                div { class: "flex flex-col md:flex-row md:items-center md:justify-between gap-4 mb-8",
                    div {
                        p { class: "text-xs font-bold tracking-widest text-accent mb-2", "ADMIN AREA" }
                        h1 { class: "text-3xl md:text-4xl font-extrabold text-dark", "Voucher List" }
                        p { class: "text-sm text-muted mt-2",
                            "All currently active vouchers issued by the rewards game."
                        }
                    }
                }

                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-8",
                    div { class: "rounded-xl border border-gray-200 bg-white p-4 shadow-sm",
                        p { class: "text-xs uppercase tracking-widest text-gray-500 mb-1", {translate(locale(), "visits.card.daily")} }
                        p { class: "text-3xl font-black text-dark", "{daily_visits()}" }
                    }
                    div { class: "rounded-xl border border-gray-200 bg-white p-4 shadow-sm",
                        p { class: "text-xs uppercase tracking-widest text-gray-500 mb-1", {translate(locale(), "visits.card.monthly")} }
                        p { class: "text-3xl font-black text-dark", "{monthly_visits()}" }
                    }
                    div { class: "rounded-xl border border-gray-200 bg-white p-4 shadow-sm",
                        p { class: "text-xs uppercase tracking-widest text-gray-500 mb-1", {translate(locale(), "visits.card.yearly")} }
                        p { class: "text-3xl font-black text-dark", "{yearly_visits()}" }
                    }
                }

                if loading() {
                    p { class: "text-sm text-muted", {translate(locale(), "common.loading")} }
                } else if !error().is_empty() {
                    p { class: "text-sm text-red-600", "{error()}" }
                } else if vouchers().is_empty() {
                    p { class: "text-sm text-muted", "No active vouchers for now." }
                } else {
                    div { class: "overflow-x-auto bg-white border border-gray-100 rounded-xl shadow-sm",
                        table { class: "min-w-full text-sm",
                            thead { class: "bg-gray-50",
                                tr {
                                    th { class: "text-left px-4 py-3 font-bold tracking-wider text-xs text-dark", "Username" }
                                    th { class: "text-left px-4 py-3 font-bold tracking-wider text-xs text-dark", "Store" }
                                    th { class: "text-left px-4 py-3 font-bold tracking-wider text-xs text-dark", "Discount" }
                                    th { class: "text-left px-4 py-3 font-bold tracking-wider text-xs text-dark", "Valid Until" }
                                }
                            }
                            tbody {
                                for voucher in vouchers() {
                                    tr { class: "border-t border-gray-100 hover:bg-gray-50/70",
                                        td { class: "px-4 py-3 text-dark font-medium", "{voucher.username}" }
                                        td { class: "px-4 py-3 text-dark", "{voucher.store}" }
                                        td { class: "px-4 py-3",
                                            span { class: "inline-flex px-2 py-1 text-xs font-semibold rounded bg-accent/10 text-accent",
                                                "-{voucher.discount}%"
                                            }
                                        }
                                        td { class: "px-4 py-3 text-dark", "{voucher.valid_until}" }
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
