use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const OPENING_HOUR_START: u8 = 11;
const OPENING_HOUR_END_EXCLUSIVE: u8 = 19;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisitStats {
    pub daily: i64,
    pub monthly: i64,
    pub yearly: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyAffluence {
    pub hour: u8,
    pub visits: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisitRecommendation {
    pub best_slots: Vec<String>,
    pub avoid_slots: Vec<String>,
}

fn hour_slot(hour: u8) -> String {
    format!("{hour:02}h-{:02}h", hour + 1)
}

#[server]
pub async fn register_visit(path: String, session_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        sqlx::query("INSERT INTO visits (path, session_id) VALUES (?, ?)")
            .bind(path)
            .bind(session_id)
            .execute(pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        return Ok(());
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = (path, session_id);
        Err(ServerFnError::new("Server feature is required".to_string()))
    }
}

#[server]
pub async fn get_daily_visits() -> Result<i64, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM visits WHERE date(visited_at, 'localtime') = date('now', 'localtime')",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        return Ok(count);
    }

    #[cfg(not(feature = "server"))]
    {
        Ok(0)
    }
}

#[server]
pub async fn get_monthly_visits() -> Result<i64, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM visits
             WHERE strftime('%Y-%m', visited_at, 'localtime') = strftime('%Y-%m', 'now', 'localtime')",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        return Ok(count);
    }

    #[cfg(not(feature = "server"))]
    {
        Ok(0)
    }
}

#[server]
pub async fn get_yearly_visits() -> Result<i64, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM visits
             WHERE strftime('%Y', visited_at, 'localtime') = strftime('%Y', 'now', 'localtime')",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        return Ok(count);
    }

    #[cfg(not(feature = "server"))]
    {
        Ok(0)
    }
}

#[server]
pub async fn get_visit_stats() -> Result<VisitStats, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let (daily,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM visits WHERE date(visited_at, 'localtime') = date('now', 'localtime')",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        let (monthly,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM visits
             WHERE strftime('%Y-%m', visited_at, 'localtime') = strftime('%Y-%m', 'now', 'localtime')",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        let (yearly,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM visits
             WHERE strftime('%Y', visited_at, 'localtime') = strftime('%Y', 'now', 'localtime')",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        return Ok(VisitStats {
            daily,
            monthly,
            yearly,
        });
    }

    #[cfg(not(feature = "server"))]
    {
        Ok(VisitStats {
            daily: 0,
            monthly: 0,
            yearly: 0,
        })
    }
}

#[server]
pub async fn get_hourly_affluence() -> Result<Vec<HourlyAffluence>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let rows: Vec<(i64, i64)> = sqlx::query_as(
            "SELECT CAST(strftime('%H', visited_at, 'localtime') AS INTEGER) AS hour_value, COUNT(*) AS visit_count
             FROM visits
             WHERE visited_at >= datetime('now', '-30 day')
               AND CAST(strftime('%H', visited_at, 'localtime') AS INTEGER) >= 11
               AND CAST(strftime('%H', visited_at, 'localtime') AS INTEGER) < 19
             GROUP BY hour_value
             ORDER BY hour_value",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let mut per_hour = [0_i64; 24];
        for (hour, count) in rows {
            if (0..24).contains(&hour) {
                per_hour[hour as usize] = count;
            }
        }

        let histogram = (OPENING_HOUR_START..OPENING_HOUR_END_EXCLUSIVE)
            .map(|hour| HourlyAffluence {
                hour,
                visits: per_hour[hour as usize],
            })
            .collect();
        return Ok(histogram);
    }

    #[cfg(not(feature = "server"))]
    {
        Ok((OPENING_HOUR_START..OPENING_HOUR_END_EXCLUSIVE)
            .map(|hour| HourlyAffluence { hour, visits: 0 })
            .collect())
    }
}

#[server]
pub async fn get_hourly_web_affluence() -> Result<Vec<HourlyAffluence>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let rows: Vec<(i64, i64)> = sqlx::query_as(
            "SELECT CAST(strftime('%H', visited_at, 'localtime') AS INTEGER) AS hour_value, COUNT(*) AS visit_count
             FROM visits
             WHERE visited_at >= datetime('now', '-30 day')
             GROUP BY hour_value
             ORDER BY hour_value",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let mut per_hour = [0_i64; 24];
        for (hour, count) in rows {
            if (0..24).contains(&hour) {
                per_hour[hour as usize] = count;
            }
        }

        let histogram = (0_u8..24_u8)
            .map(|hour| HourlyAffluence {
                hour,
                visits: per_hour[hour as usize],
            })
            .collect();
        return Ok(histogram);
    }

    #[cfg(not(feature = "server"))]
    {
        Ok((0_u8..24_u8)
            .map(|hour| HourlyAffluence { hour, visits: 0 })
            .collect())
    }
}

#[server]
pub async fn get_today_physical_recommendation() -> Result<VisitRecommendation, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let rows: Vec<(i64, i64)> = sqlx::query_as(
            "SELECT CAST(strftime('%H', visited_at, 'localtime') AS INTEGER) AS hour_value, COUNT(*) AS visit_count
             FROM visits
             WHERE date(visited_at, 'localtime') = date('now', 'localtime')
               AND CAST(strftime('%H', visited_at, 'localtime') AS INTEGER) >= 11
               AND CAST(strftime('%H', visited_at, 'localtime') AS INTEGER) < 19
             GROUP BY hour_value
             ORDER BY hour_value",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let mut per_hour = [0_i64; 24];
        for (hour, count) in rows {
            if (0..24).contains(&hour) {
                per_hour[hour as usize] = count;
            }
        }

        let mut opening_hours: Vec<(u8, i64)> = (OPENING_HOUR_START..OPENING_HOUR_END_EXCLUSIVE)
            .map(|h| (h, per_hour[h as usize]))
            .collect();

        let mut lowest = opening_hours.clone();
        lowest.sort_by_key(|(_, count)| *count);
        let best_slots = lowest
            .into_iter()
            .take(2)
            .map(|(hour, _)| hour_slot(hour))
            .collect();

        opening_hours.sort_by_key(|(_, count)| -*count);
        let avoid_slots = opening_hours
            .into_iter()
            .take(1)
            .map(|(hour, _)| hour_slot(hour))
            .collect();

        return Ok(VisitRecommendation {
            best_slots,
            avoid_slots,
        });
    }

    #[cfg(not(feature = "server"))]
    {
        Ok(VisitRecommendation {
            best_slots: vec!["11h-12h".to_string(), "17h-18h".to_string()],
            avoid_slots: vec!["15h-16h".to_string()],
        })
    }
}
