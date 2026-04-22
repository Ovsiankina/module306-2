use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DiscountRangeRule {
    pub discount_percent: u32,
    pub balls_weight: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StoreDrawRules {
    pub black_balls_min: u16,
    pub black_balls_current: u16,
    pub black_balls_max: u16,
    pub mix_seconds: u8,
    pub entropy_percent: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DiscountDrawRules {
    pub black_balls: u16,
    pub ranges: Vec<DiscountRangeRule>,
    pub mix_seconds: u8,
    pub entropy_percent: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct VoucherRules {
    pub validity_days: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GameRules {
    pub store_draw: StoreDrawRules,
    pub discount_draw: DiscountDrawRules,
    pub voucher: VoucherRules,
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
static GAME_RULES: OnceLock<Mutex<GameRules>> = OnceLock::new();

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

pub fn default_game_rules() -> GameRules {
    GameRules {
        store_draw: StoreDrawRules {
            black_balls_min: 0,
            black_balls_current: 9,
            black_balls_max: 30,
            mix_seconds: 7,
            entropy_percent: 65,
        },
        discount_draw: DiscountDrawRules {
            black_balls: 15,
            ranges: vec![
                DiscountRangeRule { discount_percent: 50, balls_weight: 1 },
                DiscountRangeRule { discount_percent: 45, balls_weight: 2 },
                DiscountRangeRule { discount_percent: 40, balls_weight: 2 },
                DiscountRangeRule { discount_percent: 35, balls_weight: 2 },
                DiscountRangeRule { discount_percent: 30, balls_weight: 2 },
                DiscountRangeRule { discount_percent: 25, balls_weight: 3 },
                DiscountRangeRule { discount_percent: 20, balls_weight: 3 },
                DiscountRangeRule { discount_percent: 15, balls_weight: 5 },
                DiscountRangeRule { discount_percent: 10, balls_weight: 5 },
                DiscountRangeRule { discount_percent: 5, balls_weight: 10 },
            ],
            mix_seconds: 7,
            entropy_percent: 65,
        },
        voucher: VoucherRules {
            validity_days: 30,
        },
    }
}

fn game_rules_state() -> &'static Mutex<GameRules> {
    GAME_RULES.get_or_init(|| Mutex::new(default_game_rules()))
}

pub fn clamp_game_rules(mut rules: GameRules) -> GameRules {
    rules.store_draw.mix_seconds = rules.store_draw.mix_seconds.clamp(3, 10);
    rules.store_draw.entropy_percent = rules.store_draw.entropy_percent.clamp(0, 100);

    rules.discount_draw.black_balls = rules.discount_draw.black_balls.clamp(0, 120);
    rules.discount_draw.mix_seconds = rules.discount_draw.mix_seconds.clamp(3, 10);
    rules.discount_draw.entropy_percent = rules.discount_draw.entropy_percent.clamp(0, 100);
    if rules.discount_draw.ranges.is_empty() {
        rules.discount_draw.ranges = default_game_rules().discount_draw.ranges;
    }
    for range in &mut rules.discount_draw.ranges {
        range.discount_percent = range.discount_percent.clamp(1, 100);
        range.balls_weight = range.balls_weight.clamp(1, 200);
    }

    rules.voucher.validity_days = rules.voucher.validity_days.clamp(1, 365);
    rules
}

pub fn get_game_rules_snapshot() -> GameRules {
    game_rules_state()
        .lock()
        .expect("game rules mutex poisoned")
        .clone()
}

pub fn save_game_rules(rules: GameRules) -> GameRules {
    let clamped = clamp_game_rules(rules);
    let mut state = game_rules_state()
        .lock()
        .expect("game rules mutex poisoned");
    *state = clamped.clone();
    clamped
}

pub fn total_unique_shops_count() -> usize {
    let mut names = HashSet::new();
    for shop in &stores_data().shops {
        names.insert(shop.name.as_str());
    }
    names.len()
}

pub fn store_black_ball_count_for_shop_count(shop_count: usize, rules: &GameRules) -> usize {
    if shop_count == 0 {
        return 0;
    }
    let limit = shop_count;
    let min = usize::from(rules.store_draw.black_balls_min).min(limit);
    let max_requested = usize::from(rules.store_draw.black_balls_max).min(limit);
    let max = max_requested.max(min);
    let current = usize::from(rules.store_draw.black_balls_current).min(limit);
    current.clamp(min, max)
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

pub const MAX_DAILY_PRIZES: u32 = crate::services::vouchers::MAX_VOUCHERS_PER_UTC_DAY;

#[derive(Debug, Default)]
struct DailyRulesState {
    day_key: String,
    players_who_played: HashSet<String>,
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
    #[cfg(feature = "server")]
    {
        crate::services::vouchers::active_daily_quota_cooldown_until_utc().is_none()
            && crate::services::vouchers::voucher_count_for_current_utc_day() < MAX_DAILY_PRIZES
    }
    #[cfg(not(feature = "server"))]
    {
        true
    }
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
    #[cfg(feature = "server")]
    {
        if crate::services::vouchers::active_daily_quota_cooldown_until_utc().is_some() {
            return false;
        }
        if crate::services::vouchers::voucher_count_for_current_utc_day() >= MAX_DAILY_PRIZES {
            return false;
        }
        let mut state = daily_rules()
            .lock()
            .expect("daily rules mutex poisoned");
        ensure_current_day(&mut state);
        *state
            .prize_count_by_shop
            .entry(shop_name.to_string())
            .or_insert(0) += 1;
        true
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = shop_name;
        true
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DailyPrizePoolSnapshot {
    pub distributed: u32,
    pub max: u32,
    pub cooldown_active: bool,
    pub cooldown_until_utc: Option<String>,
}

pub fn daily_prize_pool_snapshot() -> DailyPrizePoolSnapshot {
    #[cfg(feature = "server")]
    {
        let cooldown_until = crate::services::vouchers::active_daily_quota_cooldown_until_utc();
        DailyPrizePoolSnapshot {
            distributed: crate::services::vouchers::voucher_count_for_current_utc_day(),
            max: MAX_DAILY_PRIZES,
            cooldown_active: cooldown_until.is_some(),
            cooldown_until_utc: cooldown_until.map(|dt| dt.to_rfc3339()),
        }
    }
    #[cfg(not(feature = "server"))]
    {
        DailyPrizePoolSnapshot {
            distributed: 0,
            max: MAX_DAILY_PRIZES,
            cooldown_active: false,
            cooldown_until_utc: None,
        }
    }
}

#[server]
pub async fn get_daily_prize_pool_snapshot() -> Result<DailyPrizePoolSnapshot, ServerFnError> {
    Ok(daily_prize_pool_snapshot())
}

/// Secondes jusqu'au prochain minuit UTC (réinitialisation quota / jeu).
pub fn daily_prize_reset_countdown_secs() -> u64 {
    let now = chrono::Utc::now();
    let next_day = now
        .date_naive()
        .succ_opt()
        .expect("date")
        .and_hms_opt(0, 0, 0)
        .expect("midnight")
        .and_utc();
    next_day
        .signed_duration_since(now)
        .num_seconds()
        .max(0) as u64
}

pub fn format_daily_prize_reset_countdown_hms() -> String {
    let total_secs = daily_prize_reset_countdown_secs();
    let h = total_secs / 3600;
    let m = (total_secs % 3600) / 60;
    let s = total_secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

#[server]
pub async fn game_server_can_award_prize_today() -> Result<bool, ServerFnError> {
    Ok(can_award_prize_today())
}

#[server]
pub async fn game_server_can_start_today(player_key: String) -> Result<bool, ServerFnError> {
    Ok(can_start_game_today(&player_key))
}

#[server]
pub async fn game_server_register_session_finished(player_key: String) -> Result<(), ServerFnError> {
    register_game_start(&player_key);
    Ok(())
}

#[server]
pub async fn game_server_try_register_prize_award(shop_name: String) -> Result<bool, ServerFnError> {
    Ok(register_prize_award(&shop_name))
}

#[server]
pub async fn game_server_choose_distributed_shop(
    candidate_names: Vec<String>,
) -> Result<Option<String>, ServerFnError> {
    if candidate_names.is_empty() {
        return Ok(None);
    }
    let candidates: Vec<ShopInfo> = candidate_names
        .into_iter()
        .enumerate()
        .map(|(i, name)| ShopInfo {
            id: i as u32 + 1,
            name,
            category: String::new(),
        })
        .collect();
    Ok(choose_distributed_shop(&candidates).map(|s| s.name))
}

#[server]
pub async fn get_game_rules() -> Result<GameRules, ServerFnError> {
    Ok(get_game_rules_snapshot())
}

#[server]
pub async fn update_game_rules(rules: GameRules) -> Result<GameRules, ServerFnError> {
    Ok(save_game_rules(rules))
}
