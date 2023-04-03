use crate::{
    movimientos::{get_one_distance_upwards_diagonal_moves, PossibleMoves},
    position::Position,
};
#[derive(Debug, PartialEq)]
/// Representa la pieza Peon Blanco del ajedrez.
pub struct PeonBlanco;
/// Recibe una pieza PeonBlanco (struct PeonBlanco) y su posicion (&struct Position) y devuelve un vector de
/// struct Position donde cada elemento es una posicion dentro del tablero a la que se puede mover el PeonBlanco.
/// Por disposicion del tablero los PeonesBlancos siempre capturan hacia arriba  y solo pueden capturar piezas
/// que se encuentren a una casilla de distancia en direccion diagonal (hacia adelante). Siempre el vector que devuelve
/// tendra como maximo 2 Posiciones.
impl PossibleMoves for PeonBlanco {
    fn possible_moves(&self, position: &Position) -> Vec<Position> {
        let mut possible_moves = vec![];
        // el peon blanco solo tiene dos posibles movimientos para capturar y solo hacia arriba
        get_one_distance_upwards_diagonal_moves(&mut possible_moves, position.row, position.column);
        possible_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn peon_blanco_se_mueve_correctamente() {
        let peon = PeonBlanco;
        let posicion_peon = Position { row: 7, column: 4 };
        let posibles_movimientos = peon.possible_moves(&posicion_peon);
        let movimientos_esperados = vec![
            Position { row: 6, column: 5 },
            Position { row: 6, column: 3 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento));
        }
    }
    #[test]
    fn peon_blanco_se_mueve_correctamente_estando_en_borde_izquierdo() {
        let peon = PeonBlanco;
        let posicion_peon = Position { row: 7, column: 1 };
        let posibles_movimientos = peon.possible_moves(&posicion_peon);
        let movimientos_esperados = vec![Position { row: 6, column: 2 }];
        assert_eq!(posibles_movimientos, movimientos_esperados);
    }
    #[test]
    fn peon_blanco_se_mueve_correctamente_estando_en_borde_derecho() {
        let peon = PeonBlanco;
        let posicion_peon = Position { row: 7, column: 8 };
        let posibles_movimientos = peon.possible_moves(&posicion_peon);
        let movimientos_esperados = vec![Position { row: 6, column: 7 }];
        assert_eq!(posibles_movimientos, movimientos_esperados);
    }
    #[test]
    fn peon_blanco_no_tiene_movimientos_estando_en_extremo_opuesto() {
        let peon = PeonBlanco;
        let posicion_peon = Position { row: 1, column: 4 };
        let posibles_movimientos = peon.possible_moves(&posicion_peon);
        let movimientos_esperados = vec![];
        assert_eq!(posibles_movimientos, movimientos_esperados);
    }
}
