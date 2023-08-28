use crate::chess::*;

#[test]
fn test_pawn_movement() {
    let mut board = Board::default();
    assert_eq!(
        board.move_piece(
            Position {
                rank: Rank::Two,
                file: File::A
            },
            Position {
                rank: Rank::Three,
                file: File::A
            }
        ),
        Ok(())
    );
}
