#![allow(non_snake_case)]

use context::{
    auth::{AuthProvider, AuthState},
    cart::CartProvider,
};
use dioxus::prelude::*;
use i18n::{translate, I18nProvider, Locale};

mod components {
    pub mod cart;
    pub mod checkout;
    pub mod contact;
    pub mod directory;
    pub mod error;
    pub mod footer;
    pub mod home;
    pub mod loading;
    pub mod login;
    pub mod nav;
    pub mod product_item;
    pub mod product_page;
    pub mod not_found;
    pub mod privacy;
    pub mod rewards;
    pub mod store_page;
    pub mod terms;
}
mod context {
    pub mod auth;
    pub mod cart;
}
mod i18n;
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
                I18nProvider {
                    AuthProvider {
                        CartProvider {
                            Router::<Route> {}
                        }
                    }
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

    #[route("/cart")]
    Cart {},

    #[route("/checkout")]
    Checkout {},

    #[route("/contact")]
    Contact {},

    #[route("/privacy")]
    Privacy {},

    #[route("/terms")]
    Terms {},

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
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

fn Cart() -> Element {
    rsx! {
        components::cart::CartPage {}
    }
}

fn Checkout() -> Element {
    rsx! {
        components::checkout::CheckoutPage {}
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

fn Contact() -> Element {
    rsx! {
        components::nav::Nav {}
        components::contact::ContactPage {}
        components::footer::Footer {}
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        components::nav::Nav {}
        components::not_found::NotFoundPage { segments }
        components::footer::Footer {}
    }
}

fn Privacy() -> Element {
    rsx! {
        components::nav::Nav {}
        components::privacy::PrivacyPage {}
        components::footer::Footer {}
    }
}

fn Terms() -> Element {
    rsx! {
        components::nav::Nav {}
        components::terms::TermsPage {}
        components::footer::Footer {}
    }
}

// ─── Route guard ──────────────────────────────────────────────────────────────

#[component]
fn ProtectedRoute(children: Element) -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();
    let nav = use_navigator();

    use_effect(move || {
        if matches!(auth(), AuthState::LoggedOut) {
            nav.replace(Route::Login {});
        }
    });

    match auth() {
        AuthState::Loading => rsx! {
            div { class: "flex items-center justify-center min-h-64",
                span { class: "text-muted text-sm", {translate(locale(), "common.loading")} }
            }
        },
        AuthState::LoggedOut => rsx! {},
        AuthState::LoggedIn(_) => rsx! { {children} },
    }
}
