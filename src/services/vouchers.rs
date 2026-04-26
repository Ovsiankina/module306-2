use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use std::collections::HashMap;

/// Plafond de bons émis par période UTC (minuit → minuit), aligné sur `data/vouchers.json`.
pub const MAX_VOUCHERS_PER_UTC_DAY: u32 = 10;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoucherIssueResult {
    pub email: String,
    pub username: String,
    pub store: String,
    pub discount: u32,
    pub valid_until: String,
    pub qr_code_data_url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoucherAdminSummary {
    pub username: String,
    pub store: String,
    pub discount: u32,
    pub valid_until: String,
}

/// Full voucher row for admin list (matches `data/vouchers.json` records).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoucherAdminFull {
    pub id: u64,
    pub qr_token: String,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub store: String,
    pub discount: u32,
    pub valid_until: String,
    pub created_at: String,
    pub redeemed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoucherRecentSummary {
    /// Public winner label (given name + initial), resolved from the users table when possible.
    pub display_name: String,
    pub store: String,
    pub discount: u32,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoucherVerification {
    pub valid: bool,
    pub message: String,
    pub voucher: Option<VoucherAdminSummary>,
}

#[cfg(feature = "server")]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct VoucherRecord {
    id: u64,
    qr_token: String,
    email: String,
    username: String,
    #[serde(default)]
    first_name: String,
    #[serde(default)]
    last_name: String,
    store: String,
    discount: u32,
    valid_until: String,
    created_at: String,
    redeemed: bool,
}

#[cfg(feature = "server")]
#[derive(Debug, Serialize)]
struct VoucherQrPayload {
    email: String,
    store: String,
    discount: u32,
    valid_until: String,
}

#[cfg(feature = "server")]
fn vouchers_path() -> std::path::PathBuf {
    std::env::var("VOUCHERS_JSON_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("data/vouchers.json"))
}

#[cfg(feature = "server")]
pub(crate) fn voucher_count_for_current_utc_day() -> u32 {
    let Ok(vouchers) = load_vouchers() else {
        return 0;
    };
    let now = chrono::Utc::now();
    let start = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .expect("midnight")
        .and_utc();
    let end = start + chrono::Duration::days(1);
    vouchers
        .iter()
        .filter(|v| {
            chrono::DateTime::parse_from_rfc3339(v.created_at.trim())
                .ok()
                .map(|dt| {
                    let t = dt.with_timezone(&chrono::Utc);
                    t >= start && t < end
                })
                .unwrap_or(false)
        })
        .count() as u32
}

#[cfg(feature = "server")]
pub(crate) fn active_daily_quota_cooldown_until_utc() -> Option<chrono::DateTime<chrono::Utc>> {
    let vouchers = load_vouchers().ok()?;
    let mut by_day: HashMap<chrono::NaiveDate, Vec<chrono::DateTime<chrono::Utc>>> = HashMap::new();

    for voucher in vouchers {
        let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(voucher.created_at.trim()) else {
            continue;
        };
        let created_utc = parsed.with_timezone(&chrono::Utc);
        by_day
            .entry(created_utc.date_naive())
            .or_default()
            .push(created_utc);
    }

    let now = chrono::Utc::now();
    by_day
        .into_values()
        .filter_map(|mut created_for_day| {
            if created_for_day.len() < MAX_VOUCHERS_PER_UTC_DAY as usize {
                return None;
            }
            created_for_day.sort_unstable();
            let quota_reached_at = created_for_day[MAX_VOUCHERS_PER_UTC_DAY as usize - 1];
            let cooldown_until = quota_reached_at + chrono::Duration::hours(24);
            (cooldown_until > now).then_some(cooldown_until)
        })
        .max()
}

#[cfg(feature = "server")]
fn load_vouchers() -> Result<Vec<VoucherRecord>, ServerFnError> {
    let path = vouchers_path();
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = std::fs::read_to_string(&path).map_err(|e| ServerFnError::new(e.to_string()))?;
    if content.trim().is_empty() {
        return Ok(vec![]);
    }
    serde_json::from_str(&content).map_err(|e| ServerFnError::new(e.to_string()))
}

#[cfg(feature = "server")]
fn save_vouchers(vouchers: &[VoucherRecord]) -> Result<(), ServerFnError> {
    let path = vouchers_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| ServerFnError::new(e.to_string()))?;
    }
    let serialized =
        serde_json::to_string_pretty(vouchers).map_err(|e| ServerFnError::new(e.to_string()))?;
    std::fs::write(path, serialized).map_err(|e| ServerFnError::new(e.to_string()))
}

#[cfg(feature = "server")]
fn generate_qr_svg_data_url(payload: &str) -> Result<String, ServerFnError> {
    use base64::Engine;
    use qrcode::render::svg;
    use qrcode::QrCode;

    let qr = QrCode::new(payload.as_bytes()).map_err(|e| ServerFnError::new(e.to_string()))?;
    let svg = qr
        .render::<svg::Color>()
        .min_dimensions(240, 240)
        .dark_color(svg::Color("#111827"))
        .light_color(svg::Color("#ffffff"))
        .build();
    let encoded = base64::engine::general_purpose::STANDARD.encode(svg.as_bytes());
    Ok(format!("data:image/svg+xml;base64,{encoded}"))
}

#[cfg(feature = "server")]
fn send_voucher_email(
    to: &str,
    username: &str,
    store: &str,
    discount: u32,
    valid_until: &str,
    qr_code_data_url: &str,
    verify_url: &str,
) -> Result<(), ServerFnError> {
    use lettre::message::header::ContentType;
    use lettre::message::Mailbox;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    let smtp_fake_mode = std::env::var("SMTP_FAKE_MODE")
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false);

    let smtp_host = std::env::var("SMTP_HOST")
        .map_err(|_| ServerFnError::new("SMTP_HOST is not configured".to_string()))?;
    let smtp_port = std::env::var("SMTP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(587);
    let smtp_user = std::env::var("SMTP_USER")
        .map_err(|_| ServerFnError::new("SMTP_USER is not configured".to_string()))?;
    let smtp_pass = std::env::var("SMTP_PASS")
        .map_err(|_| ServerFnError::new("SMTP_PASS is not configured".to_string()))?;
    let smtp_from =
        std::env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@foxtown.local".to_string());

    let html_body = format!(
        r#"
        <div style="font-family: Arial, sans-serif; color: #1f2937;">
          <h2>Your FoxTown voucher is ready</h2>
          <p>Hello {username},</p>
          <p>You won <strong>-{discount}%</strong> at <strong>{store}</strong>.</p>
          <p>Valid until: <strong>{valid_until}</strong></p>
          <p>Show this QR code in store:</p>
          <img src="{qr_code_data_url}" alt="Voucher QR code" style="width: 220px; height: 220px; border: 1px solid #e5e7eb; padding: 8px;" />
          <p style="margin-top: 12px;">Verification URL: <a href="{verify_url}">{verify_url}</a></p>
        </div>
        "#
    );

    if smtp_fake_mode {
        println!(
            "[SMTP FAKE MODE] Simulated voucher email\nTo: {to}\nFrom: {smtp_from}\nSubject: Your FoxTown promo QR code\nUsername: {username}\nStore: {store}\nDiscount: {discount}%\nValid until: {valid_until}\nVerification URL: {verify_url}\nQR (data URL prefix): {}",
            &qr_code_data_url.chars().take(64).collect::<String>()
        );
        return Ok(());
    }

    let email = Message::builder()
        .from(
            smtp_from
                .parse::<Mailbox>()
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        )
        .to(to.parse::<Mailbox>().map_err(|e| ServerFnError::new(e.to_string()))?)
        .subject("Your FoxTown promo QR code")
        .header(ContentType::TEXT_HTML)
        .body(html_body)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let mailer = SmtpTransport::starttls_relay(&smtp_host)
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .port(smtp_port)
        .credentials(Credentials::new(smtp_user, smtp_pass))
        .build();

    mailer
        .send(&email)
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[cfg(feature = "server")]
fn is_voucher_active(voucher: &VoucherRecord, today: &str) -> bool {
    !voucher.redeemed && voucher.valid_until.as_str() >= today
}

#[cfg(feature = "server")]
async fn lookup_user_names(
    pool: &sqlx::SqlitePool,
    username: &str,
    email: &str,
) -> (String, String) {
    let by_username: Option<(String, String)> = sqlx::query_as(
        "SELECT first_name, last_name FROM users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();
    if let Some(names) = by_username {
        return names;
    }
    sqlx::query_as("SELECT first_name, last_name FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or_else(|| (String::new(), String::new()))
}

#[server]
pub async fn create_voucher_and_send_email(
    token: String,
    email: String,
    username: String,
    store: String,
    discount: u32,
    valid_until: String,
) -> Result<VoucherIssueResult, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Editor)?;

    #[cfg(feature = "server")]
    {
        if voucher_count_for_current_utc_day() >= MAX_VOUCHERS_PER_UTC_DAY {
            return Err(ServerFnError::new(
                "Quota journalier de bons atteint (minuit UTC).".to_string(),
            ));
        }
        let mut vouchers = load_vouchers()?;
        let next_id = vouchers.iter().map(|v| v.id).max().unwrap_or(0) + 1;
        let qr_token = uuid::Uuid::new_v4().to_string();
        let base_url =
            std::env::var("APP_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let verify_url = format!("{base_url}/voucher/verify?token={qr_token}");
        let qr_payload = VoucherQrPayload {
            email: email.clone(),
            store: store.clone(),
            discount,
            valid_until: valid_until.clone(),
        };
        let qr_payload_json =
            serde_json::to_string(&qr_payload).map_err(|e| ServerFnError::new(e.to_string()))?;
        let qr_code_data_url = generate_qr_svg_data_url(&qr_payload_json)?;

        let pool = crate::db::pool().await;
        let (fname, lname) = lookup_user_names(pool, &username, &email).await;

        let record = VoucherRecord {
            id: next_id,
            qr_token,
            email: email.clone(),
            username: username.clone(),
            first_name: fname,
            last_name: lname,
            store: store.clone(),
            discount,
            valid_until: valid_until.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
            redeemed: false,
        };
        vouchers.push(record);
        save_vouchers(&vouchers)?;
        send_voucher_email(
            &email,
            &username,
            &store,
            discount,
            &valid_until,
            &qr_code_data_url,
            &verify_url,
        )?;

        return Ok(VoucherIssueResult {
            email,
            username,
            store,
            discount,
            valid_until,
            qr_code_data_url,
        });
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = (token, email, username, store, discount, valid_until);
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

#[server]
pub async fn list_all_vouchers_admin(token: String) -> Result<Vec<VoucherAdminFull>, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Admin)?;

    #[cfg(feature = "server")]
    {
        let mut vouchers: Vec<VoucherAdminFull> = load_vouchers()?
            .into_iter()
            .map(|v| VoucherAdminFull {
                id: v.id,
                qr_token: v.qr_token,
                email: v.email,
                username: v.username,
                first_name: v.first_name,
                last_name: v.last_name,
                store: v.store,
                discount: v.discount,
                valid_until: v.valid_until,
                created_at: v.created_at,
                redeemed: v.redeemed,
            })
            .collect();
        vouchers.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        return Ok(vouchers);
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = token;
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

/// Remove all vouchers with `redeemed: true` from the JSON store. Returns how many were removed.
#[server]
pub async fn purge_redeemed_vouchers(token: String) -> Result<u32, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Admin)?;

    #[cfg(feature = "server")]
    {
        let mut vouchers = load_vouchers()?;
        let before = vouchers.len();
        vouchers.retain(|v| !v.redeemed);
        let removed = before.saturating_sub(vouchers.len()) as u32;
        save_vouchers(&vouchers)?;
        return Ok(removed);
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = token;
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

#[server]
pub async fn list_active_vouchers(token: String) -> Result<Vec<VoucherAdminSummary>, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Admin)?;

    #[cfg(feature = "server")]
    {
        let today = chrono::Utc::now().date_naive().to_string();
        let mut active: Vec<VoucherAdminSummary> = load_vouchers()?
            .into_iter()
            .filter(|v| is_voucher_active(v, &today))
            .map(|v| VoucherAdminSummary {
                username: v.username,
                store: v.store,
                discount: v.discount,
                valid_until: v.valid_until,
            })
            .collect();
        active.sort_by(|a, b| a.valid_until.cmp(&b.valid_until));
        return Ok(active);
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = token;
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

#[server]
pub async fn list_recent_vouchers(limit: usize) -> Result<Vec<VoucherRecentSummary>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let take = if limit == 0 { 8 } else { limit.min(20) };
        let mut recent = load_vouchers()?;
        recent.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let pool = crate::db::pool().await;
        let mut winners = Vec::new();
        for v in recent.into_iter().take(take) {
            let (fname, lname) = if !v.first_name.trim().is_empty() || !v.last_name.trim().is_empty() {
                (v.first_name.clone(), v.last_name.clone())
            } else {
                lookup_user_names(pool, &v.username, &v.email).await
            };
            let display_name = crate::auth::winner_public_label(&fname, &lname, &v.username);
            winners.push(VoucherRecentSummary {
                display_name,
                store: v.store,
                discount: v.discount,
                created_at: v.created_at,
            });
        }
        return Ok(winners);
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = limit;
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

#[server]
pub async fn verify_voucher(qr_token: String) -> Result<VoucherVerification, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let today = chrono::Utc::now().date_naive().to_string();
        let vouchers = load_vouchers()?;
        if let Some(voucher) = vouchers.iter().find(|v| v.qr_token == qr_token) {
            let active = is_voucher_active(voucher, &today);
            let message = if active {
                "Voucher is valid".to_string()
            } else if voucher.redeemed {
                "Voucher already redeemed".to_string()
            } else {
                "Voucher expired".to_string()
            };
            return Ok(VoucherVerification {
                valid: active,
                message,
                voucher: Some(VoucherAdminSummary {
                    username: voucher.username.clone(),
                    store: voucher.store.clone(),
                    discount: voucher.discount,
                    valid_until: voucher.valid_until.clone(),
                }),
            });
        }
        return Ok(VoucherVerification {
            valid: false,
            message: "Voucher not found".to_string(),
            voucher: None,
        });
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = qr_token;
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}
