#[cfg(feature = "server")]
pub use server::{pool, sync_brand_assets_to_public};

#[cfg(feature = "server")]
mod server {
    use sqlx::SqlitePool;
    use std::fs;
    use std::path::PathBuf;
    use tokio::sync::OnceCell;

    static POOL: OnceCell<SqlitePool> = OnceCell::const_new();

    fn is_development_mode() -> bool {
        let mode = std::env::var("APP_ENV")
            .or_else(|_| std::env::var("RUST_ENV"))
            .or_else(|_| std::env::var("ENV"))
            .unwrap_or_else(|_| "development".to_string())
            .trim()
            .to_ascii_lowercase();

        !matches!(mode.as_str(), "production" | "prod")
    }

    /// Return (or lazily initialise) the shared SQLite connection pool.
    /// Tables are created on first call; idempotent on subsequent calls.
    pub async fn pool() -> &'static SqlitePool {
        POOL.get_or_init(|| async {
            let url = std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:foxtown.db".to_string());
            let pool = SqlitePool::connect(&url)
                .await
                .expect("Failed to connect to SQLite database");
            init_schema(&pool)
                .await
                .expect("Failed to initialise database schema");
            pool
        })
        .await
    }

    /// Synchronise all files from `assets/brands` into `public/brands`.
    /// This keeps `/brands/...` URLs resolvable after store CRUD operations.
    pub async fn sync_brand_assets_to_public() -> Result<(), String> {
        tokio::task::spawn_blocking(|| {
            let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let src = root.join("assets").join("brands");
            let dst = root.join("public").join("brands");

            if !src.exists() {
                return Ok(());
            }

            fs::create_dir_all(&dst).map_err(|e| format!("create public/brands failed: {e}"))?;

            let entries = fs::read_dir(&src).map_err(|e| format!("read assets/brands failed: {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("read_dir entry failed: {e}"))?;
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }

                let Some(file_name) = path.file_name() else {
                    continue;
                };
                let target = dst.join(file_name);
                let should_copy = if !target.exists() {
                    true
                } else {
                    let src_bytes = fs::read(&path)
                        .map_err(|e| format!("read source '{}' failed: {e}", path.display()))?;
                    let dst_bytes = fs::read(&target)
                        .map_err(|e| format!("read target '{}' failed: {e}", target.display()))?;
                    src_bytes != dst_bytes
                };

                if !should_copy {
                    continue;
                }

                fs::copy(&path, &target).map_err(|e| {
                    format!(
                        "copy '{}' -> '{}' failed: {e}",
                        path.display(),
                        target.display()
                    )
                })?;
            }

            Ok(())
        })
        .await
        .map_err(|e| format!("sync task join failed: {e}"))?
    }

    async fn init_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                username        TEXT    NOT NULL UNIQUE,
                email           TEXT    NOT NULL UNIQUE,
                first_name      TEXT    NOT NULL DEFAULT '',
                last_name       TEXT    NOT NULL DEFAULT '',
                password_hash   TEXT    NOT NULL,
                role            TEXT    NOT NULL DEFAULT 'Editor',
                created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_played_at  DATE,
                daily_attempts  INTEGER DEFAULT 0
            )",
        )
        .execute(pool)
        .await?;

        for stmt in [
            "ALTER TABLE users ADD COLUMN first_name TEXT NOT NULL DEFAULT ''",
            "ALTER TABLE users ADD COLUMN last_name TEXT NOT NULL DEFAULT ''",
        ] {
            if let Err(e) = sqlx::query(stmt).execute(pool).await {
                let msg = e.to_string();
                if !msg.contains("duplicate column") {
                    return Err(e);
                }
            }
        }

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS daily_gifts (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id     INTEGER NOT NULL REFERENCES users(id),
                awarded_at  DATE    DEFAULT CURRENT_DATE,
                store       TEXT    NOT NULL
            )",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS visits (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                visited_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                path        TEXT    NOT NULL,
                session_id  TEXT    NOT NULL
            )",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS vouchers (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                qr_token      TEXT    NOT NULL UNIQUE,
                email         TEXT    NOT NULL,
                username      TEXT    NOT NULL,
                first_name    TEXT    NOT NULL DEFAULT '',
                last_name     TEXT    NOT NULL DEFAULT '',
                store         TEXT    NOT NULL,
                discount      INTEGER NOT NULL,
                valid_until   TEXT    NOT NULL,
                created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                redeemed      INTEGER NOT NULL DEFAULT 0
            )",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS stores (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                name          TEXT    NOT NULL,
                category      TEXT    NOT NULL,
                store_number  TEXT,
                level         INTEGER,
                phone         TEXT,
                website       TEXT,
                icon_path     TEXT,
                map_x         REAL,
                map_y         REAL
            )",
        )
        .execute(pool)
        .await?;

        // Backfill: ensure map_x/map_y columns exist when upgrading from an
        // older schema that predates the floor-plan editor.
        let _ = sqlx::query("ALTER TABLE stores ADD COLUMN map_x REAL")
            .execute(pool)
            .await;
        let _ = sqlx::query("ALTER TABLE stores ADD COLUMN map_y REAL")
            .execute(pool)
            .await;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS parkings (
                id                    TEXT PRIMARY KEY,
                name                  TEXT    NOT NULL,
                kind                  TEXT    NOT NULL,
                level                 TEXT,
                capacity              INTEGER NOT NULL,
                occupied              INTEGER NOT NULL,
                reserved_accessible   INTEGER NOT NULL,
                reserved_family       INTEGER NOT NULL,
                ev_capacity           INTEGER NOT NULL,
                ev_occupied           INTEGER NOT NULL,
                updated_at            TEXT    NOT NULL
            )",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS parking_charging_stations (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                parking_id    TEXT    NOT NULL REFERENCES parkings(id) ON DELETE CASCADE,
                network       TEXT    NOT NULL,
                station_type  TEXT    NOT NULL,
                power_kw      INTEGER NOT NULL,
                connectors    TEXT    NOT NULL,
                ports         INTEGER NOT NULL,
                paid          INTEGER NOT NULL,
                availability  TEXT    NOT NULL,
                notes         TEXT    NOT NULL
            )",
        )
        .execute(pool)
        .await?;

        let (stores_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stores")
            .fetch_one(pool)
            .await?;
        if stores_count == 0 {
            #[derive(serde::Deserialize)]
            struct SeedStore {
                name: String,
                category: String,
                store_number: Option<String>,
                level: Option<u8>,
                phone: Option<String>,
                website: Option<String>,
                icon_path: Option<String>,
            }
            #[derive(serde::Deserialize)]
            struct SeedStores {
                shops: Vec<SeedStore>,
            }

            let seed: SeedStores = serde_json::from_str(include_str!("../migrations/seeders/stores.json"))
                .expect("stores seed JSON must be valid");
            for shop in seed.shops {
                sqlx::query(
                    "INSERT INTO stores (name, category, store_number, level, phone, website, icon_path)
                     VALUES (?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(shop.name)
                .bind(shop.category)
                .bind(shop.store_number)
                .bind(shop.level.map(|v| v as i64))
                .bind(shop.phone)
                .bind(shop.website)
                .bind(shop.icon_path)
                .execute(pool)
                .await?;
            }
        }
        let should_seed_data = is_development_mode();
        if should_seed_data {
            let (vouchers_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vouchers")
                .fetch_one(pool)
                .await?;
            if vouchers_count == 0 {
            #[derive(serde::Deserialize)]
            struct SeedVoucher {
                id: u64,
                qr_token: String,
                email: String,
                username: String,
                #[serde(default)]
                first_name: String,
                #[serde(default)]
                last_name: String,
                store: String,
                discount: u32,
                valid_until: String,
                created_at: String,
                redeemed: bool,
            }

            let seed: Vec<SeedVoucher> = serde_json::from_str(include_str!("../migrations/seeders/vouchers.json"))
                .expect("vouchers seed JSON must be valid");

            for voucher in seed {
                sqlx::query(
                    "INSERT INTO vouchers (id, qr_token, email, username, first_name, last_name, store, discount, valid_until, created_at, redeemed)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(voucher.id as i64)
                .bind(voucher.qr_token)
                .bind(voucher.email)
                .bind(voucher.username)
                .bind(voucher.first_name)
                .bind(voucher.last_name)
                .bind(voucher.store)
                .bind(voucher.discount as i64)
                .bind(voucher.valid_until)
                .bind(voucher.created_at)
                .bind(if voucher.redeemed { 1_i64 } else { 0_i64 })
                .execute(pool)
                .await?;
            }
        }

            let (parkings_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM parkings")
                .fetch_one(pool)
                .await?;
            if parkings_count == 0 {
            #[derive(serde::Deserialize)]
            struct SeedChargingStation {
                network: String,
                station_type: String,
                power_kw: u32,
                connectors: Vec<String>,
                ports: u32,
                paid: bool,
                availability: String,
                notes: String,
            }

            #[derive(serde::Deserialize)]
            struct SeedParkingLot {
                id: String,
                name: String,
                kind: String,
                level: Option<String>,
                capacity: u32,
                occupied: u32,
                reserved_accessible: u32,
                reserved_family: u32,
                ev_capacity: u32,
                ev_occupied: u32,
                #[serde(default)]
                charging_stations: Vec<SeedChargingStation>,
            }

            #[derive(serde::Deserialize)]
            struct SeedParkings {
                lots: Vec<SeedParkingLot>,
                updated_at: String,
            }

            let seed: SeedParkings = serde_json::from_str(include_str!("../migrations/seeders/parkings.json"))
                .expect("parkings seed JSON must be valid");

            for lot in seed.lots {
                sqlx::query(
                    "INSERT INTO parkings (id, name, kind, level, capacity, occupied, reserved_accessible, reserved_family, ev_capacity, ev_occupied, updated_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&lot.id)
                .bind(&lot.name)
                .bind(&lot.kind)
                .bind(&lot.level)
                .bind(lot.capacity as i64)
                .bind(lot.occupied as i64)
                .bind(lot.reserved_accessible as i64)
                .bind(lot.reserved_family as i64)
                .bind(lot.ev_capacity as i64)
                .bind(lot.ev_occupied as i64)
                .bind(&seed.updated_at)
                .execute(pool)
                .await?;

                for station in lot.charging_stations {
                    let connectors_json =
                        serde_json::to_string(&station.connectors).unwrap_or_else(|_| "[]".to_string());
                    sqlx::query(
                        "INSERT INTO parking_charging_stations
                         (parking_id, network, station_type, power_kw, connectors, ports, paid, availability, notes)
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    )
                    .bind(&lot.id)
                    .bind(station.network)
                    .bind(station.station_type)
                    .bind(station.power_kw as i64)
                    .bind(connectors_json)
                    .bind(station.ports as i64)
                    .bind(if station.paid { 1_i64 } else { 0_i64 })
                    .bind(station.availability)
                    .bind(station.notes)
                    .execute(pool)
                    .await?;
                }
            }
            }

            // Dev seed: admin / admin  — change before going to production
            let (count,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM users WHERE username = 'admin'",
            )
            .fetch_one(pool)
            .await?;

            if count == 0 {
                let hash = crate::auth::hash_password("admin")
                    .expect("Failed to hash seed password");
                sqlx::query(
                    "INSERT INTO users (username, email, first_name, last_name, password_hash, role)
                     VALUES (?, ?, ?, ?, ?, 'Admin')",
                )
                .bind("admin")
                .bind("admin@foxtown.local")
                .bind("Admin")
                .bind("User")
                .bind(hash)
                .execute(pool)
                .await?;
            }
        }

        Ok(())
    }
}
