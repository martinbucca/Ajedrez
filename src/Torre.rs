use crate::{
    movimientos::{get_straight_moves, PossibleMoves},
    position::Position,
};
#[derive(Debug)]
pub struct Torre;
impl PossibleMoves for Torre {
    fn possible_moves(&self, position: &Position) -> Vec<Position> {
        let mut possible_moves = vec![];
        get_straight_moves(&mut possible_moves, position.row, position.column);
        possible_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn torre_se_mueve_correctamente_estando_en_esquina() {
        let torre = Torre;
        let posicion_torre = Position { row: 8, column: 8 };
        let posibles_movimientos = torre.possible_moves(&posicion_torre);
        let movimientos_esperados = vec![
            Position { row: 8, column: 1 },
            Position { row: 8, column: 2 },
            Position { row: 8, column: 3 },
            Position { row: 8, column: 4 },
            Position { row: 8, column: 5 },
            Position { row: 8, column: 6 },
            Position { row: 8, column: 7 },
            Position { row: 1, column: 8 },
            Position { row: 2, column: 8 },
            Position { row: 3, column: 8 },
            Position { row: 4, column: 8 },
            Position { row: 5, column: 8 },
            Position { row: 6, column: 8 },
            Position { row: 7, column: 8 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn torre_se_mueve_correctamente_estando_en_centro() {
        let torre = Torre;
        let posicion_torre = Position { row: 4, column: 4 };
        let posibles_movimientos = torre.possible_moves(&posicion_torre);
        let movimientos_esperados = vec![
            Position { row: 4, column: 1 },
            Position { row: 4, column: 2 },
            Position { row: 4, column: 3 },
            Position { row: 4, column: 5 },
            Position { row: 4, column: 6 },
            Position { row: 4, column: 7 },
            Position { row: 4, column: 8 },
            Position { row: 1, column: 4 },
            Position { row: 2, column: 4 },
            Position { row: 3, column: 4 },
            Position { row: 5, column: 4 },
            Position { row: 6, column: 4 },
            Position { row: 7, column: 4 },
            Position { row: 8, column: 4 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
}
