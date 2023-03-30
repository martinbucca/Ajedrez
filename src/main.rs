use std::env;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::process::exit;

#[derive(Debug, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

trait PosiblesMovientos {
    fn posibles_movimientos(&self, position: &Position) -> Vec<Position>;
}
#[derive(Debug)]
struct Alfil;
impl PosiblesMovientos for Alfil {
    fn posibles_movimientos(&self, position: &Position) -> Vec<Position> {
        let mut movimientos: Vec<Position> = vec![];
        let mut i = position.row;
        let mut j = position.column;
        while i > 0 && j > 0{
            i -= 1;
            j -= 1;
        }
        while i <= 8 && j <= 8{
            movimientos.push(Position { row: i, column: j});
            i += 1;
            j += 1;
        }
        i = position.row;
        j = position.column;
        while i > 0 && j < 8{
            i -= 1;
            j += 1;
        } 
        while i < 8 && j > 0{
            movimientos.push(Position{row: i, column: j});
            i += 1;
            j -= 1;

        }
        movimientos
    }
}
#[derive(Debug)]
struct Rey;
impl PosiblesMovientos for Rey {
    fn posibles_movimientos(&self, position: &Position) -> Vec<Position> {
        let movimientos: Vec<Position> = vec![
            Position {
                row: position.row + 1,
                column: position.column - 1,
            },
            Position {
                row: position.row + 1,
                column: position.column + 1,
            },
            Position {
                row: position.row - 1,
                column: position.column - 1,
            },
            Position {
                row: position.row - 1,
                column: position.column + 1,
            },
            Position {
                row: position.row,
                column: position.column - 1,
            },
            Position {
                row: position.row,
                column: position.column + 1,
            },
            Position {
                row: position.row + 1,
                column: position.column,
            },
            Position {
                row: position.row - 1,
                column: position.column,
            },
        ];
        movimientos
    }
}
#[derive(Debug)]
struct Dama;
impl PosiblesMovientos for Dama {
    fn posibles_movimientos(&self, position: &Position) -> Vec<Position> {
        let mut movimientos: Vec<Position> = vec![];
        for i in 1..=8 {
            movimientos.push(Position {
                row: position.row,
                column: i,
            });
            movimientos.push(Position {
                row: i,
                column: position.column,
            })
        }
        let mut i = position.row;
        let mut j = position.column;
        while i > 0 && j > 0{
            i -= 1;
            j -= 1;
        }
        while i <= 8 && j <= 8{
            movimientos.push(Position { row: i, column: j});
            i += 1;
            j += 1;
        }
        i = position.row;
        j = position.column;
        while i > 0 && j < 8{
            i -= 1;
            j += 1;
        } 
        while i < 8 && j > 0{
            movimientos.push(Position{row: i, column: j});
            i += 1;
            j -= 1;

        }
        movimientos
    }
}
#[derive(Debug)]
struct Torre;
impl PosiblesMovientos for Torre {
    fn posibles_movimientos(&self, position: &Position) -> Vec<Position> {
        let mut movimientos: Vec<Position> = vec![];
        for i in 1..=8 {
            movimientos.push(Position {
                row: position.row,
                column: i,
            });
            movimientos.push(Position {
                row: i,
                column: position.column,
            })
        }
        movimientos
    }
}
#[derive(Debug)]
struct Caballo;
impl PosiblesMovientos for Caballo {
    fn posibles_movimientos(&self, position: &Position) -> Vec<Position> {
        let movimientos: Vec<Position> = vec![
            Position {
                row: (position.row as isize - 1) as usize,
                column: (position.column as isize + 2) as usize,
            },
            Position {
                row: (position.row as isize - 1) as usize,
                column: (position.column as isize - 2) as usize,
            },
            Position {
                row: (position.row as isize + 1) as usize,
                column: (position.column as isize + 2) as usize,
            },
            Position {
                row: (position.row as isize + 1) as usize,
                column: (position.column as isize - 2) as usize,
            },
            Position {
                row: (position.row as isize - 2) as usize,
                column: (position.column as isize + 1) as usize,
            },
            Position {
                row: (position.row as isize - 2) as usize,
                column: (position.column as isize - 1) as usize,
            },
            Position {
                row: (position.row as isize + 2) as usize,
                column: (position.column as isize + 1) as usize,
            },
            Position {
                row: (position.row as isize + 2) as usize,
                column: (position.column as isize - 1) as usize,
            },
        ];
        movimientos
        // logica para obtener los posibles movimientos del alfil
    }
}
#[derive(Debug)]
struct PeonBlanco;
impl PosiblesMovientos for PeonBlanco {
    fn posibles_movimientos(&self, position: &Position) -> Vec<Position> {
        let movimientos: Vec<Position> = vec![
            Position {
                row: position.row - 1,
                column: position.column + 1,
            },
            Position {
                row: position.row - 1,
                column: position.column - 1,
            },
        ];
        movimientos
    }
}
#[derive(Debug)]
struct PeonNegro;
impl PosiblesMovientos for PeonNegro {
    fn posibles_movimientos(&self, position: &Position) -> Vec<Position> {
        let movimientos: Vec<Position> = vec![
            Position {
                row: position.row + 1,
                column: position.column - 1,
            },
            Position {
                row: position.row + 1,
                column: position.column + 1,
            },
        ];
        movimientos
    }
}
#[derive(Debug)]
enum Piece {
    Alfil(Alfil),
    Torre(Torre),
    Caballo(Caballo),
    Rey(Rey),
    Dama(Dama),
    PeonBlanco(PeonBlanco),
    PeonNegro(PeonNegro),
}

#[derive(Debug)]
struct Chessboard {
    black: Piece,
    white: Piece,
    white_position: Position,
    black_position: Position,
}

fn puede_capturar(piece: &Piece, piece_position: &Position, oponent_position: &Position) -> bool {
    match piece {
        Piece::Torre(torre) => {
            for mov in torre.posibles_movimientos(piece_position) {
                if mov == *oponent_position {
                    return true;
                }
            }
            false
        }
        Piece::Alfil(alfil) => {
            for mov in alfil.posibles_movimientos(piece_position) {
                if mov == *oponent_position {
                    return true;
                }
            }
            false
        }
        Piece::Caballo(caballo) => {
            for mov in caballo.posibles_movimientos(piece_position) {
                if mov == *oponent_position {
                    return true;
                }
            }
            false
        }
        Piece::Rey(rey) => {
            for mov in rey.posibles_movimientos(piece_position) {
                if mov == *oponent_position {
                    return true;
                }
            }
            false
        }
        Piece::Dama(dama) => {
            for mov in dama.posibles_movimientos(piece_position) {
                if mov == *oponent_position {
                    return true;
                }
            }
            false
        }
        Piece::PeonBlanco(peon) => {
            for mov in peon.posibles_movimientos(piece_position) {
                if mov == *oponent_position {
                    return true;
                }
            }
            false
        }
        Piece::PeonNegro(peon) => {
            for mov in peon.posibles_movimientos(piece_position) {
                if mov == *oponent_position {
                    return true;
                }
            }
            false
        }
    }
}

fn main() {
    let file_name = match read_arguments(env::args().collect()) {
        None => {
            println!("ERROR: cantidad de parametros invalidos");
            exit(0)
        }
        Some(name) => name,
    };
    let file = match open_file(file_name) {
        Err(e) => {
            println!("ERROR: {}", e);
            exit(0)
        }
        Ok(file) => file,
    };
    let table: Vec<Vec<char>> = match create_table(file) {
        Err(e) => {
            println!("ERROR: {}", e);
            exit(0);
        }
        Ok(table) => table,
    };
    if let Some(error) = validate_table(&table) {
        println!("{}", error);
        exit(0);
    }
    let chessboard = create_chessboard(&table);
    let black_can_capture = puede_capturar(
        &chessboard.black,
        &chessboard.black_position,
        &chessboard.white_position,
    );
    let white_can_capture = puede_capturar(
        &chessboard.white,
        &chessboard.white_position,
        &chessboard.black_position,
    );
    return_result(black_can_capture, white_can_capture);

}

fn return_result(black_can_capture: bool, white_can_capture: bool) {
    let result = (black_can_capture, white_can_capture);
    match result {
        (false, true) => println!("B"),
        (true, false) => println!("N"),
        (true, true) => println!("E"),
        (false, false) => println!("P"),
    }
}
fn create_chessboard(table: &[Vec<char>]) -> Chessboard {
    let mut black: Piece = Piece::Rey(Rey);
    let mut white: Piece = Piece::Rey(Rey);
    let mut black_position = Position { row: 0, column: 0 };
    let mut white_position = Position { row: 0, column: 0 };
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
    Chessboard {
        black,
        white,
        black_position,
        white_position,
    }
}
fn validate_table(table: &Vec<Vec<char>>) -> Option<String> {
    if table.len() != 8 {
        return Some(
            "ERROR: El tablero no es valido. Debe tener 8 filas y 8 columnas!".to_string(),
        );
    }
    let mut black_pieces = 0;
    let mut white_pieces = 0;
    for row in table {
        if row.len() != 8 {
            return Some(
                "ERROR: El tablero no es valido. Debe tener 8 filas y 8 columnas!".to_string(),
            );
        }
        for position in row {
            match position {
                'T' | 'A' | 'R' | 'D' | 'C' | 'P' => {
                    if black_pieces == 1 {
                        return Some(
                            "ERROR: No puede haber mas de una pieza negra en el tablero!"
                                .to_string(),
                        );
                    }
                    black_pieces += 1
                }
                't' | 'a' | 'r' | 'd' | 'c' | 'p' => {
                    if white_pieces == 1 {
                        return Some(
                            "ERROR: No puede haber mas de una pieza blanca en el tablero!"
                                .to_string(),
                        );
                    }
                    white_pieces += 1
                }
                '_' => (),
                _ => {
                    return Some(
                        "Error: Pieza invalida o caracter no aceptado en el tablero!".to_string(),
                    )
                }
            };
        }
    }
    if black_pieces == 0 || white_pieces == 0 {
        return Some("ERROR: Debe haber una pieza negra y una pieza blanca!".to_string());
    }
    None
}

fn read_arguments(mut arguments: Vec<String>) -> Option<String> {
    // 2 arguments must be passed!
    if arguments.len() != 2 {
        None
    } else {
        let file_name = arguments.pop()?;
        Some(file_name)
    }
}

fn open_file(file_name: String) -> Result<File, Error> {
    let opened_file = File::open(file_name)?;
    Ok(opened_file)
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
fn create_table(file: File) -> Result<Vec<Vec<char>>, Error> {
    let mut table: Vec<Vec<char>> = vec![];
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let content = line?;
        let row = get_row(content);
        table.push(row);
    }
    Ok(table)
}
