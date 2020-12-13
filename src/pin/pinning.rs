use crate::pin::RoleBoard;
use core::option::Option;
use core::option::Option::{None, Some};
use shakmaty::bitboard::Bitboard;
use shakmaty::Role::King;
use shakmaty::Square;
use shakmaty::{Board, Piece};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{BitXor, Not};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pin {
    attacker: Square,
    target: Square,
    pinned: Square,
    board: Board,
}

impl Pin {
    pub fn is_absolute(&self) -> bool {
        self.board.piece_at(self.target).unwrap().role == King
    }

    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    pub fn pinned_piece(&self) -> Piece {
        self.board.piece_at(self.pinned).unwrap()
    }

    pub fn pinned_square(&self) -> Square {
        self.pinned
    }

    pub fn target_piece(&self) -> Piece {
        self.board.piece_at(self.target).unwrap()
    }

    pub fn target_square(&self) -> Square {
        self.target
    }

    pub fn attacking_piece(&self) -> Piece {
        self.board.piece_at(self.attacker).unwrap()
    }

    pub fn attacking_square(&self) -> Square {
        self.attacker
    }
}

impl Display for Pin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The {:?} {:?} on {} now pins the {:?} {:?} on {} to the {:?} {:?} on {}",
            self.attacking_piece().color,
            self.attacking_piece().role,
            self.attacking_square(),
            self.pinned_piece().color,
            self.pinned_piece().role,
            self.pinned_square(),
            self.target_piece().color,
            self.target_piece().role,
            self.target_square()
        )
    }
}

fn is_pinned(board: &Board, attacker: Square, target: Square, pin: Square) -> bool {
    let attack_ray = shakmaty::attacks::ray(attacker, target);
    if !attack_ray.contains(pin) {
        return false;
    }

    let defender_pieces_not_target = board
        .by_color(board.piece_at(target).unwrap().color)
        .bitxor(Bitboard::from_square(target));

    (attack_ray & defender_pieces_not_target) == Bitboard::from_square(pin)
}

pub trait Pinning {
    fn pins_where_square_is_pinned(&self, square: Square) -> Vec<Pin>;
    fn pins_where_square_is_attacker(&self, square: Square) -> Vec<Pin>;
    fn pins_where_square_is_target(&self, square: Square) -> Vec<Pin>;
    fn attacker_and_pin(&self, attacker: Square, pin: Square) -> Option<Pin>;
    fn pins_involving_square(&self, square: Square) -> Vec<Pin>;
}

impl Pinning for Board {
    fn pins_where_square_is_pinned(&self, square: Square) -> Vec<Pin> {
        let piece = match self.piece_at(square) {
            None => return Vec::new(),
            Some(p) => p,
        };

        let defender = piece.color;
        let attacker = defender.not();

        let potential_targets = (self.by_color(defender)) & self.by_higher_valued_roles(piece.role);

        potential_targets
            .into_iter()
            .flat_map(|target| {
                self.attacks_to(target, attacker, Bitboard::EMPTY)
                    .into_iter()
                    .map(move |attacker| (target, attacker))
            })
            .filter(|&(target, attacker)| is_pinned(self, attacker, target, square))
            .map(|(target, attacker)| Pin {
                attacker,
                target,
                pinned: square,
                board: self.clone(),
            })
            .collect()
    }

    fn pins_where_square_is_attacker(&self, square: Square) -> Vec<Pin> {
        let pinned = self.attacks_from(square);
        pinned
            .into_iter()
            .map(|pin| self.attacker_and_pin(square, pin))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }

    fn pins_where_square_is_target(&self, square: Square) -> Vec<Pin> {
        let piece = match self.piece_at(square) {
            None => return Vec::new(),
            Some(p) => p,
        };

        let lower_value_pieces =
            (self.by_color(piece.color)) & self.by_lower_valued_roles(piece.role);
        let current_attackers = self.attacks_to(square, !piece.color, Bitboard::EMPTY);

        lower_value_pieces
            .into_iter()
            .flat_map(|potential_pin| {
                let mut board_without_pin = self.clone();
                board_without_pin.remove_piece_at(potential_pin);
                let all_attackers =
                    board_without_pin.attacks_to(square, !piece.color, Bitboard::EMPTY);
                let revealed_attackers = current_attackers ^ all_attackers;
                revealed_attackers.into_iter().map(move |new_attacker| Pin {
                    attacker: new_attacker,
                    target: square,
                    pinned: potential_pin,
                    board: self.clone(),
                })
            })
            .collect()
    }

    fn attacker_and_pin(&self, attacker: Square, pinned: Square) -> Option<Pin> {
        let pinned_piece = match self.piece_at(pinned) {
            None => return None,
            Some(p) => p,
        };

        let defender = pinned_piece.color;

        let potential_targets =
            (self.by_color(defender)) & self.by_higher_valued_roles(pinned_piece.role);

        let target = potential_targets
            .into_iter()
            .filter(|&target| is_pinned(self, attacker, target, pinned))
            .next();

        return target.map(|target: Square| Pin {
            attacker,
            target,
            pinned,
            board: self.clone(),
        });
    }

    fn pins_involving_square(&self, square: Square) -> Vec<Pin> {
        [
            self.pins_where_square_is_attacker(square),
            self.pins_where_square_is_pinned(square),
            self.pins_where_square_is_target(square),
        ]
        .concat()
    }
}
