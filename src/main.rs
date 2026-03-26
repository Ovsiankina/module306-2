#![allow(non_snake_case)]

use components::home::Home;
use components::loading::ChildrenOrLoading;
use dioxus::prelude::*;

mod components {
    pub mod directory;
    pub mod error;
    pub mod home;
    pub mod loading;
    pub mod login;
    pub mod nav;
    pub mod product_item;
    pub mod product_page;
    pub mod store_page;
}
mod api;
pub mod auth;
pub mod admin;
pub mod stores;

fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Link {
                rel: "stylesheet",
                href: asset!("/public/tailwind.css")
            }

            ChildrenOrLoading {
                Router::<Route> {}
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

    #[route("/store/:name")]
    Store { name: String },

    #[route("/login")]
    Login {},
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
        components::directory::ShopDirectory {}
    }
}

#[component]
/// Render a more sophisticated page with ssr
fn Details(product_id: usize) -> Element {
    rsx! {
        div {
            components::nav::Nav {}
            components::product_page::ProductPage {
                product_id
            }
        }
    }
}
