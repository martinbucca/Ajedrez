use crate::logica::create_chessboard;
use crate::logica::generate_result;
use crate::validacion::validate_table;
use ajedrez::{
    lectura_archivo::{create_table, open_file},
    *,
};
use std::process::exit;

/// Recibe el nombre del archivo a leer y el resultado esperado que debe generar el programa
/// es igual a la funcion main()
fn simular_programa_y_verificar_resultado(nombre_archivo: &str, reultado_esperado: String) {
    let file = match open_file(nombre_archivo.to_string()) {
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
    let result = generate_result(chessboard);
    assert_eq!(reultado_esperado, result)
}
#[test]
fn captura_solo_rey_negro() {
    simular_programa_y_verificar_resultado("./negras/rey_negro.txt", "N".to_string())
}
#[test]
fn captura_solo_dama_negra() {
    simular_programa_y_verificar_resultado("./negras/dama_negra.txt", "N".to_string())
}
#[test]
fn captura_solo_alfil_negro() {
    simular_programa_y_verificar_resultado("./negras/alfil_negro.txt", "N".to_string())
}
#[test]
fn captura_solo_caballo_negro() {
    simular_programa_y_verificar_resultado("./negras/caballo_negro.txt", "N".to_string())
}
#[test]
fn captura_solo_torre_negra() {
    simular_programa_y_verificar_resultado("./negras/torre_negra.txt", "N".to_string())
}
#[test]
fn captura_solo_peon_negro() {
    simular_programa_y_verificar_resultado("./negras/peon_negro.txt", "N".to_string())
}
#[test]
fn captura_solo_rey_blanco() {
    simular_programa_y_verificar_resultado("./blancas/rey_blanco.txt", "B".to_string())
}
#[test]
fn captura_solo_dama_blanca() {
    simular_programa_y_verificar_resultado("./blancas/dama_blanca.txt", "B".to_string())
}
#[test]
fn captura_solo_alfil_blanco() {
    simular_programa_y_verificar_resultado("./blancas/alfil_blanco.txt", "B".to_string())
}
#[test]
fn captura_solo_caballo_blanco() {
    simular_programa_y_verificar_resultado("./blancas/caballo_blanco.txt", "B".to_string())
}
#[test]
fn captura_solo_torre_blanco() {
    simular_programa_y_verificar_resultado("./blancas/torre_blanca.txt", "B".to_string())
}
#[test]
fn captura_solo_peon_blanco() {
    simular_programa_y_verificar_resultado("./blancas/peon_blanco.txt", "B".to_string())
}

#[test]
fn capturan_ambos_1() {
    simular_programa_y_verificar_resultado("./ambos/ambos_1.txt", "E".to_string())
}
#[test]
fn capturan_ambos_2() {
    simular_programa_y_verificar_resultado("./ambos/ambos_2.txt", "E".to_string())
}
#[test]
fn capturan_ambos_3() {
    simular_programa_y_verificar_resultado("./ambos/ambos_3.txt", "E".to_string())
}
#[test]
fn capturan_ambos_4() {
    simular_programa_y_verificar_resultado("./ambos/ambos_4.txt", "E".to_string())
}
#[test]
fn ninguno_captura_1() {
    simular_programa_y_verificar_resultado("./ninguno/ninguno_1.txt", "P".to_string())
}
#[test]
fn ninguno_captura_2() {
    simular_programa_y_verificar_resultado("./ninguno/ninguno_2.txt", "P".to_string())
}
#[test]
fn ninguno_captura_3() {
    simular_programa_y_verificar_resultado("./ninguno/ninguno_3.txt", "P".to_string())
}
#[test]
fn ninguno_captura_4() {
    simular_programa_y_verificar_resultado("./ninguno/ninguno_4.txt", "P".to_string())
}


