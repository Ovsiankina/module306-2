use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;
#[cfg(target_family = "wasm")]
use web_sys::wasm_bindgen::JsCast;

#[cfg(target_family = "wasm")]
const LOCALE_KEY: &str = "locale";
#[cfg(target_family = "wasm")]
const LOCALE_COOKIE: &str = "foxtown_locale";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Locale {
    En,
    It,
    De,
    Fr,
}

impl Locale {
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::It => "it",
            Self::De => "de",
            Self::Fr => "fr",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Self::En),
            "it" => Some(Self::It),
            "de" => Some(Self::De),
            "fr" => Some(Self::Fr),
            _ => None,
        }
    }
}

static EN: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../assets/i18n/en.json")).expect("valid en translations")
});
static IT: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../assets/i18n/it.json")).expect("valid it translations")
});
static DE: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../assets/i18n/de.json")).expect("valid de translations")
});
static FR: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../assets/i18n/fr.json")).expect("valid fr translations")
});

fn table(locale: Locale) -> &'static HashMap<String, String> {
    match locale {
        Locale::En => &EN,
        Locale::It => &IT,
        Locale::De => &DE,
        Locale::Fr => &FR,
    }
}

pub fn translate(locale: Locale, key: &str) -> String {
    table(locale)
        .get(key)
        .cloned()
        .or_else(|| EN.get(key).cloned())
        .unwrap_or_else(|| key.to_string())
}

pub fn translate_fmt(locale: Locale, key: &str, vars: &[(&str, String)]) -> String {
    let mut out = translate(locale, key);
    for (name, value) in vars {
        out = out.replace(&format!("{{{name}}}"), value);
    }
    out
}

fn read_locale() -> Locale {
    #[cfg(target_family = "wasm")]
    {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            if let Ok(Some(code)) = storage.get_item(LOCALE_KEY) {
                if let Some(locale) = Locale::from_code(&code) {
                    return locale;
                }
            }
        }

        if let Some(code) = read_cookie(LOCALE_COOKIE) {
            if let Some(locale) = Locale::from_code(&code) {
                return locale;
            }
        }
    }
    Locale::En
}

fn write_locale(locale: Locale) {
    #[cfg(target_family = "wasm")]
    {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item(LOCALE_KEY, locale.code());
        }
        write_cookie(LOCALE_COOKIE, locale.code(), 365);
    }

    #[cfg(not(target_family = "wasm"))]
    let _ = locale;
}

pub fn persist_locale(locale: Locale) {
    write_locale(locale);
}

#[cfg(target_family = "wasm")]
fn read_cookie(name: &str) -> Option<String> {
    let document = web_sys::window()?.document()?;
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().ok()?;
    let cookie_string = html_document.cookie().ok()?;
    cookie_string
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .find_map(|part| {
            let (key, value) = part.split_once('=').unwrap_or((part, ""));
            if key == name {
                Some(value.to_string())
            } else {
                None
            }
        })
}

#[cfg(target_family = "wasm")]
fn write_cookie(name: &str, value: &str, days: i32) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        let Ok(html_document) = document.dyn_into::<web_sys::HtmlDocument>() else {
            return;
        };
        let max_age = days.saturating_mul(24 * 60 * 60);
        let cookie = format!("{name}={value}; path=/; max-age={max_age}");
        let _ = html_document.set_cookie(&cookie);
    }
}

#[component]
pub fn I18nProvider(children: Element) -> Element {
    let locale = use_context_provider(|| Signal::new(read_locale()));
    use_effect(move || {
        write_locale(locale());
    });
    rsx! { {children} }
}

#[cfg(test)]
mod tests {
    use super::{Locale, translate, translate_fmt};

    #[test]
    fn locale_code_roundtrip_works_for_all_supported_locales() {
        for locale in [Locale::En, Locale::It, Locale::De, Locale::Fr] {
            assert_eq!(Locale::from_code(locale.code()), Some(locale));
        }
        assert_eq!(Locale::from_code("es"), None);
    }

    #[test]
    fn translate_returns_known_values_and_falls_back_to_key() {
        assert_eq!(translate(Locale::En, "nav.logo"), "FOXTOWN");
        assert_eq!(
            translate(Locale::Fr, "this.translation.key.does.not.exist"),
            "this.translation.key.does.not.exist"
        );
    }

    #[test]
    fn translate_fmt_replaces_named_placeholders() {
        let out = translate_fmt(Locale::En, "directory.floor_button", &[("level", "2".to_string())]);
        assert_eq!(out, "Level 2");
    }

    #[test]
    fn translate_falls_back_to_english_when_key_missing_in_selected_locale() {
        // This key exists in EN and is intentionally missing in FR in current assets.
        let fr_value = translate(Locale::Fr, "home.search");
        let en_value = translate(Locale::En, "home.search");
        assert_eq!(fr_value, en_value);
    }

    #[test]
    fn translate_fmt_keeps_unknown_placeholders_intact() {
        let out = translate_fmt(
            Locale::En,
            "directory.floor_button",
            &[("missing", "x".to_string())],
        );
        assert_eq!(out, "Level {level}");
    }
}
