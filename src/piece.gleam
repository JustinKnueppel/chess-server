/// PieceColor represents the team color of the piece.
pub type PieceColor {
  White
  Black
}

/// Piece represents a chess piece.
pub type Piece {
  Pawn(color: PieceColor)
  Rook(color: PieceColor)
  Knight(color: PieceColor)
  Bishop(color: PieceColor)
  Queen(color: PieceColor)
  King(color: PieceColor)
}

/// to_string returns an ASCII representation of the piece.
pub fn to_string(piece: Piece) -> String {
  case piece {
    Bishop(White) -> "B"
    Bishop(Black) -> "b"
    King(White) -> "K"
    King(Black) -> "k"
    Knight(White) -> "N"
    Knight(Black) -> "n"
    Pawn(White) -> "P"
    Pawn(Black) -> "p"
    Queen(White) -> "Q"
    Queen(Black) -> "q"
    Rook(White) -> "R"
    Rook(Black) -> "r"
  }
}

pub type ParseError {
  ParseError(msg: String)
}

pub fn from_string(str: String) -> Result(Piece, ParseError) {
  case str {
    "B" -> Ok(Bishop(White))
    "b" -> Ok(Bishop(Black))
    "K" -> Ok(King(White))
    "k" -> Ok(King(Black))
    "N" -> Ok(Knight(White))
    "n" -> Ok(Knight(Black))
    "P" -> Ok(Pawn(White))
    "p" -> Ok(Pawn(Black))
    "Q" -> Ok(Queen(White))
    "q" -> Ok(Queen(Black))
    "R" -> Ok(Rook(White))
    "r" -> Ok(Rook(Black))
    _ -> Error(ParseError("Not a valid piece"))
  }
}
