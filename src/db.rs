#[cfg(feature = "server")]
pub use server::pool;

#[cfg(feature = "server")]
mod server {
    use sqlx::SqlitePool;
    use tokio::sync::OnceCell;

    static POOL: OnceCell<SqlitePool> = OnceCell::const_new();

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

    async fn init_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                username        TEXT    NOT NULL UNIQUE,
                email           TEXT    NOT NULL UNIQUE,
                password_hash   TEXT    NOT NULL,
                role            TEXT    NOT NULL DEFAULT 'Editor',
                created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_played_at  DATE,
                daily_attempts  INTEGER DEFAULT 0
            )",
        )
        .execute(pool)
        .await?;

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
                "INSERT INTO users (username, email, password_hash, role)
                 VALUES (?, ?, ?, 'Admin')",
            )
            .bind("admin")
            .bind("admin@foxtown.local")
            .bind(hash)
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}
