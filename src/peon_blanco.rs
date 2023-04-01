use crate::{
    movimientos::{get_one_distance_upwards_diagonal_moves, PossibleMoves},
    position::Position,
};
#[derive(Debug)]
pub struct PeonBlanco;
impl PossibleMoves for PeonBlanco {
    fn possible_moves(&self, position: &Position) -> Vec<Position> {
        let mut possible_moves = vec![];
        // el peon blanco solo tiene dos posibles movimientos para capturar y solo hacia arriba
        get_one_distance_upwards_diagonal_moves(&mut possible_moves, position.row, position.column);
        possible_moves
    }
}
