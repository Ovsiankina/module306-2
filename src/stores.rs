use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

// Embedded at compile time — used by the client/WASM and as the seed for the
// SQLite store catalog on first boot.
const STORES_JSON: &str = include_str!("../migrations/seeders/stores.json");
const MAX_BRAND_ICON_SIZE_BYTES: usize = 5 * 1024 * 1024;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Store {
    pub name: String,
    pub category: Category,
    pub store_number: Option<String>,
    pub level: Option<u8>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub icon_path: Option<String>,
    /// Image-relative X position on the floor plan, in percent (0..100).
    /// `None` means the store has not been placed on the map yet.
    #[serde(default)]
    pub map_x: Option<f32>,
    /// Image-relative Y position on the floor plan, in percent (0..100).
    #[serde(default)]
    pub map_y: Option<f32>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct StoreAdminRow {
    pub id: i64,
    pub name: String,
    pub category: Category,
    pub store_number: Option<String>,
    pub level: Option<u8>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub icon_path: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category {
    HighFashion,
    LadiesMenswear,
    Casualwear,
    SportswearEquipment,
    Childrenswear,
    Footwear,
    Underwear,
    WatchesJewellery,
    Accessories,
    Electronics,
    Beauty,
    Home,
    FoodDrinks,
    Services,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::HighFashion => "High Fashion",
            Category::LadiesMenswear => "Ladies & Menswear",
            Category::Casualwear => "Casualwear",
            Category::SportswearEquipment => "Sportswear & Equipment",
            Category::Childrenswear => "Childrenswear",
            Category::Footwear => "Footwear",
            Category::Underwear => "Underwear",
            Category::WatchesJewellery => "Watches & Jewellery",
            Category::Accessories => "Accessories",
            Category::Electronics => "Electronics",
            Category::Beauty => "Beauty",
            Category::Home => "Home",
            Category::FoodDrinks => "Food & Drinks",
            Category::Services => "Services",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Category::HighFashion => "HIGH_FASHION",
            Category::LadiesMenswear => "LADIES_MENSWEAR",
            Category::Casualwear => "CASUALWEAR",
            Category::SportswearEquipment => "SPORTSWEAR_EQUIPMENT",
            Category::Childrenswear => "CHILDRENSWEAR",
            Category::Footwear => "FOOTWEAR",
            Category::Underwear => "UNDERWEAR",
            Category::WatchesJewellery => "WATCHES_JEWELLERY",
            Category::Accessories => "ACCESSORIES",
            Category::Electronics => "ELECTRONICS",
            Category::Beauty => "BEAUTY",
            Category::Home => "HOME",
            Category::FoodDrinks => "FOOD_DRINKS",
            Category::Services => "SERVICES",
        }
    }

    /// i18n key for this category's display label (shared across pages).
    pub fn label_key(&self) -> &'static str {
        match self {
            Category::HighFashion => "home.category.luxury_fashion",
            Category::LadiesMenswear => "home.category.fashion",
            Category::Casualwear => "home.category.casualwear",
            Category::SportswearEquipment => "home.category.sport_performance",
            Category::Childrenswear => "home.category.kidswear",
            Category::Footwear => "home.category.footwear",
            Category::Underwear => "home.category.underwear",
            Category::WatchesJewellery => "home.category.luxury_heritage",
            Category::Accessories => "home.category.accessories",
            Category::Electronics => "home.category.electronics",
            Category::Beauty => "home.category.beauty",
            Category::Home => "home.category.home_lifestyle",
            Category::FoodDrinks => "home.category.food_drinks",
            Category::Services => "home.category.services",
        }
    }

    pub fn all() -> Vec<Category> {
        vec![
            Category::HighFashion,
            Category::LadiesMenswear,
            Category::Casualwear,
            Category::SportswearEquipment,
            Category::Childrenswear,
            Category::Footwear,
            Category::Underwear,
            Category::WatchesJewellery,
            Category::Accessories,
            Category::Electronics,
            Category::Beauty,
            Category::Home,
            Category::FoodDrinks,
            Category::Services,
        ]
    }

    pub fn from_key(value: &str) -> Option<Self> {
        Some(match value {
            "HIGH_FASHION" => Category::HighFashion,
            "LADIES_MENSWEAR" => Category::LadiesMenswear,
            "CASUALWEAR" => Category::Casualwear,
            "SPORTSWEAR_EQUIPMENT" => Category::SportswearEquipment,
            "CHILDRENSWEAR" => Category::Childrenswear,
            "FOOTWEAR" => Category::Footwear,
            "UNDERWEAR" => Category::Underwear,
            "WATCHES_JEWELLERY" => Category::WatchesJewellery,
            "ACCESSORIES" => Category::Accessories,
            "ELECTRONICS" => Category::Electronics,
            "BEAUTY" => Category::Beauty,
            "HOME" => Category::Home,
            "FOOD_DRINKS" => Category::FoodDrinks,
            "SERVICES" => Category::Services,
            _ => return None,
        })
    }
}

#[derive(Deserialize)]
struct StoresData {
    shops: Vec<Store>,
}

fn load_stores_from_str(json: &str) -> Vec<Store> {
    serde_json::from_str::<StoresData>(json)
        .expect("stores.json is invalid")
        .shops
}

fn load_embedded_stores() -> Vec<Store> {
    load_stores_from_str(STORES_JSON)
}

// Embedded snapshot — always available, used by client-side code and
// non-server callers (e.g. SSR component bodies that need synchronous access).
static EMBEDDED_STORES: LazyLock<Vec<Store>> = LazyLock::new(load_embedded_stores);

fn embedded_stores() -> &'static [Store] {
    EMBEDDED_STORES.as_slice()
}

pub fn get_store_local(slug: &str) -> Option<Store> {
    embedded_stores()
        .iter()
        .find(|s| slugify(&s.name) == slug)
        .cloned()
}

/// Tailwind classes for a floor's marker pill (background, border, text).
/// Shared between the public map and the admin editor so the two stay
/// visually consistent.
pub fn floor_marker_classes(level: u8) -> &'static str {
    match level {
        0 => "bg-yellow-400 border-yellow-700 text-yellow-900",
        1 => "bg-red-500 border-red-800 text-white",
        2 => "bg-blue-500 border-blue-800 text-white",
        _ => "bg-green-500 border-green-800 text-white",
    }
}

// --- Slug ---

pub fn slugify(name: &str) -> String {
    let raw: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_lowercase().next().unwrap() } else { '-' })
        .collect();
    raw.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-")
}

#[cfg(feature = "server")]
fn normalize_upload_filename(file_name: &str) -> String {
    let mut normalized = file_name.trim().replace(' ', "_");
    if normalized.is_empty() {
        normalized = "brand".to_string();
    }
    normalized
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
}

#[cfg(feature = "server")]
fn split_stem_and_ext(file_name: &str) -> (&str, Option<&str>) {
    match file_name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() => (stem, Some(ext)),
        _ => (file_name, None),
    }
}

#[cfg(feature = "server")]
fn allowed_extension_from_bytes(bytes: &[u8]) -> Option<&'static str> {
    let kind = infer::get(bytes)?;
    match kind.mime_type() {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/webp" => Some("webp"),
        _ => None,
    }
}

#[cfg(feature = "server")]
async fn persist_brand_image(
    original_filename: String,
    file_bytes: Vec<u8>,
) -> Result<String, ServerFnError> {
    if file_bytes.is_empty() {
        return Err(ServerFnError::new("Uploaded file is empty"));
    }
    if file_bytes.len() > MAX_BRAND_ICON_SIZE_BYTES {
        return Err(ServerFnError::new(format!(
            "File too large: {} bytes (max {} bytes)",
            file_bytes.len(),
            MAX_BRAND_ICON_SIZE_BYTES
        )));
    }

    let sniff_len = file_bytes.len().min(512);
    let detected_ext = allowed_extension_from_bytes(&file_bytes[..sniff_len])
        .ok_or_else(|| ServerFnError::new("Only JPEG, PNG or WEBP images are allowed"))?;

    let normalized = normalize_upload_filename(&original_filename);
    let (stem, _) = split_stem_and_ext(&normalized);
    let safe_stem = if stem.is_empty() { "brand" } else { stem };
    let final_filename = format!(
        "{}_{}.{}",
        safe_stem,
        uuid::Uuid::new_v4().simple(),
        detected_ext
    );

    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let public_dir = root.join("public").join("brands");
    let target_path = public_dir.join(&final_filename);

    tokio::fs::create_dir_all(&public_dir)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create public/brands: {e}")))?;
    tokio::fs::write(&target_path, &file_bytes)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to write uploaded file: {e}")))?;

    Ok(format!("/brands/{final_filename}"))
}

#[cfg(test)]
mod tests {
    use super::{Category, get_store_local, slugify};
    use std::collections::HashSet;

    #[test]
    fn slugify_normalizes_common_store_names() {
        assert_eq!(slugify("Tommy Hilfiger"), "tommy-hilfiger");
        assert_eq!(slugify("Dolce & Gabbana"), "dolce-gabbana");
        assert_eq!(slugify("Marc O'Polo"), "marc-o-polo");
    }

    #[test]
    fn slugify_collapses_non_alnum_runs() {
        assert_eq!(slugify("  New   Balance  "), "new-balance");
        assert_eq!(slugify("A---B"), "a-b");
        assert_eq!(slugify("__A__B__"), "a-b");
    }

    #[test]
    fn slugify_preserves_numbers_and_unicode_letters() {
        assert_eq!(slugify("7 For All Mankind"), "7-for-all-mankind");
        assert_eq!(slugify("Été 2026"), "été-2026");
    }

    #[test]
    fn category_all_has_unique_and_complete_keys() {
        let all = Category::all();
        assert_eq!(all.len(), 14);

        let keys: HashSet<&'static str> = all.iter().map(Category::key).collect();
        assert_eq!(keys.len(), all.len());
        assert!(keys.contains("HIGH_FASHION"));
        assert!(keys.contains("SERVICES"));
    }

    #[test]
    fn category_labels_match_expected_copy() {
        assert_eq!(Category::HighFashion.label(), "High Fashion");
        assert_eq!(Category::LadiesMenswear.label(), "Ladies & Menswear");
        assert_eq!(Category::FoodDrinks.label(), "Food & Drinks");
    }

    #[test]
    fn get_store_local_returns_known_store_from_slug() {
        let store = get_store_local("akris");
        assert!(store.is_some());
        assert_eq!(store.expect("store should exist").name, "Akris");
    }

    #[test]
    fn get_store_local_returns_none_for_unknown_slug() {
        assert!(get_store_local("this-store-does-not-exist").is_none());
    }

}

// --- Server functions ---

#[server]
pub async fn get_store(slug: String) -> Result<Store, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let rows: Vec<(String, String, Option<String>, Option<i64>, Option<String>, Option<String>, Option<String>, Option<f64>, Option<f64>)> =
            sqlx::query_as(
                "SELECT name, category, store_number, level, phone, website, icon_path, map_x, map_y
                 FROM stores
                 ORDER BY name",
            )
            .fetch_all(pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        for (name, category, store_number, level, phone, website, icon_path, map_x, map_y) in rows {
            if slugify(&name) == slug {
                let parsed_category = Category::from_key(&category)
                    .ok_or_else(|| ServerFnError::new(format!("Unknown store category '{category}'")))?;
                return Ok(Store {
                    name,
                    category: parsed_category,
                    store_number,
                    level: level.map(|v| v as u8),
                    phone,
                    website,
                    icon_path,
                    map_x: map_x.map(|v| v as f32),
                    map_y: map_y.map(|v| v as f32),
                });
            }
        }
        return Err(ServerFnError::new(format!("Store '{}' not found", slug)));
    }

    #[cfg(not(feature = "server"))]
    {
        embedded_stores()
            .iter()
            .find(|s| slugify(&s.name) == slug)
            .cloned()
            .ok_or_else(|| ServerFnError::new(format!("Store '{}' not found", slug)))
    }
}

#[server]
pub async fn get_stores() -> Result<Vec<Store>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let rows: Vec<(String, String, Option<String>, Option<i64>, Option<String>, Option<String>, Option<String>, Option<f64>, Option<f64>)> =
            sqlx::query_as(
                "SELECT name, category, store_number, level, phone, website, icon_path, map_x, map_y
                 FROM stores
                 ORDER BY name",
            )
            .fetch_all(pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let mut stores = Vec::with_capacity(rows.len());
        for (name, category, store_number, level, phone, website, icon_path, map_x, map_y) in rows {
            let parsed_category = Category::from_key(&category)
                .ok_or_else(|| ServerFnError::new(format!("Unknown store category '{category}'")))?;
            stores.push(Store {
                name,
                category: parsed_category,
                store_number,
                level: level.map(|v| v as u8),
                phone,
                website,
                icon_path,
                map_x: map_x.map(|v| v as f32),
                map_y: map_y.map(|v| v as f32),
            });
        }
        return Ok(stores);
    }

    #[cfg(not(feature = "server"))]
    {
        Ok(embedded_stores().to_vec())
    }
}

#[server]
pub async fn search_stores(query: String) -> Result<Vec<Store>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let like = format!("%{}%", query.trim().to_lowercase());
        let rows: Vec<(String, String, Option<String>, Option<i64>, Option<String>, Option<String>, Option<String>, Option<f64>, Option<f64>)> =
            sqlx::query_as(
                "SELECT name, category, store_number, level, phone, website, icon_path, map_x, map_y
                 FROM stores
                 WHERE LOWER(name) LIKE ?
                 ORDER BY name",
            )
            .bind(like)
            .fetch_all(pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let mut stores = Vec::with_capacity(rows.len());
        for (name, category, store_number, level, phone, website, icon_path, map_x, map_y) in rows {
            let parsed_category = Category::from_key(&category)
                .ok_or_else(|| ServerFnError::new(format!("Unknown store category '{category}'")))?;
            stores.push(Store {
                name,
                category: parsed_category,
                store_number,
                level: level.map(|v| v as u8),
                phone,
                website,
                icon_path,
                map_x: map_x.map(|v| v as f32),
                map_y: map_y.map(|v| v as f32),
            });
        }
        return Ok(stores);
    }

    #[cfg(not(feature = "server"))]
    {
        let needle = query.trim().to_lowercase();
        Ok(embedded_stores()
            .iter()
            .filter(|s| needle.is_empty() || s.name.to_lowercase().contains(&needle))
            .cloned()
            .collect())
    }
}

#[server]
pub async fn get_stores_by_category(category: Category) -> Result<Vec<Store>, ServerFnError> {
    Ok(get_stores()
        .await?
        .into_iter()
        .filter(|s| s.category == category)
        .collect())
}

#[server]
pub async fn get_stores_by_level(level: u8) -> Result<Vec<Store>, ServerFnError> {
    Ok(get_stores()
        .await?
        .into_iter()
        .filter(|s| s.level == Some(level))
        .collect())
}

#[server]
pub async fn list_store_rows() -> Result<Vec<StoreAdminRow>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let rows: Vec<(
            i64,
            String,
            String,
            Option<String>,
            Option<i64>,
            Option<String>,
            Option<String>,
            Option<String>,
        )> = sqlx::query_as(
            "SELECT id, name, category, store_number, level, phone, website, icon_path
             FROM stores
             ORDER BY name, level, store_number",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let mut stores = Vec::with_capacity(rows.len());
        for (id, name, category, store_number, level, phone, website, icon_path) in rows {
            let parsed_category = Category::from_key(&category)
                .ok_or_else(|| ServerFnError::new(format!("Unknown store category '{category}'")))?;
            stores.push(StoreAdminRow {
                id,
                name,
                category: parsed_category,
                store_number,
                level: level.map(|v| v as u8),
                phone,
                website,
                icon_path,
            });
        }
        return Ok(stores);
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "list_store_rows is only available on server",
        ))
    }
}

#[server]
pub async fn create_store(
    name: String,
    category: Category,
    store_number: Option<String>,
    level: Option<u8>,
    phone: Option<String>,
    website: Option<String>,
    icon_path: Option<String>,
    upload_filename: Option<String>,
    upload_bytes: Option<Vec<u8>>,
) -> Result<i64, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let resolved_icon_path = if let (Some(filename), Some(bytes)) = (upload_filename, upload_bytes) {
            Some(persist_brand_image(filename, bytes).await?)
        } else {
            icon_path
        };

        let result = sqlx::query(
            "INSERT INTO stores (name, category, store_number, level, phone, website, icon_path)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(name)
        .bind(category.key())
        .bind(store_number)
        .bind(level.map(|v| v as i64))
        .bind(phone)
        .bind(website)
        .bind(resolved_icon_path)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        crate::db::sync_brand_assets_to_public()
            .await
            .map_err(ServerFnError::new)?;
        return Ok(result.last_insert_rowid());
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "create_store is only available on server",
        ))
    }
}

#[server]
pub async fn update_store(
    id: i64,
    name: String,
    category: Category,
    store_number: Option<String>,
    level: Option<u8>,
    phone: Option<String>,
    website: Option<String>,
    icon_path: Option<String>,
    upload_filename: Option<String>,
    upload_bytes: Option<Vec<u8>>,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let resolved_icon_path = if let (Some(filename), Some(bytes)) = (upload_filename, upload_bytes) {
            Some(persist_brand_image(filename, bytes).await?)
        } else {
            icon_path
        };

        let result = sqlx::query(
            "UPDATE stores
             SET name = ?, category = ?, store_number = ?, level = ?, phone = ?, website = ?, icon_path = ?
             WHERE id = ?",
        )
        .bind(name)
        .bind(category.key())
        .bind(store_number)
        .bind(level.map(|v| v as i64))
        .bind(phone)
        .bind(website)
        .bind(resolved_icon_path)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ServerFnError::new(format!(
                "No store found for id={id}"
            )));
        }

        crate::db::sync_brand_assets_to_public()
            .await
            .map_err(ServerFnError::new)?;
        return Ok(());
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "update_store is only available on server",
        ))
    }
}

#[server]
pub async fn delete_store(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let result = sqlx::query("DELETE FROM stores WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ServerFnError::new(format!(
                "No store found for id={id}"
            )));
        }

        crate::db::sync_brand_assets_to_public()
            .await
            .map_err(ServerFnError::new)?;
        return Ok(());
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "delete_store is only available on server",
        ))
    }
}

/// Set or clear the `(map_x, map_y)` and `level` of a store on the floor plan.
/// `level` 0..=3, `x`/`y` are image-relative percentages in 0..=100.
/// Pass `None` for `x`/`y` to clear the position. Requires Admin role.
#[server]
pub async fn set_store_position(
    token: String,
    slug: String,
    level: Option<u8>,
    x: Option<f32>,
    y: Option<f32>,
) -> Result<Store, ServerFnError> {
    #[cfg(feature = "server")]
    {
        crate::auth::require_role(&token, &crate::auth::Role::Admin)?;

        if let Some(lvl) = level {
            if lvl > 3 {
                return Err(ServerFnError::new("level must be 0..=3"));
            }
        }
        if x.is_some() != y.is_some() {
            return Err(ServerFnError::new(
                "x and y must both be set or both be cleared",
            ));
        }
        for v in [x, y].into_iter().flatten() {
            if !(0.0..=100.0).contains(&v) || !v.is_finite() {
                return Err(ServerFnError::new("x and y must be in 0..=100"));
            }
        }

        let pool = crate::db::pool().await;
        let names: Vec<(String,)> = sqlx::query_as("SELECT name FROM stores")
            .fetch_all(pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        let target_name = names
            .into_iter()
            .map(|(n,)| n)
            .find(|n| slugify(n) == slug)
            .ok_or_else(|| ServerFnError::new(format!("Store '{slug}' not found")))?;

        let result = if let Some(lvl) = level {
            sqlx::query(
                "UPDATE stores SET level = ?, map_x = ?, map_y = ? WHERE name = ?",
            )
            .bind(lvl as i64)
            .bind(x.map(|v| v as f64))
            .bind(y.map(|v| v as f64))
            .bind(&target_name)
            .execute(pool)
            .await
        } else {
            sqlx::query("UPDATE stores SET map_x = ?, map_y = ? WHERE name = ?")
                .bind(x.map(|v| v as f64))
                .bind(y.map(|v| v as f64))
                .bind(&target_name)
                .execute(pool)
                .await
        }
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ServerFnError::new(format!("Store '{slug}' not found")));
        }

        return get_store(slug).await;
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = (token, slug, level, x, y);
        Err(ServerFnError::new(
            "set_store_position is only available on server",
        ))
    }
}
