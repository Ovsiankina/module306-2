use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

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

const MAX_DAILY_PRIZES: u32 = 10;

#[derive(Debug, Default)]
struct DailyRulesState {
    day_key: String,
    players_who_played: HashSet<String>,
    distributed_prizes: u32,
    prize_count_by_shop: HashMap<String, u32>,
}

static DAILY_RULES: OnceLock<Mutex<DailyRulesState>> = OnceLock::new();

fn daily_rules() -> &'static Mutex<DailyRulesState> {
    DAILY_RULES.get_or_init(|| Mutex::new(DailyRulesState::default()))
}

fn current_day_key() -> String {
    chrono::Utc::now().date_naive().to_string()
}

fn ensure_current_day(state: &mut DailyRulesState) {
    let today = current_day_key();
    if state.day_key != today {
        state.day_key = today;
        state.players_who_played.clear();
        state.distributed_prizes = 0;
        state.prize_count_by_shop.clear();
    }
}

pub fn can_start_game_today(player_key: &str) -> bool {
    let mut state = daily_rules()
        .lock()
        .expect("daily rules mutex poisoned");
    ensure_current_day(&mut state);
    !state.players_who_played.contains(player_key)
}

pub fn register_game_start(player_key: &str) {
    let mut state = daily_rules()
        .lock()
        .expect("daily rules mutex poisoned");
    ensure_current_day(&mut state);
    state.players_who_played.insert(player_key.to_string());
}

pub fn can_award_prize_today() -> bool {
    let mut state = daily_rules()
        .lock()
        .expect("daily rules mutex poisoned");
    ensure_current_day(&mut state);
    state.distributed_prizes < MAX_DAILY_PRIZES
}

pub fn choose_distributed_shop(candidates: &[ShopInfo]) -> Option<ShopInfo> {
    if candidates.is_empty() {
        return None;
    }

    let mut state = daily_rules()
        .lock()
        .expect("daily rules mutex poisoned");
    ensure_current_day(&mut state);

    let min_count = candidates
        .iter()
        .map(|shop| state.prize_count_by_shop.get(&shop.name).copied().unwrap_or(0))
        .min()
        .unwrap_or(0);

    let best: Vec<ShopInfo> = candidates
        .iter()
        .filter(|shop| state.prize_count_by_shop.get(&shop.name).copied().unwrap_or(0) == min_count)
        .cloned()
        .collect();

    best.get(random_index(best.len())).cloned()
}

pub fn register_prize_award(shop_name: &str) -> bool {
    let mut state = daily_rules()
        .lock()
        .expect("daily rules mutex poisoned");
    ensure_current_day(&mut state);
    if state.distributed_prizes >= MAX_DAILY_PRIZES {
        return false;
    }

    state.distributed_prizes += 1;
    *state
        .prize_count_by_shop
        .entry(shop_name.to_string())
        .or_insert(0) += 1;
    true
}
