use std::fmt;

#[derive(Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Pawn,
    Rook,
    Knight,
    Bishop,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceType,
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
        write!(f, "{} ", icon)
    }
}

impl TryFrom<char> for Piece {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'K' => Ok(Piece {
                color: PieceColor::White,
                kind: PieceType::King,
            }),
            'k' => Ok(Piece {
                color: PieceColor::Black,
                kind: PieceType::King,
            }),
            'Q' => Ok(Piece {
                color: PieceColor::White,
                kind: PieceType::Queen,
            }),
            'q' => Ok(Piece {
                color: PieceColor::Black,
                kind: PieceType::Queen,
            }),
            'R' => Ok(Piece {
                color: PieceColor::White,
                kind: PieceType::Rook,
            }),
            'r' => Ok(Piece {
                color: PieceColor::Black,
                kind: PieceType::Rook,
            }),
            'N' => Ok(Piece {
                color: PieceColor::White,
                kind: PieceType::Knight,
            }),
            'n' => Ok(Piece {
                color: PieceColor::Black,
                kind: PieceType::Knight,
            }),
            'B' => Ok(Piece {
                color: PieceColor::White,
                kind: PieceType::Bishop,
            }),
            'b' => Ok(Piece {
                color: PieceColor::Black,
                kind: PieceType::Bishop,
            }),
            'P' => Ok(Piece {
                color: PieceColor::White,
                kind: PieceType::Pawn,
            }),
            'p' => Ok(Piece {
                color: PieceColor::Black,
                kind: PieceType::Pawn,
            }),
            _ => Err("Not a piece".to_string()),
        }
    }
}
