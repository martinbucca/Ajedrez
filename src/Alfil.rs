use crate::{
    movimientos::{get_diagonal_moves, PossibleMoves},
    position::Position,
};
#[derive(Debug)]

/// Representa la pieza alfil del ajedrez
pub struct Alfil;
/// Recibe una pieza alfil (struct Alfil) y su posicion (&struct Position) y devuelve un vector de
/// struct Position donde cada elemento es una posicion a la que se puede mover el alfil
/// El alfil solo puede moverse en direcciones diagonales, avanzando tantas casillas como desee.
impl PossibleMoves for Alfil {
    fn possible_moves(&self, position: &Position) -> Vec<Position> {
        let mut possible_moves = vec![];
        get_diagonal_moves(&mut possible_moves, position.row, position.column);
        possible_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alfil_se_mueve_correctamente_estando_en_el_centro() {
        let alfil = Alfil;
        let posicion_alfil = Position { row: 4, column: 4 };
        let posibles_movimientos = alfil.possible_moves(&posicion_alfil);
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
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn alfil_se_mueve_correctamente_estando_en_esquina_inferior_derecha() {
        let alfil = Alfil;
        let posicion_alfil = Position { row: 8, column: 8 };
        let posibles_movimientos = alfil.possible_moves(&posicion_alfil);
        let movimientos_esperados = vec![
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
    fn alfil_se_mueve_correctamente_estando_en_esquina_inferior_izquierda() {
        let alfil = Alfil;
        let posicion_alfil = Position { row: 8, column: 1 };
        let posibles_movimientos = alfil.possible_moves(&posicion_alfil);
        let movimientos_esperados = vec![
            Position { row: 7, column: 2 },
            Position { row: 6, column: 3 },
            Position { row: 5, column: 4 },
            Position { row: 4, column: 5 },
            Position { row: 3, column: 6 },
            Position { row: 2, column: 7 },
            Position { row: 1, column: 8 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn alfil_se_mueve_correctamente_estando_en_esquina_superior_izquierda() {
        let alfil = Alfil;
        let posicion_alfil = Position { row: 1, column: 1 };
        let posibles_movimientos = alfil.possible_moves(&posicion_alfil);
        let movimientos_esperados = vec![
            Position { row: 2, column: 2 },
            Position { row: 3, column: 3 },
            Position { row: 4, column: 4 },
            Position { row: 5, column: 5 },
            Position { row: 6, column: 6 },
            Position { row: 7, column: 7 },
            Position { row: 8, column: 8 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn alfil_se_mueve_correctamente_estando_en_esquina_superior_derecha() {
        let alfil = Alfil;
        let posicion_alfil = Position { row: 1, column: 8 };
        let posibles_movimientos = alfil.possible_moves(&posicion_alfil);
        let movimientos_esperados = vec![
            Position { row: 2, column: 7 },
            Position { row: 3, column: 6 },
            Position { row: 4, column: 5 },
            Position { row: 5, column: 4 },
            Position { row: 6, column: 3 },
            Position { row: 7, column: 2 },
            Position { row: 8, column: 1 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn alfil_se_mueve_correctamente_estando_en_borde_derecho() {
        let alfil = Alfil;
        let posicion_alfil = Position { row: 4, column: 8 };
        let posibles_movimientos = alfil.possible_moves(&posicion_alfil);
        let movimientos_esperados = vec![
            Position { row: 3, column: 7 },
            Position { row: 2, column: 6 },
            Position { row: 1, column: 5 },
            Position { row: 5, column: 7 },
            Position { row: 6, column: 6 },
            Position { row: 7, column: 5 },
            Position { row: 8, column: 4 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn alfil_se_mueve_correctamente_estando_en_borde_izquierdo() {
        let alfil = Alfil;
        let posicion_alfil = Position { row: 4, column: 1 };
        let posibles_movimientos = alfil.possible_moves(&posicion_alfil);
        let movimientos_esperados = vec![
            Position { row: 3, column: 2 },
            Position { row: 2, column: 3 },
            Position { row: 1, column: 4 },
            Position { row: 5, column: 2 },
            Position { row: 6, column: 3 },
            Position { row: 7, column: 4 },
            Position { row: 8, column: 5 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn alfil_se_mueve_correctamente_estando_en_borde_superior() {
        let alfil = Alfil;
        let posicion_alfil = Position { row: 1, column: 4 };
        let posibles_movimientos = alfil.possible_moves(&posicion_alfil);
        let movimientos_esperados = vec![
            Position { row: 2, column: 3 },
            Position { row: 3, column: 2 },
            Position { row: 4, column: 1 },
            Position { row: 2, column: 5 },
            Position { row: 3, column: 6 },
            Position { row: 4, column: 7 },
            Position { row: 5, column: 8 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn alfil_se_mueve_correctamente_estando_en_borde_inferior() {
        let alfil = Alfil;
        let posicion_alfil = Position { row: 8, column: 4 };
        let posibles_movimientos = alfil.possible_moves(&posicion_alfil);
        let movimientos_esperados = vec![
            Position { row: 7, column: 5 },
            Position { row: 6, column: 6 },
            Position { row: 5, column: 7 },
            Position { row: 4, column: 8 },
            Position { row: 7, column: 3 },
            Position { row: 6, column: 2 },
            Position { row: 5, column: 1 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
}
