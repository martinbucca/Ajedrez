use crate::{
    movimientos::{get_diagonal_moves, get_straight_moves, PossibleMoves},
    position::Position,
};
#[derive(Debug)]
pub struct Dama;
impl PossibleMoves for Dama {
    fn possible_moves(&self, position: &Position) -> Vec<Position> {
        let mut possible_moves = vec![];
        get_straight_moves(&mut possible_moves, position.row, position.column);
        get_diagonal_moves(&mut possible_moves, position.row, position.column);
        possible_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dama_se_mueve_correctemente_estando_en_el_centro() {
        let dama = Dama;
        let posicion_dama = Position { row: 4, column: 4 };
        let posibles_movimientos = dama.possible_moves(&posicion_dama);
        let movimientos_esperados = vec![
            Position { row: 1, column: 1 },
            Position { row: 2, column: 2 },
            Position { row: 3, column: 3 },
            Position { row: 5, column: 5 },
            Position { row: 6, column: 6 },
            Position { row: 7, column: 7 },
            Position { row: 8, column: 8 },
            Position { row: 1, column: 7 },
            Position { row: 2, column: 6 },
            Position { row: 3, column: 5 },
            Position { row: 5, column: 3 },
            Position { row: 6, column: 2 },
            Position { row: 7, column: 1 },
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
    #[test]
    fn dama_se_mueve_correctemente_estando_en_esquina() {
        let dama = Dama;
        let posicion_dama = Position { row: 8, column: 8 };
        let posibles_movimientos = dama.possible_moves(&posicion_dama);
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
            Position { row: 1, column: 1 },
            Position { row: 2, column: 2 },
            Position { row: 3, column: 3 },
            Position { row: 5, column: 5 },
            Position { row: 6, column: 6 },
            Position { row: 7, column: 7 },
            Position { row: 4, column: 4 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn dama_se_mueve_correctemente_estando_en_borde_inferior() {
        let dama = Dama;
        let posicion_dama = Position { row: 8, column: 4 };
        let posibles_movimientos = dama.possible_moves(&posicion_dama);
        let movimientos_esperados = vec![
            Position { row: 7, column: 5 },
            Position { row: 6, column: 6 },
            Position { row: 5, column: 7 },
            Position { row: 4, column: 8 },
            Position { row: 7, column: 3 },
            Position { row: 6, column: 2 },
            Position { row: 5, column: 1 },
            Position { row: 7, column: 4 },
            Position { row: 6, column: 4 },
            Position { row: 5, column: 4 },
            Position { row: 4, column: 4 },
            Position { row: 3, column: 4 },
            Position { row: 2, column: 4 },
            Position { row: 1, column: 4 },
            Position { row: 8, column: 1 },
            Position { row: 8, column: 2 },
            Position { row: 8, column: 3 },
            Position { row: 8, column: 5 },
            Position { row: 8, column: 6 },
            Position { row: 8, column: 7 },
            Position { row: 8, column: 8 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn dama_se_mueve_correctemente_estando_en_borde_superior() {
        let dama = Dama;
        let posicion_dama = Position { row: 1, column: 4 };
        let posibles_movimientos = dama.possible_moves(&posicion_dama);
        let movimientos_esperados = vec![
            Position { row: 2, column: 3 },
            Position { row: 3, column: 2 },
            Position { row: 4, column: 1 },
            Position { row: 2, column: 5 },
            Position { row: 3, column: 6 },
            Position { row: 4, column: 7 },
            Position { row: 5, column: 8 },
            Position { row: 7, column: 4 },
            Position { row: 6, column: 4 },
            Position { row: 5, column: 4 },
            Position { row: 4, column: 4 },
            Position { row: 3, column: 4 },
            Position { row: 2, column: 4 },
            Position { row: 8, column: 4 },
            Position { row: 1, column: 1 },
            Position { row: 1, column: 2 },
            Position { row: 1, column: 3 },
            Position { row: 1, column: 5 },
            Position { row: 1, column: 6 },
            Position { row: 1, column: 7 },
            Position { row: 1, column: 8 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn dama_se_mueve_correctemente_estando_en_borde_derecho() {
        let dama = Dama;
        let posicion_dama = Position { row: 4, column: 8 };
        let posibles_movimientos = dama.possible_moves(&posicion_dama);
        let movimientos_esperados = vec![
            Position { row: 3, column: 7 },
            Position { row: 2, column: 6 },
            Position { row: 1, column: 5 },
            Position { row: 5, column: 7 },
            Position { row: 6, column: 6 },
            Position { row: 7, column: 5 },
            Position { row: 8, column: 4 },
            Position { row: 1, column: 8 },
            Position { row: 2, column: 8 },
            Position { row: 3, column: 8 },
            Position { row: 5, column: 8 },
            Position { row: 6, column: 8 },
            Position { row: 7, column: 8 },
            Position { row: 8, column: 8 },
            Position { row: 4, column: 1 },
            Position { row: 4, column: 2 },
            Position { row: 4, column: 3 },
            Position { row: 4, column: 4 },
            Position { row: 4, column: 5 },
            Position { row: 4, column: 6 },
            Position { row: 4, column: 7 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn dama_se_mueve_correctemente_estando_en_borde_izquierdo() {
        let dama = Dama;
        let posicion_dama = Position { row: 4, column: 1 };
        let posibles_movimientos = dama.possible_moves(&posicion_dama);
        let movimientos_esperados = vec![
            Position { row: 3, column: 2 },
            Position { row: 2, column: 3 },
            Position { row: 1, column: 4 },
            Position { row: 5, column: 2 },
            Position { row: 6, column: 3 },
            Position { row: 7, column: 4 },
            Position { row: 8, column: 5 },
            Position { row: 4, column: 8 },
            Position { row: 4, column: 2 },
            Position { row: 4, column: 3 },
            Position { row: 4, column: 4 },
            Position { row: 4, column: 5 },
            Position { row: 4, column: 6 },
            Position { row: 4, column: 7 },
            Position { row: 1, column: 1 },
            Position { row: 2, column: 1 },
            Position { row: 3, column: 1 },
            Position { row: 5, column: 1 },
            Position { row: 6, column: 1 },
            Position { row: 7, column: 1 },
            Position { row: 8, column: 1 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
}
