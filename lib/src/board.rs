use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub enum CellState {
    Empty,
    Unknown,
    Bomb,
    Danger(u8),
}

pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Board {
    pub states: Vec<Vec<CellState>>,
    pub bomb_positions: Vec<Position>,
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
}

impl Board {
    pub fn new(options: &BoardOptions) -> Self {
        Self {
            states: vec![vec![CellState::Unknown; options.num_cols]; options.num_rows],
            bomb_positions: if options.bomb_probability == 0.0 {
                vec![]
            } else {
                Self::generate_random_bomb_positions(options)
            },
        }
    }

    fn generate_random_bomb_positions(options: &BoardOptions) -> Vec<Position> {
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
            .collect::<Vec<Position>>()
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
            bomb_positions: vec![],
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
}
