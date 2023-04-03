use crate::position::Position;

pub trait PossibleMoves {
    /// Recibe la posicion de la pieza y devuelve un vector con todas las posibles posiciones a las
    /// que la pieza puede moverse.
    fn possible_moves(&self, position: &Position) -> Vec<Position>;
}

// MOVIMIENTOS DE DISTANCIA 1

/// Recibe la referencia a un vector mutable de structs Position, un numero que representa la fila
/// en la que se encuentra la fila de la pieza,y otro numero que representa la columna en
/// la que se encuentra la pieza y le agrega al vector las posiciones que se encuentran a distancia uno
/// y en diagonal hacia abajo de la pieza, si son posiciones validas del tablero.
pub fn get_one_distance_downwards_diagonal_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    if piece_row + 1 < 9 {
        if piece_column - 1 > 0 {
            possible_moves.push(Position {
                row: piece_row + 1,
                column: piece_column - 1,
            })
        }
        if piece_column + 1 < 9 {
            possible_moves.push(Position {
                row: piece_row + 1,
                column: piece_column + 1,
            })
        }
    }
}

/// Recibe la referencia a un vector mutable de structs Position, un numero que representa la fila
/// en la que se encuentra la fila de la pieza,y otro numero que representa la columna en
/// la que se encuentra la pieza y le agrega al vector las posiciones que se encuentran a distancia uno
/// y en diagonal hacia arriba de la pieza, si son posiciones validas del tablero.
pub fn get_one_distance_upwards_diagonal_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    if piece_row - 1 > 0 {
        if piece_column - 1 > 0 {
            possible_moves.push(Position {
                row: piece_row - 1,
                column: piece_column - 1,
            })
        }
        if piece_column + 1 < 9 {
            possible_moves.push(Position {
                row: piece_row - 1,
                column: piece_column + 1,
            })
        }
    }
}

/// Recibe la referencia a un vector mutable de structs Position, un numero que representa la fila
/// en la que se encuentra la fila de la pieza, y otro numero que representa la columna en
/// la que se encuentra la pieza y le agrega al vector las posiciones que se encuentran a distancia uno
/// tanto vertical como horizonatalmente a esa pieza, si son posiciones validas del tablero.
pub fn get_one_distance_straight_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    if piece_row + 1 < 9 {
        possible_moves.push(Position {
            row: piece_row + 1,
            column: piece_column,
        });
    }
    if piece_row - 1 > 0 {
        possible_moves.push(Position {
            row: piece_row - 1,
            column: piece_column,
        });
    }
    if piece_column + 1 < 9 {
        possible_moves.push(Position {
            row: piece_row,
            column: piece_column + 1,
        });
    }
    if piece_column - 1 > 0 {
        possible_moves.push(Position {
            row: piece_row,
            column: piece_column - 1,
        });
    }
}

// MOVIMIENTOS EN L

/// Recibe la referencia a un vector mutable de structs Position, un numero que representa la fila
/// en la que se encuentra la fila de la pieza,y otro numero que representa la columna en
/// la que se encuentra la pieza y le agrega al vector las posiciones que se encuentran a distancia dos
/// verticalmente y uno horizontalmente a esa pieza, si son posiciones validas del tablero.
pub fn get_vertical_l_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    if piece_row as i32 - 2 > 0 {
        if piece_column + 1 < 9 {
            possible_moves.push(Position {
                row: piece_row - 2,
                column: piece_column + 1,
            });
        }
        if piece_column - 1 > 0 {
            possible_moves.push(Position {
                row: piece_row - 2,
                column: piece_column - 1,
            });
        }
    }
    if piece_row + 2 < 9 {
        if piece_column + 1 < 9 {
            possible_moves.push(Position {
                row: piece_row + 2,
                column: piece_column + 1,
            });
        }
        if piece_column - 1 > 0 {
            possible_moves.push(Position {
                row: piece_row + 2,
                column: piece_column - 1,
            });
        }
    }
}

/// Recibe la referencia a un vector mutable de structs Position, un numero que representa la fila
/// en la que se encuentra la fila de la pieza,y otro numero que representa la columna en
/// la que se encuentra la pieza y le agrega al vector las posiciones que se encuentran a distancia dos
/// horizontalmente y uno verticalmente a esa pieza, si son posiciones validas del tablero.
pub fn get_horizontal_l_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    if piece_column as i32 - 2 > 0 {
        if piece_row + 1 < 9 {
            possible_moves.push(Position {
                row: piece_row + 1,
                column: piece_column - 2,
            });
        }
        if piece_row - 1 > 0 {
            possible_moves.push(Position {
                row: piece_row - 1,
                column: piece_column - 2,
            });
        }
    }
    if piece_column + 2 < 9 {
        if piece_row + 1 < 9 {
            possible_moves.push(Position {
                row: piece_row + 1,
                column: piece_column + 2,
            });
        }
        if piece_row - 1 > 0 {
            possible_moves.push(Position {
                row: piece_row - 1,
                column: piece_column + 2,
            });
        }
    }
}

// MOVIMIENTOS DE DISTANCIA LIBRE

/// Recibe la referencia a un vector mutable de structs Position, un numero que representa la fila
/// en la que se encuentra la fila de la pieza,y otro numero que representa la columna en
/// la que se encuentra la pieza y le agrega al vector las posiciones que se encuentran en la misma
/// fila y las que se encuentran en la misma columna (estan sobre la misma linea vertical/horizontal que la pieza).
pub fn get_straight_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    for i in 1..=8 {
        if i != piece_column {
            possible_moves.push(Position {
                row: piece_row,
                column: i,
            });
        }
        if i != piece_row {
            possible_moves.push(Position {
                row: i,
                column: piece_column,
            })
        }
    }
}

/// Recibe la referencia a un vector mutable de structs Position, un numero que representa la fila
/// en la que se encuentra la fila de la pieza,y otro numero que representa la columna en
/// la que se encuentra la pieza y llama a dos funciones que se encargan de agregarle al vector
/// las posiciones que estan en la misma diagonal principal (de izquierda a derecha) y la misma
/// diagonal secundaria (de derecha a izquierda).
pub fn get_diagonal_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    get_principal_diagonal_moves(possible_moves, piece_row, piece_column);
    get_secondary_diagonal_moves(possible_moves, piece_row, piece_column);
}

/// Recibe la referencia a un vector mutable de structs Position, un numero que representa la fila
/// en la que se encuentra la fila de la pieza,y otro numero que representa la columna en
/// la que se encuentra la pieza y le agrega al vector las posiciones que se encuentran en la misma
/// diagonal principal que la pieza.
fn get_principal_diagonal_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    let mut i = piece_row;
    let mut j = piece_column;
    // me voy al comienzo de la diagonal principal
    while i > 1 && j > 1 {
        i -= 1;
        j -= 1;
    }
    // avanzo hasta el final de la diagonal
    while i <= 8 && j <= 8 {
        if i != piece_row && j != piece_column {
            possible_moves.push(Position { row: i, column: j });
        }
        i += 1;
        j += 1;
    }
}

/// Recibe la referencia a un vector mutable de structs Position, un numero que representa la fila
/// en la que se encuentra la fila de la pieza,y otro numero que representa la columna en
/// la que se encuentra la pieza y le agrega al vector las posiciones que se encuentran en la misma
/// diagonal secundaria que la pieza.
fn get_secondary_diagonal_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    let mut i = piece_row;
    let mut j = piece_column;
    // me voy al inicio de la diagonal (hacia la izq superior)
    while i > 1 && j < 8 {
        i -= 1;
        j += 1;
    }
    while i <= 8 && j > 0 {
        // voy bajando por la diagonal
        if i != piece_row && j != piece_column {
            possible_moves.push(Position { row: i, column: j });
        }
        i += 1;
        j -= 1;
    }
}

// ACLARACION: Las implementaciones del trait PossibleMoves de cada pieza lo unico que hacen es llamar a estas
// funciones de movimientos segun le corresponda a cada pieza y por lo tanto ya son testeadas en cada
// archivo de cada pieza, por lo que no es necesario volver testear lo mismo
