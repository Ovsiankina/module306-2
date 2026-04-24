use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Content types (shared between client and server) ─────────────────────────

/// A news item or announcement.
/// `body` accepts HTML produced by the WYSIWYG editor.
/// TODO: sanitize HTML server-side (e.g. with the `ammonia` crate) before storing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NewsItem {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A scheduled event (sale, pop-up, brand visit, etc.).
/// `date` / `end_date` use ISO 8601 format ("YYYY-MM-DD").
/// `description` accepts HTML from the WYSIWYG editor.
/// TODO: sanitize HTML server-side before storing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Event {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub date: String,
    pub end_date: Option<String>,
    pub location: String,
    pub created_at: DateTime<Utc>,
}

/// A promotional banner shown on the site.
/// `image_url` is currently a plain URL.
/// TODO: replace with a file-upload flow writing to object storage (S3/Cloudflare R2).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Banner {
    pub id: u32,
    pub title: String,
    pub image_url: String,
    pub link_url: Option<String>,
    /// Only active banners are shown to visitors. Inactive ones are visible in the admin panel only.
    pub active: bool,
    /// Lower value = shown first.
    pub display_order: u32,
}

/// Editable overlay for a store entry. Keyed by store slug.
/// Extends the immutable data in `stores.json` with CMS-managed fields.
/// `description` and `special_notice` accept HTML from the WYSIWYG editor.
/// TODO: sanitize HTML server-side before storing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ShopInfo {
    pub slug: String,
    pub description: String,
    /// Human-readable opening hours, e.g. "Mon–Sat 10:00–19:00, Sun 11:00–18:00"
    pub opening_hours: String,
    /// Optional temporary notice shown on the store page (sale, closure, etc.).
    pub special_notice: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
}

// ─── Server-side in-memory stores ─────────────────────────────────────────────

#[cfg(feature = "server")]
mod store {
    use super::{Banner, Event, NewsItem, ShopInfo};
    use chrono::Utc;
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    pub fn news() -> &'static Mutex<(u32, Vec<NewsItem>)> {
        static S: OnceLock<Mutex<(u32, Vec<NewsItem>)>> = OnceLock::new();
        // TODO: load from database on startup
        S.get_or_init(|| {
            let mut items = vec![
                NewsItem {
                    id: 1,
                    title: "Welcome to FoxTown".to_string(),
                    body: "<p>Discover your new favorite destination for luxury outlet shopping with over 160 premium brands, all with discounts up to 70% year-round.</p>".to_string(),
                    author: "Admin".to_string(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                NewsItem {
                    id: 2,
                    title: "Spring Collection Launch".to_string(),
                    body: "<p>Explore the latest spring collections from world-renowned designers. New arrivals in women's, men's, and kids' fashion available now.</p>".to_string(),
                    author: "Admin".to_string(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            ];
            Mutex::new((3, items))
        })
    }

    pub fn events() -> &'static Mutex<(u32, Vec<Event>)> {
        static S: OnceLock<Mutex<(u32, Vec<Event>)>> = OnceLock::new();
        // TODO: load from database on startup
        S.get_or_init(|| {
            let mut items = vec![
                Event {
                    id: 1,
                    title: "VIP Preview Night".to_string(),
                    description: "Join us for an exclusive evening shopping experience with special discounts and refreshments. Members only.".to_string(),
                    date: "2026-05-15".to_string(),
                    end_date: Some("2026-05-15".to_string()),
                    location: "FoxTown Main Hall".to_string(),
                    created_at: Utc::now(),
                },
                Event {
                    id: 2,
                    title: "Family Weekend".to_string(),
                    description: "Special family-friendly activities, kids' entertainment, and exclusive discounts across all participating stores.".to_string(),
                    date: "2026-05-18".to_string(),
                    end_date: Some("2026-05-19".to_string()),
                    location: "FoxTown Outlet".to_string(),
                    created_at: Utc::now(),
                },
            ];
            Mutex::new((3, items))
        })
    }

    pub fn banners() -> &'static Mutex<(u32, Vec<Banner>)> {
        static S: OnceLock<Mutex<(u32, Vec<Banner>)>> = OnceLock::new();
        // TODO: load from database on startup
        S.get_or_init(|| {
            let mut items = vec![
                Banner {
                    id: 1,
                    title: "Summer Collection - Up to 70% Off".to_string(),
                    image_url: "/editorial-fashion.png".to_string(),
                    link_url: Some("/stores".to_string()),
                    active: true,
                    display_order: 1,
                },
            ];
            Mutex::new((2, items))
        })
    }

    pub fn shop_info() -> &'static Mutex<HashMap<String, ShopInfo>> {
        static S: OnceLock<Mutex<HashMap<String, ShopInfo>>> = OnceLock::new();
        // TODO: load from database on startup
        S.get_or_init(|| Mutex::new(HashMap::new()))
    }
}

// ─── News ─────────────────────────────────────────────────────────────────────

/// Create a news item. Requires Editor or Admin role.
/// POST /api/create_news  —  body: { token, title, body }
#[server]
pub async fn create_news(
    token: String,
    title: String,
    body: String,
) -> Result<NewsItem, ServerFnError> {
    let user = crate::auth::require_role(&token, &crate::auth::Role::Editor)?;
    let now = Utc::now();
    let mut g = store::news().lock().unwrap();
    let (next_id, items) = &mut *g;
    let item = NewsItem { id: *next_id, title, body, author: user.username, created_at: now, updated_at: now };
    *next_id += 1;
    items.push(item.clone());
    // TODO: persist to database
    Ok(item)
}

/// List all news items (public).
/// POST /api/list_news  —  no body required
#[server]
pub async fn list_news() -> Result<Vec<NewsItem>, ServerFnError> {
    Ok(store::news().lock().unwrap().1.clone())
}

/// Update a news item. Requires Editor or Admin role.
/// POST /api/update_news  —  body: { token, id, title, body }
#[server]
pub async fn update_news(
    token: String,
    id: u32,
    title: String,
    body: String,
) -> Result<NewsItem, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Editor)?;
    let mut g = store::news().lock().unwrap();
    let item = g.1.iter_mut().find(|n| n.id == id)
        .ok_or_else(|| ServerFnError::new(format!("News item {id} not found")))?;
    item.title = title;
    item.body = body;
    item.updated_at = Utc::now();
    let result = item.clone();
    // TODO: persist to database
    Ok(result)
}

/// Delete a news item. Requires Admin role.
/// POST /api/delete_news  —  body: { token, id }
#[server]
pub async fn delete_news(token: String, id: u32) -> Result<(), ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Admin)?;
    let mut g = store::news().lock().unwrap();
    let before = g.1.len();
    g.1.retain(|n| n.id != id);
    if g.1.len() == before {
        return Err(ServerFnError::new(format!("News item {id} not found")));
    }
    // TODO: persist to database
    Ok(())
}

// ─── Events ───────────────────────────────────────────────────────────────────

/// Create an event. Requires Editor or Admin role.
/// POST /api/create_event  —  body: { token, title, description, date, end_date?, location }
#[server]
pub async fn create_event(
    token: String,
    title: String,
    description: String,
    date: String,
    end_date: Option<String>,
    location: String,
) -> Result<Event, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Editor)?;
    let mut g = store::events().lock().unwrap();
    let (next_id, items) = &mut *g;
    let item = Event { id: *next_id, title, description, date, end_date, location, created_at: Utc::now() };
    *next_id += 1;
    items.push(item.clone());
    // TODO: persist to database
    Ok(item)
}

/// List all events (public).
/// POST /api/list_events  —  no body required
#[server]
pub async fn list_events() -> Result<Vec<Event>, ServerFnError> {
    Ok(store::events().lock().unwrap().1.clone())
}

/// Update an event. Requires Editor or Admin role.
/// POST /api/update_event  —  body: { token, id, title, description, date, end_date?, location }
#[server]
pub async fn update_event(
    token: String,
    id: u32,
    title: String,
    description: String,
    date: String,
    end_date: Option<String>,
    location: String,
) -> Result<Event, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Editor)?;
    let mut g = store::events().lock().unwrap();
    let item = g.1.iter_mut().find(|e| e.id == id)
        .ok_or_else(|| ServerFnError::new(format!("Event {id} not found")))?;
    item.title = title;
    item.description = description;
    item.date = date;
    item.end_date = end_date;
    item.location = location;
    let result = item.clone();
    // TODO: persist to database
    Ok(result)
}

/// Delete an event. Requires Admin role.
/// POST /api/delete_event  —  body: { token, id }
#[server]
pub async fn delete_event(token: String, id: u32) -> Result<(), ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Admin)?;
    let mut g = store::events().lock().unwrap();
    let before = g.1.len();
    g.1.retain(|e| e.id != id);
    if g.1.len() == before {
        return Err(ServerFnError::new(format!("Event {id} not found")));
    }
    // TODO: persist to database
    Ok(())
}

// ─── Banners ──────────────────────────────────────────────────────────────────

/// Create a banner (inactive by default). Requires Editor or Admin role.
/// POST /api/create_banner  —  body: { token, title, image_url, link_url?, display_order }
#[server]
pub async fn create_banner(
    token: String,
    title: String,
    image_url: String,
    link_url: Option<String>,
    display_order: u32,
) -> Result<Banner, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Editor)?;
    let mut g = store::banners().lock().unwrap();
    let (next_id, items) = &mut *g;
    let item = Banner { id: *next_id, title, image_url, link_url, active: false, display_order };
    *next_id += 1;
    items.push(item.clone());
    // TODO: persist to database
    Ok(item)
}

/// List active banners only (public).
/// POST /api/list_banners  —  no body required
#[server]
pub async fn list_banners() -> Result<Vec<Banner>, ServerFnError> {
    let g = store::banners().lock().unwrap();
    let mut active: Vec<Banner> = g.1.iter().filter(|b| b.active).cloned().collect();
    active.sort_by_key(|b| b.display_order);
    Ok(active)
}

/// List all banners including inactive ones. Requires Editor or Admin role.
/// POST /api/list_all_banners  —  body: { token }
#[server]
pub async fn list_all_banners(token: String) -> Result<Vec<Banner>, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Editor)?;
    let g = store::banners().lock().unwrap();
    let mut all = g.1.clone();
    all.sort_by_key(|b| b.display_order);
    Ok(all)
}

/// Activate or deactivate a banner. Requires Editor or Admin role.
/// POST /api/set_banner_active  —  body: { token, id, active }
#[server]
pub async fn set_banner_active(
    token: String,
    id: u32,
    active: bool,
) -> Result<(), ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Editor)?;
    let mut g = store::banners().lock().unwrap();
    let banner = g.1.iter_mut().find(|b| b.id == id)
        .ok_or_else(|| ServerFnError::new(format!("Banner {id} not found")))?;
    banner.active = active;
    // TODO: persist to database
    Ok(())
}

/// Update a banner. Requires Editor or Admin role.
/// POST /api/update_banner  —  body: { token, id, title, image_url, link_url? }
#[server]
pub async fn update_banner(
    token: String,
    id: u32,
    title: String,
    image_url: String,
    link_url: Option<String>,
) -> Result<Banner, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Editor)?;
    let mut g = store::banners().lock().unwrap();
    let banner = g.1.iter_mut().find(|b| b.id == id)
        .ok_or_else(|| ServerFnError::new(format!("Banner {id} not found")))?;
    banner.title = title;
    banner.image_url = image_url;
    banner.link_url = link_url;
    let result = banner.clone();
    // TODO: persist to database
    Ok(result)
}

/// Delete a banner. Requires Admin role.
/// POST /api/delete_banner  —  body: { token, id }
#[server]
pub async fn delete_banner(token: String, id: u32) -> Result<(), ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Admin)?;
    let mut g = store::banners().lock().unwrap();
    let before = g.1.len();
    g.1.retain(|b| b.id != id);
    if g.1.len() == before {
        return Err(ServerFnError::new(format!("Banner {id} not found")));
    }
    // TODO: persist to database
    Ok(())
}

// ─── Shop info ────────────────────────────────────────────────────────────────

/// Create or update the editable overlay for a store. Requires Editor or Admin role.
/// POST /api/upsert_shop_info  —  body: { token, slug, description, opening_hours, special_notice? }
#[server]
pub async fn upsert_shop_info(
    token: String,
    slug: String,
    description: String,
    opening_hours: String,
    special_notice: Option<String>,
) -> Result<ShopInfo, ServerFnError> {
    let user = crate::auth::require_role(&token, &crate::auth::Role::Editor)?;
    let info = ShopInfo {
        slug: slug.clone(),
        description,
        opening_hours,
        special_notice,
        updated_at: Utc::now(),
        updated_by: user.username,
    };
    store::shop_info().lock().unwrap().insert(slug, info.clone());
    // TODO: persist to database
    Ok(info)
}

/// Get the editable overlay for a store by slug (public). Returns None if not yet set.
/// POST /api/get_shop_info  —  body: { slug }
#[server]
pub async fn get_shop_info(slug: String) -> Result<Option<ShopInfo>, ServerFnError> {
    Ok(store::shop_info().lock().unwrap().get(&slug).cloned())
}
