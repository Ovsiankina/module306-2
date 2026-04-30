use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Public types (shared between client and server) ─────────────────────────

/// Access level. Admins have full access; Editors can manage content only.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Role {
    Admin,
    Editor,
}

/// Public user profile returned by auth calls. Never includes the password hash.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
}

/// Label shown for recent winners: given name + first letter of family name (e.g. `Jean D.`).
/// Falls back to parsing `username_fallback` when structured names are empty.
pub fn winner_public_label(first_name: &str, last_name: &str, username_fallback: &str) -> String {
    let f = first_name.trim();
    let l = last_name.trim();
    if !f.is_empty() || !l.is_empty() {
        let first_fmt = capitalize_like_name(f);
        let initial = l
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "U".to_string());
        return format!("{first_fmt} {initial}.");
    }
    winner_label_from_username(username_fallback)
}

fn capitalize_like_name(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}

fn winner_label_from_username(username: &str) -> String {
    let trimmed = username.trim();
    if trimmed.is_empty() {
        return "User U.".to_string();
    }
    let mut words = trimmed.split_whitespace();
    let first_name_raw = words.next().unwrap_or(trimmed);
    let first_name = capitalize_like_name(first_name_raw);
    let initial_source = words.next().unwrap_or(first_name_raw);
    let initial = initial_source
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "U".to_string());
    format!("{first_name} {initial}.")
}

// ─── Server-only helpers ──────────────────────────────────────────────────────

/// Hash a password with argon2id + random salt.
/// Exposed at crate root so `db.rs` can call it for the seed user.
#[cfg(feature = "server")]
pub fn hash_password(password: &str) -> Result<String, String> {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::SaltString;
    use rand_core::OsRng;
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| e.to_string())
}

#[cfg(feature = "server")]
fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    let Ok(parsed) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

#[cfg(feature = "server")]
fn jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev-secret-change-in-production".to_string())
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i64,
    username: String,
    email: String,
    #[serde(default)]
    first_name: String,
    #[serde(default)]
    last_name: String,
    role: String,
    exp: usize,
}

#[cfg(feature = "server")]
fn encode_jwt(user: &UserDto) -> Result<String, String> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    let exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize;
    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        role: format!("{:?}", user.role),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
    .map_err(|e| e.to_string())
}

#[cfg(feature = "server")]
fn decode_jwt(token: &str) -> Option<UserDto> {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .ok()?;
    let c = data.claims;
    let role = if c.role == "Admin" { Role::Admin } else { Role::Editor };
    Some(UserDto {
        id: c.sub,
        username: c.username,
        email: c.email,
        first_name: c.first_name,
        last_name: c.last_name,
        role,
    })
}

/// Verify a JWT and require a minimum role. Used by admin server functions.
#[cfg(feature = "server")]
pub fn require_role(token: &str, min_role: &Role) -> Result<UserDto, ServerFnError> {
    let user = decode_jwt(token)
        .ok_or_else(|| ServerFnError::new("Unauthorized: invalid or expired token"))?;
    let allowed = match min_role {
        Role::Editor => true,
        Role::Admin => user.role == Role::Admin,
    };
    if !allowed {
        return Err(ServerFnError::new("Forbidden: insufficient permissions"));
    }
    Ok(user)
}

// ─── Server functions ─────────────────────────────────────────────────────────

/// Register a new user account and return a JWT.
/// POST /api/register — body: { username, email, first_name, last_name, password }
#[server]
pub async fn register(
    username: String,
    email: String,
    first_name: String,
    last_name: String,
    password: String,
) -> Result<String, ServerFnError> {
    let hash = hash_password(&password).map_err(ServerFnError::new)?;
    let pool = crate::db::pool().await;

    sqlx::query(
        "INSERT INTO users (username, email, first_name, last_name, password_hash, role)
         VALUES (?, ?, ?, ?, ?, 'Editor')",
    )
    .bind(&username)
    .bind(&email)
    .bind(&first_name)
    .bind(&last_name)
    .bind(&hash)
    .execute(pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("UNIQUE") {
            ServerFnError::new("Username or email already taken")
        } else {
            ServerFnError::new(e.to_string())
        }
    })?;

    let (id, uname, mail, fname, lname, role_str): (i64, String, String, String, String, String) =
        sqlx::query_as(
            "SELECT id, username, email, first_name, last_name, role FROM users WHERE username = ?",
        )
        .bind(&username)
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let role = if role_str == "Admin" { Role::Admin } else { Role::Editor };
    let user = UserDto {
        id,
        username: uname,
        email: mail,
        first_name: fname,
        last_name: lname,
        role,
    };
    encode_jwt(&user).map_err(ServerFnError::new)
}

/// Authenticate with username OR email + password and return a JWT.
/// POST /api/login — body: { username, password }
#[server]
pub async fn login(username: String, password: String) -> Result<String, ServerFnError> {
    let pool = crate::db::pool().await;

    let row: Option<(i64, String, String, String, String, String, String)> = sqlx::query_as(
        "SELECT id, username, email, first_name, last_name, password_hash, role
         FROM users
         WHERE username = ? OR email = ?",
    )
    .bind(&username)
    .bind(&username)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let (id, uname, email, fname, lname, hash, role_str) =
        row.ok_or_else(|| ServerFnError::new("Invalid credentials"))?;

    if !verify_password(&password, &hash) {
        return Err(ServerFnError::new("Invalid credentials"));
    }

    let role = if role_str == "Admin" { Role::Admin } else { Role::Editor };
    let user = UserDto {
        id,
        username: uname,
        email,
        first_name: fname,
        last_name: lname,
        role,
    };
    encode_jwt(&user).map_err(ServerFnError::new)
}

/// Decode a JWT and return the associated user, or None if invalid/expired.
/// POST /api/me — body: { token }
#[server]
pub async fn me(token: String) -> Result<Option<UserDto>, ServerFnError> {
    Ok(decode_jwt(&token))
}

/// No-op on the server — JWT is stateless. Client clears localStorage.
/// POST /api/logout — body: { token }
#[server]
pub async fn logout(_token: String) -> Result<(), ServerFnError> {
    Ok(())
}
