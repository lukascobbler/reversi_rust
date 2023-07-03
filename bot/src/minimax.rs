use super::table_state::Table;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

fn min(v1: f64, v2: f64) -> f64 {
    if v1 < v2 {
        return v1;
    }
    v2
}

fn max(v1: f64, v2: f64) -> f64 {
    if v1 > v2 {
        return v1;
    }
    v2
}

lazy_static! {
    static ref PRECOMPUTED_HEURISTICS: Mutex<HashMap<String, f64>> = {
        Mutex::new(HashMap::new())
    };
}

fn get_dynamic_heuristic(table: &Table) -> f64 {
    let mut map = PRECOMPUTED_HEURISTICS.lock().unwrap();
    return match map.get(&*table.to_string()) {
        None => {
            let heuristics_value = table.heuristic_value();
            map.insert(table.to_string(), heuristics_value);
            heuristics_value
        }
        Some(t) => *t
    }
}

pub fn min_score(table: &Table, depth: u32, alpha: f64, mut beta: f64, max_time: u128) -> Option<f64> {
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    if since_the_epoch.as_millis() > max_time {
        return None;
    }

    if table.is_terminal(table.player) {
        return Some(table.goal_value());
    }
    if depth <= 0 {
        return Some(get_dynamic_heuristic(table))
    }

    for next_table in table.next_tables(table.player) {
        let current_score = match max_score(&next_table, depth - 1, alpha, beta, max_time) {
            None => return None,
            Some(t) => t
        };
        beta = min(beta, current_score);
        if alpha >= beta {
            return Some(alpha);
        }
    }

    Some(beta)
}

pub fn max_score(table: &Table, depth: u32, mut alpha: f64, beta: f64, max_time: u128) -> Option<f64>  {
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    if since_the_epoch.as_millis() > max_time {
        return None;
    }

    if table.is_terminal(table.bot) {
        return Some(table.goal_value());
    }
    if depth <= 0 {
        return Some(get_dynamic_heuristic(table))
    }

    for next_table in table.next_tables(table.bot) {
        let current_score = match min_score(&next_table, depth - 1, alpha, beta, max_time) {
            None => return None,
            Some(t) => t
        };
        alpha = max(alpha, current_score);
        if alpha >= beta {
            return Some(beta);
        }
    }

    Some(alpha)
}