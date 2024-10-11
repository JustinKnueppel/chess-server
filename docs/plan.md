# Chess server

## Major components:

### Pieces

PieceColor: White | Black

Type: Enum

- King
- Queen
- Bishop
- Knight
- Rook
- Pawn

### Squares

Color: White | Black

Piece: Option Piece

### Board

Collection of Squares

### Player

Color

Eventually maybe name etc

## Functionality

### Initializing board

- Normal board start
- From PGN
- From FEN

### Move pieces

Try to move something from square X to square Y

- If there is not a piece on X, do nothing
- If there is a piece on X, check that it can move to square Y
  - Ensure that by piece logic X could move to Y
  - Ensure that there are no obstructions blocking movement to Y (possibly combine with piece logic)
  - Ensure that a friendly piece is not on Y
  - Ensure that X's king is not in check after a X is moved to Y

### Determine game state

- After each move, generate all legal moves for the other side. If none exist then the game is over. If the king is currently in check it is checkmate, otherwise it is stalemate
- Change which team is allowed to move

## Notes

- All previous moves must be stored
  - Needed in order to allow en-passant
  - Needed in order to enforce the 50 move rule
  - Needed in order to enforce 3-fold repetition (This could/should be stored in a hashtable of some sort)
- Loading from PGN needs to properly load full game state
- Should have a game state that is not just a history of moves - otherwise FEN will not be a valid game state load

## Core data

```text
PieceColor = White | Black
PieceType = King | Queen | Rook | Knight | Bishop | Pawn
Piece = { PieceType, PieceColor }
SquareColor = White | Black
Square = { SquareColor, Option<Piece> }
Board = Collection<Square>
Coordinate = A1, A2, B1...
Move = { piece: Piece, StartCoord: Coordinate, EndCoord: Coordinate }
Game = {
  Board,
  CurrentPlayer: PieceColor,
  PreviousMoves: []Move
}
GameStatus = InProgress | Stalemate | Checkmate<PieceType>
```

XoXoX
oXXXo
XXOXX
oXXXo
XoXoX

good: (1, 2), (2, 1),(-1, 2), (-2, 1),(1, -2), (2, -1),(-1, -2), (-2, -1),

## Core functions

```text
// This should encapsulate any logic for whether a move is legal, and have distict errors for when it is not. New game state is calculated an returned if successful
try_make_move(game Game, start_coord Coordinate, end_coord Coordinate) Result<Game, MoveError>
calculate_legal_moves(game Game, color PieceColor) []Move
is_in_check(color PieceColor, game Game) bool
legal_en_passant_squares(move Move) Option<[]Coordinate>
coordinate_by_offset(start: Coordinate, offset(i32, i32))
```
