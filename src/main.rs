#![allow(non_snake_case)]

use context::auth::{AuthProvider, AuthState};
use dioxus::prelude::*;

mod components {
    pub mod directory;
    pub mod error;
    pub mod footer;
    pub mod home;
    pub mod loading;
    pub mod login;
    pub mod nav;
    pub mod product_item;
    pub mod product_page;
    pub mod rewards;
    pub mod store_page;
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
    #[cfg(not(target_family = "wasm"))]
    dotenvy::dotenv().ok();

    dioxus::launch(|| {
        rsx! {
            document::Link {
                rel: "stylesheet",
                href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&display=swap"
            }
            document::Link {
                rel: "stylesheet",
                href: asset!("/public/tailwind.css")
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

fn Home() -> Element {
    rsx! {
        components::home::Home {}
    }
}

#[component]
fn Store(name: String) -> Element {
    rsx! {
        components::nav::Nav { active: components::nav::NavPage::Stores }
        components::store_page::StorePage { name }
        components::footer::Footer {}
    }
}

fn Login() -> Element {
    rsx! {
        components::login::LoginPage {}
    }
}

fn Map() -> Element {
    rsx! {
        components::directory::ShopDirectory {}
    }
}

fn Rewards() -> Element {
    rsx! {
        ProtectedRoute {
            components::rewards::RewardsPage {}
        }
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
                span { class: "text-muted text-sm", "Loading\u{2026}" }
            }
        },
        AuthState::LoggedOut => rsx! {},
        AuthState::LoggedIn(_) => rsx! { {children} },
    }
}
