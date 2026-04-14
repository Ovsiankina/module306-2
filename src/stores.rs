use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

// Embedded at compile time — no file I/O at runtime
const STORES_JSON: &str = include_str!("../data/stores.json");

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Store {
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
}

#[derive(Deserialize)]
struct StoresData {
    shops: Vec<Store>,
}

fn load_stores() -> Vec<Store> {
    serde_json::from_str::<StoresData>(STORES_JSON)
        .expect("stores.json is invalid")
        .shops
}

static STORES_CACHE: LazyLock<Vec<Store>> = LazyLock::new(load_stores);

fn cached_stores() -> &'static [Store] {
    STORES_CACHE.as_slice()
}

pub fn get_store_local(slug: &str) -> Option<Store> {
    cached_stores()
        .iter()
        .find(|s| slugify(&s.name) == slug)
        .cloned()
}

// --- Slug ---

pub fn slugify(name: &str) -> String {
    let raw: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_lowercase().next().unwrap() } else { '-' })
        .collect();
    raw.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-")
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
    cached_stores()
        .iter()
        .find(|s| slugify(&s.name) == slug)
        .cloned()
        .ok_or_else(|| ServerFnError::new(format!("Store '{}' not found", slug)))
}


#[server]
pub async fn get_stores() -> Result<Vec<Store>, ServerFnError> {
    Ok(cached_stores().to_vec())
}

#[server]
pub async fn get_stores_by_category(category: Category) -> Result<Vec<Store>, ServerFnError> {
    Ok(cached_stores()
        .iter()
        .filter(|s| s.category == category)
        .cloned()
        .collect())
}

#[server]
pub async fn get_stores_by_level(level: u8) -> Result<Vec<Store>, ServerFnError> {
    Ok(cached_stores()
        .iter()
        .filter(|s| s.level == Some(level))
        .cloned()
        .collect())
}
