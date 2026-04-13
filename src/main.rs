#![allow(non_snake_case)]

use context::auth::{AuthProvider, AuthState};
use dioxus::prelude::*;

mod components {
    pub mod button;
    pub mod directory;
    pub mod error;
    pub mod home;
    pub mod loading;
    pub mod login;
    pub mod nav;
    pub mod product_item;
    pub mod product_page;
    pub mod store_page;
    pub mod interactive_map;
    pub mod game;
}
mod context {
    pub mod auth;
}
mod api;
mod db;
pub mod auth;
pub mod admin;
pub mod stores;

fn main() {
    // Load .env (DATABASE_URL, JWT_SECRET) on the server; no-op on WASM.
    #[cfg(not(target_family = "wasm"))]
    dotenvy::dotenv().ok();

    dioxus::launch(|| {
        rsx! {
            document::Link {
                rel: "stylesheet",
                href: asset!("/public/tailwind.css")
            }
            document::Link {
                rel: "stylesheet",
                href: asset!("/public/figma-styles.css")
            }

            components::loading::ChildrenOrLoading {
                AuthProvider {
                    Router::<Route> {}
                }
            }
        }
    });
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/details/:product_id")]
    Details { product_id: usize },

    #[route("/map")]
    Map {},

    #[route("/rewards")]
    Rewards {},

    #[route("/store/:name")]
    Store { name: String },

    #[route("/login")]
    Login {},
}

// ─── Route components ─────────────────────────────────────────────────────────

/// Home page — public, no login required.
fn Home() -> Element {
    rsx! {
        components::nav::Nav {}
        components::home::Home {}
    }
}

#[component]
fn Store(name: String) -> Element {
    rsx! {
        components::nav::Nav {}
        components::store_page::StorePage { name }
    }
}

fn Login() -> Element {
    rsx! {
        components::nav::Nav {}
        components::login::LoginPage {}
    }
}

fn Map() -> Element {
    rsx! {
        components::nav::Nav {}
        components::interactive_map::InteractiveMap {}
    }
}

fn Rewards() -> Element {
    rsx! {
        components::nav::Nav {}
        components::game::Game {}
    }
}

#[component]
fn Details(product_id: usize) -> Element {
    rsx! {
        div {
            components::nav::Nav {}
            components::product_page::ProductPage { product_id }
        }
    }
}

// ─── Route guard ──────────────────────────────────────────────────────────────

/// Wraps protected pages. Shows a loading indicator while auth is being
/// rehydrated from localStorage, then either renders children or redirects
/// to /login.
#[component]
fn ProtectedRoute(children: Element) -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let nav = use_navigator();

    use_effect(move || {
        if matches!(auth(), AuthState::LoggedOut) {
            nav.replace(Route::Login {});
        }
    });

    match auth() {
        AuthState::Loading => rsx! {
            div { class: "flex items-center justify-center min-h-64",
                span { class: "text-gray-400 text-sm", "Loading…" }
            }
        },
        AuthState::LoggedOut => rsx! {},
        AuthState::LoggedIn(_) => rsx! { {children} },
    }
}
