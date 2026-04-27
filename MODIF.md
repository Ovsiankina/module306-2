# MODIF - Suivi des modifications depuis le 25.04.2026

Ce document synthétise les changements techniques du projet à partir du `2026-04-25`, en s'appuyant sur `git log` et `git diff`.

## 1) Perimetre et methode

- Source commits: `git log --since="2026-04-25 00:00"`.
- Source differenciel local: `git diff --stat` + `git status`.
- Perimetre: code applicatif, base SQLite locale, ressources seed, i18n, documentation.

## 2) Chronologie des commits (depuis le 25.04.2026)

### 2026-04-25

- `533f55c` - `style(rewards-draw): adjust category card horizontal padding to 24px`
  - Fichiers touches: `assets/tailwind.css`, `src/components/rewards_draw.rs`, `foxtown.db`.
  - Impact: ajustement UI/spacing de l'ecran rewards draw.

### 2026-04-26

- `f40d533` - `feat(vouchers): add admin voucher management and localized auth flows`
  - Fichiers touches (selection): `src/services/vouchers.rs`, `src/components/voucher_list.rs`, `src/auth/mod.rs`, `src/db.rs`, `assets/i18n/*`, `.env.example`, `migrations/0001_init.sql`.
  - Impact:
    - introduction/extension des flux voucher cote admin,
    - enrichissement i18n,
    - ajustements auth + schema.

- `b03cbb5` - `feat(ui): add game promo modal and refine rewards draw layout ...`
  - Fichiers touches: `src/components/home.rs`, `src/components/rewards_draw.rs`, `assets/i18n/*`, `assets/tailwind.css`, `data/vouchers.json`.
  - Impact:
    - ajout modal promo jeu sur home,
    - nettoyages layout rewards draw,
    - maj copie multilingue.

### 2026-04-27

- `4c10c1c` - `feat(data): triple test vouchers with simulated customers and redeemed statuses`
  - Fichiers touches: `data/vouchers.json`, `src/services/visits.rs`, `src/services/parking.rs`, `src/components/visits_stats.rs`, `src/components/nav.rs`, `assets/tailwind.css`, `foxtown.db`.
  - Impact:
    - enrichissement data de test vouchers,
    - evolution stats visites/parking.

## 3) Modifications locales non committees (etat courant)

Resume `git diff --stat`:

- `17 files changed, 840 insertions(+), 2333 deletions(-)` (hors fichiers non suivis).
- Ajouts non suivis:
  - `migrations/0002_vouchers_stores_parkings.sql`
  - `migrations/seeders/` (nouvelle arbo seeders JSON)

### Principaux changements en cours

- **Refonte des seed JSON**
  - anciens JSON de `data/` deplaces vers `migrations/seeders/`:
    - `stores.json`
    - `vouchers.json`
    - `parkings.json`
    - `visits.json`
  - references code/docs maj pour pointer vers `migrations/seeders/*`.

- **Initialisation DB / seed conditionnel**
  - `src/db.rs`:
    - ajout gate environnement (`APP_ENV`/`RUST_ENV`/`ENV`) pour limiter une partie des seeds au mode dev,
    - exception demandee: seed `stores` independant du mode (si table vide),
    - seeds JSON etendus pour `stores`, `vouchers`, `parkings`.

- **Configuration**
  - ajout `APP_ENV=development` dans `.env` et `.env.example`.
  - `VOUCHERS_JSON_PATH` mis a jour vers `migrations/seeders/vouchers.json`.

- **Stores / Rewards / Home**
  - ajustements `src/stores.rs`, `src/services/game.rs`, `src/components/rewards_draw.rs`, `src/components/game_rules.rs`, `src/components/home.rs`:
    - sourcing DB pour plusieurs flux,
    - filtrage des stores avec image scope a l'onglet Stores (et non global).

- **Data locale SQLite**
  - `foxtown.db` modifie (imports vouchers, generations visits, seeds de test).

## 4) Impacts techniques

- **Schema/seed**
  - separation plus nette entre schema DB et seeders JSON.
  - seeds centralises dans `migrations/seeders`.

- **Execution par environnement**
  - production: evite le seed dev (sauf `stores` explicitement maintenu).
  - development: seed automatique pour faciliter demos/tests.

- **Observabilite fonctionnelle**
  - donnees de visites enrichies (histogrammes/statistiques plus representatifs).
  - donnees vouchers synchronisees avec la table SQLite.

## 5) Captures ecran associees

Captures ajoutees dans `documentation/screenshots/`:

- `documentation/screenshots/visits-year-card.png`
  - Carte "VISITS THIS YEAR" (verification affichage stats).

- `documentation/screenshots/voucher-purge-button.png`
  - Zone admin vouchers (compteur utilises + bouton purge).

## 6) Points de vigilance avant commit/push

- Verifier que le deplacement `data/ -> migrations/seeders/` est bien complet et coherent dans tous les environnements.
- Verifier l'impact de `APP_ENV` sur les pipelines CI/CD et scripts de demarrage.
- Decider si `foxtown.db` doit rester versionne avec ces nouvelles donnees de test.
- Eventuellement finaliser le nettoyage du dossier `data/` si vide et non requis.

## 7) Etat des tables SQLite (structure + volume)

Snapshot releve dans `foxtown.db` au moment de la generation du document.

### Comptage des entrees

- `users`: **2**
- `stores`: **155**
- `vouchers`: **30**
- `visits`: **2743**
- `parkings`: **6**
- `parking_charging_stations`: **4**
- `daily_gifts`: **0**

### Structure par table

#### `users` (10 colonnes)
- `id` INTEGER PK
- `username` TEXT NOT NULL
- `email` TEXT NOT NULL
- `password_hash` TEXT NOT NULL
- `role` TEXT NOT NULL DEFAULT `'Editor'`
- `created_at` DATETIME DEFAULT `CURRENT_TIMESTAMP`
- `last_played_at` DATE
- `daily_attempts` INTEGER DEFAULT `0`
- `first_name` TEXT NOT NULL DEFAULT `''`
- `last_name` TEXT NOT NULL DEFAULT `''`

#### `stores` (8 colonnes)
- `id` INTEGER PK
- `name` TEXT NOT NULL
- `category` TEXT NOT NULL
- `store_number` TEXT
- `level` INTEGER
- `phone` TEXT
- `website` TEXT
- `icon_path` TEXT

#### `vouchers` (11 colonnes)
- `id` INTEGER PK
- `qr_token` TEXT NOT NULL
- `email` TEXT NOT NULL
- `username` TEXT NOT NULL
- `first_name` TEXT NOT NULL DEFAULT `''`
- `last_name` TEXT NOT NULL DEFAULT `''`
- `store` TEXT NOT NULL
- `discount` INTEGER NOT NULL
- `valid_until` TEXT NOT NULL
- `created_at` DATETIME NOT NULL DEFAULT `CURRENT_TIMESTAMP`
- `redeemed` INTEGER NOT NULL DEFAULT `0`

#### `visits` (4 colonnes)
- `id` INTEGER PK
- `visited_at` DATETIME NOT NULL DEFAULT `CURRENT_TIMESTAMP`
- `path` TEXT NOT NULL
- `session_id` TEXT NOT NULL

#### `parkings` (11 colonnes)
- `id` TEXT PK
- `name` TEXT NOT NULL
- `kind` TEXT NOT NULL
- `level` TEXT
- `capacity` INTEGER NOT NULL
- `occupied` INTEGER NOT NULL
- `reserved_accessible` INTEGER NOT NULL
- `reserved_family` INTEGER NOT NULL
- `ev_capacity` INTEGER NOT NULL
- `ev_occupied` INTEGER NOT NULL
- `updated_at` TEXT NOT NULL

#### `parking_charging_stations` (10 colonnes)
- `id` INTEGER PK
- `parking_id` TEXT NOT NULL
- `network` TEXT NOT NULL
- `station_type` TEXT NOT NULL
- `power_kw` INTEGER NOT NULL
- `connectors` TEXT NOT NULL
- `ports` INTEGER NOT NULL
- `paid` INTEGER NOT NULL
- `availability` TEXT NOT NULL
- `notes` TEXT NOT NULL

#### `daily_gifts` (4 colonnes)
- `id` INTEGER PK
- `user_id` INTEGER NOT NULL
- `awarded_at` DATE DEFAULT `CURRENT_DATE`
- `store` TEXT NOT NULL

## 8) Additions techniques (27.04.2026)

### 8.1 Routes/pages admin et navigation

- nouvelles pages admin connectees:
  - `/admin/stores` (`src/components/stores_admin.rs`)
  - `/admin/game-rules` (`src/components/game_rules.rs`)
- menu admin enrichi dans `src/components/nav.rs` (desktop + mobile).

### 8.2 Gestion des stores (CRUD + images)

- `src/stores.rs`:
  - CRUD serveur complet des stores (`list_store_rows`, `create_store`, `update_store`, `delete_store`),
  - upload image valide par signature bytes (JPEG/PNG/WEBP),
  - normalisation nom fichier + limite 5 MB,
  - persistence dans `public/brands`.
- `src/components/stores_admin.rs`:
  - ecran de creation/edition/suppression + preview logo.

### 8.3 Jeu et regles

- `src/services/game.rs`:
  - gestion centralisee des `GameRules`,
  - 1 partie max / jour / joueur,
  - suivi quota quotidien et cooldown,
  - choix distribue des shops gagnants.
- `src/components/rewards_draw.rs`:
  - flux complet categories -> tirage store -> tirage discount -> emission voucher.
- `src/components/game_rules.rs`:
  - configuration dynamique des regles de tirage et validite voucher.

### 8.4 Vouchers

- `src/services/vouchers.rs`:
  - creation voucher + QR code,
  - envoi SMTP (avec `SMTP_FAKE_MODE`),
  - verification QR token,
  - listing admin complet et purge vouchers redeemed,
  - listing des gagnants recents pour le ticker home.

### 8.5 Parking

- `src/services/parking.rs`:
  - etat parking persiste en SQLite,
  - endpoints entree/sortie/snapshot/simulation,
  - validations metier strictes (capacites fixes, coherence globale).

### 8.6 DB / migration / seeders

- ajout migration `migrations/0002_vouchers_stores_parkings.sql`.
- seeders ajoutes dans `migrations/seeders/`:
  - `stores.json`
  - `vouchers.json`
  - `parkings.json`
  - `visits.json`
- `src/db.rs`:
  - seed conditionnel selon `APP_ENV`/`RUST_ENV`/`ENV`,
  - seed `stores` force si table vide,
  - sync auto `assets/brands` -> `public/brands`.

### 8.7 i18n / assets / docs

- i18n ajoute/maj:
  - `assets/i18n/fr.json`
  - `assets/i18n/en.json`
  - `assets/i18n/de.json`
  - `assets/i18n/it.json`
- assets logos ajoutes dans `assets/brands/*` et `public/brands/*`.
- docs mises a jour:
  - `README.md`
  - `PARKING_API.md`

## 9) Mise a jour complementaire (27.04.2026 - 16:54)

Cette mise a jour confirme et consolide l'etat des derniers ajouts afin d'eviter les oublis dans le suivi.

### 9.1 Validation des modules ajoutes recemment

- modules/routes confirms dans `src/main.rs`:
  - `components::game_rules`
  - `components::stores_admin`
  - `services::game`
  - `services::parking`
  - `services::vouchers`
- page admin stores activee via route: `/admin/stores`.

### 9.2 Cible fonctionnelle des derniers ajouts

- **Back-office stores**: creation, edition, suppression, preview image, upload differe au save.
- **Game rules admin**: parametrage des boules noires, plages de reduction, entropie, duree de mix.
- **Rewards draw**: enchainement complet avec blocage quota journalier et seconde chance.
- **Parking**: operations de simulation persistees en base (etat + bornes de recharge).
- **Vouchers**: emission, verification, listing admin, purge redeemed et support email SMTP.

### 9.3 Rappel de verification avant livraison

- verifier que les variables d'env (`APP_ENV`, SMTP, `APP_BASE_URL`) sont coherentes selon l'environnement;
- confirmer le choix de versionner ou non `foxtown.db`;
- verifier que tous les logos attendus existent bien dans `public/brands`;
- executer une passe rapide UI admin (stores/game rules/vouchers/parking) avant commit final.

