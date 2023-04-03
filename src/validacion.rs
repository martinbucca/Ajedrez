use std::io::Error;
/// Recibe una referencia a un vector de vectores de caracteres, que representa el tablero
/// y devuelve un result que contiene Error personalizado con el mensaje en caso de que el tablero no sea valido
/// o nada en caso de que el tablero sea valido
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

/// Reciibe la cantidad de piezas blancas y negras que hay y si alguna es igual a 0
/// devuelve un Error personalizado con un mensaje en un Result, en caso contrario
/// devuelve un Ok().
fn validate_one_piece_per_color(black_pieces: i32, white_pieces: i32) -> Result<(), Error> {
    if black_pieces == 0 || white_pieces == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "ERROR: Debe haber una pieza negra y una pieza blanca en el tablero!",
        ));
    }
    Ok(())
}

/// Recibe un vector que contiene vectores de caracteres que representan el tablero
/// y devuelve un Error personalizado en caso de que el tablero no tenga 8 filas y 8 columnas,
/// en caso contrario devuelve Ok().
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
                "ERROR: El tablero no es valido. Debe tener 8 columnas!",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_table_devuelve_error_en_una_tabla_con_cantidad_de_filas_incorrectas() {
        let table = vec![
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
        ];
        let validacion = validate_table(&table);
        assert!(validacion.is_err());
        assert_eq!(
            validacion.unwrap_err().to_string(),
            "ERROR: El tablero no es valido. Debe tener 8 filas!"
        )
    }
    #[test]
    fn validate_table_devuelve_error_en_una_tabla_con_cantidad_de_columnas_incorrectas() {
        let table = vec![
            vec!['_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
        ];
        let validacion = validate_table(&table);
        assert!(validacion.is_err());
        assert_eq!(
            validacion.unwrap_err().to_string(),
            "ERROR: El tablero no es valido. Debe tener 8 columnas!"
        )
    }
    #[test]
    fn validate_table_devuelve_error_en_una_tabla_sin_piezas() {
        let table = vec![
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
        ];
        let validacion = validate_table(&table);
        assert!(validacion.is_err());
        assert_eq!(
            validacion.unwrap_err().to_string(),
            "ERROR: Debe haber una pieza negra y una pieza blanca en el tablero!"
        )
    }
    #[test]
    fn validate_table_devuelve_error_en_una_tabla_con_mas_de_una_pieza_negra() {
        let table = vec![
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', 'T', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', 'P', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
        ];
        let validacion = validate_table(&table);
        assert!(validacion.is_err());
        assert_eq!(
            validacion.unwrap_err().to_string(),
            "ERROR: No puede haber mas de una pieza negra en el tablero!"
        )
    }
    #[test]
    fn validate_table_devuelve_error_en_una_tabla_con_mas_de_una_pieza_blanca() {
        let table = vec![
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', 't', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', 'p', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
        ];
        let validacion = validate_table(&table);
        assert!(validacion.is_err());
        assert_eq!(
            validacion.unwrap_err().to_string(),
            "ERROR: No puede haber mas de una pieza blanca en el tablero!"
        )
    }
    #[test]
    fn validate_table_devuelve_error_en_una_tabla_con_caracter_o_pieza_invalido_() {
        let table = vec![
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', 'T', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', 'w', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_', '_'],
        ];
        let validacion = validate_table(&table);
        assert!(validacion.is_err());
        assert_eq!(
            validacion.unwrap_err().to_string(),
            "Error: Pieza invalida o caracter no aceptado en el tablero!"
        )
    }
    #[test]
    fn validate_table_no_devuelve_error_en_una_tabla_valida() {
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
        let validacion = validate_table(&table);
        assert_eq!(validacion.unwrap(), ())
    }
}
