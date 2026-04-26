use crate::auth::{login, me, register};
use crate::context::auth::{write_token, AuthState};
use crate::i18n::{Locale, translate};
use crate::Route;
use dioxus::prelude::*;

const LOGIN_FOOTER_LINK_HREFS: [&str; 4] = ["/contact", "/map", "/privacy", "/terms"];

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    SignIn,
    SignUp,
}

pub fn LoginPage() -> Element {
    let locale = use_context::<Signal<Locale>>();
    let mut tab = use_signal(|| Tab::SignIn);
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut first_name = use_signal(String::new);
    let mut last_name = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut auth = use_context::<Signal<AuthState>>();
    let nav = use_navigator();

    let input_class = "w-full py-3.5 px-4 text-sm bg-white border border-gray-200 rounded-lg placeholder-muted focus:ring-accent focus:border-accent focus:outline-none text-dark";

    rsx! {
        div { class: "min-h-screen flex flex-col",
            div { class: "flex flex-1",

                // ─── Left hero panel ────────────────────────────────────
                div { class: "hidden lg:flex lg:w-1/2 relative bg-gray-900 items-end",
                    // Hero image
                    img {
                        src: "/login-hero.png",
                        class: "absolute inset-0 w-full h-full object-cover",
                        alt: "",
                    }
                    // Gradient overlay
                    div { class: "absolute inset-0 bg-gradient-to-t from-black/70 via-black/30 to-black/10" }

                    // Branding content
                    div { class: "relative z-10 p-12 pb-16 w-full",
                        h1 { class: "text-5xl font-extrabold tracking-widest text-white mb-6",
                            {translate(locale(), "nav.logo")}
                        }
                        p { class: "text-white/80 text-lg leading-relaxed max-w-md",
                            {translate(locale(), "login.hero")}
                        }
                    }
                }

                // ─── Right form panel ───────────────────────────────────
                div { class: "w-full lg:w-1/2 flex flex-col",
                    div { class: "flex-1 flex items-center justify-center px-6 py-12",
                        div { class: "w-full max-w-sm",

                            // Header
                            h2 { class: "text-2xl font-bold text-dark mb-2",
                                {if tab() == Tab::SignIn { translate(locale(), "login.welcome") } else { translate(locale(), "login.create_account") }}
                            }
                            p { class: "text-sm text-body mb-8",
                                if tab() == Tab::SignIn {
                                    {translate(locale(), "login.signin_subtitle")}
                                } else {
                                    {translate(locale(), "login.signup_subtitle")}
                                }
                            }

                            // Error banner
                            if let Some(ref msg) = error() {
                                div { class: "mb-4 px-4 py-3 rounded-lg bg-red-50 border border-red-200 text-sm text-red-700 font-medium",
                                    "{msg}"
                                }
                            }

                            // Form
                            div { class: "space-y-5",

                                // Email / Username field
                                div {
                                    label { class: "block text-sm font-medium text-body mb-1.5",
                                        {if tab() == Tab::SignIn { translate(locale(), "login.identity_or_email") } else { translate(locale(), "login.username") }}
                                    }
                                    input {
                                        class: "{input_class}",
                                        r#type: "text",
                                        placeholder: if tab() == Tab::SignIn { "name@email.com or username" } else { "username" },
                                        value: "{username}",
                                        oninput: move |e| username.set(e.value()),
                                    }
                                }

                                // Email (sign-up only)
                                if tab() == Tab::SignUp {
                                    div {
                                        label { class: "block text-sm font-medium text-body mb-1.5", {translate(locale(), "login.email")} }
                                        input {
                                            class: "{input_class}",
                                            r#type: "email",
                                            placeholder: "name@email.com",
                                            value: "{email}",
                                            oninput: move |e| email.set(e.value()),
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-body mb-1.5", {translate(locale(), "login.first_name")} }
                                        input {
                                            class: "{input_class}",
                                            r#type: "text",
                                            placeholder: { translate(locale(), "login.first_name") },
                                            value: "{first_name}",
                                            oninput: move |e| first_name.set(e.value()),
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-body mb-1.5", {translate(locale(), "login.last_name")} }
                                        input {
                                            class: "{input_class}",
                                            r#type: "text",
                                            placeholder: { translate(locale(), "login.last_name") },
                                            value: "{last_name}",
                                            oninput: move |e| last_name.set(e.value()),
                                        }
                                    }
                                }

                                // Password field
                                div {
                                    div { class: "flex items-center justify-between mb-1.5",
                                        label { class: "text-sm font-medium text-body", {translate(locale(), "login.password")} }
                                        if tab() == Tab::SignIn {
                                            button {
                                                class: "text-sm font-medium text-accent hover:underline",
                                                r#type: "button",
                                                onclick: move |_| {
                                                    nav.push(Route::Contact {});
                                                },
                                                {translate(locale(), "login.forgot_password")}
                                            }
                                        }
                                    }
                                    input {
                                        class: "{input_class}",
                                        r#type: "password",
                                        placeholder: "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}",
                                        value: "{password}",
                                        oninput: move |e| password.set(e.value()),
                                    }
                                }

                                // Confirm password (sign-up only)
                                if tab() == Tab::SignUp {
                                    div {
                                        label { class: "block text-sm font-medium text-body mb-1.5", {translate(locale(), "login.confirm_password")} }
                                        input {
                                            class: "{input_class}",
                                            r#type: "password",
                                            placeholder: "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}",
                                            value: "{confirm}",
                                            oninput: move |e| confirm.set(e.value()),
                                        }
                                    }
                                }

                                // Submit button
                                button {
                                    class: "w-full py-3.5 text-sm font-bold text-white bg-accent hover:bg-amber-600 rounded-lg transition-colors",
                                    onclick: move |_| {
                                        let u = username();
                                        let em = email();
                                        let fnam = first_name();
                                        let lnam = last_name();
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
                                                        error.set(Some(translate(locale(), "login.error.password_mismatch")));
                                                        return;
                                                    }
                                                    if u.is_empty()
                                                        || em.is_empty()
                                                        || fnam.trim().is_empty()
                                                        || lnam.trim().is_empty()
                                                        || p.is_empty()
                                                    {
                                                        error.set(Some(translate(locale(), "login.error.required")));
                                                        return;
                                                    }
                                                    match register(u, em, fnam, lnam, p).await {
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
                                    {if tab() == Tab::SignIn { translate(locale(), "login.login_button") } else { translate(locale(), "login.create_button") }}
                                }

                                // Divider
                                if tab() == Tab::SignIn {
                                    div { class: "relative my-2",
                                        div { class: "absolute inset-0 flex items-center",
                                            div { class: "w-full border-t border-gray-200" }
                                        }
                                        div { class: "relative flex justify-center",
                                            span { class: "px-3 bg-white text-sm text-body", {translate(locale(), "login.or_continue")} }
                                        }
                                    }

                                    // Social login buttons
                                    div { class: "flex gap-3",
                                        button { class: "flex-1 flex items-center justify-center gap-2 py-3 px-4 border border-gray-200 rounded-lg text-sm font-medium text-dark hover:bg-gray-50 transition-colors",
                                            img { src: "/google-icon.png", width: "18", height: "18", alt: "Google" }
                                            "Google"
                                        }
                                        button { class: "flex-1 flex items-center justify-center gap-2 py-3 px-4 border border-gray-200 rounded-lg text-sm font-medium text-dark hover:bg-gray-50 transition-colors",
                                            // Apple icon placeholder
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                width: "16",
                                                height: "16",
                                                view_box: "0 0 24 24",
                                                fill: "currentColor",
                                                path { d: "M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.8-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z" }
                                            }
                                            "Apple"
                                        }
                                    }
                                }
                            }

                            // Switch tab
                            p { class: "mt-8 text-center text-sm text-body",
                                if tab() == Tab::SignIn {
                                    {format!("{} ", translate(locale(), "login.dont_have"))}
                                } else {
                                    {format!("{} ", translate(locale(), "login.already_have"))}
                                }
                                button {
                                    class: "font-semibold text-accent hover:underline",
                                    onclick: move |_| {
                                        error.set(None);
                                        if tab() == Tab::SignIn { tab.set(Tab::SignUp) } else { tab.set(Tab::SignIn) }
                                    },
                                    {if tab() == Tab::SignIn { translate(locale(), "login.sign_up") } else { translate(locale(), "login.sign_in") }}
                                }
                            }
                        }
                    }

                    // Login page footer
                    div { class: "px-6 py-8 border-t border-gray-100",
                        div { class: "max-w-sm mx-auto",
                            p { class: "text-sm font-bold tracking-widest text-dark mb-3", {translate(locale(), "nav.logo")} }
                            div { class: "flex flex-wrap gap-4 mb-4",
                                a { class: "text-xs text-body hover:text-dark", href: LOGIN_FOOTER_LINK_HREFS[0], {translate(locale(), "footer.contact")} }
                                a { class: "text-xs text-body hover:text-dark", href: LOGIN_FOOTER_LINK_HREFS[1], {translate(locale(), "footer.directions")} }
                                a { class: "text-xs text-body hover:text-dark", href: LOGIN_FOOTER_LINK_HREFS[2], {translate(locale(), "footer.privacy")} }
                                a { class: "text-xs text-body hover:text-dark", href: LOGIN_FOOTER_LINK_HREFS[3], {translate(locale(), "footer.terms")} }
                            }
                            p { class: "text-xs text-body",
                                {translate(locale(), "footer.copyright")}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LOGIN_FOOTER_LINK_HREFS;

    #[test]
    fn login_footer_links_are_real_routes() {
        for href in LOGIN_FOOTER_LINK_HREFS {
            assert_ne!(href, "#");
            assert!(href.starts_with('/'));
        }
    }
}
