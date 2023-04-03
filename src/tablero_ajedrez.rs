use crate::{logica::Piece, position::Position};
#[derive(Debug)]
/// Contiene la informacion necesaria para la resolucion del programa.
/// Gurada la pieza negra y blanca y sus respectivas posiciones
/// para tenerla accesible y poder decidir cual captura a cual.
pub struct Chessboard {
    pub black: Piece,
    pub white: Piece,
    pub white_position: Position,
    pub black_position: Position,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alfil::Alfil;
    #[test]
    fn se_crea_tablero_correctamente() {
        let pieza_negra = Piece::Alfil(Alfil);
        let pieza_blanca = Piece::Alfil(Alfil);
        let posicion_blanca = Position { row: 4, column: 4 };
        let posicion_negra = Position { row: 4, column: 3 };
        let tablero = Chessboard {
            black: pieza_negra,
            white: pieza_blanca,
            white_position: posicion_blanca,
            black_position: posicion_negra,
        };
        assert_eq!(tablero.black, Piece::Alfil(Alfil));
        assert_eq!(tablero.white, Piece::Alfil(Alfil));
        assert_eq!(tablero.black_position, Position { row: 4, column: 3 });
        assert_eq!(tablero.white_position, Position { row: 4, column: 4 })
    }
}
