use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::vec;

/// Recibe un Vector de strings que contiene los argumentos de programa y devuelve un
/// Option que es None en caso de que la cantidad de argumentos del programa se incorrecta
/// y sino devuelve el nombre del archivo en un Some().
pub fn read_arguments(mut arguments: Vec<String>) -> Option<String> {
    // 2 arguments must be passed!
    if arguments.len() != 2 {
        None
    } else {
        let file_name = arguments.pop()?;
        Some(file_name)
    }
}

/// Recibe un String que representa el nombre del archivo y devuelve
/// un Result que es ERROR en caso de que no se pueda abrir correctamente el archivo
/// o el archivo abierto (de tipo FILE) en un OK() en caso contrario.
pub fn open_file(file_name: String) -> Result<File, Error> {
    let file = File::open(file_name)?;
    Ok(file)
}

/// Recibe un String que representa una fila del tablero, y devuelve un vector de caracteres
/// donde cada elemento es un caracter distinto del espacio (' ').
fn get_row(content: String) -> Vec<char> {
    let mut row: Vec<char> = vec![];
    for c in content.chars() {
        if c != ' ' {
            row.push(c);
        }
    }
    row
}

/// Recibe un archivo abierto y devuelve un Result
/// que devuelve ERROR en caso de que no se pueda leer una linea correctamente
/// o un Ok() con un vector que contiene vectores de caracteres, donde cada vector de caracteres
/// reperesenta una fila del tabler.
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_arguments_devuelve_none_si_no_se_le_pasan_dos_argumentos() {
        let argumentos = vec!["ajedrez.rs".to_string()];
        assert_eq!(None, read_arguments(argumentos))
    }
    #[test]
    fn read_arguments_devuelve_el_archivo_si_no_se_le_pasan_dos_argumentos() {
        let argumentos = vec!["ajedrez.rs".to_string(), "table.txt".to_string()];
        assert_eq!(Some("table.txt".to_string()), read_arguments(argumentos))
    }
    #[test]
    fn open_file_devuelve_ok_con_el_archivo_si_se_pudo_abrir_correctamente() {
        let nombre = "./table.txt".to_string();
        let file = File::open(nombre);
        match file {
            Ok(..) => assert!(file.is_ok()),
            Err(..) => (),
        }
    }
    #[test]
    fn open_file_devuelve_error_si_el_archivo_no_existe() {
        let archivo = "archivo_inexistente.txt".to_string();
        let resultado = open_file(archivo);
        assert!(resultado.is_err())
    }
    #[test]
    fn get_row_elimina_los_espacios_y_devuelve_vector_de_caracteres_correctamente() {
        let linea = "_ _ _ _ _ _ _ T".to_string();
        let fila_esperada = vec!['_', '_', '_', '_', '_', '_', '_', 'T'];
        assert_eq!(fila_esperada, get_row(linea))
    }
    #[test]
    fn create_table_devuelve_una_tabla_con_archivo_valido() {
        let archivo = open_file("./table.txt".to_string());
        let tabla = create_table(archivo.unwrap());
        let tabla_esperada = vec![
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['A', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', 'p', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
        ];
        match tabla {
            Ok(t) => assert_eq!(t, tabla_esperada),
            _ => (),
        }
    }
}
