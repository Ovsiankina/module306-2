use crate::auth::{login, me, register};
use crate::context::auth::{write_token, AuthState};
use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    SignIn,
    SignUp,
}

pub fn LoginPage() -> Element {
    let mut tab = use_signal(|| Tab::SignIn);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut auth = use_context::<Signal<AuthState>>();
    let nav = use_navigator();

    rsx! {
        div { class: "node-506",
            div { class: "main-507",
                // Left side - Hero background
                div { class: "node-508",
                    img {
                        src: asset!("/public/images/node-509.png"),
                        class: "node-509",
                        alt: "luxury-retail-interior"
                    }
                    div { class: "overlay-512" }
                    div { class: "container-513",
                        div { class: "heading-1-514",
                            p { class: "text-515 text-white",
                                "FOXTOWN"
                            }
                        }
                        div { class: "shadow-516",
                            p { class: "text-517 text-white",
                                "Access the world's most exclusive luxury outlets. Your curated shopping experience starts here."
                            }
                        }
                    }
                }

                // Right side - Login form
                div { class: "node-518",
                    div { class: "container-519",
                        div { class: "container-520",
                            div { class: "heading-3-521",
                                p { class: "text-522 text-rgb-26-28-28",
                                    if tab() == Tab::SignIn { "Welcome Back" } else { "Create Account" }
                                }
                            }
                            div { class: "container-523",
                                p { class: "text-524 text-rgb-85-67-53",
                                    if tab() == Tab::SignIn {
                                        "Sign in to your member account."
                                    } else {
                                        "Join FoxTown and unlock exclusive rewards."
                                    }
                                }
                            }
                        }

                        // Error message
                        if let Some(ref msg) = error() {
                            div { class: "mb-4 px-4 py-3 rounded-md bg-red-50 border border-red-200 text-sm text-red-700 font-semibold",
                                "{msg}"
                            }
                        }

                        // Form
                        div { class: "form-525",
                            div { class: "container-526",
                                if tab() == Tab::SignUp {
                                    div { class: "email-input-527",
                                        div { class: "label-528",
                                            p { class: "text-529 text-rgb-85-67-53",
                                                "Username"
                                            }
                                        }
                                        div { class: "input-530",
                                            div { class: "container-531",
                                                input {
                                                    class: "w-full py-2 px-3 text-sm bg-transparent border-none outline-none text-rgb-85-67-53",
                                                    r#type: "text",
                                                    placeholder: "your username",
                                                    value: "{username}",
                                                    oninput: move |e| username.set(e.value()),
                                                }
                                            }
                                        }
                                    }
                                }

                                div { class: "email-input-527",
                                    div { class: "label-528",
                                        p { class: "text-529 text-rgb-85-67-53",
                                            "Email Address"
                                        }
                                    }
                                    div { class: "input-530",
                                        div { class: "container-531",
                                            input {
                                                class: "w-full py-2 px-3 text-sm bg-transparent border-none outline-none text-rgb-85-67-53",
                                                r#type: "email",
                                                placeholder: "name@email.com",
                                                value: "{email}",
                                                oninput: move |e| email.set(e.value()),
                                            }
                                        }
                                    }
                                }

                                div { class: "password-input-533",
                                    div { class: "container-534",
                                        div { class: "label-535",
                                            p { class: "text-536 text-rgb-85-67-53",
                                                "Password"
                                            }
                                        }
                                        if tab() == Tab::SignIn {
                                            div { class: "link-537",
                                                p { class: "text-538 text-rgb-237-134-6",
                                                    "Forgot Password?"
                                                }
                                            }
                                        }
                                    }
                                    div { class: "input-539",
                                        div { class: "container-540",
                                            input {
                                                class: "w-full py-2 px-3 text-sm bg-transparent border-none outline-none text-rgb-85-67-53",
                                                r#type: "password",
                                                placeholder: "••••••••",
                                                value: "{password}",
                                                oninput: move |e| password.set(e.value()),
                                            }
                                        }
                                    }
                                }

                                if tab() == Tab::SignUp {
                                    div { class: "password-input-533",
                                        div { class: "container-534",
                                            div { class: "label-535",
                                                p { class: "text-536 text-rgb-85-67-53",
                                                    "Confirm Password"
                                                }
                                            }
                                        }
                                        div { class: "input-539",
                                            div { class: "container-540",
                                                input {
                                                    class: "w-full py-2 px-3 text-sm bg-transparent border-none outline-none text-rgb-85-67-53",
                                                    r#type: "password",
                                                    placeholder: "••••••••",
                                                    value: "{confirm}",
                                                    oninput: move |e| confirm.set(e.value()),
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "container-542",
                                div { class: "node-543",
                                    button {
                                        class: "w-full px-6 py-3 text-sm font-bold text-white bg-gray-900 hover:bg-gray-700 rounded-md transition-colors",
                                        onclick: move |_| {
                                            let u = username();
                                            let em = email();
                                            let p = password();
                                            let c = confirm();
                                            let t = tab();
                                            async move {
                                                error.set(None);
                                                match t {
                                                    Tab::SignIn => {
                                                        match login(u, p).await {
                                                            Ok(token) => {
                                                                write_token(&token);
                                                                if let Ok(Some(user)) = me(token).await {
                                                                    auth.set(AuthState::LoggedIn(user));
                                                                }
                                                                nav.replace(Route::Home {});
                                                            }
                                                            Err(e) => error.set(Some(e.to_string())),
                                                        }
                                                    }
                                                    Tab::SignUp => {
                                                        if p != c {
                                                            error.set(Some("Passwords do not match.".into()));
                                                            return;
                                                        }
                                                        if u.is_empty() || em.is_empty() || p.is_empty() {
                                                            error.set(Some("All fields are required.".into()));
                                                            return;
                                                        }
                                                        match register(u, em, p).await {
                                                            Ok(token) => {
                                                                write_token(&token);
                                                                if let Ok(Some(user)) = me(token).await {
                                                                    auth.set(AuthState::LoggedIn(user));
                                                                }
                                                                nav.replace(Route::Home {});
                                                            }
                                                            Err(e) => error.set(Some(e.to_string())),
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        if tab() == Tab::SignIn { "Login" } else { "Create Account" }
                                    }
                                }

                            }
                        }

                        // Tab switcher and sign up link
                        div { class: "paragraph-563 mt-6 flex items-center justify-center gap-2",
                            p { class: "text-564 text-rgb-85-67-53 text-sm",
                                if tab() == Tab::SignIn {
                                    "Don't have an account? "
                                } else {
                                    "Already have an account? "
                                }
                            }
                            button {
                                class: "text-rgb-237-134-6 font-semibold text-sm hover:underline cursor-pointer bg-none border-none",
                                onclick: move |_| {
                                    if tab() == Tab::SignIn {
                                        tab.set(Tab::SignUp);
                                    } else {
                                        tab.set(Tab::SignIn);
                                    }
                                    error.set(None);
                                },
                                if tab() == Tab::SignIn { "Sign Up" } else { "Sign In" }
                            }
                        }
                    }
                }
            }

            // Footer
            div { class: "node-566",
                div { class: "container-567",
                    p { class: "text-568 text-rgb-26-28-28 font-bold text-lg",
                        "FOXTOWN"
                    }
                }
                div { class: "nav-margin-569",
                    div { class: "nav-570 flex gap-8",
                        div { class: "link-571",
                            p { class: "text-572 text-rgb-85-67-53 text-sm",
                                "Contact"
                            }
                        }
                        div { class: "link-573",
                            p { class: "text-574 text-rgb-85-67-53 text-sm",
                                "Directions"
                            }
                        }
                        div { class: "link-575",
                            p { class: "text-576 text-rgb-85-67-53 text-sm",
                                "Privacy"
                            }
                        }
                        div { class: "link-577",
                            p { class: "text-578 text-rgb-85-67-53 text-sm",
                                "Terms"
                            }
                        }
                    }
                }
                div { class: "margin-579",
                    div { class: "container-580",
                        p { class: "text-581 text-rgb-85-67-53 text-xs",
                            "© 2024 FOXTOWN FACTORY STORES. ALL RIGHTS RESERVED."
                        }
                    }
                }
            }
        }
    }
}
