use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Table {
    state: Vec<Vec<i32>>,
    pub bot: i32,
    pub player: i32,
}

impl Table {
    const _BLACK: i32 = 1;
    const _WHITE: i32 = -1;
    pub const EMPTY: i32 = 0;

    pub fn is_terminal(&self, player: i32) -> bool {
        !(0..8).flat_map(|row| (0..8).map(move |column| (row, column)))
            .any(|(row, column)| self.is_field_legal(row, column, player))
    }

    pub fn goal_value(&self) -> f64 {
        let mut c_num: u32 = 0;
        let mut p_num: u32 = 0;

        for row in &self.state {
            for &player in row {
                if player == self.bot {
                    c_num += 1;
                } else if player == self.player {
                    p_num += 1;
                }
            }
        }

        return match p_num.cmp(&c_num) {
            Ordering::Less => f64::INFINITY,
            Ordering::Equal => 0.0,
            Ordering::Greater => f64::NEG_INFINITY
        };
    }

    pub fn heuristic_value(&self) -> f64 {
        let field_values = [
            [20.0, -3.0, 11.0, 8.0, 8.0, 11.0, -3.0, 20.0],
            [-3.0, -7.0, -4.0, 1.0, 1.0, -4.0, -7.0, -3.0],
            [11.0, -4.0, 2.0, 2.0, 2.0, 2.0, -4.0, 11.0],
            [8.0, 1.0, 2.0, -3.0, -3.0, 2.0, 1.0, 8.0],
            [8.0, 1.0, 2.0, -3.0, -3.0, 2.0, 1.0, 8.0],
            [11.0, -4.0, 2.0, 2.0, 2.0, 2.0, -4.0, 11.0],
            [-3.0, -7.0, -4.0, 1.0, 1.0, -4.0, -7.0, -3.0],
            [20.0, -3.0, 11.0, 8.0, 8.0, 11.0, -3.0, 20.0],
        ];

        let x1: [i32; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
        let y1: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

        let mut h_field_value: f64 = 0.0;
        let mut h_disk_difference_value: f64 = 0.0;
        let mut h_edge_disk_value: f64 = 0.0;
        let h_corner_value: f64;
        let h_corner_closeness_value: f64;
        let mut h_num_legal_moves_value: f64 = 0.0;

        let mut bot_num_disks = 0;
        let mut player_num_disks = 0;
        let mut bot_num_edge_disks = 0;
        let mut player_num_edge_disks = 0;

        for i in 0..8 {
            for j in 0..8 {
                if self.state[i][j] == self.bot {
                    h_field_value += field_values[i][j];
                    bot_num_disks += 1
                } else if self.state[i][j] == self.player {
                    h_field_value -= field_values[i][j];
                    player_num_disks += 1
                }

                if self.state[i][j] != Table::EMPTY {
                    for k in 0..8 {
                        let x = i as i32 + x1[k];
                        let y = j as i32 + y1[k];

                        if Self::in_bounds(x, y) && self.state[x as usize][y as usize] == Table::EMPTY {
                            if self.state[i][j] == self.bot {
                                bot_num_edge_disks += 1
                            } else {
                                player_num_edge_disks += 1;
                            }
                            break;
                        }
                    }
                }
            }
        }

        if bot_num_disks > player_num_disks {
            h_disk_difference_value = (100.0 * bot_num_disks as f64) / (bot_num_disks + player_num_disks) as f64
        } else if player_num_disks > bot_num_disks {
            h_disk_difference_value = -(100.0 * player_num_disks as f64) / (bot_num_disks + player_num_disks) as f64
        }

        if bot_num_edge_disks > player_num_edge_disks {
            h_edge_disk_value = -(100.0 * bot_num_edge_disks as f64) / (
                bot_num_edge_disks + player_num_edge_disks) as f64
        } else if player_num_edge_disks > bot_num_edge_disks {
            h_edge_disk_value = (100.0 * player_num_edge_disks as f64) / (
                bot_num_edge_disks + player_num_edge_disks) as f64
        }

        bot_num_disks = 0;
        player_num_disks = 0;

        bot_num_disks += (self.state[0][0] == self.bot) as i32;
        player_num_disks += (self.state[0][0] == self.player) as i32;

        bot_num_disks += (self.state[0][7] == self.bot) as i32;
        player_num_disks += (self.state[0][7] == self.player) as i32;

        bot_num_disks += (self.state[7][0] == self.bot) as i32;
        player_num_disks += (self.state[7][0] == self.player) as i32;

        bot_num_disks += (self.state[7][7] == self.bot) as i32;
        player_num_disks += (self.state[7][7] == self.player) as i32;

        h_corner_value = 25.0 * (bot_num_disks - player_num_disks) as f64;

        bot_num_disks = 0;
        player_num_disks = 0;
        if self.state[0][0] == Table::EMPTY {
            bot_num_disks += (self.state[0][1] == self.bot) as i32;
            player_num_disks += (self.state[0][1] == self.player) as i32;

            bot_num_disks += (self.state[1][1] == self.bot) as i32;
            player_num_disks += (self.state[1][1] == self.player) as i32;

            bot_num_disks += (self.state[1][0] == self.bot) as i32;
            player_num_disks += (self.state[1][0] == self.player) as i32;
        }

        if self.state[0][7] == Table::EMPTY {
            bot_num_disks += (self.state[0][6] == self.bot) as i32;
            player_num_disks += (self.state[0][6] == self.player) as i32;

            bot_num_disks += (self.state[1][6] == self.bot) as i32;
            player_num_disks += (self.state[1][6] == self.player) as i32;

            bot_num_disks += (self.state[1][7] == self.bot) as i32;
            player_num_disks += (self.state[1][7] == self.player) as i32;
        }

        if self.state[7][0] == Table::EMPTY {
            bot_num_disks += (self.state[7][1] == self.bot) as i32;
            player_num_disks += (self.state[7][1] == self.player) as i32;

            bot_num_disks += (self.state[6][1] == self.bot) as i32;
            player_num_disks += (self.state[6][1] == self.player) as i32;

            bot_num_disks += (self.state[6][0] == self.bot) as i32;
            player_num_disks += (self.state[6][0] == self.player) as i32;
        }

        if self.state[7][7] == Table::EMPTY {
            bot_num_disks += (self.state[6][7] == self.bot) as i32;
            player_num_disks += (self.state[6][7] == self.player) as i32;

            bot_num_disks += (self.state[6][6] == self.bot) as i32;
            player_num_disks += (self.state[6][6] == self.player) as i32;

            bot_num_disks += (self.state[7][6] == self.bot) as i32;
            player_num_disks += (self.state[7][6] == self.player) as i32;
        }

        h_corner_closeness_value = -12.5 * (bot_num_disks - player_num_disks) as f64;

        let bot_num_moves = self.num_legal_moves(self.bot);
        let player_num_moves = self.num_legal_moves(self.player);

        if bot_num_moves > player_num_moves {
            h_num_legal_moves_value = (100.0 * bot_num_moves as f64) / (bot_num_moves + player_num_moves) as f64;
        } else if player_num_moves > bot_num_moves {
            h_num_legal_moves_value = -(100.0 * player_num_moves as f64) / (bot_num_moves + player_num_moves) as f64;
        }

        let h_final_score = (10.0 * h_disk_difference_value) + (801.724 * h_corner_value) + (382.026 * h_corner_closeness_value) +
            (78.922 * h_num_legal_moves_value) + (74.396 * h_edge_disk_value) + (10.0 * h_field_value);

        h_final_score
    }

    pub fn all_legal_moves(&self, player: i32) -> Vec<(u32, u32)> {
        (0..8).flat_map(|row: u32| (0..8).map(move |column: u32| (row, column)))
            .filter(|&(row, column)| self.is_field_legal(row, column, player))
            .collect()
    }

    pub fn is_field_legal(&self, row: u32, column: u32, player: i32) -> bool {
        if self.state[row as usize][column as usize] != Self::EMPTY {
            return false;
        }

        let opposite_player = Self::calculate_opposite_player(player);

        let directions: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];

        for (x_dir, y_dir) in directions {
            let mut x: i32 = row as i32;
            let mut y: i32 = column as i32;
            let mut counter = 0;

            x += x_dir;
            y += y_dir;

            while Self::in_bounds(x, y) && self.state[x as usize][y as usize] == opposite_player {
                counter += 1;
                x += x_dir;
                y += y_dir;
            }

            if Self::in_bounds(x, y) && self.state[x as usize][y as usize] == player && counter > 0 {
                return true;
            }
        }

        false
    }

    pub fn next_table(&self, row: u32, column: u32, player: i32) -> Self {
        let opposite_player = Self::calculate_opposite_player(player);

        let directions: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];

        let mut new_table = self.clone();

        for (x_dir, y_dir) in directions {
            let mut x: i32 = row as i32;
            let mut y: i32 = column as i32;

            x += x_dir;
            y += y_dir;

            let mut flips = vec![];

            while Self::in_bounds(x, y) {
                let player_at_position = self.state[x as usize][y as usize];

                if player_at_position == opposite_player {
                    flips.push((x as usize, y as usize))
                } else if player_at_position == player {
                    for (x_flip, y_flip) in &flips {
                        new_table.state[*x_flip][*y_flip] = player;
                    }
                    if !flips.is_empty() {
                        new_table.state[row as usize][column as usize] = player;
                    }
                    break;
                } else {
                    break;
                }
                x += x_dir;
                y += y_dir;
            }
        }

        new_table
    }

    pub fn next_tables(&self, player: i32) -> Vec<Table> {
        self.all_legal_moves(player).iter()
            .map(|&(row, column)| self.next_table(row, column, player))
            .collect()
    }

    fn in_bounds(row: i32, column: i32) -> bool {
        return match (row, column) {
            (0..=7, 0..=7) => true,
            _ => false
        };
    }

    fn calculate_opposite_player(player: i32) -> i32 {
        player * -1
    }

    fn num_legal_moves(&self, player: i32) -> u32 {
        (0..8).flat_map(|row| (0..8).map(move |column| (row, column)))
            .filter(|&(row, column)| self.is_field_legal(row, column, player))
            .count() as u32
    }

    pub fn new() -> Self {
        let empty_row = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let black_white_row = vec![0, 0, 0, Self::_BLACK, Self::_WHITE, 0, 0, 0];
        let white_black_row = vec![0, 0, 0, Self::_WHITE, Self::_BLACK, 0, 0, 0];

        let mut table: Vec<Vec<i32>> = vec![];

        for _ in 0..3 {
            table.push(empty_row.clone());
        }

        table.push(black_white_row);
        table.push(white_black_row);

        for _ in 0..3 {
            table.push(empty_row.clone());
        }

        Table {
            state: table,
            bot: Self::_WHITE,
            player: Self::_BLACK,
        }
    }

    pub fn from_arguments(state: Vec<Vec<i32>>, player: i32, bot: i32) -> Self {
        Table {
            state,
            bot,
            player,
        }
    }

    pub fn get_state_cloned(&self) -> Vec<Vec<i32>> {
        self.state.clone()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.state)
    }
}