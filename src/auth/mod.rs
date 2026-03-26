use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Public types (shared between client and server) ─────────────────────────

/// Access level. Admins have full access; Editors can manage content only.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Role {
    Admin,
    Editor,
}

/// Public profile returned by session checks. Never includes the password hash.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub role: Role,
}

// ─── Server-side internals ────────────────────────────────────────────────────

#[cfg(feature = "server")]
mod store {
    use super::{Role, User};
    use rand::Rng;
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    pub struct StoredUser {
        pub username: String,
        pub password_hash: String,
        pub role: Role,
    }

    impl StoredUser {
        pub fn to_user(&self) -> User {
            User { username: self.username.clone(), role: self.role.clone() }
        }
    }

    pub fn users() -> &'static Mutex<HashMap<String, StoredUser>> {
        static U: OnceLock<Mutex<HashMap<String, StoredUser>>> = OnceLock::new();
        U.get_or_init(|| {
            let mut map = HashMap::new();
            // TODO: load from database on startup
            // Development seed — change before going to production
            map.insert("admin".into(), StoredUser {
                username: "admin".into(),
                password_hash: hash_password("admin"),
                role: Role::Admin,
            });
            Mutex::new(map)
        })
    }

    pub fn sessions() -> &'static Mutex<HashMap<String, String>> {
        static S: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
        // TODO: persist sessions to Redis/DB so they survive server restarts
        S.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// Hash a password with a static pepper.
    /// TODO: replace with argon2 or bcrypt with per-user salts.
    pub fn hash_password(password: &str) -> String {
        let mut h = Sha256::new();
        h.update(b"foxtown-v1-");
        h.update(password.as_bytes());
        format!("{:x}", h.finalize())
    }

    pub fn generate_token() -> String {
        let bytes: [u8; 32] = rand::thread_rng().gen();
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn get_user_by_token(token: &str) -> Option<User> {
        let sessions = sessions().lock().unwrap();
        let username = sessions.get(token)?.clone();
        drop(sessions);
        users().lock().unwrap().get(&username).map(|u| u.to_user())
    }
}

/// Server-only guard: verify token and require a minimum role.
/// Import this in admin server functions via `crate::auth::require_role`.
#[cfg(feature = "server")]
pub fn require_role(token: &str, min_role: &Role) -> Result<User, ServerFnError> {
    let user = store::get_user_by_token(token)
        .ok_or_else(|| ServerFnError::new("Unauthorized: invalid or expired session"))?;
    let allowed = match min_role {
        Role::Editor => true, // Editor or Admin both pass
        Role::Admin => user.role == Role::Admin,
    };
    if !allowed {
        return Err(ServerFnError::new("Forbidden: insufficient permissions"));
    }
    Ok(user)
}

// ─── Server functions ─────────────────────────────────────────────────────────

/// Register a new user account.
/// POST /api/register
/// Body: { username, password, role }
/// TODO: restrict to Admin callers before going to production.
#[server]
pub async fn register(
    username: String,
    password: String,
    role: Role,
) -> Result<(), ServerFnError> {
    let hash = store::hash_password(&password);
    let mut users = store::users().lock().unwrap();
    if users.contains_key(&username) {
        return Err(ServerFnError::new("Username already taken"));
    }
    users.insert(username.clone(), store::StoredUser { username, password_hash: hash, role });
    // TODO: persist to database
    Ok(())
}

/// Authenticate and get a session token.
/// POST /api/login
/// Body: { username, password }
/// Returns: session token string.
/// TODO: set as HTTP-only cookie instead of returning in body (XSS protection).
#[server]
pub async fn login(username: String, password: String) -> Result<String, ServerFnError> {
    let hash = store::hash_password(&password);
    let users = store::users().lock().unwrap();
    let user = users.get(&username)
        .ok_or_else(|| ServerFnError::new("Invalid credentials"))?;
    if user.password_hash != hash {
        return Err(ServerFnError::new("Invalid credentials"));
    }
    drop(users);
    let token = store::generate_token();
    store::sessions().lock().unwrap().insert(token.clone(), username);
    Ok(token)
}

/// Invalidate a session token.
/// POST /api/logout
/// Body: { token }
#[server]
pub async fn logout(token: String) -> Result<(), ServerFnError> {
    store::sessions().lock().unwrap().remove(&token);
    Ok(())
}

/// Return the user for a session token, or None if the token is invalid.
/// POST /api/whoami
/// Body: { token }
#[server]
pub async fn whoami(token: String) -> Result<Option<User>, ServerFnError> {
    Ok(store::get_user_by_token(&token))
}
