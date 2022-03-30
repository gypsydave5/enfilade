use shakmaty;
use shakmaty::{Bitboard, Board, Chess, Position, Role, Setup, Square};

use crate::pin::Pins::{Absolute, AbsoluteAndRelative, Relative};

mod points;
use points::RolePoints;

mod pinning;
pub use pinning::Pin;
pub use pinning::Pinning;

#[derive(Debug, Eq, PartialEq)]
pub enum Pins {
    Absolute,
    Relative,
    AbsoluteAndRelative,
}

pub fn is_pin(position: &Chess, pinned_piece: Square) -> Option<Pins> {
    let abs = is_absolutely_pinned(position, pinned_piece);
    let rel = is_relative_pin(position, pinned_piece);
    match (abs, rel) {
        (true, true) => Some(AbsoluteAndRelative),
        (true, false) => Some(Absolute),
        (false, true) => Some(Relative),
        _ => None,
    }
}

pub fn is_absolutely_pinned(position: &Chess, pinned_piece: Square) -> bool {
    position
        .board()
        .pins_where_square_is_pinned(pinned_piece)
        .into_iter()
        .any(|pin| pin.is_absolute())
}

pub fn is_relative_pin(position: &Chess, pinned_piece: Square) -> bool {
    position
        .board()
        .pins_where_square_is_pinned(pinned_piece)
        .into_iter()
        .any(|pin| pin.is_relative())
}

trait RoleBoard {
    fn by_higher_valued_roles(&self, r: Role) -> Bitboard;
    fn by_lower_valued_roles(&self, r: Role) -> Bitboard;
}

impl RoleBoard for Board {
    fn by_higher_valued_roles(&self, r: Role) -> Bitboard {
        r.higher_value_roles()
            .iter()
            .map(|&&r| self.by_role(r))
            .fold(Bitboard::EMPTY, |acc, bb| acc | bb)
    }

    fn by_lower_valued_roles(&self, r: Role) -> Bitboard {
        r.lower_value_roles()
            .iter()
            .map(|&&r| self.by_role(r))
            .fold(Bitboard::EMPTY, |acc, bb| acc | bb)
    }
}

#[cfg(test)]
mod tests;
