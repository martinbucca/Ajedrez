#[derive(Debug, PartialEq)]
/// Representa una posicion en el tablero de ajedrez. Donde el campo "row" representa la fila y "column" la columna.
/// La fila 1 es la de mas arriba y 8 la fila de abajo del tablero.
/// La columna 1 es la de mas a la izquierda y la 8 la columna de mas a la derecha del tablero.
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn se_crea_una_posicion_correctamente() {
        let posicion = Position { row: 1, column: 1 };
        assert_eq!(posicion.row, 1);
        assert_eq!(posicion.column, 1)
    }
    #[test]
    fn se_comparan_dos_posiciones_iguales_correctamente() {
        let posicion1 = Position { row: 1, column: 1 };
        let posicion2 = Position { row: 1, column: 1 };
        assert_eq!(posicion1, posicion2)
    }
    #[test]
    fn se_comparan_dos_posiciones_que_no_son_iguales_correctamente() {
        let posicion1 = Position { row: 1, column: 1 };
        let posicion2 = Position { row: 8, column: 8 };
        assert_ne!(posicion1, posicion2)
    }
}
