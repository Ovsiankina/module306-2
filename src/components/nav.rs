use dioxus::prelude::*;
use crate::components::button::{ButtonLink, ButtonClick, ButtonVariant};
use crate::Route;
use crate::context::auth::AuthState;

#[component]
pub fn Nav() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let nav = use_navigator();

    let handle_logout = move |_| {
        // TODO: Implement logout logic
        nav.push(Route::Login {});
    };

    rsx! {
        nav { class: "flex items-center justify-between px-6 md:px-12 py-6 border-b border-gray-200 bg-white",
            // Left - Logo
            a {
                href: "/",
                class: "flex-shrink-0",
                img {
                    src: asset!("/public/fox_town/fox_icon.svg"),
                    alt: "FoxTown",
                    class: "h-8 w-auto"
                }
            }

            // Center - Navigation links
            ul { class: "hidden md:flex items-center gap-8 font-semibold text-sm",
                li {
                    a {
                        href: "/",
                        class: "text-gray-700 hover:text-rgb-237-134-6 transition",
                        "STORES"
                    }
                }
                li {
                    a {
                        href: "/map",
                        class: "text-gray-700 hover:text-rgb-237-134-6 transition",
                        "MAP"
                    }
                }
                li {
                    a {
                        href: "/rewards",
                        class: "text-gray-700 hover:text-rgb-237-134-6 transition",
                        "REWARDS"
                    }
                }
                li {
                    a {
                        href: "/",
                        class: "text-gray-700 hover:text-rgb-237-134-6 transition",
                        "VISIT"
                    }
                }
            }

            // Right - Auth button
            div { class: "flex items-center gap-4",
                if matches!(auth(), AuthState::LoggedIn(_)) {
                    ButtonClick {
                        variant: ButtonVariant::Orange,
                        onclick: handle_logout,
                        "LOGOUT"
                    }
                } else {
                    ButtonLink {
                        variant: ButtonVariant::Orange,
                        href: "/login".to_string(),
                        "LOGIN"
                    }
                }

                // Mobile menu button (hidden on desktop)
                button { class: "md:hidden text-gray-700",
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            d: "M4 6h16M4 12h16M4 18h16",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round"
                        }
                    }
                }
            }
        }

        // Mobile menu (optional - can be expanded later)
        div { class: "hidden md:hidden border-t border-gray-200 bg-white px-6 py-4",
            ul { class: "space-y-4 font-semibold text-sm",
                li {
                    a {
                        href: "/",
                        class: "text-gray-700 hover:text-rgb-237-134-6 transition block",
                        "STORES"
                    }
                }
                li {
                    a {
                        href: "/map",
                        class: "text-gray-700 hover:text-rgb-237-134-6 transition block",
                        "MAP"
                    }
                }
                li {
                    a {
                        href: "/rewards",
                        class: "text-gray-700 hover:text-rgb-237-134-6 transition block",
                        "REWARDS"
                    }
                }
                li {
                    a {
                        href: "/",
                        class: "text-gray-700 hover:text-rgb-237-134-6 transition block",
                        "VISIT"
                    }
                }
            }
        }
    }
}
