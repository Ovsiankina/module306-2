use crate::admin::{list_banners, list_events, list_news};
use crate::i18n::{Locale, translate};
use dioxus::prelude::*;

#[component]
pub fn LandingSection() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut banners = use_signal(Vec::new);
    let mut events = use_signal(Vec::new);
    let mut news = use_signal(Vec::new);
    let mut active_banner_idx = use_signal(|| 0usize);
    let mut fetched = use_signal(|| false);

    // Fetch all content on mount only
    use_effect(move || {
        if fetched() {
            return;
        }

        fetched.set(true);

        spawn(async move {
            // Fetch banners
            if let Ok(banner_list) = list_banners().await {
                banners.set(banner_list);
            }
            // Fetch events
            if let Ok(event_list) = list_events().await {
                events.set(event_list);
            }
            // Fetch news
            if let Ok(news_list) = list_news().await {
                news.set(news_list);
            }
        });
    });

    let banner_list = banners();
    let event_list = events();
    let news_list = news();
    let current_banner = banner_list.get(active_banner_idx()).cloned();

    rsx! {
        div { class: "space-y-0",
            // ─── Hero + Banner section ──────────────────────────────────
            if !banner_list.is_empty() {
                section { class: "relative bg-gray-900 overflow-hidden",
                    if let Some(ref banner) = current_banner {
                        div { class: "relative h-48 md:h-56 bg-gray-800 overflow-hidden",
                            img {
                                src: &banner.image_url,
                                alt: &banner.title,
                                class: "w-full h-full object-cover object-center",
                                onerror: move |_| {
                                    // Fallback if image fails to load
                                }
                            }
                            if let Some(ref link) = banner.link_url {
                                a {
                                    href: link,
                                    class: "absolute inset-0",
                                    title: &banner.title,
                                }
                            }
                            // Dark overlay for readability
                            div { class: "absolute inset-0 bg-black/20" }

                            // Banner title overlay (bottom-left)
                            div { class: "absolute bottom-0 left-0 right-0 p-6 md:p-8 bg-gradient-to-t from-black/80 via-black/30 to-transparent",
                                h2 { class: "text-2xl md:text-3xl font-extrabold text-white",
                                    "{banner.title}"
                                }
                            }
                        }
                    }

                    // Banner carousel indicators
                    if banner_list.len() > 1 {
                        div { class: "absolute bottom-4 left-0 right-0 flex justify-center gap-2 z-10",
                            for (idx, _banner) in banner_list.iter().enumerate() {
                                button {
                                    class: if active_banner_idx() == idx {
                                        "w-2 h-2 rounded-full bg-white"
                                    } else {
                                        "w-2 h-2 rounded-full bg-white/50 hover:bg-white/75 transition-colors"
                                    },
                                    onclick: move |_| active_banner_idx.set(idx),
                                    aria_label: "{idx + 1}",
                                }
                            }
                        }
                    }
                }
            }

            // ─── Featured Events section ─────────────────────────────────
            if !event_list.is_empty() {
                section { class: "bg-white",
                    div { class: "max-w-7xl mx-auto px-6 py-16",
                        div { class: "mb-12",
                            h2 { class: "text-3xl md:text-4xl font-extrabold text-dark tracking-tight mb-2",
                                {translate(locale(), "landing.events_title")}
                            }
                            p { class: "text-body text-lg",
                                {translate(locale(), "landing.events_subtitle")}
                            }
                        }

                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                            for (idx, event) in event_list.iter().take(6).enumerate() {
                                div {
                                    key: "{event.id}",
                                    class: "group bg-gray-50 rounded-xl p-6 border border-gray-200 hover:border-accent hover:shadow-lg transition-all",

                                    // Event date badge
                                    div { class: "inline-flex items-center gap-2 mb-3",
                                        span { class: "inline-block px-3 py-1 bg-accent/10 text-accent text-xs font-bold tracking-wider rounded-full",
                                            "{event.date}"
                                        }
                                        if let Some(ref end) = event.end_date {
                                            span { class: "text-muted text-sm",
                                                "— {end}"
                                            }
                                        }
                                    }

                                    // Event title
                                    h3 { class: "text-lg font-bold text-dark mb-2 group-hover:text-accent transition-colors",
                                        "{event.title}"
                                    }

                                    // Location
                                    div { class: "flex items-center gap-2 text-sm text-muted mb-3",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            width: "16",
                                            height: "16",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            path { d: "M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" }
                                            circle { cx: "12", cy: "10", r: "3" }
                                        }
                                        "{event.location}"
                                    }

                                    // Description preview
                                    p { class: "text-sm text-body line-clamp-2 mb-4",
                                        "{event.description}"
                                    }

                                    // Learn more link
                                    a { class: "inline-flex items-center gap-2 text-accent font-semibold text-sm hover:gap-3 transition-all",
                                        href: "#events",
                                        "{translate(locale(), \"landing.learn_more\")}"
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            width: "16",
                                            height: "16",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                                            polyline { points: "12 5 19 12 12 19" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ─── Latest News section ────────────────────────────────────
            if !news_list.is_empty() {
                section { class: "bg-gray-50 border-t border-gray-200",
                    div { class: "max-w-7xl mx-auto px-6 py-16",
                        div { class: "mb-12",
                            h2 { class: "text-3xl md:text-4xl font-extrabold text-dark tracking-tight mb-2",
                                {translate(locale(), "landing.news_title")}
                            }
                            p { class: "text-body text-lg",
                                {translate(locale(), "landing.news_subtitle")}
                            }
                        }

                        div { class: "space-y-6",
                            for (idx, news) in news_list.iter().take(4).enumerate() {
                                div {
                                    key: "{news.id}",
                                    class: "bg-white rounded-xl p-6 border border-gray-200 hover:border-accent hover:shadow-md transition-all",

                                    // Meta info
                                    div { class: "flex items-center justify-between gap-4 mb-3 text-xs text-muted",
                                        span { class: "font-semibold text-dark",
                                            "{news.author}"
                                        }
                                        span {
                                            {format_date(news.created_at)}
                                        }
                                    }

                                    // Title
                                    h3 { class: "text-xl font-bold text-dark mb-3",
                                        "{news.title}"
                                    }

                                    // Preview
                                    p { class: "text-body text-sm line-clamp-3 mb-4",
                                        {strip_html(&news.body)}
                                    }

                                    // Read more
                                    a { class: "inline-flex items-center gap-2 text-accent font-semibold text-sm hover:gap-3 transition-all",
                                        href: "#news",
                                        "{translate(locale(), \"landing.read_more\")}"
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            width: "16",
                                            height: "16",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                                            polyline { points: "12 5 19 12 12 19" }
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

/// Simple HTML tag stripper for news body preview
fn strip_html(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    // Clean up multiple spaces and trim
    result
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

/// Format DateTime for display
fn format_date(dt: chrono::DateTime<chrono::Utc>) -> String {
    dt.format("%B %d, %Y").to_string()
}
