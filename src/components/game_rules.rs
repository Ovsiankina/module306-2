use crate::auth::Role;
use crate::components::footer::Footer;
use crate::components::nav::{Nav, NavPage};
use crate::context::auth::AuthState;
use crate::i18n::{translate, translate_fmt, Locale};
use crate::services::game::{
    default_game_rules, get_game_rules, total_unique_shops_count, update_game_rules,
    DiscountRangeRule, GameRules,
};
use crate::Route;
use dioxus::prelude::*;

const MAX_DISCOUNT_RANGES: usize = 20;

pub fn GameRulesPage() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let locale = use_context::<Signal<Locale>>();
    let nav = use_navigator();

    let mut rules = use_signal(default_game_rules);
    let mut discount_units_percent =
        use_signal(|| vec![true; default_game_rules().discount_draw.ranges.len()]);
    let mut loading = use_signal(|| true);
    let mut saving = use_signal(|| false);
    let mut status = use_signal(String::new);

    use_effect(move || {
        if matches!(auth(), AuthState::LoggedOut) {
            let _ = nav.replace(Route::Login {});
            return;
        }
        if matches!(auth(), AuthState::LoggedIn(user) if user.role != Role::Admin) {
            let _ = nav.replace(Route::Rewards {});
        }
    });

    use_effect(move || {
        if !loading() {
            return;
        }
        spawn(async move {
            match get_game_rules().await {
                Ok(server_rules) => {
                    let n = server_rules.discount_draw.ranges.len();
                    discount_units_percent.set(vec![true; n]);
                    rules.set(server_rules);
                    status.set(String::new());
                }
                Err(_) => status.set(translate(locale(), "game_rules.status.load_error")),
            }
            loading.set(false);
        });
    });

    let total_stores = total_unique_shops_count();
    let current = rules();
    let store_max_limit = total_stores.max(1).min(u16::MAX as usize) as u16;
    let store_min = current.store_draw.black_balls_min.min(store_max_limit);
    let store_max = current.store_draw.black_balls_max.min(store_max_limit);
    let store_current = current
        .store_draw
        .black_balls_current
        .clamp(store_min.min(store_max), store_min.max(store_max));

    let discount_colored_balls_sum: u32 = current
        .discount_draw
        .ranges
        .iter()
        .map(|r| u32::from(r.balls_weight))
        .sum();
    let discount_total_balls =
        discount_colored_balls_sum + u32::from(current.discount_draw.black_balls);

    rsx! {
        div { class: "min-h-screen bg-gray-50 font-heading",
            Nav { active: NavPage::None }
            main { class: "max-w-7xl mx-auto px-6 py-10",
                div { class: "flex items-center justify-between mb-6",
                    h1 { class: "text-3xl font-extrabold text-dark",
                        {translate(locale(), "game_rules.title")}
                    }
                    span { class: "text-xs uppercase tracking-widest text-accent font-bold",
                        {translate(locale(), "game_rules.admin_badge")}
                    }
                }
                p { class: "text-sm text-muted mb-8",
                    {translate_fmt(locale(), "game_rules.subtitle", &[("count", total_stores.to_string())])}
                }

                if loading() {
                    p { class: "text-sm text-muted", {translate(locale(), "common.loading")} }
                } else {
                    div { class: "grid min-w-0 grid-cols-1 items-start gap-6 sm:grid-cols-2",
                        section { class: "min-w-0 rounded-2xl border border-gray-200 bg-white p-6 shadow-sm",
                            h2 { class: "text-lg font-extrabold text-dark mb-4",
                                {translate(locale(), "game_rules.store.title")}
                            }
                            p { class: "text-xs text-muted mb-4",
                                {translate(locale(), "game_rules.store.subtitle")}
                            }

                            label { class: "block text-xs font-bold tracking-wider text-gray-600 mb-1",
                                {translate(locale(), "game_rules.store.black_min")}
                            }
                            input {
                                class: "w-full mb-3 rounded border border-gray-300 px-3 py-2 text-sm",
                                r#type: "number",
                                min: "0",
                                max: "{store_max_limit}",
                                value: "{store_min}",
                                oninput: move |evt| {
                                    if let Ok(v) = evt.value().parse::<u16>() {
                                        let mut next = rules();
                                        next.store_draw.black_balls_min = v.min(store_max_limit);
                                        rules.set(next);
                                    }
                                },
                            }

                            label { class: "block text-xs font-bold tracking-wider text-gray-600 mb-1",
                                {translate(locale(), "game_rules.store.black_current")}
                            }
                            input {
                                class: "w-full mb-3 rounded border border-gray-300 px-3 py-2 text-sm",
                                r#type: "number",
                                min: "0",
                                max: "{store_max_limit}",
                                value: "{store_current}",
                                oninput: move |evt| {
                                    if let Ok(v) = evt.value().parse::<u16>() {
                                        let mut next = rules();
                                        next.store_draw.black_balls_current = v.min(store_max_limit);
                                        rules.set(next);
                                    }
                                },
                            }

                            label { class: "block text-xs font-bold tracking-wider text-gray-600 mb-1",
                                {translate(locale(), "game_rules.store.black_max")}
                            }
                            input {
                                class: "w-full mb-3 rounded border border-gray-300 px-3 py-2 text-sm",
                                r#type: "number",
                                min: "0",
                                max: "{store_max_limit}",
                                value: "{store_max}",
                                oninput: move |evt| {
                                    if let Ok(v) = evt.value().parse::<u16>() {
                                        let mut next = rules();
                                        next.store_draw.black_balls_max = v.min(store_max_limit);
                                        rules.set(next);
                                    }
                                },
                            }

                            label { class: "block text-xs font-bold tracking-wider text-gray-600 mb-1",
                                {translate(locale(), "game_rules.mix_seconds")}
                            }
                            input {
                                class: "w-full mb-3",
                                r#type: "range",
                                min: "3",
                                max: "10",
                                value: "{current.store_draw.mix_seconds}",
                                oninput: move |evt| {
                                    if let Ok(v) = evt.value().parse::<u8>() {
                                        let mut next = rules();
                                        next.store_draw.mix_seconds = v;
                                        rules.set(next);
                                    }
                                },
                            }
                            p { class: "text-xs text-muted mb-4", "{current.store_draw.mix_seconds}s" }

                            label { class: "block text-xs font-bold tracking-wider text-gray-600 mb-1",
                                {translate(locale(), "game_rules.entropy")}
                            }
                            input {
                                class: "w-full mb-3",
                                r#type: "range",
                                min: "0",
                                max: "100",
                                value: "{current.store_draw.entropy_percent}",
                                oninput: move |evt| {
                                    if let Ok(v) = evt.value().parse::<u8>() {
                                        let mut next = rules();
                                        next.store_draw.entropy_percent = v;
                                        rules.set(next);
                                    }
                                },
                            }
                            p { class: "text-xs text-muted", "{current.store_draw.entropy_percent}%" }

                            div { class: "mt-6 border-t border-gray-200 pt-6",
                                h2 { class: "text-lg font-extrabold text-dark mb-2",
                                    {translate(locale(), "game_rules.voucher.title")}
                                }
                                label { class: "block text-xs font-bold tracking-wider text-gray-600 mb-1",
                                    {translate(locale(), "game_rules.voucher.validity_days")}
                                }
                                input {
                                    class: "w-full max-w-sm rounded border border-gray-300 px-3 py-2 text-sm",
                                    r#type: "number",
                                    min: "1",
                                    max: "365",
                                    value: "{current.voucher.validity_days}",
                                    oninput: move |evt| {
                                        if let Ok(v) = evt.value().parse::<u16>() {
                                            let mut next = rules();
                                            next.voucher.validity_days = v.clamp(1, 365);
                                            rules.set(next);
                                        }
                                    },
                                }
                            }
                        }

                        section { class: "min-w-0 rounded-2xl border border-gray-200 bg-white p-6 shadow-sm",
                            h2 { class: "text-lg font-extrabold text-dark mb-4",
                                {translate(locale(), "game_rules.discount.title")}
                            }
                            p { class: "text-xs text-muted mb-4",
                                {translate(locale(), "game_rules.discount.subtitle")}
                            }

                            div { class: "mb-3 grid grid-cols-2 gap-4 text-center",
                                p { class: "text-xs font-bold tracking-wider text-gray-600",
                                    {translate(locale(), "game_rules.discount.amount_column")}
                                }
                                p { class: "text-xs font-bold tracking-wider text-gray-600",
                                    {translate(locale(), "game_rules.discount.balls_column")}
                                }
                            }

                            div { class: "mb-4 space-y-2",
                                for (idx, range) in current.discount_draw.ranges.iter().enumerate() {
                                    div { class: "grid grid-cols-2 gap-4 items-center",
                                        div { class: "flex min-w-0 flex-wrap items-center gap-2",
                                            input {
                                                class: "w-20 shrink-0 rounded border border-gray-300 px-2 py-1 text-sm",
                                                r#type: "number",
                                                min: "1",
                                                max: "100",
                                                value: "{range.discount_percent}",
                                                oninput: move |evt| {
                                                    if let Ok(v) = evt.value().parse::<u32>() {
                                                        let mut next = rules();
                                                        if let Some(item) = next.discount_draw.ranges.get_mut(idx) {
                                                            item.discount_percent = v.clamp(1, 100);
                                                        }
                                                        rules.set(next);
                                                    }
                                                },
                                            }
                                            div { class: "flex items-center gap-2 text-xs text-gray-700 whitespace-nowrap",
                                                label { class: "inline-flex items-center gap-1",
                                                    input {
                                                        r#type: "radio",
                                                        name: "discount-unit-{idx}",
                                                        checked: discount_units_percent().get(idx).copied().unwrap_or(true),
                                                        onchange: move |_| {
                                                            let mut next_units = discount_units_percent();
                                                            if next_units.len() <= idx {
                                                                next_units.resize(idx + 1, true);
                                                            }
                                                            next_units[idx] = true;
                                                            discount_units_percent.set(next_units);
                                                        },
                                                    }
                                                    span { "%" }
                                                }
                                                label { class: "inline-flex items-center gap-1",
                                                    input {
                                                        r#type: "radio",
                                                        name: "discount-unit-{idx}",
                                                        checked: !discount_units_percent().get(idx).copied().unwrap_or(true),
                                                        onchange: move |_| {
                                                            let mut next_units = discount_units_percent();
                                                            if next_units.len() <= idx {
                                                                next_units.resize(idx + 1, true);
                                                            }
                                                            next_units[idx] = false;
                                                            discount_units_percent.set(next_units);
                                                        },
                                                    }
                                                    span { "CHF" }
                                                }
                                            }
                                        }
                                        div { class: "flex min-w-0 items-center justify-between gap-2",
                                            div { class: "flex items-center gap-2",
                                                input {
                                                    class: "w-20 rounded border border-gray-300 px-2 py-1 text-sm",
                                                    r#type: "number",
                                                    min: "1",
                                                    max: "200",
                                                    value: "{range.balls_weight}",
                                                    oninput: move |evt| {
                                                        if let Ok(v) = evt.value().parse::<u16>() {
                                                            let mut next = rules();
                                                            if let Some(item) = next.discount_draw.ranges.get_mut(idx) {
                                                                item.balls_weight = v.clamp(1, 200);
                                                            }
                                                            rules.set(next);
                                                        }
                                                    },
                                                }
                                                span { class: "text-xs text-gray-700 whitespace-nowrap",
                                                    {translate(locale(), "game_rules.discount.ball_unit")}
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "inline-flex h-8 w-8 shrink-0 items-center justify-center rounded border border-gray-300 bg-white text-base font-bold leading-none text-gray-800 shadow-sm hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-40",
                                                title: translate(locale(), "game_rules.discount.remove_range"),
                                                disabled: current.discount_draw.ranges.len() <= 1,
                                                onclick: move |_| {
                                                    if rules().discount_draw.ranges.len() <= 1 {
                                                        return;
                                                    }
                                                    let mut next = rules();
                                                    if idx < next.discount_draw.ranges.len() {
                                                        next.discount_draw.ranges.remove(idx);
                                                    }
                                                    rules.set(next);
                                                    let mut units = discount_units_percent();
                                                    if idx < units.len() {
                                                        units.remove(idx);
                                                    }
                                                    discount_units_percent.set(units);
                                                },
                                                "−"
                                            }
                                        }
                                    }
                                }
                                div { class: "flex justify-end pt-1",
                                    button {
                                        r#type: "button",
                                        class: "inline-flex h-8 w-8 shrink-0 items-center justify-center rounded border border-gray-300 bg-white text-base font-bold leading-none text-gray-800 shadow-sm hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-40",
                                        title: translate(locale(), "game_rules.discount.add_range"),
                                        disabled: current.discount_draw.ranges.len() >= MAX_DISCOUNT_RANGES,
                                        onclick: move |_| {
                                            if rules().discount_draw.ranges.len() >= MAX_DISCOUNT_RANGES {
                                                return;
                                            }
                                            let mut next = rules();
                                            next.discount_draw.ranges.push(DiscountRangeRule {
                                                discount_percent: 10,
                                                balls_weight: 1,
                                            });
                                            rules.set(next);
                                            let mut units = discount_units_percent();
                                            units.push(true);
                                            discount_units_percent.set(units);
                                        },
                                        "+"
                                    }
                                }
                            }

                            div { class: "mb-4 space-y-3 border-t border-gray-200 pt-4",
                                div { class: "flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between",
                                    label { class: "text-xs font-bold tracking-wider text-gray-600",
                                        {translate(locale(), "game_rules.discount.colored_balls")}
                                    }
                                    div { class: "flex items-center gap-2",
                                        input {
                                            class: "w-24 rounded border border-gray-300 bg-gray-50 px-2 py-1 text-sm text-gray-900",
                                            r#type: "text",
                                            readonly: true,
                                            value: "{discount_colored_balls_sum}",
                                        }
                                        span { class: "text-xs text-gray-700 whitespace-nowrap",
                                            {translate(locale(), "game_rules.discount.ball_unit")}
                                        }
                                    }
                                }
                                div { class: "flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between",
                                    label { class: "text-xs font-bold tracking-wider text-gray-600",
                                        {translate(locale(), "game_rules.discount.black_balls")}
                                    }
                                    div { class: "flex items-center gap-2",
                                        input {
                                            class: "w-24 rounded border border-gray-300 px-2 py-1 text-sm",
                                            r#type: "number",
                                            min: "0",
                                            max: "120",
                                            value: "{current.discount_draw.black_balls}",
                                            oninput: move |evt| {
                                                if let Ok(v) = evt.value().parse::<u16>() {
                                                    let mut next = rules();
                                                    next.discount_draw.black_balls = v.min(120);
                                                    rules.set(next);
                                                }
                                            },
                                        }
                                        span { class: "text-xs text-gray-700 whitespace-nowrap",
                                            {translate(locale(), "game_rules.discount.ball_unit")}
                                        }
                                    }
                                }
                                div { class: "flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between",
                                    label { class: "text-xs font-bold tracking-wider text-gray-600",
                                        {translate(locale(), "game_rules.discount.sum_total_balls")}
                                    }
                                    div { class: "flex items-center gap-2",
                                        input {
                                            class: "w-24 rounded border border-gray-300 bg-gray-50 px-2 py-1 text-sm text-gray-900",
                                            r#type: "text",
                                            readonly: true,
                                            value: "{discount_total_balls}",
                                        }
                                        span { class: "text-xs text-gray-700 whitespace-nowrap",
                                            {translate(locale(), "game_rules.discount.ball_unit")}
                                        }
                                    }
                                }
                            }

                            label { class: "block text-xs font-bold tracking-wider text-gray-600 mb-1",
                                {translate(locale(), "game_rules.mix_seconds")}
                            }
                            input {
                                class: "w-full mb-3",
                                r#type: "range",
                                min: "3",
                                max: "10",
                                value: "{current.discount_draw.mix_seconds}",
                                oninput: move |evt| {
                                    if let Ok(v) = evt.value().parse::<u8>() {
                                        let mut next = rules();
                                        next.discount_draw.mix_seconds = v;
                                        rules.set(next);
                                    }
                                },
                            }
                            p { class: "text-xs text-muted mb-4", "{current.discount_draw.mix_seconds}s" }

                            label { class: "block text-xs font-bold tracking-wider text-gray-600 mb-1",
                                {translate(locale(), "game_rules.entropy")}
                            }
                            input {
                                class: "w-full mb-3",
                                r#type: "range",
                                min: "0",
                                max: "100",
                                value: "{current.discount_draw.entropy_percent}",
                                oninput: move |evt| {
                                    if let Ok(v) = evt.value().parse::<u8>() {
                                        let mut next = rules();
                                        next.discount_draw.entropy_percent = v;
                                        rules.set(next);
                                    }
                                },
                            }
                            p { class: "text-xs text-muted", "{current.discount_draw.entropy_percent}%" }
                        }
                    }

                    div { class: "mt-6 flex items-center gap-3",
                        button {
                            class: "px-5 py-2 rounded bg-dark text-white text-sm font-bold disabled:bg-gray-300",
                            disabled: saving(),
                            onclick: move |_| {
                                let payload: GameRules = rules();
                                let loc = locale();
                                saving.set(true);
                                status.set(String::new());
                                spawn(async move {
                                    match update_game_rules(payload).await {
                                        Ok(saved) => {
                                            rules.set(saved);
                                            status.set(translate(loc, "game_rules.status.saved"));
                                        }
                                        Err(_) => status.set(translate(loc, "game_rules.status.save_error")),
                                    }
                                    saving.set(false);
                                });
                            },
                            {translate(locale(), "game_rules.save")}
                        }
                        if !status().is_empty() {
                            p { class: "text-sm text-muted", "{status()}" }
                        }
                    }
                }
            }
            Footer {}
        }
    }
}
