use std::ops::{BitXor, Not};

use shakmaty;
use shakmaty::{Bitboard, Board, Chess, Role, Role::*, Setup, Square};

use crate::pin::Pin::{Absolute, AbsoluteAndRelative, Relative};

#[derive(Debug, PartialEq, Eq)]
pub enum Pin {
    Absolute,
    Relative,
    AbsoluteAndRelative,
}

pub fn is_pin(position: &Chess, pinned_piece: Square) -> Option<Pin> {
    let abs = is_absolutely_pinned(position, pinned_piece);
    let rel = is_relative_pin(position, pinned_piece);
    match (abs, rel) {
        (true, true) => Some(AbsoluteAndRelative),
        (true, false) => Some(Absolute),
        (false, true) => Some(Relative),
        _ => None,
    }
}

pub fn is_relative_pin(position: &Chess, pinned_piece: Square) -> bool {
    let board = position.board();
    let piece = match board.piece_at(pinned_piece) {
        None => return false,
        Some(p) => p,
    };

    let defender = piece.color;
    let attacker = defender.not();

    let potential_targets = (board.by_color(defender)) & board.by_higher_valued_roles(piece.role);

    potential_targets
        .into_iter()
        .flat_map(|target| {
            position
                .board()
                .attacks_to(target, attacker, Bitboard::EMPTY)
                .into_iter()
                .map(move |attacker| (target, attacker))
        })
        .map(|(target, attacker)| is_pinned(board.clone(), attacker, target, pinned_piece))
        .any(|b| b)
}

pub fn is_absolutely_pinned(position: &Chess, pinned_piece: Square) -> bool {
    let board = position.board();
    let piece = match board.piece_at(pinned_piece) {
        None => return false,
        Some(p) => p,
    };
    let defender = piece.color;
    let attacker = defender.not();

    let defending_king = position.board().king_of(defender).unwrap();
    if defending_king == pinned_piece {
        return false;
    };

    let attacking_target_on_empty_board =
        position
            .board()
            .attacks_to(defending_king, attacker, Bitboard::EMPTY)
            & board.sliders();

    attacking_target_on_empty_board
        .into_iter()
        .map(|attacker| is_pinned(board.clone(), attacker, defending_king, pinned_piece))
        .any(|b| b)
}

fn is_pinned(board: Board, attacker: Square, target: Square, pin: Square) -> bool {
    let attack_ray = shakmaty::attacks::ray(attacker, target);
    if !attack_ray.contains(pin) {
        return false;
    }

    let defender_pieces_not_target = board
        .by_color(board.piece_at(target).unwrap().color)
        .bitxor(Bitboard::from_square(target));

    (attack_ray & defender_pieces_not_target) == Bitboard::from_square(pin)
}

trait RolePoints {
    fn points(&self) -> Option<u8>;
    fn higher_value_roles(&self) -> Vec<&Role>;
    fn is_higher_value(&self, r: Role) -> bool;
}

trait RoleBoard {
    fn by_higher_valued_roles(&self, r: Role) -> Bitboard;
}

impl RoleBoard for Board {
    fn by_higher_valued_roles(&self, r: Role) -> Bitboard {
        r.higher_value_roles()
            .iter()
            .map(|&&r| self.by_role(r))
            .fold(Bitboard::EMPTY, |acc, bb| acc | bb)
    }
}

const ROLES: [Role; 6] = [Pawn, Knight, Bishop, Rook, Queen, King];

impl RolePoints for Role {
    fn points(&self) -> Option<u8> {
        match self {
            Pawn => Some(1),
            Knight => Some(3),
            Bishop => Some(3),
            Rook => Some(5),
            Queen => Some(9),
            King => None,
        }
    }

    fn higher_value_roles(&self) -> Vec<&Role> {
        ROLES.iter().filter(|&&r| self.is_higher_value(r)).collect()
    }

    fn is_higher_value(&self, r: Role) -> bool {
        r.points()
            .and_then(|r| self.points().map(|p| r > p))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests;
