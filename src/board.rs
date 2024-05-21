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
        let squares: Vec<Vec<Square>> = (0..8)
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

pub fn board_from_str(board: &str) -> Result<Board, String> {
    let lines = board.trim().split("\n");
    let lines_count = lines.count();
    if lines_count != 8 {
        println!("{}", lines_count);
        return Err("Should be 8 rows".to_string());
    }
    // TODO: This should parse in reverse order
    let squares: Vec<Vec<Square>> = board
        .trim()
        .split("\n")
        .zip(0..8)
        .map(|(line, row)| {
            return line
                .trim()
                .chars()
                .zip(0..8)
                .map(|(c, col)| Square {
                    color: get_color(row, col),
                    piece: pieces::Piece::try_from(c).ok(),
                })
                .collect();
        })
        .collect();
    if !squares.iter().all(|row| row.len() == 8) {
        return Err("Each row must have 8 columns".to_string());
    }
    Ok(Board { squares })
}
