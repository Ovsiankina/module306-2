use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::cart::CartState;
use crate::Route;
use dioxus::prelude::*;

pub fn CartPage() -> Element {
    let mut cart = use_context::<Signal<CartState>>();
    let nav = use_navigator();

    let item_count = cart().total_items();
    let subtotal = cart().subtotal();

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white font-heading",
            Nav { active: NavPage::None }

            section { class: "max-w-6xl w-full mx-auto px-6 py-12 flex-1",
                h1 { class: "text-3xl md:text-4xl font-extrabold text-dark mb-2", "Your Cart" }
                p { class: "text-sm text-body mb-8", "{item_count} item(s) in your cart" }

                if cart().items.is_empty() {
                    div { class: "rounded-xl border border-gray-200 bg-gray-50 p-8 text-center",
                        p { class: "text-body mb-5", "Your cart is empty." }
                        button {
                            class: "inline-flex items-center px-5 py-2.5 text-xs font-bold tracking-widest text-white bg-dark rounded hover:bg-gray-700 transition-colors",
                            onclick: move |_| {
                                nav.push(Route::Home {});
                            },
                            "Continue shopping"
                        }
                    }
                } else {
                    div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                        div { class: "lg:col-span-2 space-y-4",
                            for item in cart().items.clone() {
                                div { key: "{item.product_id}", class: "rounded-xl border border-gray-200 p-4 md:p-5",
                                    div { class: "flex flex-col md:flex-row md:items-center gap-4 justify-between",
                                        div {
                                            h2 { class: "font-bold text-dark", "{item.title}" }
                                            p { class: "text-sm text-muted", "${item.unit_price:.2} each" }
                                        }
                                        div { class: "flex items-center gap-3",
                                            input {
                                                class: "w-20 px-3 py-2 text-sm border border-gray-200 rounded-md",
                                                r#type: "number",
                                                min: "1",
                                                value: "{item.quantity}",
                                                oninput: move |evt| {
                                                    if let Ok(parsed) = evt.value().parse::<u32>() {
                                                        cart.write().update_quantity(item.product_id, parsed);
                                                    }
                                                }
                                            }
                                            button {
                                                class: "text-sm font-semibold text-red-600 hover:text-red-700",
                                                onclick: move |_| cart.write().remove_item(item.product_id),
                                                "Remove"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "rounded-xl border border-gray-200 p-5 h-fit",
                            h2 { class: "text-lg font-bold text-dark mb-4", "Order summary" }
                            div { class: "flex items-center justify-between text-sm mb-2",
                                span { class: "text-body", "Subtotal" }
                                span { class: "font-semibold text-dark", "${subtotal:.2}" }
                            }
                            div { class: "flex items-center justify-between text-sm mb-2",
                                span { class: "text-body", "Shipping" }
                                span { class: "font-semibold text-dark", "Free" }
                            }
                            div { class: "border-t border-gray-100 my-4" }
                            div { class: "flex items-center justify-between",
                                span { class: "font-bold text-dark", "Total" }
                                span { class: "font-extrabold text-dark", "${subtotal:.2}" }
                            }
                            button {
                                class: "mt-5 w-full py-3 text-xs font-bold tracking-widest text-white bg-accent rounded hover:bg-amber-600 transition-colors",
                                onclick: move |_| {
                                    nav.push(Route::Checkout {});
                                },
                                "Proceed to checkout"
                            }
                        }
                    }
                }
            }

            Footer {}
        }
    }
}
