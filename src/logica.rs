use crate::movimientos::PossibleMoves;
use crate::{
    alfil::Alfil, caballo::Caballo, dama::Dama, peon_blanco::PeonBlanco, peon_negro::PeonNegro,
    rey::Rey, torre::Torre,
};
use crate::{position::Position, tablero_ajedrez::Chessboard};

use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::vec;

#[derive(Debug)]
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
pub fn generate_result(chessboard: Chessboard) {
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
    print_result(black_can_capture, white_can_capture);
}
fn print_result(black_can_capture: bool, white_can_capture: bool) {
    let result = (black_can_capture, white_can_capture);
    match result {
        (false, true) => println!("B"),
        (true, false) => println!("N"),
        (true, true) => println!("E"),
        (false, false) => println!("P"),
    }
}

//ARGUMENTOS, ARCHIVOS Y TABLA
pub fn read_arguments(mut arguments: Vec<String>) -> Option<String> {
    // 2 arguments must be passed!
    if arguments.len() != 2 {
        None
    } else {
        let file_name = arguments.pop()?;
        Some(file_name)
    }
}
pub fn open_file(file_name: String) -> Result<File, Error> {
    let file = File::open(file_name)?;
    Ok(file)
}
fn get_row(content: String) -> Vec<char> {
    let mut row: Vec<char> = vec![];
    for c in content.chars() {
        if c != ' ' {
            row.push(c);
        }
    }
    row
}
pub fn create_table(file: File) -> Result<Vec<Vec<char>>, Error> {
    let mut table: Vec<Vec<char>> = vec![];
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let content = line?;
        let row = get_row(content);
        table.push(row);
    }
    Ok(table)
}

//VALIDACION DE TABLA
pub fn validate_table(table: &Vec<Vec<char>>) -> Result<(), Error> {
    validate_table_dimensions(&table)?;
    let (mut black_pieces, mut white_pieces) = (0, 0);
    for row in table {
        for position in row {
            match position {
                'T' | 'A' | 'R' | 'D' | 'C' | 'P' => {
                    black_pieces += 1;
                    if black_pieces > 1 {
                        return Err(Error::new(
                            std::io::ErrorKind::Other,
                            "ERROR: No puede haber mas de una pieza negra en el tablero!",
                        ));
                    }
                }
                't' | 'a' | 'r' | 'd' | 'c' | 'p' => {
                    white_pieces += 1;
                    if white_pieces > 1 {
                        return Err(Error::new(
                            std::io::ErrorKind::Other,
                            "ERROR: No puede haber mas de una pieza blanca en el tablero!",
                        ));
                    }
                }
                '_' => (),
                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Error: Pieza invalida o caracter no aceptado en el tablero!",
                    ));
                }
            };
        }
    }
    validate_one_piece_per_color(black_pieces, white_pieces)?;
    Ok(())
}
fn validate_one_piece_per_color(black_pieces: i32, white_pieces: i32) -> Result<(), Error> {
    if black_pieces == 0 || white_pieces == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "ERROR: Debe haber una pieza negra y una pieza blanca en el tablero!",
        ));
    }
    Ok(())
}
fn validate_table_dimensions(table: &&Vec<Vec<char>>) -> Result<(), Error> {
    if table.len() != 8 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "ERROR: El tablero no es valido. Debe tener 8 filas!",
        ));
    }
    for row in *table {
        if row.len() != 8 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "ERROR: El tablero no es valido, debe tener 8 columnas!",
            ));
        }
    }
    Ok(())
}

// CREACION DEL TABLERO DE AJEDREZ VALIDO
fn get_black_piece_and_position(table: &&[Vec<char>]) -> (Piece, Position) {
    //inicilaizo las dos variables para poder guardarles un valor en el for
    // ya fue validada por lo que estoy seguro que las variables "black" y "black position" van a
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
fn get_white_piece_and_position(table: &&[Vec<char>]) -> (Piece, Position) {
    //inicilaizo las dos variables para poder guardarles un valor en el for
    // ya fue validada por lo que estoy seguro que las variables "black" y "black position" van a
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
fn can_capture(possible_moves: Vec<Position>, oponent_position: &Position) -> bool {
    for chess_box in possible_moves {
        if chess_box == *oponent_position {
            return true;
        }
    }
    false
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error_al_pasar_cantidad_de_parametros_incorrectos_al_programa() {

    }
}*/
