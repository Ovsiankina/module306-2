use dioxus::prelude::*;

#[component]
pub fn Footer(#[props(default = false)] dark: bool) -> Element {
    let bg = if dark { "bg-dark" } else { "bg-white border-t border-gray-100" };
    let logo_color = if dark { "text-white" } else { "text-dark" };
    let link_color = if dark { "text-muted hover:text-white" } else { "text-muted hover:text-dark" };
    let divider_color = if dark { "border-gray-700" } else { "border-gray-200" };
    let copy_color = if dark { "text-surface" } else { "text-muted" };

    rsx! {
        footer { class: "mt-auto {bg}",
            div { class: "max-w-7xl mx-auto px-6 py-12",
                // Logo
                p { class: "text-lg font-extrabold tracking-widest {logo_color} mb-6",
                    "FOXTOWN"
                }

                // Links
                div { class: "flex flex-wrap gap-6 mb-8",
                    a { class: "text-xs font-semibold tracking-widest {link_color} transition-colors", href: "#", "CONTACT" }
                    a { class: "text-xs font-semibold tracking-widest {link_color} transition-colors", href: "#", "DIRECTIONS" }
                    a { class: "text-xs font-semibold tracking-widest {link_color} transition-colors", href: "#", "PRIVACY" }
                    a { class: "text-xs font-semibold tracking-widest {link_color} transition-colors", href: "#", "TERMS" }
                }

                // Divider
                div { class: "border-t {divider_color} mb-6" }

                // Copyright
                p { class: "text-xs {copy_color}",
                    "\u{00A9} 2024 FOXTOWN FACTORY STORES. ALL RIGHTS RESERVED."
                }
            }
        }
    }
}
