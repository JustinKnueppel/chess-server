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
            .rev()
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
            .map(|row| match row {
                0 => BACK_ROW
                    .iter()
                    .zip(0..8)
                    .map(|(piece_type, col)| Square {
                        color: get_color(row, col),
                        piece: Some(pieces::Piece {
                            color: pieces::PieceColor::White,
                            kind: piece_type.clone(),
                        }),
                    })
                    .collect::<Vec<Square>>(),
                1 => (0..8)
                    .map(|col| Square {
                        color: get_color(row, col),
                        piece: Some(pieces::Piece {
                            color: pieces::PieceColor::White,
                            kind: pieces::PieceType::Pawn,
                        }),
                    })
                    .collect::<Vec<Square>>(),
                6 => (0..8)
                    .map(|col| Square {
                        color: get_color(row, col),
                        piece: Some(pieces::Piece {
                            color: pieces::PieceColor::Black,
                            kind: pieces::PieceType::Pawn,
                        }),
                    })
                    .collect::<Vec<Square>>(),
                7 => BACK_ROW
                    .iter()
                    .zip(0..8)
                    .map(|(piece_type, col)| Square {
                        color: get_color(row, col),
                        piece: Some(pieces::Piece {
                            color: pieces::PieceColor::Black,
                            kind: piece_type.clone(),
                        }),
                    })
                    .collect::<Vec<Square>>(),
                _ => (0..8)
                    .map(|col| Square {
                        color: get_color(row, col),
                        piece: None,
                    })
                    .collect::<Vec<Square>>(),
            })
            .collect();
        return Board { squares };
    }
}

fn get_color(row: i32, col: i32) -> SquareColor {
    return if (row + col) % 2 == 0 {
        SquareColor::White
    } else {
        SquareColor::Black
    };
}
