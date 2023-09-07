use std::collections::HashMap;

pub enum Error {
    RankParse,
    FileParse,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Board {
    pub squares: HashMap<File, HashMap<Rank, Square>>,
    pub state: BoardState,
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
    pub file: File,
    pub rank: Rank,
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
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
