use std::env;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::process::exit;


struct Position{
    row: usize,
    column: usize,
}
struct Piece{
    position: Position,
    typ: char,
    color: String,
    posible_moves: Vec<Position>,
}

struct Chessboard {
    black: Piece,
    white: Piece,
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
    create_chessboard(&table);
    println!("MI TABLERO ES VALIDO!!!\n {:?}", table);
}


fn create_chessboard(table: &Vec<Vec<char>>) -> Chessboard{
    let black_piece: Piece;
    let white_piece: Piece;
    
    for (number_of_row, row) in table.into_iter().enumerate(){
        for (number_of_column, char) in row.into_iter().enumerate(){
            match char{
                '_' => (),
                'T' | 'A' | 'R' | 'D' | 'C' | 'P' => {
                    black_piece = Piece {
                        position: Position { row: (number_of_row), column: (number_of_column) },
                        typ: *char,
                        color: "black".to_string(),
                        posible_moves: get_posible_moves_of(char, position)
                    }
                },
                't' | 'a' | 'r' | 'd' | 'c' | 'p' => (),
                _ => (),
            }
        }
    }
    Chessboard { black: (Piece { position: (    Position { row: (), column: () }), typ: (), color: (), posible_moves: () }), white: () }
    
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
