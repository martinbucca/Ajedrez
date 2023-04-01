use crate::{
    alfil::Alfil, caballo::Caballo, dama::Dama, peon_blanco::PeonBlanco, peon_negro::PeonNegro,
    rey::Rey, torre::Torre,
};
#[derive(Debug)]
pub enum Piece {
    Alfil(Alfil),
    Torre(Torre),
    Caballo(Caballo),
    Rey(Rey),
    Dama(Dama),
    PeonBlanco(PeonBlanco),
    PeonNegro(PeonNegro),
    None,
}
