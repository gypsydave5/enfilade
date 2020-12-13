use shakmaty::{fen, Board, Rank, Square};
use shakmaty::{Chess, File};
use std::str::FromStr;

fn position_of(fen: &str) -> Chess {
    let fen = fen::Fen::from_str(fen).unwrap();
    fen.position().unwrap()
}

fn pawn_absolutely_pinned_on_e2() -> Chess {
    position_of("8/1k6/8/8/8/8/1r2P1K1/8 w - - 0 1")
}

fn pawn_not_pinned_on_e2() -> Chess {
    position_of("8/1k6/8/8/8/8/4P1K1/8 w - - 0 1")
}

fn queen_attacks_pawn_and_king() -> Chess {
    position_of("1qk5/8/8/8/8/8/1P5K/8 w - - 0 1")
}

fn queen_attacks_king_through_2_pawns() -> Chess {
    position_of("kq6/8/3P4/4P3/8/8/7K/8 w - - 0 1")
}

fn rook_attacks_rook_through_bishop() -> Chess {
    position_of("4r2k/8/8/8/4B3/8/8/4R2K w - - 0 1")
}

fn rook_attacks_queen_and_bishop() -> Chess {
    position_of("r6B/1K6/8/3k4/8/8/8/Q7 w - - 0 1")
}

fn rook_attacks_bishop_through_queen() -> Chess {
    position_of("4r2k/8/8/8/4Q3/8/8/4B2K w - - 0 1")
}

mod is_pin {
    use crate::pin::is_pin;
    use crate::pin::tests::position_of;
    use crate::pin::Pins::*;
    use shakmaty::Square;

    macro_rules! test_is_pin {
        ($name:ident, $fen:expr, $square:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let position = position_of($fen);
                let result = is_pin(&position, $square);
                assert_eq!(result, Some($expected));
            }
        };
    }

    macro_rules! test_is_not_pin {
        ($name:ident, $fen:expr, $square:expr) => {
            #[test]
            fn $name() {
                let position = position_of($fen);
                let result = is_pin(&position, $square);
                assert_eq!(result, None);
            }
        };
    }

    test_is_pin!(
        pin_on_e2,
        "8/1k6/8/8/8/8/1r2P1K1/8 w - - 0 1",
        Square::E2,
        Absolute
    );
    test_is_not_pin!(
        no_pin_on_e3,
        "8/1k6/8/8/8/8/1r2P1K1/8 w - - 0 1",
        Square::E3
    );
}

mod absolute_pin {
    use crate::pin::is_absolutely_pinned;
    use crate::pin::tests::{
        pawn_absolutely_pinned_on_e2, pawn_not_pinned_on_e2, queen_attacks_king_through_2_pawns,
        queen_attacks_pawn_and_king,
    };
    use shakmaty::Square;

    #[test]
    fn detects_a_rook_pin() {
        let position = pawn_absolutely_pinned_on_e2();
        assert!(is_absolutely_pinned(&position, Square::E2))
    }

    #[test]
    fn empty_squares_are_not_pinned() {
        let position = pawn_absolutely_pinned_on_e2();
        assert!(!is_absolutely_pinned(&position, Square::E4))
    }

    #[test]
    fn kings_are_not_pinned() {
        let position = pawn_absolutely_pinned_on_e2();
        assert!(!is_absolutely_pinned(&position, Square::G2))
    }

    #[test]
    fn pieces_that_are_not_pinned_are_not_pinned() {
        let position = pawn_not_pinned_on_e2();
        assert!(!is_absolutely_pinned(&position, Square::E2))
    }

    #[test]
    fn pinning_attack_must_go_through_the_piece() {
        let position = queen_attacks_pawn_and_king();
        assert!(!is_absolutely_pinned(&position, Square::B2))
    }

    #[test]
    fn not_an_absolute_pin_if_more_than_two_pieces_are_in_the_way() {
        let position = queen_attacks_king_through_2_pawns();
        assert!(!is_absolutely_pinned(&position, Square::D6));
        assert!(!is_absolutely_pinned(&position, Square::E5));
    }
}

mod is_relative_pin {
    use crate::pin::is_relative_pin;
    use crate::pin::tests::{
        rook_attacks_bishop_through_queen, rook_attacks_queen_and_bishop,
        rook_attacks_rook_through_bishop,
    };
    use shakmaty::Square;

    #[test]
    fn detects_a_relative_pin() {
        let position = rook_attacks_rook_through_bishop();
        assert!(is_relative_pin(&position, Square::E4))
    }

    #[test]
    fn relative_pin_must_go_through_pieces() {
        let position = rook_attacks_queen_and_bishop();
        assert!(!is_relative_pin(&position, Square::H8));
        assert!(!is_relative_pin(&position, Square::A1));
    }

    #[test]
    fn pinned_piece_must_be_of_lower_value_than_target() {
        let position = rook_attacks_bishop_through_queen();
        assert!(!is_relative_pin(&position, Square::E4));
    }
}

pub fn pretty_board(board: &Board) -> String {
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
