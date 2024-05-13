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
                // match row {
                //     0 => BACK_ROW.clone(),
                //     7 => BACK_ROW
                //         .iter()
                //         .rev()
                //         .collect::<Vec<Square>>()
                //         .try_into()
                //         .unwrap(),
                //     _ => BACK_ROW.clone(),
                // }
                (0..8)
                    .map(|col| {
                        let color = if (row + col) % 2 == 0 {
                            SquareColor::White
                        } else {
                            SquareColor::Black
                        };
                        Square { color, piece: None }
                    })
                    .collect()
            })
            .collect();
        return Board { squares };
    }
}
