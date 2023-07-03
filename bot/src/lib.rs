pub mod minimax;
pub mod table_state;

const MAX_DEPTH: u32 = 64;

use std::collections::{HashMap};
use std::time::{SystemTime, UNIX_EPOCH};
use pyo3::prelude::*;
use crate::table_state::Table;

fn move_priority<'a>(map: &HashMap<&'a Table, f64>) -> Vec<&'a Table> {
    let mut vector: Vec<(&&Table, &f64)> = map.iter().collect();
    vector.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    vector.iter().map(|pair| *pair.0).collect()
}

/// Takes a Python object of a board matrix and returns the best board matrix
#[pyfunction]
pub fn computer_turn(matrix: Vec<Vec<i32>>, player: i32, bot: i32) -> (Vec<Vec<i32>>, u32, f64) {
    let mut best_found_table: Option<&Table> = None;
    let mut best_found_alpha = f64::NEG_INFINITY;
    let mut depth_best_table: Option<&Table> = None;

    let mut depth = 3;

    let operating_table = Table::from_arguments(matrix, player, bot);

    let mut move_ordering_table = HashMap::new();
    let all_moves = operating_table.next_tables(operating_table.bot);

    for next_table in all_moves.iter() {
        move_ordering_table.insert(next_table, 0.0);
    }

    let max_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + 3 * 1000;

    while depth < MAX_DEPTH && SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() < max_time {
        let mut alpha = f64::NEG_INFINITY;

        for next_table in move_priority(&move_ordering_table) {
            if let None = depth_best_table {
                depth_best_table = Some(next_table);
            }

            let current_score = match minimax::min_score(next_table, depth - 1, alpha, f64::INFINITY, max_time) {
                None => return (best_found_table.unwrap().get_state_cloned(), depth - 1, best_found_alpha),
                Some(t) => t
            };

            move_ordering_table.insert(next_table, current_score);

            if current_score > alpha {
                alpha = current_score;
                depth_best_table = Some(next_table);
            }
        }

        best_found_table = depth_best_table;
        best_found_alpha = alpha;

        if alpha >= f64::INFINITY {
            return (best_found_table.unwrap().get_state_cloned(), depth, best_found_alpha);
        }

        depth += 1;
    }

    (best_found_table.unwrap().get_state_cloned(), depth, best_found_alpha)
}

#[pymodule]
fn bot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(computer_turn, m)?)?;
    Ok(())
}
