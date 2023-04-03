use crate::movimientos::PossibleMoves;
use crate::{
    alfil::Alfil, caballo::Caballo, dama::Dama, peon_blanco::PeonBlanco, peon_negro::PeonNegro,
    rey::Rey, torre::Torre,
};
use crate::{position::Position, tablero_ajedrez::Chessboard};

#[derive(Debug, PartialEq)]
/// Tiene enumeradas todas las posibles piezas que un tablero puede tener y dentro de cada pieza
/// una tupla simple que tiene un struct de la pieza correspondiente.
/// Tambien tiene la opcion de None para que pueda ser inicializada una pieza que todavia no sé cual es
/// pero que sé que existe.
pub enum Piece {
    Alfil(Alfil),
    Torre(Torre),
    Caballo(Caballo),
    Rey(Rey),
    Dama(Dama),
    PeonBlanco(PeonBlanco),
    PeonNegro(PeonNegro),
    None,
}

//GENERACION DEL RESULTADO

/// Recibe un struct Chessboard que tiene la informacion justa y necesaria para que se pueda
/// generar el resultado del programa y guarda el resultado de si la pieza negra puede capturar
/// y si la pieza blanca puede capturar y llama a una funcion que decide el resultado.
pub fn generate_result(chessboard: Chessboard) -> String {
    let black_can_capture = can_capture_oponent(
        &chessboard.black,
        &chessboard.black_position,
        &chessboard.white_position,
    );
    let white_can_capture = can_capture_oponent(
        &chessboard.white,
        &chessboard.white_position,
        &chessboard.black_position,
    );
    get_result(black_can_capture, white_can_capture)
}

/// Recibe dos valores true/false que representan si las piezas pueden capturar a la otra o no
/// y devuelve un String resultado siendo "B" si solo capturan las blancas, "N" si solo capturan las negras,
/// "E" si capturan las dos y "P" si no captura ninguna.
fn get_result(black_can_capture: bool, white_can_capture: bool) -> String {
    let result = (black_can_capture, white_can_capture);
    match result {
        (false, true) => "B".to_string(),
        (true, false) => "N".to_string(),
        (true, true) => "E".to_string(),
        (false, false) => "P".to_string(),
    }
}

// CREACION DEL TABLERO DE AJEDREZ VALIDO

/// Recibe un tablero valido y devuelve una tupla donde en la primer
/// posicion esta la pieza negra y en la segunda su posicion.
fn get_black_piece_and_position(table: &&[Vec<char>]) -> (Piece, Position) {
    // inicilaizo las variable "black" y "black_position" con una pieza None y una posicion en (0,0) para poder
    // guardarles un valor en el for.
    // ya fue validada por lo que estoy seguro que las variables "black" y "black_position" van a
    // ser asignadas a su valor correcto
    let mut black: Piece = Piece::None;
    let mut black_position = Position { row: 0, column: 0 };
    for (number_of_row, row) in table.iter().enumerate() {
        for (number_of_column, char) in row.iter().enumerate() {
            let position = Position {
                row: number_of_row + 1,
                column: number_of_column + 1,
            };
            match char {
                '_' => (),
                'T' => {
                    black = Piece::Torre(Torre);
                    black_position = position
                }
                'A' => {
                    black = Piece::Alfil(Alfil);
                    black_position = position
                }
                'R' => {
                    black = Piece::Rey(Rey);
                    black_position = position
                }
                'D' => {
                    black = Piece::Dama(Dama);
                    black_position = position
                }
                'C' => {
                    black = Piece::Caballo(Caballo);
                    black_position = position
                }
                'P' => {
                    black = Piece::PeonNegro(PeonNegro);
                    black_position = position
                }

                _ => (),
            }
        }
    }
    (black, black_position)
}

/// Recibe un tablero valido y devuelve una tupla donde en la primer
/// posicion esta la pieza blanca y en la segunda su posicion.
fn get_white_piece_and_position(table: &&[Vec<char>]) -> (Piece, Position) {
    // inicilaizo las variable "white" y "white_position" con una pieza None y una posicion en (0,0) para poder
    // guardarles un valor en el for.
    // ya fue validada por lo que estoy seguro que las variables "white" y "white_position" van a
    // ser asignadas a su valor correcto
    let mut white: Piece = Piece::None;
    let mut white_position = Position { row: 0, column: 0 };
    for (number_of_row, row) in table.iter().enumerate() {
        for (number_of_column, char) in row.iter().enumerate() {
            let position = Position {
                row: number_of_row + 1,
                column: number_of_column + 1,
            };
            match char {
                '_' => (),
                't' => {
                    white = Piece::Torre(Torre);
                    white_position = position
                }
                'a' => {
                    white = Piece::Alfil(Alfil);
                    white_position = position
                }
                'r' => {
                    white = Piece::Rey(Rey);
                    white_position = position
                }
                'd' => {
                    white = Piece::Dama(Dama);
                    white_position = position
                }
                'c' => {
                    white = Piece::Caballo(Caballo);
                    white_position = position
                }
                'p' => {
                    white = Piece::PeonBlanco(PeonBlanco);
                    white_position = position
                }

                _ => (),
            }
        }
    }
    (white, white_position)
}

/// Recibe una vector de vectores de Char que representa un tablero de ajedrez valido
/// y devuelve un struct Chessboard con los datos necesarios para generar el resultado
/// del programa.
pub fn create_chessboard(table: &[Vec<char>]) -> Chessboard {
    let (black, black_position) = get_black_piece_and_position(&table);
    let (white, white_position) = get_white_piece_and_position(&table);
    Chessboard {
        black,
        white,
        black_position,
        white_position,
    }
}

// CAPTURA

/// Recibe una pieza, su posicion y la posicion de la pieza oponente y devuelve true en caso de que la
/// pieza que recibio pueda capturar a la oponente o false en caso contario.
pub fn can_capture_oponent(
    piece: &Piece,
    piece_position: &Position,
    oponent_position: &Position,
) -> bool {
    match piece {
        Piece::Torre(torre) => can_capture(torre.possible_moves(piece_position), oponent_position),
        Piece::Alfil(alfil) => can_capture(alfil.possible_moves(piece_position), oponent_position),
        Piece::Caballo(caballo) => {
            can_capture(caballo.possible_moves(piece_position), oponent_position)
        }
        Piece::Rey(rey) => can_capture(rey.possible_moves(piece_position), oponent_position),
        Piece::Dama(dama) => can_capture(dama.possible_moves(piece_position), oponent_position),
        Piece::PeonBlanco(peon) => {
            can_capture(peon.possible_moves(piece_position), oponent_position)
        }
        Piece::PeonNegro(peon) => {
            can_capture(peon.possible_moves(piece_position), oponent_position)
        }
        // no podria ser None (esta validado) pero igual hay que contemplar este caso por el warning
        _ => false,
    }
}

/// Recibe un vector de posiciones, que representa todos los posibles movimientos que tiene la pieza,
/// y la posicion de la pieza oponente y devuelve true si alguna de las posiciones del
/// vector es igual a la posicion del oponente o false en caso contrario.
fn can_capture(possible_moves: Vec<Position>, oponent_position: &Position) -> bool {
    for chess_box in possible_moves {
        if chess_box == *oponent_position {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn se_genera_el_resultado_correctamente_de_empate() {
        let tablero = Chessboard {
            black: Piece::Torre(Torre),
            white: Piece::Torre(Torre),
            black_position: Position { row: 8, column: 8 },
            white_position: Position { row: 1, column: 8 },
        };
        assert_eq!("E", generate_result(tablero));
    }
    #[test]
    fn se_genera_el_resultado_correctamente_de_ganan_blancas() {
        let tablero = Chessboard {
            black: Piece::PeonNegro(PeonNegro),
            white: Piece::Torre(Torre),
            black_position: Position { row: 8, column: 8 },
            white_position: Position { row: 1, column: 8 },
        };
        assert_eq!("B", generate_result(tablero));
    }
    #[test]
    fn se_genera_el_resultado_correctamente_de_ganan_negras() {
        let tablero = Chessboard {
            black: Piece::Torre(Torre),
            white: Piece::Alfil(Alfil),
            black_position: Position { row: 1, column: 8 },
            white_position: Position { row: 7, column: 8 },
        };
        assert_eq!("N", generate_result(tablero));
    }
    #[test]
    fn se_genera_el_resultado_correctamente_de_no_gana_ninguna() {
        let tablero = Chessboard {
            black: Piece::Torre(Torre),
            white: Piece::Alfil(Alfil),
            black_position: Position { row: 1, column: 7 },
            white_position: Position { row: 7, column: 8 },
        };
        assert_eq!("P", generate_result(tablero));
    }
    #[test]
    fn get_result_devuelve_la_letra_correcta_si_ambos_capturan() {
        assert_eq!("E", get_result(true, true))
    }
    #[test]
    fn get_result_devuelve_la_letra_correcta_si_blanco_captura() {
        assert_eq!("B", get_result(false, true))
    }
    #[test]
    fn get_result_devuelve_la_letra_correcta_si_negro_captura() {
        assert_eq!("N", get_result(true, false))
    }
    #[test]
    fn get_result_devuelve_la_letra_correcta_si_ninguno_captura() {
        assert_eq!("P", get_result(false, false))
    }
    #[test]
    fn se_crea_correctamente_el_tablero_con_tabla_valida() {
        let table = vec![
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', 'T', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', 'p', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
        ];
        let tablero = create_chessboard(&table);
        assert_eq!(Piece::Torre(Torre), tablero.black);
        assert_eq!(Piece::PeonBlanco(PeonBlanco), tablero.white);
        assert_eq!(Position { row: 3, column: 5 }, tablero.black_position);
        assert_eq!(Position { row: 5, column: 5 }, tablero.white_position);
    }
    #[test]
    fn can_capture_devuelve_resultado_correcto_1() {
        let pieza = Piece::Torre(Torre);
        let posicion_pieza = Position { row: 3, column: 5 };
        let posicion_oponente = Position { row: 5, column: 5 };
        assert_eq!(
            true,
            can_capture_oponent(&pieza, &posicion_pieza, &posicion_oponente)
        )
    }
    #[test]
    fn can_capture_devuelve_resultado_correcto_2() {
        let pieza = Piece::Torre(Torre);
        let posicion_pieza = Position { row: 5, column: 5 };
        let posicion_oponente = Position { row: 3, column: 5 };
        assert_eq!(
            true,
            can_capture_oponent(&pieza, &posicion_pieza, &posicion_oponente)
        )
    }
    #[test]
    fn can_capture_devuelve_resultado_correcto_3() {
        let pieza = Piece::Dama(Dama);
        let posicion_pieza = Position { row: 3, column: 5 };
        let posicion_oponente = Position { row: 6, column: 8 };
        assert_eq!(
            true,
            can_capture_oponent(&pieza, &posicion_pieza, &posicion_oponente)
        )
    }
    #[test]
    fn can_capture_devuelve_resultado_correcto_4() {
        let pieza = Piece::Dama(Dama);
        let posicion_pieza = Position { row: 3, column: 5 };
        let posicion_oponente = Position { row: 4, column: 7 };
        assert_eq!(
            false,
            can_capture_oponent(&pieza, &posicion_pieza, &posicion_oponente)
        )
    }
}
