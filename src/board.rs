use crate::pieces;
use std::fmt;

const BACK_ROW: &'static [pieces::PieceType] = &[
    pieces::PieceType::Rook,
    pieces::PieceType::Knight,
    pieces::PieceType::Bishop,
    pieces::PieceType::Queen,
    pieces::PieceType::King,
    pieces::PieceType::Bishop,
    pieces::PieceType::Knight,
    pieces::PieceType::Rook,
];

pub enum SquareColor {
    White,
    Black,
}

pub struct Square {
    color: SquareColor,
    piece: Option<pieces::Piece>,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = match &self.piece {
            Some(piece) => piece.to_string(),
            None => match self.color {
                SquareColor::White => "⬜".to_string(),
                SquareColor::Black => "⬛".to_string(),
            },
        };
        write!(f, "{}", icon)
    }
}

pub struct Board {
    squares: Vec<Vec<Square>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icons = self
            .squares
            .iter()
            .map(|row| {
                row.iter()
                    .map(|square| square.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", icons)
    }
}

impl Board {
    pub fn new() -> Self {
        let squares = (0..8)
            .map(|row| {
                match row {
                    0 => BACK_ROW
                        .iter()
                        .map(|piece_type| return Some(piece_type.clone()))
                        .collect::<Vec<Option<pieces::PieceType>>>(),
                    7 => BACK_ROW
                        .iter()
                        .rev()
                        .map(|piece_type| return Some(piece_type.clone()))
                        .collect::<Vec<Option<pieces::PieceType>>>(),
                    _ => (0..8)
                        .map(|_| None)
                        .collect::<Vec<Option<pieces::PieceType>>>(),
                }
                .iter()
                .map(|piece_type| Square {
                    color: SquareColor::White,
                    piece: match piece_type {
                        Some(pt) => Some(pieces::Piece {
                            color: pieces::PieceColor::White,
                            kind: pt.clone(),
                        }),
                        None => None,
                    },
                })
                .collect::<Vec<Square>>()
            })
            .collect();
        return Board { squares };
    }
}
