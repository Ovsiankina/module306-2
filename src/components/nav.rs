use dioxus::prelude::*;

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
    let link_class = |page: NavPage| {
        if active == page {
            "text-sm font-semibold tracking-widest text-accent"
        } else {
            "text-sm font-semibold tracking-widest text-nav hover:text-dark transition-colors"
        }
    };

    rsx! {
        nav { class: "sticky top-0 z-50 bg-white border-b border-gray-100",
            div { class: "max-w-7xl mx-auto px-6 h-16 flex items-center justify-between",

                // Logo
                a { class: "text-xl font-extrabold tracking-widest text-dark",
                    href: "/",
                    "FOXTOWN"
                }

                // Center nav links (desktop)
                div { class: "hidden md:flex items-center gap-8",
                    a { class: link_class(NavPage::Stores), href: "/", "STORES" }
                    a { class: link_class(NavPage::Map), href: "/map", "MAP" }
                    a { class: link_class(NavPage::Rewards), href: "/rewards", "REWARDS" }
                    a { class: link_class(NavPage::Visit), href: "#", "VISIT" }
                }

                // Right actions
                div { class: "flex items-center gap-3",
                    // Search button
                    button { class: "p-2 text-dark hover:text-accent transition-colors",
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
                        "LOGIN"
                    }

                    // Mobile hamburger
                    button { class: "md:hidden p-2 text-dark",
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
        }
    }
}
