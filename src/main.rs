use ajedrez::logica::{
    create_chessboard, create_table, generate_result, open_file, read_arguments, validate_table,
};
use std::env;
use std::process::exit;

fn main() {
    let file_name = match read_arguments(env::args().collect()) {
        None => {
            println!("ERROR: cantidad de parametros invalidos");
            exit(-1)
        }
        Some(name) => name,
    };
    let file = match open_file(file_name) {
        Err(e) => {
            println!("ERROR: {}", e);
            exit(-1)
        }
        Ok(file) => file,
    };
    let table: Vec<Vec<char>> = match create_table(file) {
        Err(e) => {
            println!("ERROR: {}", e);
            exit(-1)
        }
        Ok(table) => table,
    };
    if let Err(error) = validate_table(&table) {
        println!("{}", error);
        exit(-1)
    }
    let chessboard = create_chessboard(&table);
    generate_result(chessboard);
}
