use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct StoreCategory {
    pub key: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct ShopInfo {
    pub id: u32,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Deserialize)]
struct StoresDataRaw {
    categories: BTreeMap<String, String>,
    shops: Vec<ShopRaw>,
}

#[derive(Debug, Deserialize)]
struct ShopRaw {
    name: String,
    category: String,
}

#[derive(Debug)]
struct StoresData {
    categories: Vec<StoreCategory>,
    shops: Vec<ShopRaw>,
}

static STORES_DATA: OnceLock<StoresData> = OnceLock::new();

fn stores_data() -> &'static StoresData {
    STORES_DATA.get_or_init(|| {
        let raw = include_str!("../../data/stores.json");
        let parsed: StoresDataRaw =
            serde_json::from_str(raw).expect("Le fichier data/stores.json est invalide");

        let categories = parsed
            .categories
            .into_iter()
            .map(|(key, label)| StoreCategory { key, label })
            .collect();

        StoresData {
            categories,
            shops: parsed.shops,
        }
    })
}

pub fn all_categories() -> Vec<StoreCategory> {
    stores_data().categories.clone()
}

pub fn random_categories(count: usize) -> Vec<StoreCategory> {
    let mut indices: Vec<usize> = (0..stores_data().categories.len()).collect();
    let mut selected = Vec::new();
    let max = count.min(indices.len());

    for _ in 0..max {
        let idx = random_index(indices.len());
        let category_idx = indices.remove(idx);
        selected.push(stores_data().categories[category_idx].clone());
    }

    selected
}

pub fn stores_by_category_keys(keys: &[String]) -> Vec<String> {
    shops_by_category_keys(keys)
        .into_iter()
        .map(|shop| shop.name)
        .collect()
}

pub fn shops_by_category_keys(keys: &[String]) -> Vec<ShopInfo> {
    if keys.is_empty() {
        return Vec::new();
    }

    let selected: HashSet<&str> = keys.iter().map(|k| k.as_str()).collect();
    let mut seen_names = HashSet::new();
    let mut result = Vec::new();

    for (idx, shop) in stores_data().shops.iter().enumerate() {
        if selected.contains(shop.category.as_str()) && seen_names.insert(shop.name.clone()) {
            result.push(ShopInfo {
                id: (idx as u32) + 1,
                name: shop.name.clone(),
                category: shop.category.clone(),
            });
        }
    }

    result
}

fn random_unit() -> f64 {
    #[cfg(target_family = "wasm")]
    {
        return js_sys::Math::random();
    }

    #[cfg(not(target_family = "wasm"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        return (seed % 10_000_000) as f64 / 10_000_000_f64;
    }
}

pub fn random_index(len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    ((random_unit() * len as f64).floor() as usize).min(len - 1)
}

pub async fn delay_ms(ms: u64) {
    #[cfg(target_family = "wasm")]
    {
        gloo_timers::future::TimeoutFuture::new(ms as u32).await;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}
