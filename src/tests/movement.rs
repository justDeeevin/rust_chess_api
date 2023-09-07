pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;
use crate::chess::*;

#[test]
fn blocked_path() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::One,
            },
            Position {
                file: File::A,
                rank: Rank::Three,
            },
        ),
        Err(Error::Move(MoveError::PathIsBlocked))
    );
}
