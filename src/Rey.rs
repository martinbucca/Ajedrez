use crate::{
    movimientos::{
        get_one_distance_downwards_diagonal_moves, get_one_distance_straight_moves,
        get_one_distance_upwards_diagonal_moves, PossibleMoves,
    },
    position::Position,
};
#[derive(Debug, PartialEq)]
/// Representa la pieza Rey del ajedrez.
pub struct Rey;
/// Recibe una pieza Rey (struct Rey) y su posicion (&struct Position) y devuelve un vector de
/// struct Position donde cada elemento es una posicion dentro del tablero a la que se puede mover el Rey.
/// El Rey puede moverse en cualquier dirección (vertical, horizontal y diagonal), avanzando siempre una casilla.
impl PossibleMoves for Rey {
    fn possible_moves(&self, position: &Position) -> Vec<Position> {
        let mut possible_moves = vec![];
        get_one_distance_downwards_diagonal_moves(
            &mut possible_moves,
            position.row,
            position.column,
        );
        get_one_distance_upwards_diagonal_moves(&mut possible_moves, position.row, position.column);
        get_one_distance_straight_moves(&mut possible_moves, position.row, position.column);
        possible_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rey_se_mueve_correctamente_estando_en_el_medio() {
        let rey = Rey;
        let posicion_rey = Position { row: 4, column: 4 };
        let posibles_movimientos = rey.possible_moves(&posicion_rey);
        let movimientos_esperados = vec![
            Position { row: 3, column: 3 },
            Position { row: 4, column: 3 },
            Position { row: 5, column: 3 },
            Position { row: 3, column: 4 },
            Position { row: 5, column: 4 },
            Position { row: 3, column: 5 },
            Position { row: 4, column: 5 },
            Position { row: 5, column: 5 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn rey_se_mueve_correctamente_estando_en_el_borde_derecho() {
        let rey = Rey;
        let posicion_rey = Position { row: 5, column: 8 };
        let posibles_movimientos = rey.possible_moves(&posicion_rey);
        let movimientos_esperados = vec![
            Position { row: 4, column: 7 },
            Position { row: 5, column: 7 },
            Position { row: 6, column: 7 },
            Position { row: 4, column: 8 },
            Position { row: 6, column: 8 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn rey_se_mueve_correctamente_estando_en_el_borde_izquierdo() {
        let rey = Rey;
        let posicion_rey = Position { row: 5, column: 1 };
        let posibles_movimientos = rey.possible_moves(&posicion_rey);
        let movimientos_esperados = vec![
            Position { row: 4, column: 2 },
            Position { row: 5, column: 2 },
            Position { row: 6, column: 2 },
            Position { row: 4, column: 1 },
            Position { row: 6, column: 1 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn rey_se_mueve_correctamente_estando_en_esquina() {
        let rey = Rey;
        let posicion_rey = Position { row: 8, column: 8 };
        let posibles_movimientos = rey.possible_moves(&posicion_rey);
        let movimientos_esperados = vec![
            Position { row: 7, column: 7 },
            Position { row: 7, column: 8 },
            Position { row: 8, column: 7 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
}
