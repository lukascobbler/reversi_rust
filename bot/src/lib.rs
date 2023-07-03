pub mod minimax;
pub mod table_state;

const MAX_DEPTH: u32 = 64;

use std::collections::{HashMap};
use std::time::{SystemTime, UNIX_EPOCH};
use pyo3::prelude::*;
use crate::table_state::Table;

fn move_priority(map: &HashMap<(u32, u32), f64>) -> Vec<(u32, u32)> {
    let mut vector: Vec<(&(u32, u32), &f64)> = map.iter().collect();
    vector.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    vector.iter().map(|pair| *pair.0).collect()
}

/// Takes a Python object of a board matrix and returns the best board matrix
#[pyfunction]
pub fn computer_turn(matrix: Vec<Vec<i32>>, player: i32, bot: i32) -> (Vec<Vec<i32>>, u32, f64) {
    let mut best_found_table = Table::new();
    let mut best_found_alpha = f64::NEG_INFINITY;
    let mut depth_best_table: Option<Table> = None;

    let mut depth = 3;

    let operating_table = Table::from_arguments(matrix, player, bot);

    let max_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + 3 * 1000;

    let mut move_ordering_table = HashMap::new();
    for key in operating_table.all_legal_moves(operating_table.bot) {
        move_ordering_table.insert(key, 0.0);
    }

    while depth < MAX_DEPTH && SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() < max_time {
        let mut alpha = f64::NEG_INFINITY;

        for (row, column) in move_priority(&move_ordering_table) {
            let next_table = operating_table.next_table(row, column, operating_table.bot);

            if let None = depth_best_table {
                depth_best_table = Some(next_table.clone());
            }

            let current_score = match minimax::min_score(&next_table, depth - 1, alpha, f64::INFINITY, max_time) {
                None => return (best_found_table.get_state_cloned(), depth, best_found_alpha),
                Some(t) => t
            };

            move_ordering_table.insert((row, column), current_score);

            if current_score > alpha {
                alpha = current_score;
                depth_best_table = Some(next_table.clone());
            }
        }

        best_found_table = depth_best_table.clone().unwrap();
        best_found_alpha = alpha;

        if alpha >= f64::INFINITY {
            return (best_found_table.get_state_cloned(), depth, best_found_alpha);
        }

        depth += 1;
    }

    (best_found_table.get_state_cloned(), depth, best_found_alpha)
}

#[pymodule]
fn bot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(computer_turn, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_move_priorty() {
        let mut test_hm = HashMap::new();
        test_hm.insert((2, 4), 50.0);
        test_hm.insert((3, 5), 40.0);
        test_hm.insert((4, 2), 70.0);
        test_hm.insert((5, 3), -110.0);

        assert_eq!(vec![(4, 2), (2, 4), (3, 5), (5, 3)], move_priority(&test_hm));
    }
}