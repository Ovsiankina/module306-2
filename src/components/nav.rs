use dioxus::prelude::*;
use crate::i18n::{translate, Locale};

const DESKTOP_ACTIVE_LANG_BTN_CLASS: &str = "text-xs font-bold tracking-wider text-accent";
const DESKTOP_INACTIVE_LANG_BTN_CLASS: &str =
    "text-xs font-bold tracking-wider text-nav hover:text-dark transition-colors";
const MOBILE_ACTIVE_LANG_BTN_CLASS: &str = "text-xs font-bold tracking-wider text-white";
const MOBILE_INACTIVE_LANG_BTN_CLASS: &str =
    "text-xs font-bold tracking-wider text-white/80 hover:text-white transition-colors";
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
    Stores,
    Map,
    Rewards,
    Visit,
    #[default]
    None,
}

#[component]
pub fn Nav(#[props(default)] active: NavPage) -> Element {
    let mut locale = use_context::<Signal<Locale>>();
    let mut mobile_menu_open = use_signal(|| false);
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

                // Logo
                a { class: "text-xl font-extrabold tracking-widest text-dark",
                    href: "/",
                    {translate(locale(), "nav.logo")}
                }

                // Center nav links (desktop)
                div { class: "hidden md:flex items-center gap-8",
                    a { class: link_class(NavPage::Stores), href: "/", {translate(locale(), "nav.stores")} }
                    a { class: link_class(NavPage::Map), href: "/map", {translate(locale(), "nav.map")} }
                    a { class: link_class(NavPage::Rewards), href: "/rewards", {translate(locale(), "nav.rewards")} }
                    a { class: link_class(NavPage::Visit), href: "/map", {translate(locale(), "nav.visit")} }
                }

                // Right actions
                div { class: "flex items-center gap-3",
                    div { class: "hidden md:flex items-center gap-2",
                        for (idx, &(target_locale, label)) in LANGUAGE_BUTTONS.iter().enumerate() {
                            button {
                                class: desktop_language_button_class(locale(), target_locale),
                                onclick: move |_| locale.set(target_locale),
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

                    // Login button
                    a { class: "hidden md:inline-flex items-center px-5 py-2 text-xs font-bold tracking-widest text-white bg-dark rounded hover:bg-gray-700 transition-colors",
                        href: "/login",
                        {translate(locale(), "nav.login")}
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
                        // Force a solid panel background behind all content.
                        div { class: "absolute inset-0", style: "background-color: #ED8606;" }

                        div { class: "relative z-10 p-6 flex flex-col h-full",
                            div { class: "flex items-center justify-between mb-8",
                                p { class: "text-sm font-bold tracking-widest text-white", {translate(locale(), "nav.logo")} }
                                button {
                                    class: "p-2 text-white",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    "✕"
                                }
                            }

                            div { class: "flex flex-col gap-5",
                                a {
                                    class: if active == NavPage::Stores { "text-sm font-semibold tracking-widest text-white" } else { "text-sm font-semibold tracking-widest text-white/80 hover:text-white transition-colors" },
                                    href: "/",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "nav.stores")}
                                }
                                a {
                                    class: if active == NavPage::Map { "text-sm font-semibold tracking-widest text-white" } else { "text-sm font-semibold tracking-widest text-white/80 hover:text-white transition-colors" },
                                    href: "/map",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "nav.map")}
                                }
                                a {
                                    class: if active == NavPage::Rewards { "text-sm font-semibold tracking-widest text-white" } else { "text-sm font-semibold tracking-widest text-white/80 hover:text-white transition-colors" },
                                    href: "/rewards",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "nav.rewards")}
                                }
                                a {
                                    class: if active == NavPage::Visit { "text-sm font-semibold tracking-widest text-white" } else { "text-sm font-semibold tracking-widest text-white/80 hover:text-white transition-colors" },
                                    href: "/map",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "nav.visit")}
                                }
                                a {
                                    class: "text-sm font-semibold tracking-widest text-white/80 hover:text-white transition-colors",
                                    href: "/login",
                                    onclick: move |_| mobile_menu_open.set(false),
                                    {translate(locale(), "nav.login")}
                                }
                            }

                            div { class: "mt-auto pt-8 border-t border-white/30",
                                p { class: "text-xs font-semibold tracking-wider text-white/80 mb-3", {translate(locale(), "nav.language")} }
                                div { class: "flex items-center gap-2",
                                    for (idx, &(target_locale, label)) in LANGUAGE_BUTTONS.iter().enumerate() {
                                        button {
                                            class: mobile_language_button_class(locale(), target_locale),
                                            onclick: move |_| locale.set(target_locale),
                                            "{label}"
                                        }
                                        if idx < LANGUAGE_BUTTONS.len() - 1 {
                                            span { class: "text-xs text-white/60", "|" }
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
