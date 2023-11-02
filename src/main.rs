#[derive(Copy , Clone)]
struct Piece {
    ptype: PieceType,
    pcolor: Color,
}

#[derive(Copy , Clone, Debug)]
enum PieceType{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
#[derive(Copy , Clone)]
enum Color{
    White,
    Black,
} 

struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

fn print_board(board: &Board) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            match board.squares[rank][file] {
                Some(piece) => {
                    let piece_symbol = match piece.ptype {
                        PieceType::Pawn => "♙",
                        PieceType::Rook => "♖",
                        PieceType::Knight => "♘",
                        PieceType::Bishop => "♗",
                        PieceType::Queen => "♕",
                        PieceType::King => "♔",
                    };
                    let color_symbol = match piece.pcolor {
                        Color::White => "W",
                        Color::Black => "B",
                    };
                    print!("{}{} ", piece_symbol, color_symbol);
                }
                None => {
                    print!(".  ");
                }
            }
        }
        println!(); // Start a new line for the next rank
    }
}

impl Board{
    fn new() -> Board{
        let mut board = Board {
            squares: [[None; 8]; 8],
        };
    
        // Set up the white pieces (top of the board)
        board.squares[0][0] = Some(Piece {
            ptype: PieceType::Rook,
            pcolor: Color::White,
        });
        board.squares[0][1] = Some(Piece {
            ptype: PieceType::Knight,
            pcolor: Color::White,
        });
        board.squares[0][2] = Some(Piece {
            ptype: PieceType::Bishop,
            pcolor: Color::White,
        });
        board.squares[0][3] = Some(Piece {
            ptype: PieceType::Queen,
            pcolor: Color::White,
        });
        board.squares[0][4] = Some(Piece {
            ptype: PieceType::King,
            pcolor: Color::White,
        });
        board.squares[0][5] = Some(Piece {
            ptype: PieceType::Bishop,
            pcolor: Color::White,
        });
        board.squares[0][6] = Some(Piece {
            ptype: PieceType::Knight,
            pcolor: Color::White,
        });
        board.squares[0][7] = Some(Piece {
            ptype: PieceType::Rook,
            pcolor: Color::White,
        });
    
        for i in 0..8 {
            board.squares[1][i] = Some(Piece {
                ptype: PieceType::Pawn,
                pcolor: Color::White,
            });
        }
    
        // Set up the black pieces (bottom of the board)
        board.squares[7][0] = Some(Piece {
            ptype: PieceType::Rook,
            pcolor: Color::Black,
        });
        board.squares[7][1] = Some(Piece {
            ptype: PieceType::Knight,
            pcolor: Color::Black,
        });
        board.squares[7][2] = Some(Piece {
            ptype: PieceType::Bishop,
            pcolor: Color::Black,
        });
        board.squares[7][3] = Some(Piece {
            ptype: PieceType::Queen,
            pcolor: Color::Black,
        });
        board.squares[7][4] = Some(Piece {
            ptype: PieceType::King,
            pcolor: Color::Black,
        });
        board.squares[7][5] = Some(Piece {
            ptype: PieceType::Bishop,
            pcolor: Color::Black,
        });
        board.squares[7][6] = Some(Piece {
            ptype: PieceType::Knight,
            pcolor: Color::Black,
        });
        board.squares[7][7] = Some(Piece {
            ptype: PieceType::Rook,
            pcolor: Color::Black,
        });
    
        for i in 0..8 {
            board.squares[6][i] = Some(Piece {
                ptype: PieceType::Pawn,
                pcolor: Color::Black,
            });
        }
        board
    }
}


fn main() {

    let board = Board::new();

    print_board(&board);
}