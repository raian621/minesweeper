use crate::board::{Board, CellState, Position};

pub struct PositionBombProbability {
    pub position: Position,
    pub probability: f64,
}

impl PositionBombProbability {
    pub fn new(position: Position, probability: f64) -> Self {
        Self {
            position,
            probability,
        }
    }
}

pub fn rank_positions(board: &Board) -> Vec<PositionBombProbability> {
    if board
        .states
        .iter()
        .all(|row| row.iter().all(|state| *state == CellState::Unknown))
    {
        let equal_probability = 1.0 / (board.states.len() as f64 * board.states[0].len() as f64);
        return board
            .states
            .iter()
            .enumerate()
            .flat_map(|(row, row_vec)| {
                row_vec.iter().enumerate().map(move |(col, _)| {
                    return PositionBombProbability::new(
                        Position::new(row, col),
                        equal_probability,
                    );
                })
            })
            .collect::<Vec<PositionBombProbability>>();
    }
    panic!("unimplemented")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_fresh_board_ranking() {
        let board = Board {
            states: vec![vec![CellState::Unknown; 3]; 3],
            bomb_positions: HashSet::new(),
        };
        let positions_ranked = rank_positions(&board);
        let expected = 1.0 / 9.0;
        assert!(
            positions_ranked
                .iter()
                .all(|position_ranked| position_ranked.probability == expected)
        );
    }
}
