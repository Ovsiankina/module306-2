use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// Embedded at compile time — no file I/O at runtime
#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
#[derive(Deserialize)]
struct StoresData {
    shops: Vec<Store>,
}

#[cfg(feature = "server")]
fn load_stores() -> Vec<Store> {
    serde_json::from_str::<StoresData>(STORES_JSON)
        .expect("stores.json is invalid")
        .shops
}

// --- Slug ---

pub fn slugify(name: &str) -> String {
    let raw: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_lowercase().next().unwrap() } else { '-' })
        .collect();
    raw.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-")
}

// --- Server functions ---

#[server]
pub async fn get_store(slug: String) -> Result<Store, ServerFnError> {
    load_stores()
        .into_iter()
        .find(|s| slugify(&s.name) == slug)
        .ok_or_else(|| ServerFnError::new(format!("Store '{}' not found", slug)))
}


#[server]
pub async fn get_stores() -> Result<Vec<Store>, ServerFnError> {
    Ok(load_stores())
}

#[server]
pub async fn get_stores_by_category(category: Category) -> Result<Vec<Store>, ServerFnError> {
    Ok(load_stores()
        .into_iter()
        .filter(|s| s.category == category)
        .collect())
}

#[server]
pub async fn get_stores_by_level(level: u8) -> Result<Vec<Store>, ServerFnError> {
    Ok(load_stores()
        .into_iter()
        .filter(|s| s.level == Some(level))
        .collect())
}
