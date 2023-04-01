use crate::{
    movimientos::{get_one_distance_downwards_diagonal_moves, PossibleMoves},
    position::Position,
};
#[derive(Debug)]
pub struct PeonNegro;
impl PossibleMoves for PeonNegro {
    fn possible_moves(&self, position: &Position) -> Vec<Position> {
        let mut possible_moves = vec![];
        // el peon negro solo tiene dos posibles movimientos para capturar y solo hacia abajo
        get_one_distance_downwards_diagonal_moves(
            &mut possible_moves,
            position.row,
            position.column,
        );
        possible_moves
    }
}
