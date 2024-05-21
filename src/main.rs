mod board;
mod pieces;

fn main() {
    let board = board::board_from_str(
        "
        kkkkqqqq
        pppppppp
        ........
        ........
        ........
        ........
        PPPPPPPP
        KKKKQQQQ
    ",
    )
    .unwrap();
    println!("{}", board);
}
