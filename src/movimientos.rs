use crate::position::Position;
pub trait PossibleMoves {
    fn possible_moves(&self, position: &Position) -> Vec<Position>;
}

// MOVIMIENTOS DE DISTANCIA 1

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

pub fn get_diagonal_moves(
    possible_moves: &mut Vec<Position>,
    piece_row: usize,
    piece_column: usize,
) {
    get_principal_diagonal_moves(possible_moves, piece_row, piece_column);
    get_secondary_diagonal_moves(possible_moves, piece_row, piece_column);
}

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
        if i != piece_row && j != piece_row {
            possible_moves.push(Position { row: i, column: j });
        }
        i += 1;
        j += 1;
    }
}

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
