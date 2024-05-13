use std::fmt;

pub enum PieceColor {
    White,
    Black,
}

pub enum PieceType {
    King,
    Queen,
    Pawn,
    Rook,
    Knight,
    Bishop,
}

pub struct Piece {
    color: PieceColor,
    kind: PieceType,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = match self.kind {
            PieceType::King => match self.color {
                PieceColor::White => "K",
                PieceColor::Black => "k",
            },
            PieceType::Queen => match self.color {
                PieceColor::White => "Q",
                PieceColor::Black => "q",
            },

            PieceType::Pawn => match self.color {
                PieceColor::White => "P",
                PieceColor::Black => "p",
            },

            PieceType::Rook => match self.color {
                PieceColor::White => "R",
                PieceColor::Black => "r",
            },

            PieceType::Knight => match self.color {
                PieceColor::White => "N",
                PieceColor::Black => "n",
            },

            PieceType::Bishop => match self.color {
                PieceColor::White => "B",
                PieceColor::Black => "b",
            },
        };
        write!(f, "{}", icon)
    }
}
