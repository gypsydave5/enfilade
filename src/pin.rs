use shakmaty;
use shakmaty::attacks::attacks;
use shakmaty::{Bitboard, Chess, Color, File, Piece, Pieces, Rank, Role, Setup, Square};
use std::ops::Not;
use std::str::FromStr;

pub fn is_absolutely_pinned(position: shakmaty::Chess, square: shakmaty::Square) -> bool {
    let board = position.board();
    let piece = match board.piece_at(square) {
        None => return false,
        Some(p) => p,
    };
    let defender = piece.color;
    let attacker = defender.not();

    let same_color_king = position.board().king_of(defender).unwrap();
    if (same_color_king == square) {
        return false;
    };

    let attacking_king_on_empty_board =
        position
            .board()
            .attacks_to(same_color_king, attacker, Bitboard::EMPTY);
    let attacking_potential_pin_on_empty_board =
        position
            .board()
            .attacks_to(square, attacker, Bitboard::EMPTY);

    let attacking_both = attacking_potential_pin_on_empty_board & attacking_king_on_empty_board;

    let attacks_go_through = attacking_both
        .into_iter()
        .filter(|s| shakmaty::attacks::aligned(*s, square, same_color_king));

    attacks_go_through.count() != 0
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    fn position_of(fen: &str) -> shakmaty::Chess {
        let fen = shakmaty::fen::Fen::from_str(fen).unwrap();
        fen.position().unwrap()
    }

    fn pawn_absolutely_pinned_on_e2() -> shakmaty::Chess {
        position_of("8/1k6/8/8/8/8/1r2P1K1/8 w - - 0 1")
    }

    fn pawn_not_pinned_on_e2() -> shakmaty::Chess {
        position_of("8/1k6/8/8/8/8/4P1K1/8 w - - 0 1")
    }

    fn queen_attacks_pawn_and_king() -> shakmaty::Chess {
        position_of("1qk5/8/8/8/8/8/1P5K/8 w - - 0 1")
    }

    fn queen_attacks_king_through_2_pawns() -> shakmaty::Chess {
        position_of("kq6/8/3P4/4P3/8/8/7K/8 w - - 0 1")
    }

    #[test]
    fn testing() {
        assert!(true)
    }

    mod absolute_pin {
        use crate::pin::tests::{
            pawn_absolutely_pinned_on_e2, pawn_not_pinned_on_e2,
            queen_attacks_king_through_2_pawns, queen_attacks_pawn_and_king,
        };
        use crate::pin::{is_absolutely_pinned, pretty_board};
        use shakmaty::Setup;

        #[test]
        fn detects_a_rook_pin() {
            let position = pawn_absolutely_pinned_on_e2();
            assert!(is_absolutely_pinned(position, shakmaty::Square::E2))
        }

        #[test]
        fn empty_squares_are_not_pinned() {
            let position = pawn_absolutely_pinned_on_e2();
            assert!(!is_absolutely_pinned(position, shakmaty::Square::E4))
        }

        #[test]
        fn kings_are_not_pinned() {
            let position = pawn_absolutely_pinned_on_e2();
            assert!(!is_absolutely_pinned(position, shakmaty::Square::G2))
        }

        #[test]
        fn pieces_that_are_not_pinned_are_not_pinned() {
            let position = pawn_not_pinned_on_e2();
            assert!(!is_absolutely_pinned(position, shakmaty::Square::E2))
        }

        #[test]
        fn pinning_attack_must_go_through_the_piece() {
            let position = queen_attacks_pawn_and_king();
            assert!(!is_absolutely_pinned(position, shakmaty::Square::B2))
        }

        #[test]
        fn not_an_absolute_pin_if_more_than_two_pieces_are_in_the_way() {
            let position = queen_attacks_king_through_2_pawns();
            assert!(!is_absolutely_pinned(
                position.clone(),
                shakmaty::Square::D6
            ));
            assert!(!is_absolutely_pinned(position, shakmaty::Square::E5));
        }
    }
}

fn pretty_board(board: &shakmaty::Board) -> String {
    let mut s = String::new();
    for rank in (0..8).map(Rank::new).rev() {
        for file in (0..8).map(File::new) {
            let sq = Square::from_coords(file, rank);
            let piece = board.piece_at(sq).map(|p| p.char()).unwrap_or(' ');
            s.push(piece);
        }
        s.push('\n')
    }
    return s;
}
