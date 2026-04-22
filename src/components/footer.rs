use crate::Route;
use crate::i18n::{Locale, translate};
use dioxus::prelude::*;

#[component]
pub fn Footer(
    #[props(default = false)] dark: bool,
    #[props(default = true)] stick_to_bottom: bool,
) -> Element {
    let locale = use_context::<Signal<Locale>>();
    let bg = if dark { "bg-dark" } else { "bg-white border-t border-gray-100" };
    let spacing = if stick_to_bottom { "mt-auto" } else { "" };
    let logo_color = if dark { "text-white" } else { "text-dark" };
    let link_color = if dark { "text-muted hover:text-white" } else { "text-muted hover:text-dark" };
    let divider_color = if dark { "border-gray-700" } else { "border-gray-200" };
    let copy_color = if dark { "text-surface" } else { "text-muted" };

    rsx! {
        footer { class: "{spacing} {bg}",
            div { class: "max-w-7xl mx-auto px-6 py-12",
                // Logo
                p { class: "text-lg font-extrabold tracking-widest {logo_color} mb-6",
                    {translate(locale(), "nav.logo")}
                }

                // Links
                div { class: "flex flex-wrap gap-6 mb-8",
                    Link { class: "text-xs font-semibold tracking-widest {link_color} transition-colors", to: Route::Contact {}, {translate(locale(), "footer.contact")} }
                    Link { class: "text-xs font-semibold tracking-widest {link_color} transition-colors", to: Route::Map {}, {translate(locale(), "footer.directions")} }
                    Link { class: "text-xs font-semibold tracking-widest {link_color} transition-colors", to: Route::Privacy {}, {translate(locale(), "footer.privacy")} }
                    Link { class: "text-xs font-semibold tracking-widest {link_color} transition-colors", to: Route::Terms {}, {translate(locale(), "footer.terms")} }
                }

                // Divider
                div { class: "border-t {divider_color} mb-6" }

                // Copyright
                p { class: "text-xs {copy_color}",
                    {translate(locale(), "footer.copyright")}
                }
            }
        }
    }
}
