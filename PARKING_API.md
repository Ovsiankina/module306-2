# API Parkings (Fake API)

Reference de l'API parking implementee dans `src/services/parking.rs`.

Les endpoints sont exposes via les server functions Dioxus en `POST /api/<function_name>`.

## Regles metier

- Capacite globale maximale: `1800` places
- Nombre de parkings: `6` (exactement)
- Plus grand parking: `600` places
- Repartition figee des capacites:
  - `P1`: `220`
  - `P2`: `280`
  - `P3`: `300`
  - `FoxPark`: `180`
  - `P5`: `600`
  - `P6`: `220`
- Validation cote serveur sur toutes les operations (capacites, occupation EV, total global)

## Donnees persistees

- Fichier: `data/parkings.json`
- Variables d'environnement:
  - `PARKINGS_JSON_PATH` (optionnel): chemin alternatif pour le JSON

## Modeles principaux

### `ParkingSystemState`

Etat complet de la plateforme parking:

```json
{
  "max_total_capacity": 1800,
  "lots": [/* ParkingLot */],
  "updated_at": "2026-04-21T10:00:00Z"
}
```

### `ParkingLot`

```json
{
  "id": "p5",
  "name": "Parking P5",
  "kind": "outdoor",
  "level": "Entree principale",
  "capacity": 600,
  "occupied": 360,
  "reserved_accessible": 20,
  "reserved_family": 24,
  "ev_capacity": 2,
  "ev_occupied": 1,
  "charging_stations": [/* ChargingStation */]
}
```

### `ChargingStation`

```json
{
  "network": "GOFAST",
  "station_type": "Ultra-fast",
  "power_kw": 100,
  "connectors": ["CCS", "CHAdeMO", "Tesla"],
  "ports": 1,
  "paid": true,
  "availability": "24/7",
  "notes": "Ideal pour les visites de moins d'une heure"
}
```

### `ParkingSnapshot`

Vue simplifiee pour les jauges UI:

```json
{
  "zones": [
    {
      "id": "p5",
      "name": "Parking P5",
      "occupied": 360,
      "capacity": 600,
      "ev_occupied": 1,
      "ev_capacity": 2
    }
  ],
  "updated_at": "2026-04-21T10:00:00Z"
}
```

---

## Endpoints

### `POST /api/get_parking_system_state` (public)

Retourne l'etat detaille complet (`ParkingSystemState`).

Body:

```json
{}
```

Response: `ParkingSystemState`.

---

### `POST /api/get_parking_snapshot` (public)

Retourne la vue simplifiee (`ParkingSnapshot`) utilisee par la page parking.

Body:

```json
{}
```

Response: `ParkingSnapshot`.

---

### `POST /api/update_parking_zone_occupancy` (Admin)

Met a jour manuellement l'occupation d'un parking.

Body:

```json
{
  "token": "<jwt>",
  "zone_id": "p6",
  "occupied": 140,
  "ev_occupied": 4
}
```

Response: `ParkingSnapshot` (mis a jour).

---

### `POST /api/parking_vehicle_entry` (public)

Simule l'entree d'un vehicule dans un parking.

Body:

```json
{
  "lot_id": "p5",
  "is_electric": true
}
```

Response (`ParkingOperationResult`):

```json
{
  "accepted": true,
  "message": "Entree enregistree dans Parking P5.",
  "state": { /* ParkingSystemState */ }
}
```

Si plein (ou pas de borne dispo pour EV), `accepted` vaut `false`.

---

### `POST /api/parking_vehicle_exit` (public)

Simule la sortie d'un vehicule d'un parking.

Body:

```json
{
  "lot_id": "p5",
  "was_electric": true
}
```

Response: `ParkingOperationResult`.

---

### `POST /api/parking_simulate_tick` (Admin)

Applique une variation automatique pseudo-realiste sur tous les parkings
(utile pour demo / test).

Body:

```json
{
  "token": "<jwt>"
}
```

Response: `ParkingSystemState`.

---

## Notes sur les bornes (selon les specs FoxTown)

- P5:
  - 1 borne **GOFAST ultra-rapide 100kW** (payante, 24/7, multi-connecteurs)
  - 1 borne **AC Type 2 lente** (gratuite, horaires d'ouverture)
- Bornes lentes publiques eCarUp:
  - 4 a **FoxPark**
  - 6 a **P6 niveau -1**

## Allocation par aire (methode)

La repartition des capacites est basee sur une estimation de surface apparente
depuis 6 captures prises au meme zoom:

1. Delimitation visuelle de chaque parking sur la carte (`P1..P6` + `FoxPark`).
2. Comparaison des aires relatives (en pixels) entre parkings.
3. Normalisation des capacites avec contraintes metier:
   - total `1800`
   - 6 parkings
   - `P5` plus grand a `600`.

Cette methode donne une allocation coherente pour la simulation et le dashboard,
sans pretendre fournir des surfaces metriques exactes.

## Codes d'erreur courants

- `Parking introuvable`
- `La capacite globale doit rester fixee a 1800 places.`
- `La configuration doit contenir exactement 6 parkings.`
- `La somme des capacites parking doit faire 1800 places.`
- `Le plus grand parking doit avoir 600 places.`

