use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use chrono::Datelike;
#[cfg(feature = "server")]
use chrono::Timelike;

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

#[cfg(feature = "server")]
fn mix32(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x
}

#[cfg(feature = "server")]
fn simulated_hourly_histogram(
    start_hour: u8,
    end_hour_exclusive: u8,
    preferred_peak_start: u8,
    preferred_peak_end: u8,
    base_scale: i64,
    jitter_pct: i64,
    profile_seed: u32,
) -> Vec<HourlyAffluence> {
    let day_index = chrono::Utc::now().date_naive().num_days_from_ce() as u32;
    let day_seed = mix32(profile_seed ^ day_index ^ 0x1f12_3bb5);
    let peak_span = (preferred_peak_end - preferred_peak_start + 1) as u32;
    let peak_hour = preferred_peak_start + (day_seed % peak_span) as u8;
    let mut result = Vec::with_capacity((end_hour_exclusive - start_hour) as usize);

    for hour in start_hour..end_hour_exclusive {
        let distance = (hour as i32 - peak_hour as i32).unsigned_abs() as i64;
        let peak_shape = (32 - 5 * distance).max(8); // highest around peak hour

        // Slight bump for evening hours (shopping after work).
        let evening_bonus = if (17..=19).contains(&hour) { 5 } else { 0 };

        // Deterministic jitter per histogram/day/hour.
        let noise_seed =
            mix32(profile_seed ^ day_index ^ ((hour as u32 + 1).wrapping_mul(0x9e37_79b9)));
        let centered_jitter = (noise_seed % (2 * jitter_pct as u32 + 1)) as i64 - jitter_pct;

        let value = (base_scale * (peak_shape + evening_bonus) / 32)
            .saturating_mul(100 + centered_jitter)
            / 100;

        result.push(HourlyAffluence {
            hour,
            visits: value.max(1),
        });
    }

    result
}

#[cfg(feature = "server")]
fn current_local_hour_from_utc() -> u8 {
    let utc_now = chrono::Utc::now();
    let local_offset_secs = chrono::Local::now().offset().local_minus_utc();
    let local_now_from_utc = utc_now + chrono::Duration::seconds(local_offset_secs as i64);
    local_now_from_utc.hour() as u8
}

#[cfg(feature = "server")]
fn has_enough_real_data(per_hour: &[i64; 24], min_non_zero_buckets: usize, min_total: i64) -> bool {
    let non_zero = per_hour.iter().filter(|v| **v > 0).count();
    let total: i64 = per_hour.iter().sum();
    non_zero >= min_non_zero_buckets && total >= min_total
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

        let histogram = if has_enough_real_data(&per_hour, 5, 50) {
            (OPENING_HOUR_START..OPENING_HOUR_END_EXCLUSIVE)
                .map(|hour| HourlyAffluence {
                    hour,
                    visits: per_hour[hour as usize],
                })
                .collect()
        } else {
            // Simulated "physical crowd" profile with organic peak in afternoon.
            simulated_hourly_histogram(
                OPENING_HOUR_START,
                OPENING_HOUR_END_EXCLUSIVE,
                14,
                17,
                140,
                16,
                0x5048_5953, // "PHYS"
            )
        };
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

        let histogram = if has_enough_real_data(&per_hour, 10, 120) {
            (0_u8..24_u8)
                .map(|hour| HourlyAffluence {
                    hour,
                    visits: per_hour[hour as usize],
                })
                .collect()
        } else {
            // Simulated "website traffic" profile with afternoon peak drift.
            simulated_hourly_histogram(0, 24, 14, 18, 180, 22, 0x5745_4221) // "WEB!"
        };
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
pub async fn get_sliding_24h_web_affluence() -> Result<Vec<HourlyAffluence>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = crate::db::pool().await;
        let current_local_hour = current_local_hour_from_utc();
        let (current_hour_visits,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM visits
             WHERE visited_at >= datetime('now', '-24 hour')
               AND CAST(strftime('%H', visited_at, 'localtime') AS INTEGER) = CAST(strftime('%H', 'now', 'localtime') AS INTEGER)",
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Variant requested by UI: simulated values for all hours except the current local one.
        let mut histogram = simulated_hourly_histogram(0, 24, 14, 18, 160, 20, 0x524f_4c4c); // "ROLL"
        for bucket in &mut histogram {
            if bucket.hour == current_local_hour {
                bucket.visits = current_hour_visits.max(0);
            }
        }
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
