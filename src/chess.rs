use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    RankParse,
    FileParse,
    Move(MoveError),
}
impl From<Error> for actix_web::Error {
    fn from(err: Error) -> actix_web::Error {
        match err {
            Error::RankParse => actix_web::error::ErrorBadRequest("Invalid rank"),
            Error::FileParse => actix_web::error::ErrorBadRequest("Invalid file"),
            Error::Move(move_error) => match move_error {
                MoveError::EmptyStartingSquare => {
                    actix_web::error::ErrorBadRequest("Starting square is empty")
                }
                MoveError::NotYourTurn => actix_web::error::ErrorBadRequest("Not your turn"),
                MoveError::FriendlyFire => {
                    actix_web::error::ErrorBadRequest("Friendly fire is not allowed")
                }
                MoveError::InvalidPath => actix_web::error::ErrorBadRequest("Invalid path"),
                MoveError::PathIsBlocked => actix_web::error::ErrorBadRequest("Path is blocked"),
                MoveError::NoMotion => actix_web::error::ErrorBadRequest("No motion"),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    EmptyStartingSquare,
    NotYourTurn,
    FriendlyFire,
    InvalidPath,
    PathIsBlocked,
    NoMotion,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Board {
    pub squares: HashMap<File, HashMap<Rank, Square>>,
    pub state: BoardState,
}

impl Default for Board {
    fn default() -> Self {
        let mut squares = HashMap::new();
        for file in 1..=8 {
            let mut rank_map = HashMap::new();
            for rank in 1..=8 {
                let file = File::try_from(file).unwrap();
                let rank = Rank::try_from(rank).unwrap();
                let position = Position { rank, file };
                let troop = match (rank, file) {
                    (Rank::Two, _) => Some(Troop {
                        piece: Piece::Pawn,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Seven, _) => Some(Troop {
                        piece: Piece::Pawn,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::A) | (Rank::One, File::H) => Some(Troop {
                        piece: Piece::Rook,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::A) | (Rank::Eight, File::H) => Some(Troop {
                        piece: Piece::Rook,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::B) | (Rank::One, File::G) => Some(Troop {
                        piece: Piece::Knight,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::B) | (Rank::Eight, File::G) => Some(Troop {
                        piece: Piece::Knight,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::C) | (Rank::One, File::F) => Some(Troop {
                        piece: Piece::Bishop,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::C) | (Rank::Eight, File::F) => Some(Troop {
                        piece: Piece::Bishop,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::D) => Some(Troop {
                        piece: Piece::Queen,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::D) => Some(Troop {
                        piece: Piece::Queen,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::E) => Some(Troop {
                        piece: Piece::King,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::E) => Some(Troop {
                        piece: Piece::King,
                        color: Color::Black,
                        position,
                    }),
                    _ => None,
                };
                rank_map.insert(rank, Square { troop, position });
            }
            squares.insert(File::try_from(file).unwrap(), rank_map);
        }
        Board {
            squares,
            state: BoardState::ToMove(Color::White),
        }
    }
}

impl Board {
    pub fn fmt(&self) -> String {
        let mut board = String::new();
        for rank in 1..=8 {
            for file in 1..=8 {
                let file = File::try_from(file).unwrap();
                let rank = Rank::try_from(rank).unwrap();
                let square = self.squares.get(&file).unwrap().get(&rank).unwrap();
                let troop = match square.troop {
                    Some(ref troop) => match troop.color {
                        Color::White => match troop.piece {
                            Piece::Pawn => '♙',
                            Piece::Knight => '♘',
                            Piece::Bishop => '♗',
                            Piece::Rook => '♖',
                            Piece::Queen => '♕',
                            Piece::King => '♔',
                        },
                        Color::Black => match troop.piece {
                            Piece::Pawn => '♟',
                            Piece::Knight => '♞',
                            Piece::Bishop => '♝',
                            Piece::Rook => '♜',
                            Piece::Queen => '♛',
                            Piece::King => '♚',
                        },
                    },
                    None => '.',
                };
                board.push(troop);
            }
            board.push('\n');
        }
        board
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), Error> {
        let from_square = self
            .squares
            .get(&from.file)
            .unwrap()
            .get(&from.rank)
            .unwrap();
        let to_square = self.squares.get(&to.file).unwrap().get(&to.rank).unwrap();
        if from_square.troop.is_none() {
            return Err(Error::Move(MoveError::EmptyStartingSquare));
        }
        let from_troop = from_square.troop.as_ref().unwrap();
        if !self.state.can_move(from_troop.color) {
            return Err(Error::Move(MoveError::NotYourTurn));
        }
        let mut capturing = false;
        if let Some(troop) = &to_square.troop {
            capturing = true;
            if troop.color == from_troop.color {
                return Err(Error::Move(MoveError::FriendlyFire));
            }
        }

        let path = Self::make_path(from_troop, from, to, capturing)?;
        if from_troop.piece != Piece::Knight {
            for position in path {
                let square = self
                    .squares
                    .get(&position.file)
                    .unwrap()
                    .get(&position.rank)
                    .unwrap();
                if square.troop.is_some() {
                    return Err(Error::Move(MoveError::PathIsBlocked));
                }
            }
        }

        Ok(())
    }

    fn make_path(
        troop: &Troop,
        from: Position,
        to: Position,
        capturing: bool,
    ) -> Result<Vec<Position>, Error> {
        if from == to {
            return Err(Error::Move(MoveError::NoMotion));
        }
        let mut path = Vec::new();
        let file_diff = (from.file as i8 - to.file as i8).abs();
        let rank_diff = (from.rank as i8 - to.rank as i8).abs();
        match troop.piece {
            Piece::Pawn => {
                if file_diff == 0 {
                    return Err(Error::Move(MoveError::InvalidPath));
                }
                if file_diff == 2 {
                    match troop.color {
                        Color::White => {
                            if from.rank != Rank::Seven {
                                return Err(Error::Move(MoveError::InvalidPath));
                            }
                        }
                        Color::Black => {
                            if from.rank != Rank::Two {
                                return Err(Error::Move(MoveError::InvalidPath));
                            }
                        }
                    }
                }
                if file_diff > 2 {
                    return Err(Error::Move(MoveError::InvalidPath));
                }
                if rank_diff > 1 {
                    return Err(Error::Move(MoveError::InvalidPath));
                }
                if rank_diff == 1 && !capturing {
                    return Err(Error::Move(MoveError::InvalidPath));
                }
            }
            _ => todo!(),
        }

        Ok(path)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}
impl TryFrom<u8> for Rank {
    type Error = Error;
    fn try_from(rank: u8) -> Result<Self, Self::Error> {
        match rank {
            1 => Ok(Rank::One),
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            6 => Ok(Rank::Six),
            7 => Ok(Rank::Seven),
            8 => Ok(Rank::Eight),
            _ => Err(Error::RankParse),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
impl TryFrom<u8> for File {
    type Error = Error;
    fn try_from(file: u8) -> Result<Self, Self::Error> {
        match file {
            1 => Ok(File::A),
            2 => Ok(File::B),
            3 => Ok(File::C),
            4 => Ok(File::D),
            5 => Ok(File::E),
            6 => Ok(File::F),
            7 => Ok(File::G),
            8 => Ok(File::H),
            _ => Err(Error::FileParse),
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Position {
    pub rank: Rank,
    pub file: File,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Square {
    pub troop: Option<Troop>,
    pub position: Position,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Troop {
    pub piece: Piece,
    pub color: Color,
    pub position: Position,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum BoardState {
    ToMove(Color),
    Check(Color),
    Checkmate(Color),
    Stalemate,
    Draw,
}
impl BoardState {
    fn can_move(&self, team: Color) -> bool {
        match self {
            BoardState::ToMove(color) => *color == team,
            BoardState::Check(color) => *color == team,
            _ => false,
        }
    }
}
