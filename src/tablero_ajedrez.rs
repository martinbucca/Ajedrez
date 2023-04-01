use crate::{logica::Piece, position::Position};
#[derive(Debug)]
pub struct Chessboard {
    pub black: Piece,
    pub white: Piece,
    pub white_position: Position,
    pub black_position: Position,
}
