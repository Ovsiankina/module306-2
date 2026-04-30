use crate::auth::Role;
use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::auth::{read_token, AuthState};
use crate::i18n::{translate, translate_fmt, Locale};
use crate::services::vouchers::{list_all_vouchers_admin, purge_redeemed_vouchers, VoucherAdminFull};
use crate::Route;
use dioxus::prelude::*;

/// Affiche une date RFC3339 comme `2026-04-26 18:09:14` (sans fuseau ni fractions de seconde).
fn format_created_at_display(iso: &str) -> String {
    chrono::DateTime::parse_from_rfc3339(iso.trim())
        .ok()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| iso.to_string())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortColumn {
    Id,
    Email,
    Username,
    FirstName,
    LastName,
    Store,
    Discount,
    ValidUntil,
    CreatedAt,
    Redeemed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortDir {
    Asc,
    Desc,
}

fn sort_vouchers(items: &[VoucherAdminFull], col: SortColumn, dir: SortDir) -> Vec<VoucherAdminFull> {
    let mut v: Vec<VoucherAdminFull> = items.to_vec();
    v.sort_by(|a, b| {
        let ord = match col {
            SortColumn::Id => a.id.cmp(&b.id),
            SortColumn::Email => a
                .email
                .to_lowercase()
                .cmp(&b.email.to_lowercase()),
            SortColumn::Username => a
                .username
                .to_lowercase()
                .cmp(&b.username.to_lowercase()),
            SortColumn::FirstName => a
                .first_name
                .to_lowercase()
                .cmp(&b.first_name.to_lowercase()),
            SortColumn::LastName => a
                .last_name
                .to_lowercase()
                .cmp(&b.last_name.to_lowercase()),
            SortColumn::Store => a.store.to_lowercase().cmp(&b.store.to_lowercase()),
            SortColumn::Discount => a.discount.cmp(&b.discount),
            SortColumn::ValidUntil => a.valid_until.cmp(&b.valid_until),
            SortColumn::CreatedAt => {
                let pa = chrono::DateTime::parse_from_rfc3339(a.created_at.trim()).ok();
                let pb = chrono::DateTime::parse_from_rfc3339(b.created_at.trim()).ok();
                match (pa, pb) {
                    (Some(x), Some(y)) => x.cmp(&y),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.created_at.cmp(&b.created_at),
                }
            }
            SortColumn::Redeemed => a.redeemed.cmp(&b.redeemed),
        };
        match dir {
            SortDir::Asc => ord,
            SortDir::Desc => ord.reverse(),
        }
    });
    v
}

fn sort_arrow(col: SortColumn, active: SortColumn, dir: SortDir) -> &'static str {
    if col != active {
        return "";
    }
    match dir {
        SortDir::Asc => " \u{2191}",
        SortDir::Desc => " \u{2193}",
    }
}

pub fn VoucherListPage() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();
    let nav = use_navigator();

    let mut vouchers = use_signal(Vec::<VoucherAdminFull>::new);
    let mut loading = use_signal(|| false);
    let mut refresh_tick = use_signal(|| 0_u32);
    let mut error = use_signal(String::new);
    let mut purge_busy = use_signal(|| false);
    let mut purge_message = use_signal(String::new);
    let mut sort_column = use_signal(|| SortColumn::CreatedAt);
    let mut sort_dir = use_signal(|| SortDir::Desc);

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
        if !is_admin {
            return;
        }
        let _ = refresh_tick();
        spawn(async move {
            let Some(token) = read_token() else {
                error.set("Cannot load vouchers: missing auth token.".to_string());
                return;
            };
            loading.set(true);
            error.set(String::new());
            match list_all_vouchers_admin(token).await {
                Ok(items) => vouchers.set(items),
                Err(err) => error.set(format!("Cannot load vouchers: {err}")),
            }
            loading.set(false);
        });
    });

    let redeemed_count = vouchers().iter().filter(|v| v.redeemed).count();
    let displayed = sort_vouchers(&vouchers(), sort_column(), sort_dir());

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::Rewards }

            section { class: "max-w-7xl mx-auto w-full px-6 py-16 flex-1",
                div { class: "mb-8",
                    div {
                        p { class: "text-xs font-bold tracking-widest text-accent mb-2",
                            {translate(locale(), "visits.admin.badge")}
                        }
                        h1 { class: "text-3xl md:text-4xl font-extrabold text-dark",
                            {translate(locale(), "voucher_list.title")}
                        }
                        p { class: "text-sm text-muted mt-2",
                            {translate(locale(), "voucher_list.subtitle")}
                        }
                    }
                }

                if !purge_message().is_empty() {
                    p { class: "text-sm text-gray-700 mb-4 px-3 py-2 rounded-lg bg-slate-50 border border-slate-200",
                        "{purge_message()}"
                    }
                }

                if loading() {
                    p { class: "text-sm text-muted", {translate(locale(), "common.loading")} }
                } else if !error().is_empty() {
                    p { class: "text-sm text-red-600", "{error()}" }
                } else if vouchers().is_empty() {
                    p { class: "text-sm text-muted", {translate(locale(), "voucher_list.empty")} }
                } else {
                    div { class: "overflow-x-auto bg-white border border-gray-100 rounded-xl shadow-sm",
                        table { class: "min-w-[960px] w-full text-sm",
                            thead { class: "bg-gray-50",
                                tr {
                                    th { class: "px-3 py-2", colspan: "9" }
                                    th { class: "px-3 py-2",
                                        div { class: "flex flex-col items-end gap-2",
                                            if redeemed_count > 0 {
                                                p { class: "text-xs text-gray-500 whitespace-nowrap",
                                                    {translate_fmt(
                                                        locale(),
                                                        "voucher_list.redeemed_count",
                                                        &[("n", redeemed_count.to_string())],
                                                    )}
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: if !(purge_busy() || loading() || redeemed_count == 0) {
                                                    "px-4 py-2 text-xs font-bold tracking-wider rounded-lg bg-accent text-white hover:bg-amber-600 transition-colors whitespace-nowrap"
                                                } else {
                                                    "px-4 py-2 text-xs font-bold tracking-wider rounded-lg bg-gray-300 text-white cursor-not-allowed transition-colors whitespace-nowrap"
                                                },
                                                disabled: purge_busy() || loading() || redeemed_count == 0,
                                                onclick: move |_| {
                                                    let loc = locale();
                                                    purge_busy.set(true);
                                                    purge_message.set(String::new());
                                                    spawn(async move {
                                                        let Some(token) = read_token() else {
                                                            purge_message.set(translate(loc, "voucher_list.purge_error_auth"));
                                                            purge_busy.set(false);
                                                            return;
                                                        };
                                                        match purge_redeemed_vouchers(token).await {
                                                            Ok(n) => {
                                                                if n == 0 {
                                                                    purge_message.set(translate(loc, "voucher_list.purge_none"));
                                                                } else {
                                                                    purge_message.set(translate_fmt(
                                                                        loc,
                                                                        "voucher_list.purge_done",
                                                                        &[("n", n.to_string())],
                                                                    ));
                                                                }
                                                                refresh_tick.set(refresh_tick() + 1);
                                                            }
                                                            Err(e) => {
                                                                purge_message.set(format!(
                                                                    "{} {}",
                                                                    translate(loc, "voucher_list.purge_error"),
                                                                    e
                                                                ));
                                                            }
                                                        }
                                                        purge_busy.set(false);
                                                    });
                                                },
                                                if purge_busy() {
                                                    {translate(locale(), "voucher_list.purge_working")}
                                                } else {
                                                    {translate(locale(), "voucher_list.purge_button")}
                                                }
                                            }
                                        }
                                    }
                                }
                                tr {
                                    th { class: "text-left px-3 py-3 text-xs text-dark whitespace-nowrap",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::Id {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::Id);
                                                    sort_dir.set(SortDir::Asc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.id") }
                                                { sort_arrow(SortColumn::Id, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                    th { class: "text-left px-3 py-3 text-xs text-dark",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::Email {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::Email);
                                                    sort_dir.set(SortDir::Asc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.email") }
                                                { sort_arrow(SortColumn::Email, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                    th { class: "text-left px-3 py-3 text-xs text-dark",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::Username {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::Username);
                                                    sort_dir.set(SortDir::Asc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.username") }
                                                { sort_arrow(SortColumn::Username, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                    th { class: "text-left px-3 py-3 text-xs text-dark",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::FirstName {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::FirstName);
                                                    sort_dir.set(SortDir::Asc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.first_name") }
                                                { sort_arrow(SortColumn::FirstName, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                    th { class: "text-left px-3 py-3 text-xs text-dark",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::LastName {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::LastName);
                                                    sort_dir.set(SortDir::Asc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.last_name") }
                                                { sort_arrow(SortColumn::LastName, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                    th { class: "text-left px-3 py-3 text-xs text-dark",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::Store {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::Store);
                                                    sort_dir.set(SortDir::Asc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.store") }
                                                { sort_arrow(SortColumn::Store, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                    th { class: "text-left px-3 py-3 text-xs text-dark whitespace-nowrap",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::Discount {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::Discount);
                                                    sort_dir.set(SortDir::Asc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.discount") }
                                                { sort_arrow(SortColumn::Discount, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                    th { class: "text-left px-3 py-3 text-xs text-dark whitespace-nowrap",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::ValidUntil {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::ValidUntil);
                                                    sort_dir.set(SortDir::Asc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.valid_until") }
                                                { sort_arrow(SortColumn::ValidUntil, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                    th { class: "text-left px-3 py-3 text-xs text-dark min-w-[140px]",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::CreatedAt {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::CreatedAt);
                                                    sort_dir.set(SortDir::Desc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.created_at") }
                                                { sort_arrow(SortColumn::CreatedAt, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                    th { class: "text-left px-3 py-3 text-xs text-dark whitespace-nowrap",
                                        button {
                                            r#type: "button",
                                            class: "font-bold tracking-wider text-left w-full flex items-center gap-1 hover:text-accent transition-colors",
                                            title: translate(locale(), "voucher_list.sort_hint"),
                                            onclick: move |_| {
                                                if sort_column() == SortColumn::Redeemed {
                                                    sort_dir.set(match sort_dir() {
                                                        SortDir::Asc => SortDir::Desc,
                                                        SortDir::Desc => SortDir::Asc,
                                                    });
                                                } else {
                                                    sort_column.set(SortColumn::Redeemed);
                                                    sort_dir.set(SortDir::Asc);
                                                }
                                            },
                                            span {
                                                { translate(locale(), "voucher_list.col.redeemed") }
                                                { sort_arrow(SortColumn::Redeemed, sort_column(), sort_dir()) }
                                            }
                                        }
                                    }
                                }
                            }
                            tbody {
                                for voucher in displayed {
                                    tr { class: "border-t border-gray-100 hover:bg-gray-50/70",
                                        td { class: "px-3 py-3 text-dark tabular-nums", "{voucher.id}" }
                                        td { class: "px-3 py-3 text-dark break-all", "{voucher.email}" }
                                        td { class: "px-3 py-3 text-dark font-medium", "{voucher.username}" }
                                        td { class: "px-3 py-3 text-dark", "{voucher.first_name}" }
                                        td { class: "px-3 py-3 text-dark", "{voucher.last_name}" }
                                        td { class: "px-3 py-3 text-dark", "{voucher.store}" }
                                        td { class: "px-3 py-3",
                                            span { class: "inline-flex px-2 py-1 text-xs font-semibold rounded bg-accent/10 text-accent",
                                                "-{voucher.discount}%"
                                            }
                                        }
                                        td { class: "px-3 py-3 text-dark whitespace-nowrap", "{voucher.valid_until}" }
                                        td { class: "px-3 py-3 text-xs text-gray-600 tabular-nums whitespace-nowrap",
                                            {format_created_at_display(&voucher.created_at)}
                                        }
                                        td { class: "px-3 py-3",
                                            if voucher.redeemed {
                                                span { class: "inline-flex px-2 py-1 text-xs font-semibold rounded bg-gray-200 text-gray-700",
                                                    {translate(locale(), "voucher_list.redeemed_yes")}
                                                }
                                            } else {
                                                span { class: "inline-flex px-2 py-1 text-xs font-semibold rounded bg-emerald-100 text-emerald-800",
                                                    {translate(locale(), "voucher_list.redeemed_no")}
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

            Footer { dark: true }
        }
    }
}
