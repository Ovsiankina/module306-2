use crate::auth::{me, UserDto};
use dioxus::prelude::*;

// ─── Auth state ───────────────────────────────────────────────────────────────

/// Current session state, provided globally via `use_context_provider`.
#[derive(Clone, Debug, PartialEq)]
pub enum AuthState {
    /// Rehydration from localStorage is in progress (initial SSR/hydration state).
    Loading,
    /// No valid session exists.
    LoggedOut,
    /// A valid JWT was found and decoded to this user.
    LoggedIn(UserDto),
}

// ─── localStorage helpers (WASM only, no-ops on the server) ──────────────────

/// Read the JWT from localStorage.
pub fn read_token() -> Option<String> {
    #[cfg(target_family = "wasm")]
    {
        return web_sys::window()?
            .local_storage()
            .ok()??
            .get_item("auth_token")
            .ok()?;
    }
    #[allow(unreachable_code)]
    None
}

/// Persist the JWT to localStorage.
pub fn write_token(token: &str) {
    #[cfg(target_family = "wasm")]
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item("auth_token", token);
    }
    let _ = token;
}

/// Remove the JWT from localStorage.
pub fn clear_token() {
    #[cfg(target_family = "wasm")]
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.remove_item("auth_token");
    }
}

// ─── AuthProvider component ───────────────────────────────────────────────────

/// Mount this above the router to expose `Signal<AuthState>` to all descendants.
///
/// On startup it reads the JWT from localStorage, validates it with `me()`,
/// and sets the signal to `LoggedIn(user)` or `LoggedOut` accordingly.
/// Protected routes read the signal and redirect while it is still `Loading`.
#[component]
pub fn AuthProvider(children: Element) -> Element {
    let mut auth = use_context_provider(|| Signal::new(AuthState::Loading));

    use_effect(move || {
        spawn(async move {
            match read_token() {
                None => auth.set(AuthState::LoggedOut),
                Some(token) => match me(token).await {
                    Ok(Some(user)) => auth.set(AuthState::LoggedIn(user)),
                    _ => {
                        clear_token();
                        auth.set(AuthState::LoggedOut);
                    }
                },
            }
        });
    });

    rsx! { {children} }
}
