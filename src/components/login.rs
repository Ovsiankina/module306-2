use crate::auth::{login, register, Role};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    SignIn,
    SignUp,
}

pub fn LoginPage() -> Element {
    let mut tab = use_signal(|| Tab::SignIn);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut success = use_signal(|| Option::<String>::None);

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
                    onclick: move |_| { tab.set(Tab::SignIn); error.set(None); success.set(None); },
                    "Sign in"
                }
                button {
                    class: if tab() == Tab::SignUp { tab_active } else { tab_inactive },
                    onclick: move |_| { tab.set(Tab::SignUp); error.set(None); success.set(None); },
                    "Create account"
                }
            }

            // Feedback banners
            if let Some(ref msg) = error() {
                div { class: "mb-4 px-4 py-3 rounded-md bg-red-50 border border-red-200 text-sm text-red-700 font-semibold",
                    "{msg}"
                }
            }
            if let Some(ref msg) = success() {
                div { class: "mb-4 px-4 py-3 rounded-md bg-green-50 border border-green-200 text-sm text-green-700 font-semibold",
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
                        let p = password();
                        let c = confirm();
                        let t = tab();
                        async move {
                            error.set(None);
                            success.set(None);
                            match t {
                                Tab::SignIn => {
                                    match login(u, p).await {
                                        Ok(_token) => {
                                            // TODO: store token in context / localStorage and redirect
                                            success.set(Some("Signed in successfully.".into()));
                                        }
                                        Err(e) => error.set(Some(e.to_string())),
                                    }
                                }
                                Tab::SignUp => {
                                    if p != c {
                                        error.set(Some("Passwords do not match.".into()));
                                        return;
                                    }
                                    if u.is_empty() || p.is_empty() {
                                        error.set(Some("Username and password are required.".into()));
                                        return;
                                    }
                                    match register(u, p, Role::Editor).await {
                                        Ok(()) => {
                                            success.set(Some("Account created. You can now sign in.".into()));
                                            tab.set(Tab::SignIn);
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
