use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use dioxus::prelude::*;

pub fn RewardsPage() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::Rewards }

            // ─── How To Play ────────────────────────────────────────
            section { class: "max-w-7xl mx-auto px-6 py-16",
                div { class: "text-center mb-12",
                    h2 { class: "text-2xl font-extrabold tracking-widest text-dark mb-3",
                        "HOW TO PLAY"
                    }
                    div { class: "w-16 h-0.5 bg-accent mx-auto" }
                }

                div { class: "grid grid-cols-1 md:grid-cols-3 gap-10 max-w-4xl mx-auto",
                    StepCard {
                        number: "01",
                        title: "LOG IN",
                        description: "Sign in to your FoxTown Rewards account to access your daily lucky spin and save your winnings instantly.",
                    }
                    StepCard {
                        number: "02",
                        title: "SPIN DAILY",
                        description: "Every 24 hours you get a fresh chance. The wheel is reset with new designer perks and exclusive boutique vouchers.",
                    }
                    StepCard {
                        number: "03",
                        title: "REDEEM",
                        description: "Present your winning digital voucher at the participating boutique in FoxTown Mendrisio to claim your prize.",
                    }
                }
            }

            // ─── Hero Game Section ──────────────────────────────────
            section { class: "bg-gray-50",
                div { class: "max-w-7xl mx-auto px-6 py-16",
                    div { class: "flex flex-col lg:flex-row items-center gap-12",

                        // Left content
                        div { class: "flex-1",
                            p { class: "text-xs font-bold tracking-widest text-accent mb-4",
                                "EXCLUSIVE REWARDS"
                            }
                            h1 { class: "text-4xl md:text-5xl font-extrabold text-dark leading-tight mb-6",
                                "SPIN\n"
                                span { class: "text-accent", "TO WIN" }
                            }
                            p { class: "text-body leading-relaxed max-w-lg mb-10",
                                "Experience the thrill of luxury rewards. Spin the wheel daily for a chance to unlock exclusive designer discounts and premium boutique prizes."
                            }

                            // Recent Winners
                            div { class: "bg-white rounded-xl border border-gray-100 p-6",
                                div { class: "flex items-center justify-between mb-5",
                                    h3 { class: "text-lg font-bold text-dark", "Recent Winners" }
                                    div { class: "flex items-center gap-2",
                                        span { class: "w-2 h-2 bg-accent rounded-full animate-pulse" }
                                        span { class: "text-xs font-bold text-accent", "LIVE" }
                                    }
                                }

                                div { class: "space-y-4",
                                    WinnerRow { name: "MARCO R.", prize: "WON 20% OFF ARMANI", time: "2M AGO" }
                                    WinnerRow { name: "ELENA S.", prize: "WON SHOPPING VOUCHER", time: "14M AGO" }
                                    WinnerRow { name: "JULIAN B.", prize: "WON 15% OFF PRADA", time: "1H AGO" }
                                }
                            }
                        }

                        // Right: Spin wheel area
                        div { class: "flex flex-col items-center gap-6",
                            // Wheel placeholder
                            div { class: "w-72 h-72 md:w-80 md:h-80 rounded-full bg-gradient-to-br from-accent/20 via-amber-100 to-accent/10 border-4 border-accent/30 flex items-center justify-center shadow-xl",
                                div { class: "w-56 h-56 md:w-64 md:h-64 rounded-full bg-white border-2 border-gray-200 flex items-center justify-center",
                                    span { class: "text-4xl font-extrabold text-accent", "SPIN" }
                                }
                            }

                            // Spin button + counter
                            div { class: "text-center",
                                button { class: "px-10 py-4 text-sm font-bold tracking-widest text-white bg-accent hover:bg-amber-600 rounded-lg transition-colors shadow-lg shadow-accent/30 mb-3",
                                    "SPIN THE WHEEL"
                                }
                                p { class: "text-xs text-muted tracking-wider",
                                    "1 SPIN REMAINING TODAY"
                                }
                            }
                        }
                    }
                }
            }

            // ─── Participating Brands ───────────────────────────────
            section { class: "max-w-7xl mx-auto px-6 py-16",
                div { class: "flex flex-col md:flex-row items-start md:items-center justify-between mb-10",
                    div {
                        h2 { class: "text-2xl font-extrabold tracking-widest text-dark mb-2",
                            "PARTICIPATING BRANDS"
                        }
                        p { class: "text-body max-w-lg",
                            "Discover exclusive prizes redeemable at over 160 luxury stores across the outlet."
                        }
                    }
                    a { class: "mt-4 md:mt-0 text-sm font-bold tracking-wider text-accent hover:underline",
                        href: "/",
                        "VIEW ALL STORES"
                    }
                }

                div { class: "grid grid-cols-2 lg:grid-cols-4 gap-4",
                    BrandCard { name: "ARMANI", subtitle: "Outlet Store", image: "/brands/armani-spin.png" }
                    BrandCard { name: "BURBERRY", subtitle: "Factory Store", image: "/brands/burberry-spin.png" }
                    BrandCard { name: "PRADA", subtitle: "Outlet", image: "/brands/prada-spin.png" }
                    BrandCard { name: "GUCCI", subtitle: "Factory Shop", image: "/brands/gucci-spin.png" }
                }
            }

            Footer { dark: true }
        }
    }
}

#[component]
fn StepCard(number: &'static str, title: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "text-center",
            p { class: "text-3xl font-extrabold text-accent mb-3", "{number}" }
            h4 { class: "text-sm font-extrabold tracking-widest text-dark mb-3", "{title}" }
            p { class: "text-sm text-body leading-relaxed", "{description}" }
        }
    }
}

#[component]
fn WinnerRow(name: &'static str, prize: &'static str, time: &'static str) -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            div { class: "w-10 h-10 bg-gray-100 rounded-full flex items-center justify-center shrink-0",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                    circle { cx: "12", cy: "7", r: "4" }
                }
            }
            div { class: "flex-1 min-w-0",
                p { class: "text-sm font-bold text-dark", "{name}" }
                p { class: "text-xs font-semibold text-accent truncate", "{prize}" }
            }
            span { class: "text-xs text-muted shrink-0", "{time}" }
        }
    }
}

#[component]
fn BrandCard(name: &'static str, subtitle: &'static str, image: &'static str) -> Element {
    rsx! {
        div { class: "group relative h-52 w-full bg-gray-900 rounded-xl overflow-hidden cursor-pointer",
            // Brand image
            img { src: "{image}", class: "absolute inset-0 w-full h-full object-cover", alt: "{name}" }
            // Gradient overlay
            div { class: "absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent" }

            // Label
            div { class: "absolute bottom-0 left-0 right-0 p-5",
                p { class: "text-sm font-extrabold text-white tracking-wider mb-1", "{name}" }
                p { class: "text-xs text-white/70", "{subtitle}" }
            }
        }
    }
}
