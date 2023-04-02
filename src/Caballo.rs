use crate::{
    movimientos::{get_horizontal_l_moves, get_vertical_l_moves, PossibleMoves},
    position::Position,
};
#[derive(Debug)]
pub struct Caballo;
impl PossibleMoves for Caballo {
    fn possible_moves(&self, position: &Position) -> Vec<Position> {
        let mut possible_moves: Vec<Position> = vec![];
        get_horizontal_l_moves(&mut possible_moves, position.row, position.column);
        get_vertical_l_moves(&mut possible_moves, position.row, position.column);
        possible_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn caballo_se_mueve_correctamente_estando_en_una_posicion_a_mas_de_dos_de_distancia_del_borde()
    {
        let caballo = Caballo;
        let posicion_caballo = Position { row: 5, column: 4 };
        let posibles_movimientos = caballo.possible_moves(&posicion_caballo);
        let movimientos_esperados = vec![
            Position { row: 3, column: 3 },
            Position { row: 3, column: 5 },
            Position { row: 4, column: 2 },
            Position { row: 4, column: 6 },
            Position { row: 6, column: 2 },
            Position { row: 6, column: 6 },
            Position { row: 7, column: 3 },
            Position { row: 7, column: 5 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn caballo_se_mueve_correctamente_estando_en_borde() {
        let caballo = Caballo;
        let posicion_caballo = Position { row: 5, column: 8 };
        let posibles_movimientos = caballo.possible_moves(&posicion_caballo);
        let movimientos_esperados = vec![
            Position { row: 4, column: 6 },
            Position { row: 6, column: 6 },
            Position { row: 3, column: 7 },
            Position { row: 7, column: 7 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn caballo_se_mueve_correctamente_estando_en_una_posicion_a_menos_de_dos_del_borde() {
        let caballo = Caballo;
        let posicion_caballo = Position { row: 2, column: 4 };
        let posibles_movimientos = caballo.possible_moves(&posicion_caballo);
        let movimientos_esperados = vec![
            Position { row: 1, column: 2 },
            Position { row: 1, column: 6 },
            Position { row: 3, column: 2 },
            Position { row: 3, column: 6 },
            Position { row: 4, column: 3 },
            Position { row: 4, column: 5 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
    #[test]
    fn caballo_se_mueve_correctamente_estando_en_esquina() {
        let caballo = Caballo;
        let posicion_caballo = Position { row: 1, column: 1 };
        let posibles_movimientos = caballo.possible_moves(&posicion_caballo);
        let movimientos_esperados = vec![
            Position { row: 2, column: 3 },
            Position { row: 3, column: 2 },
        ];
        for movimiento in movimientos_esperados {
            assert!(posibles_movimientos.contains(&movimiento))
        }
    }
}
