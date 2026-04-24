use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use std::sync::{Mutex, OnceLock};

pub const MAX_TOTAL_PARKING_CAPACITY: u32 = 1800;
const OPENING_HOURS_CANONICAL: &str = "OPEN_7_7_11_19";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParkingZoneStatus {
    pub id: String,
    pub name: String,
    pub occupied: u32,
    pub capacity: u32,
    pub ev_occupied: u32,
    pub ev_capacity: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParkingSnapshot {
    pub zones: Vec<ParkingZoneStatus>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargingStation {
    pub network: String,
    pub station_type: String,
    pub power_kw: u32,
    pub connectors: Vec<String>,
    pub ports: u32,
    pub paid: bool,
    pub availability: String,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParkingLot {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub level: Option<String>,
    pub capacity: u32,
    pub occupied: u32,
    pub reserved_accessible: u32,
    pub reserved_family: u32,
    pub ev_capacity: u32,
    pub ev_occupied: u32,
    pub charging_stations: Vec<ChargingStation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParkingSystemState {
    pub max_total_capacity: u32,
    pub lots: Vec<ParkingLot>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParkingOperationResult {
    pub accepted: bool,
    pub message: String,
    pub state: ParkingSystemState,
}

#[cfg(feature = "server")]
struct LotMeta {
    id: &'static str,
    spot_start: usize,
    spot_end: usize,       // exclusive
    ev_bit_start: usize,   // within ev u16
    ev_bit_end: usize,     // exclusive
}

#[cfg(feature = "server")]
const LOT_META: [LotMeta; 6] = [
    LotMeta { id: "p1", spot_start:    0, spot_end:  220, ev_bit_start:  0, ev_bit_end:  0 },
    LotMeta { id: "p2", spot_start:  220, spot_end:  500, ev_bit_start:  0, ev_bit_end:  0 },
    LotMeta { id: "p3", spot_start:  500, spot_end:  800, ev_bit_start:  0, ev_bit_end:  0 },
    LotMeta { id: "p4", spot_start:  800, spot_end:  980, ev_bit_start:  0, ev_bit_end:  4 },
    LotMeta { id: "p5", spot_start:  980, spot_end: 1580, ev_bit_start:  4, ev_bit_end:  6 },
    LotMeta { id: "p6", spot_start: 1580, spot_end: 1800, ev_bit_start:  6, ev_bit_end: 12 },
];

#[cfg(feature = "server")]
struct ParkingBits {
    spots: [u32; 57],
    ev: u16,
}

// Bitwise helper functions
#[cfg(feature = "server")]
fn count_range(spots: &[u32; 57], start: usize, end: usize) -> u32 {
    let mut count = 0u32;
    for bit in start..end {
        let word = bit / 32;
        let shift = bit % 32;
        count += (spots[word] >> shift) & 1;
    }
    count
}

#[cfg(feature = "server")]
fn set_first_free_bit(spots: &mut [u32; 57], start: usize, end: usize) -> Option<usize> {
    for bit in start..end {
        let word = bit / 32;
        let shift = bit % 32;
        if (spots[word] >> shift) & 1 == 0 {
            spots[word] |= 1 << shift;
            return Some(bit);
        }
    }
    None
}

#[cfg(feature = "server")]
fn clear_first_occupied_bit(spots: &mut [u32; 57], start: usize, end: usize) -> bool {
    for bit in start..end {
        let word = bit / 32;
        let shift = bit % 32;
        if (spots[word] >> shift) & 1 == 1 {
            spots[word] &= !(1u32 << shift);
            return true;
        }
    }
    false
}

#[cfg(feature = "server")]
fn count_ev_range(ev: u16, start: usize, end: usize) -> u32 {
    (start..end).map(|i| ((ev >> i) & 1) as u32).sum()
}

#[cfg(feature = "server")]
fn set_first_free_ev_bit(ev: &mut u16, start: usize, end: usize) -> bool {
    for i in start..end {
        if (*ev >> i) & 1 == 0 {
            *ev |= 1 << i;
            return true;
        }
    }
    false
}

#[cfg(feature = "server")]
fn clear_first_occupied_ev_bit(ev: &mut u16, start: usize, end: usize) -> bool {
    for i in start..end {
        if (*ev >> i) & 1 == 1 {
            *ev &= !(1u16 << i);
            return true;
        }
    }
    false
}

#[cfg(feature = "server")]
fn bits_from_state(state: &ParkingSystemState) -> ParkingBits {
    let mut bits = ParkingBits { spots: [0u32; 57], ev: 0u16 };
    for meta in &LOT_META {
        let lot = match state.lots.iter().find(|l| l.id == meta.id) {
            Some(l) => l,
            None => continue,
        };
        let fill = (lot.occupied as usize).min(meta.spot_end - meta.spot_start);
        for bit in meta.spot_start..(meta.spot_start + fill) {
            bits.spots[bit / 32] |= 1 << (bit % 32);
        }
        if meta.ev_bit_end > meta.ev_bit_start {
            let ev_fill = (lot.ev_occupied as usize).min(meta.ev_bit_end - meta.ev_bit_start);
            for i in meta.ev_bit_start..(meta.ev_bit_start + ev_fill) {
                bits.ev |= 1u16 << i;
            }
        }
    }
    bits
}

#[cfg(feature = "server")]
fn state_from_bits(bits: &ParkingBits, template: &ParkingSystemState) -> ParkingSystemState {
    let mut state = template.clone();
    for (i, meta) in LOT_META.iter().enumerate() {
        state.lots[i].occupied = count_range(&bits.spots, meta.spot_start, meta.spot_end);
        state.lots[i].ev_occupied = count_ev_range(bits.ev, meta.ev_bit_start, meta.ev_bit_end);
    }
    state.updated_at = chrono::Utc::now().to_rfc3339();
    state
}

#[cfg(feature = "server")]
fn default_state() -> ParkingSystemState {
    ParkingSystemState {
        max_total_capacity: MAX_TOTAL_PARKING_CAPACITY,
        lots: vec![
            ParkingLot {
                id: "p1".to_string(),
                name: "Parking P1".to_string(),
                kind: "indoor".to_string(),
                level: Some("L0".to_string()),
                capacity: 220,
                occupied: 120,
                reserved_accessible: 8,
                reserved_family: 10,
                ev_capacity: 0,
                ev_occupied: 0,
                charging_stations: vec![],
            },
            ParkingLot {
                id: "p2".to_string(),
                name: "Parking P2".to_string(),
                kind: "indoor".to_string(),
                level: Some("L-1".to_string()),
                capacity: 280,
                occupied: 170,
                reserved_accessible: 10,
                reserved_family: 12,
                ev_capacity: 0,
                ev_occupied: 0,
                charging_stations: vec![],
            },
            ParkingLot {
                id: "p3".to_string(),
                name: "Parking P3".to_string(),
                kind: "outdoor".to_string(),
                level: Some("L1".to_string()),
                capacity: 300,
                occupied: 210,
                reserved_accessible: 10,
                reserved_family: 12,
                ev_capacity: 0,
                ev_occupied: 0,
                charging_stations: vec![],
            },
            ParkingLot {
                id: "p4".to_string(),
                name: "FoxPark".to_string(),
                kind: "outdoor".to_string(),
                level: None,
                capacity: 180,
                occupied: 108,
                reserved_accessible: 6,
                reserved_family: 8,
                ev_capacity: 4,
                ev_occupied: 2,
                charging_stations: vec![ChargingStation {
                    network: "eCarUp".to_string(),
                    station_type: "AC Type 2".to_string(),
                    power_kw: 22,
                    connectors: vec!["Type 2".to_string()],
                    ports: 4,
                    paid: true,
                    availability: OPENING_HOURS_CANONICAL.to_string(),
                    notes: "4 bornes publiques lentes".to_string(),
                }],
            },
            ParkingLot {
                id: "p5".to_string(),
                name: "Parking P5".to_string(),
                kind: "outdoor".to_string(),
                level: Some("L0".to_string()),
                capacity: 600,
                occupied: 360,
                reserved_accessible: 20,
                reserved_family: 24,
                ev_capacity: 2,
                ev_occupied: 1,
                charging_stations: vec![
                    ChargingStation {
                        network: "GOFAST".to_string(),
                        station_type: "Ultra-fast".to_string(),
                        power_kw: 100,
                        connectors: vec![
                            "CCS".to_string(),
                            "CHAdeMO".to_string(),
                            "Tesla".to_string(),
                        ],
                        ports: 1,
                        paid: true,
                        availability: OPENING_HOURS_CANONICAL.to_string(),
                        notes: "Ideal pour les visites de moins d'une heure".to_string(),
                    },
                    ChargingStation {
                        network: "FoxTown".to_string(),
                        station_type: "AC Type 2".to_string(),
                        power_kw: 22,
                        connectors: vec!["Type 2".to_string()],
                        ports: 1,
                        paid: false,
                        availability: OPENING_HOURS_CANONICAL.to_string(),
                        notes: "Recharge lente gratuite".to_string(),
                    },
                ],
            },
            ParkingLot {
                id: "p6".to_string(),
                name: "Parking P6".to_string(),
                kind: "indoor".to_string(),
                level: Some("L0".to_string()),
                capacity: 220,
                occupied: 132,
                reserved_accessible: 8,
                reserved_family: 10,
                ev_capacity: 6,
                ev_occupied: 3,
                charging_stations: vec![ChargingStation {
                    network: "eCarUp".to_string(),
                    station_type: "AC Type 2".to_string(),
                    power_kw: 22,
                    connectors: vec!["Type 2".to_string()],
                    ports: 6,
                    paid: true,
                    availability: OPENING_HOURS_CANONICAL.to_string(),
                    notes: "6 bornes publiques lentes".to_string(),
                }],
            },
        ],
        updated_at: chrono::Utc::now().to_rfc3339(),
    }
}

#[cfg(feature = "server")]
fn validate_state(state: &ParkingSystemState) -> Result<(), ServerFnError> {
    let expected_capacities = [
        ("p1", 220_u32),
        ("p2", 280_u32),
        ("p3", 300_u32),
        ("p4", 180_u32),
        ("p5", 600_u32),
        ("p6", 220_u32),
    ];

    if state.max_total_capacity != MAX_TOTAL_PARKING_CAPACITY {
        return Err(ServerFnError::new(
            "La capacite globale doit rester fixee a 1800 places.".to_string(),
        ));
    }
    if state.lots.len() != 6 {
        return Err(ServerFnError::new(
            "La configuration doit contenir exactement 6 parkings.".to_string(),
        ));
    }

    let total_capacity: u32 = state.lots.iter().map(|z| z.capacity).sum();
    if total_capacity != MAX_TOTAL_PARKING_CAPACITY {
        return Err(ServerFnError::new(
            "La somme des capacites parking doit faire 1800 places.".to_string(),
        ));
    }

    let max_capacity = state.lots.iter().map(|z| z.capacity).max().unwrap_or(0);
    if max_capacity != 600 {
        return Err(ServerFnError::new(
            "Le plus grand parking doit avoir 600 places.".to_string(),
        ));
    }

    for (lot_id, expected) in expected_capacities {
        let Some(lot) = state.lots.iter().find(|z| z.id == lot_id) else {
            return Err(ServerFnError::new(format!(
                "Le parking {lot_id} est manquant dans la configuration."
            )));
        };
        if lot.capacity != expected {
            return Err(ServerFnError::new(format!(
                "Capacite invalide pour {}: attendu {}, recu {}.",
                lot.name, expected, lot.capacity
            )));
        }
    }

    let total_occupied: u32 = state.lots.iter().map(|z| z.occupied).sum();
    if total_occupied > MAX_TOTAL_PARKING_CAPACITY {
        return Err(ServerFnError::new(
            "Le nombre de vehicules ne peut pas depasser 1800.".to_string(),
        ));
    }

    for lot in &state.lots {
        if lot.kind != "indoor" && lot.kind != "outdoor" {
            return Err(ServerFnError::new(format!(
                "Type de parking invalide pour {}.",
                lot.name
            )));
        }
        if let Some(level) = &lot.level {
            if !matches!(level.as_str(), "L-1" | "L0" | "L1") {
                return Err(ServerFnError::new(format!(
                    "Niveau de parking invalide pour {}.",
                    lot.name
                )));
            }
        }
        if lot.occupied > lot.capacity {
            return Err(ServerFnError::new(format!(
                "Occupation invalide pour {}.",
                lot.name
            )));
        }
        if lot.ev_occupied > lot.ev_capacity {
            return Err(ServerFnError::new(format!(
                "Occupation EV invalide pour {}.",
                lot.name
            )));
        }
        for station in &lot.charging_stations {
            if station.availability != OPENING_HOURS_CANONICAL {
                return Err(ServerFnError::new(format!(
                    "Disponibilite de borne invalide pour {}.",
                    lot.name
                )));
            }
        }
    }
    Ok(())
}

#[cfg(feature = "server")]
fn normalize_level(value: Option<String>) -> Option<String> {
    match value.as_deref() {
        None => None,
        Some("L-1") | Some("Niveau -1") | Some("Level -1") => Some("L-1".to_string()),
        Some("L0") | Some("Niveau 0") | Some("Level 0") | Some("Entree principale") => {
            Some("L0".to_string())
        }
        Some("L1") | Some("Niveau 1") | Some("Level 1") => Some("L1".to_string()),
        Some(_) => None,
    }
}

#[cfg(feature = "server")]
fn normalize_kind(value: String) -> String {
    match value.as_str() {
        "indoor" | "interieur" | "interior" => "indoor".to_string(),
        "outdoor" | "exterieur" | "exterior" => "outdoor".to_string(),
        _ => value,
    }
}

#[cfg(feature = "server")]
fn normalize_availability(value: String) -> String {
    match value.as_str() {
        OPENING_HOURS_CANONICAL | "7/7 11h-19h" | "Horaires d'ouverture FoxTown" | "24/7" => {
            OPENING_HOURS_CANONICAL.to_string()
        }
        _ => value,
    }
}

#[cfg(feature = "server")]
fn normalize_state(state: &mut ParkingSystemState) {
    for lot in &mut state.lots {
        lot.kind = normalize_kind(std::mem::take(&mut lot.kind));
        lot.level = normalize_level(lot.level.take());
        for station in &mut lot.charging_stations {
            station.availability = normalize_availability(std::mem::take(&mut station.availability));
        }
    }
}

#[cfg(feature = "server")]
fn to_snapshot(state: &ParkingSystemState) -> ParkingSnapshot {
    ParkingSnapshot {
        zones: state
            .lots
            .iter()
            .map(|lot| ParkingZoneStatus {
                id: lot.id.clone(),
                name: lot.name.clone(),
                occupied: lot.occupied,
                capacity: lot.capacity,
                ev_occupied: lot.ev_occupied,
                ev_capacity: lot.ev_capacity,
            })
            .collect(),
        updated_at: state.updated_at.clone(),
    }
}


// Adapter implementations
#[cfg(feature = "server")]
struct BitwiseFakeAdapter {
    bits: ParkingBits,
    template: ParkingSystemState,
}

#[cfg(feature = "server")]
impl BitwiseFakeAdapter {
    fn new(initial_state: ParkingSystemState) -> Self {
        let bits = bits_from_state(&initial_state);
        BitwiseFakeAdapter {
            bits,
            template: initial_state,
        }
    }

    fn get_state(&self) -> ParkingSystemState {
        state_from_bits(&self.bits, &self.template)
    }

    fn set_zone_occupancy(
        &mut self,
        zone_id: &str,
        occupied: u32,
        ev_occupied: u32,
    ) -> Result<ParkingSystemState, ServerFnError> {
        let meta = LOT_META
            .iter()
            .find(|m| m.id == zone_id)
            .ok_or_else(|| ServerFnError::new("Parking introuvable".to_string()))?;

        let lot = self
            .template
            .lots
            .iter()
            .find(|l| l.id == zone_id)
            .ok_or_else(|| ServerFnError::new("Parking introuvable".to_string()))?;

        let occupied_clamped = occupied.min(lot.capacity);
        let ev_occupied_clamped = ev_occupied.min(lot.ev_capacity);

        // Clear all bits in this lot
        for bit in meta.spot_start..meta.spot_end {
            self.bits.spots[bit / 32] &= !(1u32 << (bit % 32));
        }
        for i in meta.ev_bit_start..meta.ev_bit_end {
            self.bits.ev &= !(1u16 << i);
        }

        // Set new occupied bits
        for bit in meta.spot_start..(meta.spot_start + occupied_clamped as usize) {
            self.bits.spots[bit / 32] |= 1u32 << (bit % 32);
        }
        for i in meta.ev_bit_start..(meta.ev_bit_start + ev_occupied_clamped as usize) {
            self.bits.ev |= 1u16 << i;
        }

        Ok(self.get_state())
    }

    fn vehicle_entry(
        &mut self,
        lot_id: &str,
        is_electric: bool,
    ) -> Result<ParkingOperationResult, ServerFnError> {
        let meta = LOT_META
            .iter()
            .find(|m| m.id == lot_id)
            .ok_or_else(|| ServerFnError::new("Parking introuvable".to_string()))?;

        let lot = self
            .template
            .lots
            .iter()
            .find(|l| l.id == lot_id)
            .ok_or_else(|| ServerFnError::new("Parking introuvable".to_string()))?;

        let current_occupied = count_range(&self.bits.spots, meta.spot_start, meta.spot_end);
        if current_occupied >= lot.capacity {
            return Ok(ParkingOperationResult {
                accepted: false,
                message: format!("{} est complet.", lot.name),
                state: self.get_state(),
            });
        }

        if is_electric {
            let current_ev = count_ev_range(self.bits.ev, meta.ev_bit_start, meta.ev_bit_end);
            if current_ev >= lot.ev_capacity {
                return Ok(ParkingOperationResult {
                    accepted: false,
                    message: format!("Plus de place de recharge disponible dans {}.", lot.name),
                    state: self.get_state(),
                });
            }
            set_first_free_ev_bit(&mut self.bits.ev, meta.ev_bit_start, meta.ev_bit_end);
        }

        set_first_free_bit(&mut self.bits.spots, meta.spot_start, meta.spot_end);

        Ok(ParkingOperationResult {
            accepted: true,
            message: format!("Entree enregistree dans {}.", lot.name),
            state: self.get_state(),
        })
    }

    fn vehicle_exit(
        &mut self,
        lot_id: &str,
        was_electric: bool,
    ) -> Result<ParkingOperationResult, ServerFnError> {
        let meta = LOT_META
            .iter()
            .find(|m| m.id == lot_id)
            .ok_or_else(|| ServerFnError::new("Parking introuvable".to_string()))?;

        let lot = self
            .template
            .lots
            .iter()
            .find(|l| l.id == lot_id)
            .ok_or_else(|| ServerFnError::new("Parking introuvable".to_string()))?;

        let current_occupied = count_range(&self.bits.spots, meta.spot_start, meta.spot_end);
        if current_occupied == 0 {
            return Ok(ParkingOperationResult {
                accepted: false,
                message: format!("{} est deja vide.", lot.name),
                state: self.get_state(),
            });
        }

        clear_first_occupied_bit(&mut self.bits.spots, meta.spot_start, meta.spot_end);
        if was_electric && meta.ev_bit_end > meta.ev_bit_start {
            let current_ev = count_ev_range(self.bits.ev, meta.ev_bit_start, meta.ev_bit_end);
            if current_ev > 0 {
                clear_first_occupied_ev_bit(&mut self.bits.ev, meta.ev_bit_start, meta.ev_bit_end);
            }
        }

        Ok(ParkingOperationResult {
            accepted: true,
            message: format!("Sortie enregistree depuis {}.", lot.name),
            state: self.get_state(),
        })
    }

    fn simulate_tick(&mut self, seed: i32) -> ParkingSystemState {
        for (i, meta) in LOT_META.iter().enumerate() {
            let lot = &self.template.lots[i];
            let swing = ((seed + i as i32 * 37) % 7) - 3;
            let current = count_range(&self.bits.spots, meta.spot_start, meta.spot_end) as i32;
            let next = (current + swing).clamp(0, lot.capacity as i32) as usize;

            // Clear all bits for this lot
            for bit in meta.spot_start..meta.spot_end {
                self.bits.spots[bit / 32] &= !(1u32 << (bit % 32));
            }
            // Set new occupied bits
            for bit in meta.spot_start..(meta.spot_start + next) {
                self.bits.spots[bit / 32] |= 1u32 << (bit % 32);
            }

            // EV spots
            if meta.ev_bit_end > meta.ev_bit_start {
                let ev_swing = ((seed + i as i32 * 13) % 3) - 1;
                let current_ev = count_ev_range(self.bits.ev, meta.ev_bit_start, meta.ev_bit_end) as i32;
                let next_ev = (current_ev + ev_swing).clamp(0, lot.ev_capacity as i32) as usize;
                let clamped_ev = next_ev.min(next);

                // Clear EV bits for this lot
                for j in meta.ev_bit_start..meta.ev_bit_end {
                    self.bits.ev &= !(1u16 << j);
                }
                // Set new EV bits
                for j in meta.ev_bit_start..(meta.ev_bit_start + clamped_ev) {
                    self.bits.ev |= 1u16 << j;
                }
            }
        }

        self.get_state()
    }

    fn shuffle_all_bits(&mut self) {
        let seed = chrono::Utc::now().timestamp_subsec_millis() as u32;
        for (i, meta) in LOT_META.iter().enumerate() {
            let lot = &self.template.lots[i];
            let capacity = lot.capacity as usize;

            // Random occupancy between 20% and 80% of capacity
            let random_ratio = ((seed.wrapping_mul(2654435761u32).wrapping_add(i as u32)) % 60 + 20) as usize;
            let target_occupied = (capacity * random_ratio / 100).min(capacity);

            // Clear all bits
            for bit in meta.spot_start..meta.spot_end {
                self.bits.spots[bit / 32] &= !(1u32 << (bit % 32));
            }
            // Set random occupied bits
            for bit in meta.spot_start..(meta.spot_start + target_occupied) {
                self.bits.spots[bit / 32] |= 1u32 << (bit % 32);
            }

            // EV spots: random 0-100% of EV capacity
            if meta.ev_bit_end > meta.ev_bit_start {
                let ev_capacity = (meta.ev_bit_end - meta.ev_bit_start) as usize;
                let ev_random = ((seed.wrapping_add(i as u32 * 73)) % 101) as usize;
                let target_ev = (ev_capacity * ev_random / 100).min(target_occupied);

                // Clear EV bits
                for j in meta.ev_bit_start..meta.ev_bit_end {
                    self.bits.ev &= !(1u16 << j);
                }
                // Set random EV bits
                for j in meta.ev_bit_start..(meta.ev_bit_start + target_ev) {
                    self.bits.ev |= 1u16 << j;
                }
            }
        }
    }
}

#[cfg(feature = "server")]
struct RealParkingApiAdapter {
    api_key: String,
    inner: BitwiseFakeAdapter,
}

#[cfg(feature = "server")]
impl RealParkingApiAdapter {
    fn get_state(&self) -> ParkingSystemState {
        // TODO: call real API with self.api_key
        self.inner.get_state()
    }

    fn set_zone_occupancy(
        &mut self,
        zone_id: &str,
        occupied: u32,
        ev_occupied: u32,
    ) -> Result<ParkingSystemState, ServerFnError> {
        // TODO: call real API with self.api_key
        self.inner.set_zone_occupancy(zone_id, occupied, ev_occupied)
    }

    fn vehicle_entry(
        &mut self,
        lot_id: &str,
        is_electric: bool,
    ) -> Result<ParkingOperationResult, ServerFnError> {
        // TODO: call real API with self.api_key
        self.inner.vehicle_entry(lot_id, is_electric)
    }

    fn vehicle_exit(
        &mut self,
        lot_id: &str,
        was_electric: bool,
    ) -> Result<ParkingOperationResult, ServerFnError> {
        // TODO: call real API with self.api_key
        self.inner.vehicle_exit(lot_id, was_electric)
    }

    fn simulate_tick(&mut self, seed: i32) -> ParkingSystemState {
        // TODO: call real API with self.api_key
        self.inner.simulate_tick(seed)
    }

    fn shuffle_all_bits(&mut self) {
        // TODO: call real API with self.api_key
        self.inner.shuffle_all_bits();
    }
}

#[cfg(feature = "server")]
enum ParkingAdapter {
    Fake(BitwiseFakeAdapter),
    Real(RealParkingApiAdapter),
}

#[cfg(feature = "server")]
impl ParkingAdapter {
    fn get_state(&self) -> ParkingSystemState {
        match self {
            ParkingAdapter::Fake(a) => a.get_state(),
            ParkingAdapter::Real(a) => a.get_state(),
        }
    }

    fn set_zone_occupancy(
        &mut self,
        zone_id: &str,
        occupied: u32,
        ev_occupied: u32,
    ) -> Result<ParkingSystemState, ServerFnError> {
        match self {
            ParkingAdapter::Fake(a) => a.set_zone_occupancy(zone_id, occupied, ev_occupied),
            ParkingAdapter::Real(a) => a.set_zone_occupancy(zone_id, occupied, ev_occupied),
        }
    }

    fn vehicle_entry(
        &mut self,
        lot_id: &str,
        is_electric: bool,
    ) -> Result<ParkingOperationResult, ServerFnError> {
        match self {
            ParkingAdapter::Fake(a) => a.vehicle_entry(lot_id, is_electric),
            ParkingAdapter::Real(a) => a.vehicle_entry(lot_id, is_electric),
        }
    }

    fn vehicle_exit(
        &mut self,
        lot_id: &str,
        was_electric: bool,
    ) -> Result<ParkingOperationResult, ServerFnError> {
        match self {
            ParkingAdapter::Fake(a) => a.vehicle_exit(lot_id, was_electric),
            ParkingAdapter::Real(a) => a.vehicle_exit(lot_id, was_electric),
        }
    }

    fn simulate_tick(&mut self, seed: i32) -> ParkingSystemState {
        match self {
            ParkingAdapter::Fake(a) => a.simulate_tick(seed),
            ParkingAdapter::Real(a) => a.simulate_tick(seed),
        }
    }

    fn shuffle_all_bits(&mut self) {
        match self {
            ParkingAdapter::Fake(a) => a.shuffle_all_bits(),
            ParkingAdapter::Real(a) => a.shuffle_all_bits(),
        }
    }
}

#[cfg(feature = "server")]
static PARKING_ADAPTER: OnceLock<Mutex<ParkingAdapter>> = OnceLock::new();

#[cfg(feature = "server")]
fn spawn_shuffle_task() {
    tokio::spawn(async {
        let refresh_sec_str = std::env::var("PARKING_REFRESH_RATE_SEC").unwrap_or_else(|_| "60".to_string());
        let refresh_sec = refresh_sec_str.parse::<u64>().unwrap_or(60);
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(refresh_sec));

        loop {
            interval.tick().await;
            if let Ok(mut adapter) = PARKING_ADAPTER.get().unwrap().lock() {
                adapter.shuffle_all_bits();
            }
        }
    });
}

#[cfg(feature = "server")]
fn parking_adapter() -> &'static Mutex<ParkingAdapter> {
    PARKING_ADAPTER.get_or_init(|| {
        let initial_state = default_state();
        let api_key = std::env::var("PARKING_API_KEY").unwrap_or_default();
        let adapter = if api_key.is_empty() || api_key == "fake" {
            ParkingAdapter::Fake(BitwiseFakeAdapter::new(initial_state))
        } else {
            ParkingAdapter::Real(RealParkingApiAdapter {
                api_key,
                inner: BitwiseFakeAdapter::new(initial_state),
            })
        };
        let mutex = Mutex::new(adapter);
        spawn_shuffle_task();
        mutex
    })
}

#[server]
pub async fn get_parking_snapshot() -> Result<ParkingSnapshot, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let adapter = parking_adapter()
            .lock()
            .map_err(|_| ServerFnError::new("adapter mutex poisoned".to_string()))?;
        return Ok(to_snapshot(&adapter.get_state()));
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(ParkingSnapshot {
            zones: vec![],
            updated_at: String::new(),
        })
    }
}

#[server]
pub async fn get_parking_system_state() -> Result<ParkingSystemState, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let adapter = parking_adapter()
            .lock()
            .map_err(|_| ServerFnError::new("adapter mutex poisoned".to_string()))?;
        return Ok(adapter.get_state());
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(ParkingSystemState {
            max_total_capacity: MAX_TOTAL_PARKING_CAPACITY,
            lots: vec![],
            updated_at: String::new(),
        })
    }
}

#[server]
pub async fn update_parking_zone_occupancy(
    token: String,
    zone_id: String,
    occupied: u32,
    ev_occupied: u32,
) -> Result<ParkingSnapshot, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Admin)?;

    #[cfg(feature = "server")]
    {
        let mut adapter = parking_adapter()
            .lock()
            .map_err(|_| ServerFnError::new("adapter mutex poisoned".to_string()))?;
        let state = adapter.set_zone_occupancy(&zone_id, occupied, ev_occupied)?;
        return Ok(to_snapshot(&state));
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = (zone_id, occupied, ev_occupied);
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

#[server]
pub async fn parking_vehicle_entry(
    lot_id: String,
    is_electric: bool,
) -> Result<ParkingOperationResult, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let mut adapter = parking_adapter()
            .lock()
            .map_err(|_| ServerFnError::new("adapter mutex poisoned".to_string()))?;
        let result = adapter.vehicle_entry(&lot_id, is_electric)?;
        return Ok(result);
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (lot_id, is_electric);
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

#[server]
pub async fn parking_vehicle_exit(
    lot_id: String,
    was_electric: bool,
) -> Result<ParkingOperationResult, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let mut adapter = parking_adapter()
            .lock()
            .map_err(|_| ServerFnError::new("adapter mutex poisoned".to_string()))?;
        let result = adapter.vehicle_exit(&lot_id, was_electric)?;
        return Ok(result);
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (lot_id, was_electric);
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

#[server]
pub async fn parking_simulate_tick(token: String) -> Result<ParkingSystemState, ServerFnError> {
    crate::auth::require_role(&token, &crate::auth::Role::Admin)?;

    #[cfg(feature = "server")]
    {
        let seed = chrono::Utc::now().timestamp_subsec_millis() as i32;
        let mut adapter = parking_adapter()
            .lock()
            .map_err(|_| ServerFnError::new("adapter mutex poisoned".to_string()))?;
        let state = adapter.simulate_tick(seed);
        return Ok(state);
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = token;
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

#[server]
pub async fn refresh_parking() -> Result<ParkingSnapshot, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let mut adapter = parking_adapter()
            .lock()
            .map_err(|_| ServerFnError::new("adapter mutex poisoned".to_string()))?;
        adapter.shuffle_all_bits();
        let state = adapter.get_state();
        return Ok(to_snapshot(&state));
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(ParkingSnapshot {
            zones: vec![],
            updated_at: String::new(),
        })
    }
}
