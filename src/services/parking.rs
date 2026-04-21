use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
fn parkings_path() -> std::path::PathBuf {
    std::env::var("PARKINGS_JSON_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("data/parkings.json"))
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

#[cfg(feature = "server")]
fn load_state() -> Result<ParkingSystemState, ServerFnError> {
    let path = parkings_path();
    if !path.exists() {
        let state = default_state();
        save_state(&state)?;
        return Ok(state);
    }
    let content = std::fs::read_to_string(&path).map_err(|e| ServerFnError::new(e.to_string()))?;
    if content.trim().is_empty() {
        let state = default_state();
        save_state(&state)?;
        return Ok(state);
    }

    if let Ok(mut state) = serde_json::from_str::<ParkingSystemState>(&content) {
        normalize_state(&mut state);
        validate_state(&state)?;
        save_state(&state)?;
        return Ok(state);
    }

    // Compatibilite: ancien format snapshot.
    if let Ok(old_snapshot) = serde_json::from_str::<ParkingSnapshot>(&content) {
        let mut state = default_state();
        for zone in old_snapshot.zones {
            if let Some(lot) = state.lots.iter_mut().find(|l| l.id == zone.id) {
                lot.occupied = zone.occupied.min(lot.capacity);
                lot.ev_occupied = zone.ev_occupied.min(lot.ev_capacity);
            }
        }
        state.updated_at = chrono::Utc::now().to_rfc3339();
        validate_state(&state)?;
        save_state(&state)?;
        return Ok(state);
    }

    Err(ServerFnError::new(
        "Format data/parkings.json invalide".to_string(),
    ))
}

#[cfg(feature = "server")]
fn save_state(state: &ParkingSystemState) -> Result<(), ServerFnError> {
    validate_state(state)?;
    let path = parkings_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| ServerFnError::new(e.to_string()))?;
    }
    let serialized =
        serde_json::to_string_pretty(state).map_err(|e| ServerFnError::new(e.to_string()))?;
    std::fs::write(path, serialized).map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_parking_snapshot() -> Result<ParkingSnapshot, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let state = load_state()?;
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

#[server]
pub async fn get_parking_system_state() -> Result<ParkingSystemState, ServerFnError> {
    #[cfg(feature = "server")]
    {
        return load_state();
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
        let mut state = load_state()?;
        let zone = state
            .lots
            .iter_mut()
            .find(|z| z.id == zone_id)
            .ok_or_else(|| ServerFnError::new("Parking introuvable".to_string()))?;

        zone.occupied = occupied.min(zone.capacity);
        zone.ev_occupied = ev_occupied.min(zone.ev_capacity);
        state.updated_at = chrono::Utc::now().to_rfc3339();
        save_state(&state)?;
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
        let mut state = load_state()?;
        let success_message = {
            let lot = state
                .lots
                .iter_mut()
                .find(|z| z.id == lot_id)
                .ok_or_else(|| ServerFnError::new("Parking introuvable".to_string()))?;

            if lot.occupied >= lot.capacity {
                return Ok(ParkingOperationResult {
                    accepted: false,
                    message: format!("{} est complet.", lot.name),
                    state,
                });
            }
            if is_electric && lot.ev_occupied >= lot.ev_capacity {
                return Ok(ParkingOperationResult {
                    accepted: false,
                    message: format!("Plus de place de recharge disponible dans {}.", lot.name),
                    state,
                });
            }

            lot.occupied += 1;
            if is_electric {
                lot.ev_occupied += 1;
            }

            format!("Entree enregistree dans {}.", lot.name)
        };
        state.updated_at = chrono::Utc::now().to_rfc3339();
        save_state(&state)?;
        return Ok(ParkingOperationResult {
            accepted: true,
            message: success_message,
            state,
        });
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
        let mut state = load_state()?;
        let success_message = {
            let lot = state
                .lots
                .iter_mut()
                .find(|z| z.id == lot_id)
                .ok_or_else(|| ServerFnError::new("Parking introuvable".to_string()))?;

            if lot.occupied == 0 {
                return Ok(ParkingOperationResult {
                    accepted: false,
                    message: format!("{} est deja vide.", lot.name),
                    state,
                });
            }
            lot.occupied -= 1;
            if was_electric && lot.ev_occupied > 0 {
                lot.ev_occupied -= 1;
            }

            format!("Sortie enregistree depuis {}.", lot.name)
        };
        state.updated_at = chrono::Utc::now().to_rfc3339();
        save_state(&state)?;
        return Ok(ParkingOperationResult {
            accepted: true,
            message: success_message,
            state,
        });
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
        let mut state = load_state()?;
        let seed = chrono::Utc::now().timestamp_subsec_millis() as i32;
        for (i, lot) in state.lots.iter_mut().enumerate() {
            let swing = ((seed + i as i32 * 37) % 7) - 3;
            let next = (lot.occupied as i32 + swing).clamp(0, lot.capacity as i32) as u32;
            lot.occupied = next;

            if lot.ev_capacity > 0 {
                let ev_swing = ((seed + i as i32 * 13) % 3) - 1;
                let next_ev =
                    (lot.ev_occupied as i32 + ev_swing).clamp(0, lot.ev_capacity as i32) as u32;
                lot.ev_occupied = next_ev.min(lot.occupied);
            }
        }
        state.updated_at = chrono::Utc::now().to_rfc3339();
        save_state(&state)?;
        return Ok(state);
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = token;
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}
