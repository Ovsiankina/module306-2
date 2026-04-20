use crate::context::auth::AuthState;
use crate::i18n::{translate, translate_fmt, Locale};
use crate::services::game::{
    all_categories,
    can_award_prize_today,
    can_start_game_today,
    choose_distributed_shop,
    delay_ms,
    random_index,
    register_game_start,
    register_prize_award,
    shops_by_category_keys,
    ShopInfo,
};
use dioxus::prelude::*;
use std::collections::HashMap;

const DISCOUNT_BLACK_BALLS: usize = 15;
const DISCOUNT_DISTRIBUTION: [(u32, usize); 10] = [
    (50, 1),
    (45, 2),
    (40, 2),
    (35, 2),
    (30, 2),
    (25, 3),
    (20, 3),
    (15, 5),
    (10, 5),
    (5, 10),
];
const CONTAINER_RADIUS: f64 = 120.0;
const BALL_RADIUS: f64 = 16.0;
const PHYSICS_DT_MS: u64 = 16;
const STORE_DRAW_STEPS: usize = 460;
const DISCOUNT_DRAW_STEPS: usize = 520;

#[derive(Clone, Debug, PartialEq)]
pub struct WinnerEvent {
    pub user_name: String,
    pub user_email: String,
    pub shop_name: String,
    pub discount_percent: u32,
    pub valid_until_iso: String,
    pub qr_payload: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawPhase {
    SelectCategories,
    DrawStore,
    DrawDiscount,
    Completed,
}

#[derive(Clone, Debug, PartialEq)]
struct SimulatedEmail {
    to: String,
    subject: String,
    body_preview: String,
    qr_payload: String,
}

#[derive(Clone, Copy, Debug)]
struct Ball3D {
    value: u32,
    is_black: bool,
    hue: f64,
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    r: f64,
}

fn category_translation_key(key: &str) -> Option<&'static str> {
    match key {
        "HIGH_FASHION" => Some("home.category.luxury_fashion"),
        "LADIES_MENSWEAR" => Some("home.category.fashion"),
        "CASUALWEAR" => Some("home.category.casualwear"),
        "SPORTSWEAR_EQUIPMENT" => Some("home.category.sport_performance"),
        "CHILDRENSWEAR" => Some("home.category.kidswear"),
        "FOOTWEAR" => Some("home.category.footwear"),
        "UNDERWEAR" => Some("home.category.underwear"),
        "WATCHES_JEWELLERY" => Some("home.category.luxury_heritage"),
        "ACCESSORIES" => Some("home.category.accessories"),
        "ELECTRONICS" => Some("home.category.electronics"),
        "BEAUTY" => Some("home.category.beauty"),
        "HOME" => Some("home.category.home_lifestyle"),
        "FOOD_DRINKS" => Some("home.category.food_drinks"),
        "SERVICES" => Some("home.category.services"),
        _ => None,
    }
}

fn init_discount_balls() -> Vec<Ball3D> {
    let mut color_values = Vec::with_capacity(35);
    for (value, count) in DISCOUNT_DISTRIBUTION {
        for _ in 0..count {
            color_values.push(value);
        }
    }

    let mut balls: Vec<Ball3D> = color_values
        .into_iter()
        .enumerate()
        .map(|(i, value)| {
            let len = (35 + DISCOUNT_BLACK_BALLS) as f64;
            let a = i as f64 / len * std::f64::consts::TAU;
            let ring = 55.0 + ((i % 3) as f64) * 10.0;
            let t = ((value as f64 - 5.0) / 45.0).clamp(0.0, 1.0);
            Ball3D {
                value,
                is_black: false,
                hue: 220.0 * (1.0 - t),
                x: a.cos() * ring * 0.75,
                y: a.sin() * ring * 0.55,
                z: ((i % 5) as f64 - 2.0) * 16.0,
                vx: (((i * 37) % 11) as f64 - 5.0) * 14.0,
                vy: (((i * 19) % 9) as f64 - 4.0) * 10.0,
                vz: (((i * 29) % 13) as f64 - 6.0) * 12.0,
                r: BALL_RADIUS,
            }
        })
        .collect();

    let offset = balls.len();
    for i in 0..DISCOUNT_BLACK_BALLS {
        let idx = i + offset;
        let len = (offset + DISCOUNT_BLACK_BALLS) as f64;
        let a = idx as f64 / len * std::f64::consts::TAU;
        let ring = 50.0 + ((i % 4) as f64) * 8.0;
        balls.push(Ball3D {
            value: 0,
            is_black: true,
            hue: 0.0,
            x: a.cos() * ring * 0.76,
            y: a.sin() * ring * 0.54,
            z: ((i % 6) as f64 - 2.0) * 13.0,
            vx: (((idx * 41) % 13) as f64 - 6.0) * 12.0,
            vy: (((idx * 23) % 11) as f64 - 5.0) * 9.0,
            vz: (((idx * 31) % 15) as f64 - 7.0) * 11.0,
            r: BALL_RADIUS,
        });
    }

    balls
}

fn init_store_balls(active_categories: &[String], shops: &[ShopInfo]) -> Vec<Ball3D> {
    let mut category_hues: HashMap<&str, f64> = HashMap::new();
    let total_categories = active_categories.len().max(1) as f64;
    for (i, category) in active_categories.iter().enumerate() {
        let ratio = i as f64 / total_categories;
        category_hues.insert(category.as_str(), 25.0 + ratio * 260.0);
    }

    let total_store_balls = shops.len().max(1);
    let mut balls: Vec<Ball3D> = shops
        .iter()
        .enumerate()
        .map(|(i, shop)| {
            let len = total_store_balls as f64;
            let a = i as f64 / len * std::f64::consts::TAU;
            let ring = 50.0 + ((i % 4) as f64) * 8.0;
            let hue = category_hues.get(shop.category.as_str()).copied().unwrap_or(210.0);
            Ball3D {
                value: i as u32 + 1,
                is_black: false,
                hue,
                x: a.cos() * ring * 0.75,
                y: a.sin() * ring * 0.55,
                z: ((i % 6) as f64 - 2.0) * 15.0,
                vx: (((i * 37) % 11) as f64 - 5.0) * 14.0,
                vy: (((i * 19) % 9) as f64 - 4.0) * 10.0,
                vz: (((i * 29) % 13) as f64 - 6.0) * 12.0,
                r: BALL_RADIUS,
            }
        })
        .collect();

    let black_ball_count = 9_usize.max(active_categories.len());
    let offset = balls.len();
    for i in 0..black_ball_count {
        let idx = i + offset;
        let len = (offset + black_ball_count) as f64;
        let a = idx as f64 / len * std::f64::consts::TAU;
        let ring = 48.0 + ((i % 4) as f64) * 8.0;
        balls.push(Ball3D {
            value: 0,
            is_black: true,
            hue: 0.0,
            x: a.cos() * ring * 0.78,
            y: a.sin() * ring * 0.52,
            z: ((i % 5) as f64 - 2.0) * 13.0,
            vx: (((idx * 41) % 13) as f64 - 6.0) * 12.0,
            vy: (((idx * 23) % 11) as f64 - 5.0) * 9.0,
            vz: (((idx * 31) % 15) as f64 - 7.0) * 11.0,
            r: BALL_RADIUS,
        });
    }

    balls
}

fn step_collision_physics(balls: &mut [Ball3D], dt: f64, air_power: f64, tick: usize) {
    let gravity = 120.0;
    let buoyancy = 25.0 + 320.0 * air_power;
    let swirl_force = 40.0 + 120.0 * air_power;
    let linear_damping = 0.998;
    let wall_bounce = 1.9;
    let wall_damping = 0.985;
    let collision_restitution = 0.9;

    for (i, ball) in balls.iter_mut().enumerate() {
        let phase = tick as f64 * 0.05 + (ball.value as f64) * 0.17 + (i as f64) * 0.11;
        let swirl_x = phase.cos() + (ball.z / CONTAINER_RADIUS) * 0.7;
        let swirl_z = phase.sin() - (ball.x / CONTAINER_RADIUS) * 0.7;
        ball.vx += swirl_x * swirl_force * dt;
        ball.vz += swirl_z * swirl_force * dt;

        if ball.y < -18.0 {
            ball.vy += (buoyancy + 35.0) * dt;
        } else {
            ball.vy += buoyancy * dt;
        }

        ball.vy -= gravity * dt;
        ball.x += ball.vx * dt;
        ball.y += ball.vy * dt;
        ball.z += ball.vz * dt;

        ball.vx *= linear_damping;
        ball.vy *= linear_damping;
        ball.vz *= linear_damping;

        let dist = (ball.x * ball.x + ball.y * ball.y + ball.z * ball.z).sqrt();
        let max_dist = CONTAINER_RADIUS - ball.r;
        if dist > max_dist && dist > 0.0001 {
            let nx = ball.x / dist;
            let ny = ball.y / dist;
            let nz = ball.z / dist;

            ball.x = nx * max_dist;
            ball.y = ny * max_dist;
            ball.z = nz * max_dist;

            let vn = ball.vx * nx + ball.vy * ny + ball.vz * nz;
            if vn > 0.0 {
                ball.vx -= wall_bounce * vn * nx;
                ball.vy -= wall_bounce * vn * ny;
                ball.vz -= wall_bounce * vn * nz;
            }

            ball.vx *= wall_damping;
            ball.vy *= wall_damping;
            ball.vz *= wall_damping;
        }
    }

    for i in 0..balls.len() {
        for j in (i + 1)..balls.len() {
            let dx = balls[j].x - balls[i].x;
            let dy = balls[j].y - balls[i].y;
            let dz = balls[j].z - balls[i].z;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            let min_dist = balls[i].r + balls[j].r;
            if dist_sq < min_dist * min_dist && dist_sq > 0.000001 {
                let dist = dist_sq.sqrt();
                let nx = dx / dist;
                let ny = dy / dist;
                let nz = dz / dist;

                let overlap = min_dist - dist;
                balls[i].x -= nx * overlap * 0.5;
                balls[i].y -= ny * overlap * 0.5;
                balls[i].z -= nz * overlap * 0.5;
                balls[j].x += nx * overlap * 0.5;
                balls[j].y += ny * overlap * 0.5;
                balls[j].z += nz * overlap * 0.5;

                let rvx = balls[j].vx - balls[i].vx;
                let rvy = balls[j].vy - balls[i].vy;
                let rvz = balls[j].vz - balls[i].vz;
                let vel_along_normal = rvx * nx + rvy * ny + rvz * nz;

                if vel_along_normal < 0.0 {
                    let impulse = -(1.0 + collision_restitution) * vel_along_normal * 0.5;
                    balls[i].vx -= impulse * nx;
                    balls[i].vy -= impulse * ny;
                    balls[i].vz -= impulse * nz;
                    balls[j].vx += impulse * nx;
                    balls[j].vy += impulse * ny;
                    balls[j].vz += impulse * nz;
                }
            }
        }
    }
}

fn render_machine(
    ordered_balls: &[Ball3D],
    extracted_value: Option<u32>,
    show_value: bool,
    cracked_store_label: Option<&str>,
    show_percent_stamp: bool,
    stamp_ink_progress: f64,
) -> Element {
    const BALL_SIZE_FACTOR: f64 = 0.75;
    let shell_top = 16.0;
    let shell_radius = 120.0;
    let shell_center_y = shell_top + shell_radius;
    let extracted_hue = extracted_value
        .map(|value| {
            let t = ((value as f64 - 5.0) / 45.0).clamp(0.0, 1.0);
            220.0 * (1.0 - t)
        })
        .unwrap_or(215.0);
    let stamp_rotation_deg = extracted_value
        .map(|value| ((value as i32 * 13) % 21) - 10)
        .unwrap_or(0);
    let extracted_text_style = "position: absolute; left: 50%; top: 52%; transform: translate(-50%, -50%) rotateX(18deg); font-size: 30px; line-height: 1; font-weight: 900; text-shadow: 0 1px 0 #e2e8f0, 0 2px 4px rgba(15, 23, 42, 0.55); background: linear-gradient(180deg, #ffffff 0%, #dbeafe 44%, #93c5fd 100%); -webkit-background-clip: text; background-clip: text; -webkit-text-fill-color: transparent; user-select: none;";

    rsx! {
        div {
            class: "mx-auto relative",
            style: "position: relative; width: min(100%, 300px); height: clamp(270px, 70vw, 320px); margin: 0 auto; overflow: hidden;",
            div {
                style: "position: absolute; left: 50%; top: 0; width: 260px; height: 320px; transform: translateX(-50%) translateY(-6px) scale(0.82); transform-origin: top center;",
                div {
                    style: "position: absolute; left: 50%; top: 16px; width: 240px; height: 240px; transform: translateX(-50%); border: 2px solid rgba(148,163,184,0.55); border-radius: 9999px; background: radial-gradient(circle at 32% 24%, rgba(255,255,255,0.08), rgba(15,23,42,0.20) 58%, rgba(2,6,23,0.28) 100%); box-shadow: inset 0 0 22px rgba(255,255,255,0.08), 0 8px 18px rgba(2,6,23,0.28);"
                }
                div {
                    style: "position: absolute; left: 50%; top: 60px; width: 50px; height: 154px; transform: translateX(-50%); border-radius: 25px 25px 0 0; background: linear-gradient(90deg, rgba(148,163,184,0.18) 0%, rgba(226,232,240,0.26) 50%, rgba(148,163,184,0.18) 100%); border: 1px solid rgba(148,163,184,0.30); backdrop-filter: blur(4px); -webkit-backdrop-filter: blur(4px); z-index: 5;"
                }
                div {
                    style: "position: absolute; left: 50%; top: 62px; width: 46px; height: 150px; transform: translateX(-50%); border-radius: 23px 23px 0 0; background: linear-gradient(90deg, rgba(71,85,105,0.24) 0%, rgba(148,163,184,0.18) 50%, rgba(71,85,105,0.24) 100%); border: 1px solid rgba(148,163,184,0.20); backdrop-filter: blur(6px); -webkit-backdrop-filter: blur(6px); z-index: 6;"
                }
                div {
                    style: "position: absolute; left: 50%; top: 210px; width: 78px; height: 30px; transform: translateX(-50%); background: linear-gradient(180deg, rgba(148,163,184,0.22) 0%, rgba(148,163,184,0.26) 45%, rgba(100,116,139,0.20) 100%); border: 1px solid rgba(148,163,184,0.26); clip-path: polygon(17.95% 0%, 82.05% 0%, 100% 100%, 0% 100%); z-index: 5;"
                }
                div {
                    style: "position: absolute; left: 50%; top: 220px; width: 70px; height: 58px; transform: translateX(-50%); border-radius: 9999px; background: radial-gradient(circle at 50% 34%, rgba(226,232,240,0.22) 0%, rgba(71,85,105,0.32) 100%); box-shadow: 0 0 0 2px rgba(148,163,184,0.18), inset 0 0 12px rgba(15,23,42,0.16); z-index: 10;"
                }
                for (i, ball) in ordered_balls.iter().enumerate() {
                    {
                        let camera_z = 360.0;
                        let depth = (camera_z - ball.z).max(120.0);
                        let scale = camera_z / depth;
                        let mut x2d = ball.x * scale;
                        let mut y2d = -ball.y * scale;
                        let size = ball.r * 2.0 * scale * BALL_SIZE_FACTOR;
                        let allowed = (shell_radius - size * 0.5 - 2.0).max(4.0);
                        let d2 = (x2d * x2d + y2d * y2d).sqrt();
                        if d2 > allowed && d2 > 0.0001 {
                            let k = allowed / d2;
                            x2d *= k;
                            y2d *= k;
                        }
                        let left_delta = x2d - size * 0.5;
                        let top = shell_center_y + y2d - size * 0.5;
                        let hue = ball.hue;

                        rsx! {
                            div {
                                key: "{i}",
                                style: format!(
                                    "position: absolute; left: calc(50% + {left_delta:.2}px); top: {top:.2}px; width: {size:.2}px; height: {size:.2}px;"
                                ),
                                div {
                                    style: format!(
                                        "position: absolute; left: 6px; right: 6px; bottom: -12px; height: 10px; border-radius: 9999px; background: rgba(2, 6, 23, 0.45); filter: blur(4px); transform: scale({:.2});",
                                        (1.05 - (ball.y / 180.0)).clamp(0.75, 1.15)
                                    )
                                }
                                div {
                                    class: "absolute inset-0 rounded-full",
                                    style: if ball.is_black {
                                        "position: absolute; inset: 0; border-radius: 9999px; border: 1px solid rgba(148,163,184,0.65); background: radial-gradient(circle at 32% 26%, #64748b 0%, #334155 20%, #111827 58%, #020617 100%); box-shadow: 0 9px 22px rgba(2,6,23,0.6), inset -8px -10px 16px rgba(0, 0, 0, 0.52), inset 4px 5px 8px rgba(148, 163, 184, 0.20);".to_string()
                                    } else {
                                        format!(
                                            "position: absolute; inset: 0; border-radius: 9999px; border: 1px solid rgba(255,255,255,0.55); background: radial-gradient(circle at 30% 25%, hsl({hue:.0}, 95%, 92%) 0%, hsl({hue:.0}, 90%, 72%) 22%, hsl({hue:.0}, 85%, 52%) 52%, hsl({hue:.0}, 80%, 40%) 72%, hsl({hue:.0}, 70%, 24%) 100%); box-shadow: 0 9px 22px hsla({hue:.0}, 85%, 40%, 0.35), inset -8px -10px 16px rgba(0, 0, 0, 0.24), inset 5px 6px 10px rgba(255, 255, 255, 0.28);"
                                        )
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(value) = extracted_value {
                    if let Some(store_label) = cracked_store_label {
                        div {
                            style: "position: absolute; left: 50%; top: 238px; width: 190px; height: 110px; transform: translateX(-50%); z-index: 14;",
                            div { style: "position: absolute; left: 30px; right: 30px; bottom: 6px; height: 11px; border-radius: 9999px; background: rgba(2, 6, 23, 0.40); filter: blur(5px);" }
                            div {
                                style: if value == 0 {
                                    "position: absolute; left: 63px; top: 46px; width: 28px; height: 56px; border-radius: 9999px 0 0 9999px; border: 1px solid rgba(148,163,184,0.65); border-right: 0; background: radial-gradient(circle at 28% 28%, #64748b 0%, #334155 26%, #111827 62%, #020617 100%); box-shadow: inset -6px -8px 12px rgba(0,0,0,0.42), inset 3px 4px 7px rgba(148,163,184,0.18); transform: rotate(-12deg) translateX(-4px);".to_string()
                                } else {
                                    format!("position: absolute; left: 63px; top: 46px; width: 28px; height: 56px; border-radius: 9999px 0 0 9999px; border: 1px solid rgba(255,255,255,0.55); border-right: 0; background: radial-gradient(circle at 28% 28%, hsl({extracted_hue:.0}, 95%, 92%) 0%, hsl({extracted_hue:.0}, 90%, 72%) 26%, hsl({extracted_hue:.0}, 85%, 52%) 62%, hsl({extracted_hue:.0}, 70%, 24%) 100%); box-shadow: inset -6px -8px 12px rgba(0,0,0,0.26), inset 3px 4px 7px rgba(255,255,255,0.22); transform: rotate(-12deg) translateX(-4px);")
                                }
                            }
                            div {
                                style: if value == 0 {
                                    "position: absolute; right: 63px; top: 46px; width: 28px; height: 56px; border-radius: 0 9999px 9999px 0; border: 1px solid rgba(148,163,184,0.65); border-left: 0; background: radial-gradient(circle at 72% 28%, #64748b 0%, #334155 26%, #111827 62%, #020617 100%); box-shadow: inset 6px -8px 12px rgba(0,0,0,0.42), inset -3px 4px 7px rgba(148,163,184,0.18); transform: rotate(12deg) translateX(4px);".to_string()
                                } else {
                                    format!("position: absolute; right: 63px; top: 46px; width: 28px; height: 56px; border-radius: 0 9999px 9999px 0; border: 1px solid rgba(255,255,255,0.55); border-left: 0; background: radial-gradient(circle at 72% 28%, hsl({extracted_hue:.0}, 95%, 92%) 0%, hsl({extracted_hue:.0}, 90%, 72%) 26%, hsl({extracted_hue:.0}, 85%, 52%) 62%, hsl({extracted_hue:.0}, 70%, 24%) 100%); box-shadow: inset 6px -8px 12px rgba(0,0,0,0.26), inset -3px 4px 7px rgba(255,255,255,0.22); transform: rotate(12deg) translateX(4px);")
                                }
                            }
                            div {
                                style: "position: absolute; left: 50%; top: 0; width: 172px; height: 64px; padding: 1px; transform: translateX(-50%); border-radius: 8px; border: 1px solid rgba(148,163,184,0.45); background: linear-gradient(180deg, #fff 0%, #f8fafc 100%); box-shadow: 0 8px 18px rgba(15,23,42,0.22); z-index: 16; display: flex; align-items: center; justify-content: center; text-align: center; font-size: 18px; line-height: 1.05; font-weight: 900; color: #1e293b; white-space: normal; overflow-wrap: anywhere;",
                                "{store_label}"
                            }
                        }
                    } else {
                        div {
                            style: "position: absolute; left: 50%; top: 252px; width: 55.5px; height: 55.5px; transform: translateX(-50%); z-index: 12;",
                            div { style: "position: absolute; left: 8px; right: 8px; bottom: -10px; height: 9px; border-radius: 9999px; background: rgba(2, 6, 23, 0.52); filter: blur(4px);" }
                            div {
                                style: if value == 0 {
                                    "position: absolute; inset: 0; border-radius: 9999px; border: 1px solid rgba(148,163,184,0.65); background: radial-gradient(circle at 32% 26%, #64748b 0%, #334155 20%, #111827 58%, #020617 100%); box-shadow: 0 9px 22px rgba(2,6,23,0.6), inset -8px -10px 16px rgba(0, 0, 0, 0.52), inset 4px 5px 8px rgba(148, 163, 184, 0.20);".to_string()
                                } else {
                                    format!("position: absolute; inset: 0; border-radius: 9999px; border: 1px solid rgba(255,255,255,0.55); background: radial-gradient(circle at 30% 25%, hsl({extracted_hue:.0}, 95%, 92%) 0%, hsl({extracted_hue:.0}, 90%, 72%) 22%, hsl({extracted_hue:.0}, 85%, 52%) 52%, hsl({extracted_hue:.0}, 80%, 40%) 72%, hsl({extracted_hue:.0}, 70%, 24%) 100%); box-shadow: 0 9px 22px hsla({extracted_hue:.0}, 85%, 40%, 0.35), inset -8px -10px 16px rgba(0, 0, 0, 0.24), inset 5px 6px 10px rgba(255, 255, 255, 0.28);")
                                }
                            }
                            if value != 0 && show_value {
                                span { style: "{extracted_text_style}", "{value}" }
                            }
                            if show_percent_stamp && value != 0 {
                                div {
                                    style: "position: absolute; left: 0; top: 0; width: 136px; height: 60px; border: 8px solid rgba(239,68,68,0); border-radius: 16px; box-shadow: 0 6px 16px rgba(220,38,38,0.18), inset 0 0 0 1px rgba(239,68,68,0); background: transparent;",
                                    div {
                                        style: format!(
                                            "position: absolute; left: -2px; top: -2px; width: calc(100% + 4px); height: calc(100% + 4px); border-radius: 18px; pointer-events: none; opacity: {:.3}; background: radial-gradient(circle at 74% 44%, rgba(239,68,68,0.35) 0%, rgba(239,68,68,0.22) 28%, rgba(239,68,68,0.07) 58%, rgba(239,68,68,0.0) 76%); filter: blur({:.2}px);",
                                            (0.15 + 0.35 * stamp_ink_progress).clamp(0.0, 0.5),
                                            (0.4 + 4.8 * stamp_ink_progress).clamp(0.0, 5.2)
                                        )
                                    }
                                    div {
                                        style: format!(
                                            "position: absolute; left: 44px; top: 22px; width: 16px; height: {:.2}px; border-radius: 0 0 9999px 9999px; background: linear-gradient(180deg, rgba(239,68,68,0.50) 0%, rgba(239,68,68,0.18) 72%, rgba(239,68,68,0.0) 100%); opacity: {:.3}; filter: blur({:.2}px);",
                                            6.0 + 20.0 * stamp_ink_progress,
                                            (0.06 + 0.44 * stamp_ink_progress).clamp(0.0, 0.5),
                                            (0.3 + 2.2 * stamp_ink_progress).clamp(0.0, 2.5)
                                        )
                                    }
                                    div {
                                        style: format!(
                                            "position: absolute; left: 58px; top: 24px; width: 10px; height: {:.2}px; border-radius: 0 0 9999px 9999px; background: linear-gradient(180deg, rgba(239,68,68,0.42) 0%, rgba(239,68,68,0.12) 72%, rgba(239,68,68,0.0) 100%); opacity: {:.3}; filter: blur({:.2}px);",
                                            4.0 + 14.0 * stamp_ink_progress,
                                            (0.04 + 0.36 * stamp_ink_progress).clamp(0.0, 0.4),
                                            (0.2 + 1.6 * stamp_ink_progress).clamp(0.0, 1.8)
                                        )
                                    }
                                    div {
                                        style: format!(
                                            "position: absolute; left: 58px; top: 50%; transform: translateY(-50%) rotate({}deg); color: rgba(239,68,68,0.95); font-size: 54px; font-weight: 900; line-height: 1; text-shadow: 0 1px 0 rgba(239,68,68,0.2);",
                                            stamp_rotation_deg
                                        ),
                                        "%"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RewardsDraw(on_win: EventHandler<WinnerEvent>) -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();
    let mut phase = use_signal(|| DrawPhase::SelectCategories);
    let categories = all_categories();
    let mut selected_categories = use_signal(Vec::<String>::new);
    let mut shops = use_signal(Vec::<ShopInfo>::new);

    let mut store_balls = use_signal(Vec::<Ball3D>::new);
    let mut discount_balls = use_signal(init_discount_balls);
    let mut drawing = use_signal(|| false);
    let mut extracted_store_ball = use_signal(|| None::<u32>);
    let mut extracted_discount_ball = use_signal(|| None::<u32>);
    let mut extracted_store = use_signal(|| None::<String>);
    let mut second_chance_used = use_signal(|| false);
    let mut discount_stamp_visible = use_signal(|| false);
    let mut discount_stamp_ink_progress = use_signal(|| 0.0_f64);
    let mut status_message =
        use_signal(|| translate(locale(), "rewards_draw.status.pick_categories"));
    let mut qr_email_status = use_signal(String::new);
    let mut simulated_email = use_signal(|| None::<SimulatedEmail>);

    let mut toggle_category = move |key: String| {
        if drawing() {
            return;
        }

        let mut next = selected_categories();
        if next.contains(&key) {
            next.retain(|k| k != &key);
        } else if next.len() < 3 {
            next.push(key);
        }
        selected_categories.set(next);
    };

    let build_store_draw = move |_| {
        if drawing() {
            return;
        }
        let player_key = match auth() {
            AuthState::LoggedIn(user) => format!("user:{}", user.id),
            _ => "guest".to_string(),
        };
        if !can_start_game_today(&player_key) {
            status_message.set(translate(
                locale(),
                "rewards_draw.status.one_game_per_day_limit",
            ));
            return;
        }
        let selected = selected_categories();
        if selected.is_empty() {
            status_message.set(translate(locale(), "rewards_draw.status.min_one_category"));
            return;
        }
        if selected.len() > 3 {
            status_message.set(translate(locale(), "rewards_draw.status.max_three_categories"));
            return;
        }

        let available_shops = shops_by_category_keys(&selected);
        if available_shops.is_empty() {
            status_message.set(translate(locale(), "rewards_draw.status.no_store_for_selection"));
            return;
        }

        shops.set(available_shops.clone());
        store_balls.set(init_store_balls(&selected, &available_shops));
        discount_balls.set(init_discount_balls());
        extracted_store_ball.set(None);
        extracted_discount_ball.set(None);
        extracted_store.set(None);
        second_chance_used.set(false);
        qr_email_status.set(String::new());
        simulated_email.set(None);
        phase.set(DrawPhase::DrawStore);
        status_message.set(translate_fmt(
            locale(),
            "rewards_draw.status.store_draw_ready",
            &[
                ("stores", available_shops.len().to_string()),
                ("black", 9_usize.max(selected.len()).to_string()),
            ],
        ));
    };

    let mut draw_store = move |_| {
        if drawing() || phase() != DrawPhase::DrawStore {
            return;
        }

        drawing.set(true);
        extracted_store_ball.set(None);
        let tries_used = second_chance_used();
        let player_key = match auth() {
            AuthState::LoggedIn(user) => format!("user:{}", user.id),
            _ => "guest".to_string(),
        };
        let active_balls = store_balls();
        let active_shops = shops();

        spawn(async move {
            for step in 0..STORE_DRAW_STEPS {
                let mut next = store_balls();
                let pulse_period = 20;
                let pulse_is_on = (step % pulse_period) < 10;
                let power = if pulse_is_on { 1.0 } else { 0.0 };
                step_collision_physics(&mut next, 0.016, power, step);
                store_balls.set(next);
                delay_ms(PHYSICS_DT_MS).await;
            }

            let mut next = store_balls();
            let picked_index = if active_balls.is_empty() {
                0
            } else {
                random_index(active_balls.len())
            };
            let picked = active_balls.get(picked_index).copied();
            if !next.is_empty() && picked_index < next.len() {
                next.remove(picked_index);
            }
            store_balls.set(next);

            if let Some(ball) = picked {
                extracted_store_ball.set(Some(ball.value));
                if ball.is_black {
                    if tries_used {
                        register_game_start(&player_key);
                        status_message.set(translate(locale(), "rewards_draw.status.no_promo"));
                        phase.set(DrawPhase::Completed);
                    } else {
                        second_chance_used.set(true);
                        status_message
                            .set(translate(locale(), "rewards_draw.status.second_chance"));
                    }
                } else {
                    let store_index = (ball.value as usize).saturating_sub(1);
                    let selected_shop = active_shops
                        .get(store_index)
                        .map(|shop| shop.name.clone())
                        .unwrap_or_else(|| {
                            translate(locale(), "rewards_draw.store.unknown")
                        });
                    let distributed_shop = choose_distributed_shop(&active_shops)
                        .map(|shop| shop.name)
                        .unwrap_or(selected_shop);
                    extracted_store.set(Some(distributed_shop.clone()));
                    phase.set(DrawPhase::DrawDiscount);
                    extracted_discount_ball.set(None);
                    status_message.set(translate_fmt(
                        locale(),
                        "rewards_draw.status.store_targeted",
                        &[("store", distributed_shop)],
                    ));
                }
            }

            drawing.set(false);
        });
    };

    let mut draw_discount = move |_| {
        if drawing() || phase() != DrawPhase::DrawDiscount {
            return;
        }
        let player_key = match auth() {
            AuthState::LoggedIn(user) => format!("user:{}", user.id),
            _ => "guest".to_string(),
        };
        if !can_award_prize_today() {
            status_message.set(translate(
                locale(),
                "rewards_draw.status.daily_prize_limit_reached",
            ));
            phase.set(DrawPhase::Completed);
            return;
        }

        drawing.set(true);
        extracted_discount_ball.set(None);
        discount_stamp_visible.set(false);
        discount_stamp_ink_progress.set(0.0);

        spawn(async move {
            for step in 0..DISCOUNT_DRAW_STEPS {
                let mut next = discount_balls();
                let pulse_period = 20;
                let pulse_is_on = (step % pulse_period) < 10;
                let power = if pulse_is_on { 1.0 } else { 0.0 };
                step_collision_physics(&mut next, 0.016, power, step);
                discount_balls.set(next);
                delay_ms(PHYSICS_DT_MS).await;
            }

            let mut next = discount_balls();
            let picked_ball = if next.is_empty() {
                None
            } else {
                Some(next.remove(random_index(next.len())))
            };
            discount_balls.set(next);
            let picked = picked_ball.map(|ball| ball.value).unwrap_or(0);
            extracted_discount_ball.set(Some(picked));
            phase.set(DrawPhase::Completed);

            if picked_ball.map(|ball| ball.is_black).unwrap_or(false) {
                register_game_start(&player_key);
                status_message.set(translate(locale(), "rewards_draw.status.no_promo"));
                qr_email_status.set(String::new());
                drawing.set(false);
                return;
            }

            spawn(async move {
                delay_ms(1_000).await;
                discount_stamp_visible.set(true);
                for step in 0..=20 {
                    discount_stamp_ink_progress.set(step as f64 / 20.0);
                    delay_ms(100).await;
                }
            });

            let store_name = extracted_store()
                .unwrap_or_else(|| translate(locale(), "rewards_draw.store.unknown"));
            register_game_start(&player_key);
            if !register_prize_award(&store_name) {
                extracted_discount_ball.set(Some(0));
                status_message.set(translate(
                    locale(),
                    "rewards_draw.status.daily_prize_limit_just_reached",
                ));
                qr_email_status.set(String::new());
                drawing.set(false);
                return;
            }
            let (user_name, user_email) = match auth() {
                AuthState::LoggedIn(user) => (user.username, user.email),
                _ => ("Guest".to_string(), "guest@example.com".to_string()),
            };
            let valid_until = (chrono::Utc::now() + chrono::Duration::days(30)).date_naive();
            let valid_until_iso = valid_until.to_string();
            let qr_payload = format!(
                "user={};store={};discount={}%;valid_until={}",
                user_name, store_name, picked, valid_until_iso
            );
            let email_preview = SimulatedEmail {
                to: user_email.clone(),
                subject: translate(locale(), "rewards_draw.email.subject"),
                body_preview: translate_fmt(
                    locale(),
                    "rewards_draw.email.body",
                    &[
                        ("user", user_name.clone()),
                        ("discount", picked.to_string()),
                        ("store", store_name.clone()),
                        ("date", valid_until_iso.clone()),
                    ],
                ),
                qr_payload: qr_payload.clone(),
            };
            simulated_email.set(Some(email_preview));
            qr_email_status.set(translate_fmt(
                locale(),
                "rewards_draw.status.email_ready",
                &[("email", user_email.clone())],
            ));
            status_message.set(translate_fmt(
                locale(),
                "rewards_draw.status.win",
                &[("discount", picked.to_string()), ("store", store_name.clone())],
            ));
            on_win.call(WinnerEvent {
                user_name,
                user_email,
                shop_name: store_name,
                discount_percent: picked,
                valid_until_iso,
                qr_payload,
            });
            drawing.set(false);
        });
    };

    let restart = move |_| {
        selected_categories.set(Vec::new());
        shops.set(Vec::new());
        store_balls.set(Vec::new());
        discount_balls.set(init_discount_balls());
        extracted_store_ball.set(None);
        extracted_discount_ball.set(None);
        extracted_store.set(None);
        second_chance_used.set(false);
        discount_stamp_visible.set(false);
        discount_stamp_ink_progress.set(0.0);
        qr_email_status.set(String::new());
        simulated_email.set(None);
        status_message.set(translate(locale(), "rewards_draw.status.pick_categories"));
        phase.set(DrawPhase::SelectCategories);
    };

    let mut ordered_store_balls = store_balls();
    ordered_store_balls
        .sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or(std::cmp::Ordering::Equal));
    let mut ordered_discount_balls = discount_balls();
    ordered_discount_balls
        .sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or(std::cmp::Ordering::Equal));
    let can_show_machine = matches!(phase(), DrawPhase::DrawStore | DrawPhase::DrawDiscount | DrawPhase::Completed);
    let show_discount_stamp = phase() == DrawPhase::Completed
        && extracted_discount_ball().map(|v| v > 0).unwrap_or(false)
        && discount_stamp_visible();

    rsx! {
        div { class: "relative flex flex-col items-center gap-6",
            if can_show_machine {
                p {
                    class: "text-xs text-muted tracking-wider",
                    style: "position: relative; top: -132px; left: 24px; width: 544px; height: 40px; display: grid; grid-template-columns: repeat(1, 1fr); grid-template-rows: repeat(1, 1fr); transform: none;",
                    if phase() == DrawPhase::DrawStore {
                        if second_chance_used() {
                            {translate(locale(), "rewards_draw.phase.second_chance")}
                        } else {
                            {translate(locale(), "rewards_draw.phase.first_store_draw")}
                        }
                    } else if phase() == DrawPhase::DrawDiscount {
                        {translate(locale(), "rewards_draw.phase.discount_draw")}
                    } else {
                        {translate(locale(), "rewards_draw.phase.session_finished")}
                    }
                }
            }
            div { class: "w-full max-w-3xl bg-white border border-gray-100 rounded-xl p-4",
                p { class: "text-xs font-bold tracking-widest text-accent mb-3",
                    {translate(locale(), "rewards_draw.title.pick_categories")}
                }
                div { class: "flex flex-wrap gap-2",
                    for category in categories {
                        button {
                            key: "{category.key}",
                            class: if selected_categories().contains(&category.key) {
                                "px-3 py-2 text-xs font-bold rounded-full bg-dark text-white"
                            } else if selected_categories().len() >= 3 {
                                "px-3 py-2 text-xs font-bold rounded-full bg-gray-100 text-gray-400 cursor-not-allowed"
                            } else {
                                "px-3 py-2 text-xs font-bold rounded-full bg-gray-100 text-dark hover:bg-gray-200"
                            },
                            disabled: !selected_categories().contains(&category.key) && selected_categories().len() >= 3,
                            onclick: {
                                let key = category.key.clone();
                                move |_| toggle_category(key.clone())
                            },
                            {
                                category_translation_key(&category.key)
                                    .map(|key| translate(locale(), key))
                                    .unwrap_or_else(|| category.label.clone())
                            }
                        }
                    }
                }
                p { class: "mt-3 text-xs text-muted",
                    {translate_fmt(
                        locale(),
                        "rewards_draw.selected_count",
                        &[("count", selected_categories().len().to_string())]
                    )}
                }
                button {
                    class: "mt-4 px-4 py-2 text-xs font-bold tracking-wider rounded-lg bg-accent text-white disabled:bg-gray-300",
                    disabled: drawing() || selected_categories().is_empty() || phase() != DrawPhase::SelectCategories,
                    onclick: build_store_draw,
                    {translate(locale(), "rewards_draw.button.validate_categories")}
                }
                if phase() == DrawPhase::SelectCategories {
                    p { class: "mt-3 text-xs font-semibold text-amber-700",
                        "{status_message()}"
                    }
                }
            }

            if can_show_machine {
                div { class: "w-full max-w-5xl grid grid-cols-2 gap-6 items-start",
                    div { class: "bg-white border border-gray-100 rounded-xl p-3",
                        p { class: "text-xs font-bold tracking-widest text-muted text-center mb-2",
                            style: "transform: none;",
                            {translate(locale(), "rewards_draw.phase.first_store_draw")}
                        }
                        {render_machine(
                            &ordered_store_balls,
                            extracted_store_ball(),
                            false,
                            extracted_store().as_deref(),
                            false,
                            0.0,
                        )}
                    }
                    div { class: "bg-white border border-gray-100 rounded-xl p-3",
                        p { class: "text-xs font-bold tracking-widest text-muted text-center mb-2",
                            style: "transform: none;",
                            {translate(locale(), "rewards_draw.phase.discount_draw")}
                        }
                        {render_machine(
                            &ordered_discount_balls,
                            extracted_discount_ball(),
                            true,
                            None,
                            show_discount_stamp,
                            discount_stamp_ink_progress(),
                        )}
                    }
                }
            }

            if can_show_machine {
                div { class: "text-center",
                    div { class: "flex items-center justify-center gap-3 mb-3",
                        button {
                            class: if !drawing() && phase() == DrawPhase::DrawStore {
                                "px-6 py-3 text-xs font-bold tracking-wider text-white bg-accent hover:bg-amber-600 rounded-lg transition-colors shadow-lg shadow-accent/30"
                            } else {
                                "px-6 py-3 text-xs font-bold tracking-wider text-white bg-gray-300 rounded-lg transition-colors cursor-not-allowed"
                            },
                            disabled: drawing() || phase() != DrawPhase::DrawStore,
                            onclick: draw_store,
                            if drawing() && phase() == DrawPhase::DrawStore {
                                {translate(locale(), "rewards_draw.button.drawing")}
                            } else {
                                {translate(locale(), "rewards_draw.button.draw_store")}
                            }
                        }
                        button {
                            class: if !drawing() && phase() == DrawPhase::DrawDiscount {
                                "px-6 py-3 text-xs font-bold tracking-wider text-white bg-accent hover:bg-amber-600 rounded-lg transition-colors shadow-lg shadow-accent/30"
                            } else {
                                "px-6 py-3 text-xs font-bold tracking-wider text-white bg-gray-300 rounded-lg transition-colors cursor-not-allowed"
                            },
                            disabled: drawing() || phase() != DrawPhase::DrawDiscount,
                            onclick: draw_discount,
                            if drawing() && phase() == DrawPhase::DrawDiscount {
                                {translate(locale(), "rewards_draw.button.drawing")}
                            } else {
                                {translate(locale(), "rewards_draw.button.draw_discount")}
                            }
                        }
                    }

                p { class: "text-xs text-muted mt-2",
                    "{status_message()}"
                }
                if let Some(store) = extracted_store() {
                    p { class: "text-xs font-semibold text-accent mt-1",
                        {translate_fmt(locale(), "rewards_draw.target_store", &[("store", store)])}
                    }
                }
                if let Some(value) = extracted_discount_ball() {
                    if phase() == DrawPhase::Completed && value > 0 {
                        p { class: "text-xs font-semibold text-green-700 mt-1",
                            {translate_fmt(locale(), "rewards_draw.final_discount", &[("discount", value.to_string())])}
                        }
                    }
                }
                if !qr_email_status().is_empty() {
                    p { class: "text-xs text-muted mt-1", "{qr_email_status()}" }
                }
                if let Some(email) = simulated_email() {
                    div { class: "mt-3 text-left bg-gray-50 border border-gray-200 rounded-lg p-3 max-w-xl mx-auto",
                        p { class: "text-xs font-bold tracking-wider text-dark mb-2",
                            {translate(locale(), "rewards_draw.email.preview_title")}
                        }
                        p { class: "text-xs text-muted", "To: {email.to}" }
                        p { class: "text-xs text-muted", "Subject: {email.subject}" }
                        p { class: "text-xs text-muted mt-1", "{email.body_preview}" }
                        p { class: "text-[11px] text-accent mt-2 break-all",
                            {translate_fmt(locale(), "rewards_draw.email.qr_payload", &[("payload", email.qr_payload)])}
                        }
                    }
                }
                if phase() == DrawPhase::Completed {
                    button {
                        class: "mt-3 px-4 py-2 text-xs font-bold tracking-wider rounded-lg bg-dark text-white hover:bg-gray-800",
                        onclick: restart,
                        {translate(locale(), "rewards_draw.button.new_game")}
                    }
                }
                }
            }
        }
    }
}
