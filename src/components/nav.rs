use crate::auth::{logout, Role};
use crate::components::home::HomeWinnersTickerBar;
use crate::context::auth::{clear_token, read_token, AuthState};
use crate::i18n::{persist_locale, translate, Locale};
use crate::Route;
use dioxus::prelude::*;

const DESKTOP_ACTIVE_LANG_BTN_CLASS: &str = "text-xs font-bold tracking-wider text-accent";
const DESKTOP_INACTIVE_LANG_BTN_CLASS: &str =
    "text-xs font-bold tracking-wider text-nav hover:text-dark transition-colors";
const MOBILE_ACTIVE_LANG_BTN_CLASS: &str = "text-xs font-bold tracking-wider text-dark";
const MOBILE_INACTIVE_LANG_BTN_CLASS: &str =
    "text-xs font-bold tracking-wider text-nav hover:text-dark transition-colors";
const LANGUAGE_BUTTONS: &[(Locale, &str)] = &[
    (Locale::It, "IT"),
    (Locale::De, "DE"),
    (Locale::Fr, "FR"),
    (Locale::En, "EN"),
];

fn desktop_language_button_class(current: Locale, target: Locale) -> &'static str {
    if current == target {
        DESKTOP_ACTIVE_LANG_BTN_CLASS
    } else {
        DESKTOP_INACTIVE_LANG_BTN_CLASS
    }
}

fn mobile_language_button_class(current: Locale, target: Locale) -> &'static str {
    if current == target {
        MOBILE_ACTIVE_LANG_BTN_CLASS
    } else {
        MOBILE_INACTIVE_LANG_BTN_CLASS
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum NavPage {
    Home,
    Parking,
    Stores,
    Map,
    Rewards,
    #[default]
    None,
}

#[component]
pub fn Nav(#[props(default)] active: NavPage) -> Element {
    let mut locale = use_context::<Signal<Locale>>();
    let mut auth = use_context::<Signal<AuthState>>();
    let nav = use_navigator();
    let mut mobile_menu_open = use_signal(|| false);
    let mut account_menu_open = use_signal(|| false);
    let link_class = |page: NavPage| {
        if active == page {
            "text-sm font-semibold tracking-widest text-accent"
        } else {
            "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors"
        }
    };

    rsx! {
        nav { class: "sticky top-0 z-50 bg-white border-b border-gray-100 font-heading",
            div { class: "max-w-7xl mx-auto px-6 h-16 flex items-center justify-between",

                // Logo bitmap (/public/fox_town_logo.png); alt reprend la clé i18n nav.logo
                a { class: "relative z-10 shrink-0 flex items-center",
                    href: "/",
                    img {
                        src: "/fox_town_logo.png",
                        class: "h-8 w-auto shrink-0 object-contain object-left",
                        alt: translate(locale(), "nav.logo"),
                    }
                }

                // Center nav links (desktop)
                div { class: "hidden md:flex items-center gap-8",
                    a { class: link_class(NavPage::Home), href: "/", {translate(locale(), "common.home")} }
                    a { class: link_class(NavPage::Parking), href: "/parking", {translate(locale(), "nav.parking")} }
                    a { class: link_class(NavPage::Stores), href: "/stores", {translate(locale(), "nav.stores")} }
                    a { class: link_class(NavPage::Map), href: "/map", {translate(locale(), "nav.map")} }
                    a { class: link_class(NavPage::Rewards), href: "/rewards", {translate(locale(), "nav.rewards")} }
                }

                // Right actions
                div { class: "flex items-center gap-3",
                    div { class: "hidden md:flex items-center gap-2",
                        for (idx, &(target_locale, label)) in LANGUAGE_BUTTONS.iter().enumerate() {
                            button {
                                class: desktop_language_button_class(locale(), target_locale),
                                onclick: move |_| {
                                    locale.set(target_locale);
                                    persist_locale(target_locale);
                                },
                                "{label}"
                            }
                            if idx < LANGUAGE_BUTTONS.len() - 1 {
                                span { class: "text-xs text-muted", "|" }
                            }
                        }
                    }

                    // Search button
                    a {
                        class: "p-2 text-dark hover:text-accent transition-colors",
                        href: "/map",
                        title: "Find a store",
                        "aria-label": "Find a store",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "20",
                            height: "20",
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

                    // Account: menu profil (icône SVG = toujours visible si connecté, mobile inclus)
                    if matches!(auth(), AuthState::LoggedIn(_)) {
                        div { class: "relative z-20",
                            if account_menu_open() {
                                button {
                                    class: "fixed inset-x-0 top-16 bottom-0 z-40 cursor-default border-0 bg-transparent p-0",
                                    r#type: "button",
                                    "aria-label": "Close menu",
                                    onclick: move |_| account_menu_open.set(false),
                                }
                            }
                            div { class: "relative z-50",
                                button {
                                    class: "inline-flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-gray-200 bg-gray-100 p-0 text-dark shadow-sm hover:bg-gray-50 transition-colors",
                                    r#type: "button",
                                    title: translate(locale(), "nav.account"),
                                    "aria-label": translate(locale(), "nav.account"),
                                    "aria-expanded": "{account_menu_open()}",
                                    "aria-haspopup": "true",
                                    onclick: move |_| account_menu_open.set(!account_menu_open()),
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "22",
                                        height: "22",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                                        circle { cx: "12", cy: "7", r: "4" }
                                    }
                                }
                                if account_menu_open() {
                                    div { class: "absolute right-0 top-full mt-2 w-56 rounded-xl border border-gray-100 bg-white py-2 shadow-xl",
                                        if matches!(auth(), AuthState::LoggedIn(user) if user.role == Role::Admin) {
                                            a {
                                                class: "block px-4 py-2.5 text-xs font-bold tracking-widest text-dark hover:bg-gray-50",
                                                href: "/admin/vouchers",
                                                onclick: move |_| account_menu_open.set(false),
                                                {translate(locale(), "nav.admin.voucher_list")}
                                            }
                                            a {
                                                class: "block px-4 py-2.5 text-xs font-bold tracking-widest text-dark hover:bg-gray-50",
                                                href: "/admin/visits",
                                                onclick: move |_| account_menu_open.set(false),
                                                {translate(locale(), "nav.admin.visits_stats")}
                                            }
                                            a {
                                                class: "block px-4 py-2.5 text-xs font-bold tracking-widest text-dark hover:bg-gray-50",
                                                href: "/admin/parking-occupancy",
                                                onclick: move |_| account_menu_open.set(false),
                                                {translate(locale(), "nav.admin.parking_occupancy")}
                                            }
                                            a {
                                                class: "block px-4 py-2.5 text-xs font-bold tracking-widest text-dark hover:bg-gray-50",
                                                href: "/admin/game-rules",
                                                onclick: move |_| account_menu_open.set(false),
                                                {translate(locale(), "nav.admin.game_rules")}
                                            }
                                        }
                                        // Logout: all logged-in roles (Editor + Admin)
                                        button {
                                            class: "w-full border-0 border-t border-gray-100 bg-transparent px-4 py-2.5 text-left text-xs font-semibold tracking-wider text-red-700 hover:bg-red-50",
                                            r#type: "button",
                                            onclick: move |_| {
                                                account_menu_open.set(false);
                                                spawn(async move {
                                                    if let Some(t) = read_token() {
                                                        let _ = logout(t).await;
                                                    }
                                                    clear_token();
                                                    auth.set(AuthState::LoggedOut);
                                                    let _ = nav.replace(Route::Home {});
                                                });
                                            },
                                            {translate(locale(), "nav.logout")}
                                        }
                                    }
                                }
                            }
                        }
                    } else if matches!(auth(), AuthState::LoggedOut) {
                        a { class: "hidden md:inline-flex items-center px-5 py-2 text-xs font-bold tracking-widest text-white bg-dark rounded hover:bg-gray-700 transition-colors",
                            href: "/login",
                            {translate(locale(), "nav.login")}
                        }
                    } else {
                        span { class: "hidden md:inline-block w-10 h-10 rounded-full bg-gray-100 animate-pulse shrink-0",
                            "aria-hidden": "true"
                        }
                    }

                    // Mobile hamburger
                    button {
                        class: "md:hidden p-2 text-dark",
                        onclick: move |_| mobile_menu_open.set(true),
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "3", y1: "6", x2: "21", y2: "6" }
                            line { x1: "3", y1: "12", x2: "21", y2: "12" }
                            line { x1: "3", y1: "18", x2: "21", y2: "18" }
                        }
                    }
                }
            }

            if mobile_menu_open() {
                div {
                    class: "md:hidden fixed inset-0 z-50",
                    button {
                        class: "absolute inset-0 bg-black/40",
                        onclick: move |_| mobile_menu_open.set(false),
                    }

                    div { class: "absolute top-0 right-0 h-full w-72 shadow-2xl overflow-y-auto",
                        div { class: "min-h-full bg-accent p-6 flex flex-col",
                            div { class: "flex items-center justify-between mb-8",
                                p { class: "text-sm font-bold tracking-widest text-dark", {translate(locale(), "nav.logo")} }
                                button {
                                    class: "p-2 text-dark hover:text-accent transition-colors",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    "✕"
                                }
                            }

                            div { class: "flex flex-col gap-5",
                                a {
                                    class: if active == NavPage::Home { "text-sm font-semibold tracking-widest text-accent" } else { "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors" },
                                    href: "/",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "common.home")}
                                }
                                a {
                                    class: if active == NavPage::Parking { "text-sm font-semibold tracking-widest text-accent" } else { "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors" },
                                    href: "/parking",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "nav.parking")}
                                }
                                a {
                                    class: if active == NavPage::Stores { "text-sm font-semibold tracking-widest text-accent" } else { "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors" },
                                    href: "/stores",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "nav.stores")}
                                }
                                a {
                                    class: if active == NavPage::Map { "text-sm font-semibold tracking-widest text-accent" } else { "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors" },
                                    href: "/map",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "nav.map")}
                                }
                                a {
                                    class: if active == NavPage::Rewards { "text-sm font-semibold tracking-widest text-accent" } else { "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors" },
                                    href: "/rewards",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "nav.rewards")}
                                }
                                if matches!(auth(), AuthState::LoggedIn(user) if user.role == Role::Admin) {
                                    a {
                                        class: "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors",
                                        href: "/admin/vouchers",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        {translate(locale(), "nav.admin.voucher_list")}
                                    }
                                    a {
                                        class: "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors",
                                        href: "/admin/visits",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        {translate(locale(), "nav.admin.visits_stats")}
                                    }
                                    a {
                                        class: "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors",
                                        href: "/admin/parking-occupancy",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        {translate(locale(), "nav.admin.parking_occupancy")}
                                    }
                                    a {
                                        class: "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors",
                                        href: "/admin/game-rules",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        {translate(locale(), "nav.admin.game_rules")}
                                    }
                                }
                                if matches!(auth(), AuthState::LoggedIn(_)) {
                                    button {
                                        class: "text-left text-sm font-semibold tracking-widest text-red-700 hover:text-red-800 transition-colors",
                                        r#type: "button",
                                        onclick: move |_| {
                                            mobile_menu_open.set(false);
                                            spawn(async move {
                                                if let Some(t) = read_token() {
                                                    let _ = logout(t).await;
                                                }
                                                clear_token();
                                                auth.set(AuthState::LoggedOut);
                                                let _ = nav.replace(Route::Home {});
                                            });
                                        },
                                        {translate(locale(), "nav.logout")}
                                    }
                                } else {
                                    a {
                                        class: "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors",
                                        href: "/login",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        {translate(locale(), "nav.login")}
                                    }
                                }
                            }

                            div { class: "mt-auto pt-8 border-t border-gray-200",
                                p { class: "text-xs font-semibold tracking-wider text-muted mb-3", {translate(locale(), "nav.language")} }
                                div { class: "flex items-center gap-2",
                                    for (idx, &(target_locale, label)) in LANGUAGE_BUTTONS.iter().enumerate() {
                                        button {
                                            class: mobile_language_button_class(locale(), target_locale),
                                            onclick: move |_| {
                                                locale.set(target_locale);
                                                persist_locale(target_locale);
                                            },
                                            "{label}"
                                        }
                                        if idx < LANGUAGE_BUTTONS.len() - 1 {
                                            span { class: "text-xs text-muted", "|" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        HomeWinnersTickerBar {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_buttons_match_expected_order_and_locales() {
        assert_eq!(
            LANGUAGE_BUTTONS,
            &[
                (Locale::It, "IT"),
                (Locale::De, "DE"),
                (Locale::Fr, "FR"),
                (Locale::En, "EN"),
            ]
        );
    }

    #[test]
    fn desktop_language_button_class_highlights_only_active_locale() {
        for &(target, _) in LANGUAGE_BUTTONS {
            let class = desktop_language_button_class(Locale::Fr, target);
            let expected = if target == Locale::Fr {
                DESKTOP_ACTIVE_LANG_BTN_CLASS
            } else {
                DESKTOP_INACTIVE_LANG_BTN_CLASS
            };
            assert_eq!(class, expected);
        }
    }

    #[test]
    fn mobile_language_button_class_highlights_only_active_locale() {
        for &(target, _) in LANGUAGE_BUTTONS {
            let class = mobile_language_button_class(Locale::De, target);
            let expected = if target == Locale::De {
                MOBILE_ACTIVE_LANG_BTN_CLASS
            } else {
                MOBILE_INACTIVE_LANG_BTN_CLASS
            };
            assert_eq!(class, expected);
        }
    }
}
