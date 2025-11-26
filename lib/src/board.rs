use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Empty,
    Unknown,
    Bomb,
    Danger(u8),
}

#[derive(PartialEq, Eq, Hash)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Board {
    pub states: Vec<Vec<CellState>>,
    pub bomb_positions: HashSet<Position>,
}

pub struct BoardOptions {
    pub num_rows: usize,
    pub num_cols: usize,
    pub bomb_probability: f64,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn surrounding(&self, board: &Board) -> Vec<Self> {
        DIRECTIONS
            .iter()
            .filter_map(|(d_row, d_col)| {
                let new_row = d_row + self.row as isize;
                let new_col = d_col + self.col as isize;
                if new_row < 0 && new_col < 0 {
                    return None;
                }
                let new_pos = Position::new(new_row as usize, new_col as usize);
                if board.position_on_board(&new_pos) {
                    Some(new_pos)
                } else {
                    None
                }
            })
            .collect::<Vec<Self>>()
    }
}

impl Board {
    pub fn new(options: &BoardOptions) -> Self {
        Self {
            states: vec![vec![CellState::Unknown; options.num_cols]; options.num_rows],
            bomb_positions: if options.bomb_probability == 0.0 {
                HashSet::new()
            } else {
                Self::generate_random_bomb_positions(options)
            },
        }
    }

    fn generate_random_bomb_positions(options: &BoardOptions) -> HashSet<Position> {
        (0..options.num_cols)
            .flat_map(|row| {
                (0..options.num_cols).filter_map(move |col| {
                    if rand::random::<f64>() < options.bomb_probability {
                        Some(Position::new(row, col))
                    } else {
                        None
                    }
                })
            })
            .collect::<HashSet<Position>>()
    }

    fn position_on_board(&self, pos: &Position) -> bool {
        pos.row < self.states.len() && pos.col < self.states[0].len()
    }

    // Reveals a cell and returns the state that the cell is in. If the cell
    // does not neighbor any bombs, its neighboring cell states are revealed as
    // well. This operation is recursive.
    pub fn reveal_cell(&mut self, pos: &Position) -> CellState {
        let surrounding = pos.surrounding(self);
        if self.bomb_positions.contains(&pos) {
            self.states[pos.row][pos.col] = CellState::Bomb;
            return CellState::Bomb;
        }

        let surrounding_bomb_count = surrounding
            .iter()
            .filter(|neighbor| self.bomb_positions.contains(neighbor))
            .count();
        let state = if surrounding_bomb_count == 0 {
            CellState::Empty
        } else {
            CellState::Danger(surrounding_bomb_count as u8)
        };
        self.states[pos.row][pos.col] = state;
        if state == CellState::Empty {
            surrounding.iter().for_each(|neighbor| {
                if self.states[neighbor.row][neighbor.col] == CellState::Unknown {
                    self.reveal_cell(neighbor);
                }
            });
        }
        state
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.states.iter().for_each(|row| {
            row.iter().for_each(|state| {
                write!(
                    f,
                    "{} ",
                    match state {
                        CellState::Empty => " ".to_string(),
                        CellState::Danger(x) => format!("{x}"),
                        CellState::Bomb => "X".to_string(),
                        CellState::Unknown => "-".to_string(),
                    }
                )
                .unwrap();
            });
            writeln!(f).unwrap();
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CELL_STATE_TABLE: [CellState; 11] = [
        CellState::Empty,
        CellState::Danger(1),
        CellState::Danger(2),
        CellState::Danger(3),
        CellState::Danger(4),
        CellState::Danger(5),
        CellState::Danger(6),
        CellState::Danger(7),
        CellState::Danger(8),
        CellState::Unknown,
        CellState::Bomb,
    ];
    const E: usize = 0;
    const U: usize = 9;
    const B: usize = 10;

    fn to_cell_state_grid(grid: Vec<Vec<usize>>) -> Vec<Vec<CellState>> {
        grid.iter()
            .map(|row| {
                row.iter()
                    .map(|n| CELL_STATE_TABLE[*n as usize])
                    .collect::<Vec<CellState>>()
            })
            .collect::<Vec<Vec<CellState>>>()
    }

    #[test]
    fn test_display() {
        let board = Board {
            states: to_cell_state_grid(vec![
                vec![1, 2, 2, 1, E],
                vec![1, B, U, 1, E],
                vec![1, 2, 2, 1, E],
                vec![E, E, E, E, E],
                vec![E, E, E, E, E],
            ]),
            bomb_positions: HashSet::new(),
        };
        assert_eq!(
            format!("{board}"),
            vec![
                "1 2 2 1   ",
                "1 X - 1   ",
                "1 2 2 1   ",
                "          ",
                "          ",
                ""
            ]
            .join("\n")
        );
    }

    #[test]
    fn test_reveal_recurses() {
        let mut board = Board {
            states: to_cell_state_grid(vec![
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
            ]),
            bomb_positions: HashSet::from_iter(vec![Position::new(2, 2)].into_iter()),
        };
        assert_eq!(board.reveal_cell(&Position::new(0, 0)), CellState::Empty);
        assert_eq!(
            board.states,
            to_cell_state_grid(vec![
                vec![E, E, E, E, E],
                vec![E, 1, 1, 1, E],
                vec![E, 1, U, 1, E],
                vec![E, 1, 1, 1, E],
                vec![E, E, E, E, E],
            ])
        );
    }

    #[test]
    fn test_reveal_bomb() {
        let mut board = Board {
            states: to_cell_state_grid(vec![
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
            ]),
            bomb_positions: HashSet::from_iter(vec![Position::new(2, 2)].into_iter()),
        };
        assert_eq!(board.reveal_cell(&Position::new(2, 2)), CellState::Bomb);
        assert_eq!(
            board.states,
            to_cell_state_grid(vec![
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, B, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
            ])
        );
    }

    #[test]
    fn test_reveal_number() {
        let mut board = Board {
            states: to_cell_state_grid(vec![
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
            ]),
            bomb_positions: HashSet::from_iter(vec![Position::new(2, 2)].into_iter()),
        };
        assert_eq!(
            board.reveal_cell(&Position::new(1, 2)),
            CellState::Danger(1)
        );
        assert_eq!(
            board.states,
            to_cell_state_grid(vec![
                vec![U, U, U, U, U],
                vec![U, U, 1, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
                vec![U, U, U, U, U],
            ])
        );
    }
}
