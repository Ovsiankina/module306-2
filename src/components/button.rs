use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Orange,
}

#[component]
pub fn ButtonLink(
    variant: ButtonVariant,
    children: Element,
    href: String,
) -> Element {
    let base_class = "font-bold text-sm transition-all";

    let variant_class = match variant {
        ButtonVariant::Orange => {
            "px-6 py-3 bg-rgb-237-134-6 text-gray-900 hover:bg-yellow-500 rounded-lg"
        }
    };

    let full_class = format!("{} {}", base_class, variant_class);

    rsx! {
        a {
            class: full_class,
            href: href,
            {children}
        }
    }
}

#[component]
pub fn ButtonClick(
    variant: ButtonVariant,
    children: Element,
    onclick: EventHandler<MouseEvent>,
    #[props(default)] disabled: bool,
) -> Element {
    let base_class = "font-bold text-sm transition-all";

    let variant_class = match variant {
        ButtonVariant::Orange => {
            "px-6 py-3 bg-rgb-237-134-6 text-gray-900 hover:bg-yellow-500 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
        }
    };

    let full_class = format!("{} {}", base_class, variant_class);

    rsx! {
        button {
            class: full_class,
            disabled: disabled,
            onclick: onclick,
            {children}
        }
    }
}
