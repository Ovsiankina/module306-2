use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::cart::CartState;
use crate::Route;
use dioxus::prelude::*;

pub fn CheckoutPage() -> Element {
    let mut cart = use_context::<Signal<CartState>>();
    let nav = use_navigator();

    let mut full_name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut address = use_signal(String::new);
    let mut city = use_signal(String::new);
    let mut postal = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut success = use_signal(|| false);

    let subtotal = cart().subtotal();

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::None }

            section { class: "max-w-6xl w-full mx-auto px-6 py-12 flex-1",
                h1 { class: "text-3xl md:text-4xl font-extrabold text-dark mb-8", "Checkout" }

                if cart().items.is_empty() && !success() {
                    div { class: "rounded-xl border border-gray-200 bg-gray-50 p-8 text-center",
                        p { class: "text-body mb-5", "Your cart is empty. Add items before checking out." }
                        button {
                            class: "inline-flex items-center px-5 py-2.5 text-xs font-bold tracking-widest text-white bg-dark rounded hover:bg-gray-700 transition-colors",
                            onclick: move |_| {
                                nav.push(Route::Cart {});
                            },
                            "Go to cart"
                        }
                    }
                } else if success() {
                    div { class: "rounded-xl border border-green-200 bg-green-50 p-8 text-center",
                        h2 { class: "text-2xl font-bold text-green-900 mb-3", "Order confirmed" }
                        p { class: "text-green-800 mb-6", "Thanks for your order. We have sent a confirmation by email." }
                        button {
                            class: "inline-flex items-center px-5 py-2.5 text-xs font-bold tracking-widest text-white bg-dark rounded hover:bg-gray-700 transition-colors",
                            onclick: move |_| {
                                nav.push(Route::Home {});
                            },
                            "Back to home"
                        }
                    }
                } else {
                    div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                        form {
                            class: "lg:col-span-2 rounded-xl border border-gray-200 p-5 md:p-6 space-y-4",
                            onsubmit: move |_| {
                                error.set(None);
                                if full_name().trim().is_empty()
                                    || email().trim().is_empty()
                                    || address().trim().is_empty()
                                    || city().trim().is_empty()
                                    || postal().trim().is_empty()
                                {
                                    error.set(Some("Please complete all checkout fields.".to_string()));
                                    return;
                                }
                                cart.write().clear();
                                success.set(true);
                            },

                            h2 { class: "text-lg font-bold text-dark mb-1", "Shipping information" }

                            if let Some(ref err) = error() {
                                p { class: "text-sm text-red-700 bg-red-50 border border-red-200 rounded-md px-3 py-2", "{err}" }
                            }

                            input {
                                class: "w-full px-4 py-3 text-sm border border-gray-200 rounded-lg focus:ring-accent focus:border-accent focus:outline-none",
                                placeholder: "Full name",
                                value: "{full_name}",
                                oninput: move |e| full_name.set(e.value()),
                            }
                            input {
                                class: "w-full px-4 py-3 text-sm border border-gray-200 rounded-lg focus:ring-accent focus:border-accent focus:outline-none",
                                r#type: "email",
                                placeholder: "Email address",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                            }
                            input {
                                class: "w-full px-4 py-3 text-sm border border-gray-200 rounded-lg focus:ring-accent focus:border-accent focus:outline-none",
                                placeholder: "Street address",
                                value: "{address}",
                                oninput: move |e| address.set(e.value()),
                            }
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                input {
                                    class: "w-full px-4 py-3 text-sm border border-gray-200 rounded-lg focus:ring-accent focus:border-accent focus:outline-none",
                                    placeholder: "City",
                                    value: "{city}",
                                    oninput: move |e| city.set(e.value()),
                                }
                                input {
                                    class: "w-full px-4 py-3 text-sm border border-gray-200 rounded-lg focus:ring-accent focus:border-accent focus:outline-none",
                                    placeholder: "Postal code",
                                    value: "{postal}",
                                    oninput: move |e| postal.set(e.value()),
                                }
                            }

                            button {
                                class: "w-full py-3 text-xs font-bold tracking-widest text-white bg-accent rounded hover:bg-amber-600 transition-colors",
                                r#type: "submit",
                                "Place order"
                            }
                        }

                        aside { class: "rounded-xl border border-gray-200 p-5 h-fit",
                            h2 { class: "text-lg font-bold text-dark mb-4", "Order summary" }
                            for item in cart().items.clone() {
                                div { key: "{item.product_id}", class: "flex items-center justify-between text-sm mb-2",
                                    span { class: "text-body", "{item.title} x {item.quantity}" }
                                    span { class: "font-semibold text-dark", "${(item.unit_price * item.quantity as f32):.2}" }
                                }
                            }
                            div { class: "border-t border-gray-100 my-4" }
                            div { class: "flex items-center justify-between",
                                span { class: "font-bold text-dark", "Total" }
                                span { class: "font-extrabold text-dark", "${subtotal:.2}" }
                            }
                        }
                    }
                }
            }

            Footer {}
        }
    }
}
