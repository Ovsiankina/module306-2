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
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut auth = use_context::<Signal<AuthState>>();
    let nav = use_navigator();

    let input_class = "w-full py-4 px-6 text-sm font-semibold font-heading bg-gray-50 border border-gray-200 rounded-md placeholder-gray-400 focus:ring-blue-300 focus:border-blue-300 focus:outline-none";
    let tab_active = "pb-3 border-b-2 border-gray-900 text-sm font-bold font-heading";
    let tab_inactive = "pb-3 border-b-2 border-transparent text-sm font-semibold font-heading text-gray-400 hover:text-gray-700";
    let heading = if tab() == Tab::SignIn { "Sign in" } else { "Create account" };
    let submit_label = if tab() == Tab::SignIn { "Sign in" } else { "Create account" };

    rsx! {
        section { class: "max-w-md mx-auto px-6 py-16",
            h1 { class: "text-3xl font-bold font-heading mb-8", "{heading}" }

            // Tab switcher
            div { class: "flex gap-8 mb-8",
                button {
                    class: if tab() == Tab::SignIn { tab_active } else { tab_inactive },
                    onclick: move |_| { tab.set(Tab::SignIn); error.set(None); },
                    "Sign in"
                }
                button {
                    class: if tab() == Tab::SignUp { tab_active } else { tab_inactive },
                    onclick: move |_| { tab.set(Tab::SignUp); error.set(None); },
                    "Create account"
                }
            }

            // Error banner
            if let Some(ref msg) = error() {
                div { class: "mb-4 px-4 py-3 rounded-md bg-red-50 border border-red-200 text-sm text-red-700 font-semibold",
                    "{msg}"
                }
            }

            // Form fields
            div { class: "flex flex-col gap-3",
                input {
                    class: "{input_class}",
                    r#type: "text",
                    placeholder: "Username",
                    value: "{username}",
                    oninput: move |e| username.set(e.value()),
                }
                if tab() == Tab::SignUp {
                    input {
                        class: "{input_class}",
                        r#type: "email",
                        placeholder: "Email",
                        value: "{email}",
                        oninput: move |e| email.set(e.value()),
                    }
                }
                input {
                    class: "{input_class}",
                    r#type: "password",
                    placeholder: "Password",
                    value: "{password}",
                    oninput: move |e| password.set(e.value()),
                }
                if tab() == Tab::SignUp {
                    input {
                        class: "{input_class}",
                        r#type: "password",
                        placeholder: "Confirm password",
                        value: "{confirm}",
                        oninput: move |e| confirm.set(e.value()),
                    }
                }

                button {
                    class: "w-full py-4 px-6 text-sm font-bold font-heading text-white bg-gray-900 hover:bg-gray-700 rounded-md transition-colors",
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
                    "{submit_label}"
                }
            }
        }
    }
}
