use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// Embedded at compile time — no file I/O at runtime
const STORES_JSON: &str = include_str!("../data/stores.json");

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Store {
    pub name: String,
    pub category: Category,
    pub store_number: String,
    pub level: u8,
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

// --- Server functions ---

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
        .filter(|s| s.level == level)
        .collect())
}
