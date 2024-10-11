import gleam/int
import gleam/list
import gleam/result
import gleam/string
import piece

type SquareColor {
  Black
  White
}

type Square {
  Occupied(color: SquareColor, piece: piece.Piece)
  Vacant(color: SquareColor)
}

pub opaque type Board {
  Board(List(Square))
}

pub type ParseError {
  ParseError(msg: String)
}

pub fn from_string(str: String) -> Result(Board, ParseError) {
  split_8(str, "\n")
  |> result.try(fn(lines) { list.try_map(lines, split_8(_, "")) })
  |> result.try(fn(squares) {
    list.try_map(squares, list.try_map(_, square_from_string))
  })
  |> result.map(fn(squares) { list.flatten(squares) |> Board })
}

fn split_8(str: String, split: String) -> Result(List(String), ParseError) {
  let split = string.split(str, split)
  case list.length(split) {
    8 -> Ok(split)
    _ as n -> Error(ParseError("Split length: " <> int.to_string(n)))
  }
}

pub fn from_string_bak(str: String) -> Result(Board, ParseError) {
  let lines = string.split(str, "\n")
  let chars = list.map(lines, string.split(_, ""))
  let parsed_lines = list.try_map(chars, list.try_map(_, square_from_string))

  result.map(parsed_lines, fn(squares) { list.flatten(squares) |> Board })
}

fn square_from_string(str: String) -> Result(Square, ParseError) {
  piece.from_string(str)
  |> result.map(fn(piece) { Occupied(White, piece) })
  |> result.try_recover(fn(_err) {
    case str {
      "." -> Ok(Vacant(White))
      _ -> Error(ParseError("Not a valid square"))
    }
  })
}
