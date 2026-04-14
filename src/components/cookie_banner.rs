use crate::i18n::{Locale, translate};
#[cfg(target_family = "wasm")]
use crate::cookies::{build_cookie_header, parse_cookie_value};
use dioxus::prelude::*;
#[cfg(target_family = "wasm")]
use web_sys::wasm_bindgen::JsCast;

#[cfg(target_family = "wasm")]
const CONSENT_KEY: &str = "cookie_consent";
#[cfg(target_family = "wasm")]
const CONSENT_COOKIE: &str = "foxtown_cookie_consent";

#[component]
pub fn CookieBanner() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut consent = use_signal(read_consent);

    let is_visible = consent().is_none();
    if !is_visible {
        return rsx! {};
    }

    rsx! {
        div { class: "fixed inset-x-0 bottom-0 z-50 p-4 sm:p-6",
            div { class: "mx-auto max-w-5xl rounded-xl border border-gray-200 bg-white shadow-2xl",
                div { class: "p-4 sm:p-5 flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between",
                    div { class: "max-w-3xl",
                        p { class: "text-sm font-semibold font-heading text-gray-900 mb-1",
                            {translate(locale(), "cookie.title")}
                        }
                        p { class: "text-xs sm:text-sm text-gray-600 leading-relaxed",
                            {translate(locale(), "cookie.body")}
                            " "
                            a { class: "text-gray-900 underline hover:text-accent", href: "/privacy", {translate(locale(), "cookie.privacy")} }
                            " "
                            {translate(locale(), "cookie.and")}
                            " "
                            a { class: "text-gray-900 underline hover:text-accent", href: "/terms", {translate(locale(), "cookie.terms")} }
                            "."
                        }
                    }

                    div { class: "flex items-center gap-2 sm:gap-3",
                        button {
                            class: "px-4 py-2 text-xs font-semibold tracking-wide rounded border border-gray-300 text-gray-700 hover:bg-gray-50",
                            onclick: move |_| set_consent(&mut consent, "rejected"),
                            {translate(locale(), "cookie.reject")}
                        }
                        button {
                            class: "px-4 py-2 text-xs font-semibold tracking-wide rounded bg-gray-900 text-white hover:bg-gray-700",
                            onclick: move |_| set_consent(&mut consent, "accepted"),
                            {translate(locale(), "cookie.accept")}
                        }
                    }
                }
            }
        }
    }
}

fn set_consent(consent: &mut Signal<Option<String>>, value: &str) {
    consent.set(Some(value.to_string()));
    write_consent(value);
}

fn read_consent() -> Option<String> {
    #[cfg(target_family = "wasm")]
    {
        if let Some(value) = read_cookie(CONSENT_COOKIE) {
            if value == "accepted" || value == "rejected" {
                return Some(value);
            }
        }
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            if let Ok(Some(value)) = storage.get_item(CONSENT_KEY) {
                if value == "accepted" || value == "rejected" {
                    return Some(value);
                }
            }
        }
    }
    None
}

fn write_consent(value: &str) {
    #[cfg(target_family = "wasm")]
    {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item(CONSENT_KEY, value);
        }
        write_cookie(CONSENT_COOKIE, value, 180);
    }

    #[cfg(not(target_family = "wasm"))]
    let _ = value;
}

#[cfg(target_family = "wasm")]
fn read_cookie(name: &str) -> Option<String> {
    let document = web_sys::window()?.document()?;
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().ok()?;
    let cookie_string = html_document.cookie().ok()?;
    parse_cookie_value(&cookie_string, name)
}

#[cfg(target_family = "wasm")]
fn write_cookie(name: &str, value: &str, days: i32) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        let Ok(html_document) = document.dyn_into::<web_sys::HtmlDocument>() else {
            return;
        };
        let cookie = build_cookie_header(name, value, days);
        let _ = html_document.set_cookie(&cookie);
    }
}
