use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::components::rewards_draw::{RewardsDraw, WinnerEvent};
use crate::i18n::{Locale, translate};
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct WinnerFeedItem {
    name: String,
    prize: String,
    time: String,
}

pub fn RewardsPage() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut winners = use_signal(|| {
        vec![
            WinnerFeedItem {
                name: "MARCO R.".to_string(),
                prize: "20% OFF ARMANI".to_string(),
                time: "2M AGO".to_string(),
            },
            WinnerFeedItem {
                name: "ELENA S.".to_string(),
                prize: "35% OFF BURBERRY".to_string(),
                time: "15M AGO".to_string(),
            },
            WinnerFeedItem {
                name: "JULIAN B.".to_string(),
                prize: "15% OFF PRADA".to_string(),
                time: "37M AGO".to_string(),
            },
        ]
    });

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::Rewards }

            // ─── How To Play ────────────────────────────────────────
            section { class: "max-w-7xl mx-auto px-6 py-16",
                div { class: "text-center mb-12",
                    h2 { class: "text-2xl font-extrabold tracking-widest text-dark mb-3",
                        {translate(locale(), "rewards.how_to_play")}
                    }
                    div { class: "w-16 h-0.5 bg-accent mx-auto" }
                }

                div { class: "grid grid-cols-1 md:grid-cols-3 gap-10 max-w-4xl mx-auto",
                    StepCard {
                        number: "01",
                        title: translate(locale(), "rewards.step1_title"),
                        description: translate(locale(), "rewards.step1_desc"),
                    }
                    StepCard {
                        number: "02",
                        title: translate(locale(), "rewards.step2_title"),
                        description: translate(locale(), "rewards.step2_desc"),
                    }
                    StepCard {
                        number: "03",
                        title: translate(locale(), "rewards.step3_title"),
                        description: translate(locale(), "rewards.step3_desc"),
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
                                {translate(locale(), "rewards.exclusive")}
                            }
                            h1 { class: "text-4xl md:text-5xl font-extrabold text-dark leading-tight mb-6",
                                {format!("{} ", translate(locale(), "rewards.spin"))}
                                span { class: "text-accent", {translate(locale(), "rewards.to_win")} }
                            }
                            p { class: "text-body leading-relaxed max-w-lg mb-10",
                                {translate(locale(), "rewards.hero_subtitle")}
                            }

                            // Recent Winners
                            div { class: "bg-white rounded-xl border border-gray-100 p-6",
                                div { class: "flex items-center justify-between mb-5",
                                    h3 { class: "text-lg font-bold text-dark", {translate(locale(), "rewards.recent_winners")} }
                                    div { class: "flex items-center gap-2",
                                        span { class: "w-2 h-2 bg-accent rounded-full animate-pulse" }
                                        span { class: "text-xs font-bold text-accent", {translate(locale(), "rewards.live")} }
                                    }
                                }
                                p { class: "text-xs text-muted mb-4",
                                    {translate(locale(), "rewards.disclaimer_live")}
                                }

                                div { class: "space-y-4",
                                    for entry in winners() {
                                        WinnerRow {
                                            name: entry.name.clone(),
                                            prize: entry.prize.clone(),
                                            time: entry.time.clone(),
                                        }
                                    }
                                }
                            }
                        }

                        // Right: Ball draw game (from game-promo service)
                        RewardsDraw {
                            on_win: move |win: WinnerEvent| {
                                let display_name = format!("{}.", win.user_name.to_uppercase());
                                let prize = format!(
                                    "{}% OFF {}",
                                    win.discount_percent,
                                    win.shop_name.to_uppercase()
                                );
                                let mut next = winners();
                                next.insert(0, WinnerFeedItem {
                                    name: display_name,
                                    prize,
                                    time: "JUST NOW".to_string(),
                                });
                                next.truncate(8);
                                winners.set(next);
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
                            {translate(locale(), "rewards.participating")}
                        }
                        p { class: "text-body max-w-lg",
                            {translate(locale(), "rewards.participating_subtitle")}
                        }
                    }
                    a { class: "mt-4 md:mt-0 text-sm font-bold tracking-wider text-accent hover:underline",
                        href: "/",
                        {translate(locale(), "rewards.view_all")}
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
fn StepCard(number: &'static str, title: String, description: String) -> Element {
    rsx! {
        div { class: "text-center",
            p { class: "text-3xl font-extrabold text-accent mb-3", "{number}" }
            h4 { class: "text-sm font-extrabold tracking-widest text-dark mb-3", "{title}" }
            p { class: "text-sm text-body leading-relaxed", "{description}" }
        }
    }
}

#[component]
fn WinnerRow(name: String, prize: String, time: String) -> Element {
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
