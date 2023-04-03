use ajedrez::lectura_archivo::{create_table, open_file, read_arguments};
use ajedrez::logica::{create_chessboard, generate_result};
use ajedrez::validacion::validate_table;
use std::env;
use std::process::exit;

/// Es la funcion principal del porgrama que se encarga de llamar a todas las funciones que contienen la logica del
/// programa y en caso de que alguna parte del programa devuelva ERROR se encarga de mostrarle por pantalla
/// y cortar la ejecucion del programa.
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
    println!("{}", generate_result(chessboard));
}
