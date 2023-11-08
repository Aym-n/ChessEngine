extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::EV_CNT;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};


#[derive(Copy , Clone)]
struct Piece {
    ptype: PieceType,
    pColor: pColor,
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

#[derive(Copy , Clone, PartialEq)]
enum pColor{
    White,
    Black,
} 

struct Board {
    squares: [[Option<Piece>; 8]; 8],
    selected_piece: Option<(usize, usize)>,
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
                    let color_symbol = match piece.pColor {
                        pColor::White => "W",
                        pColor::Black => "B",
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
fn move_generation(board: &Board) -> Vec<(usize, usize)>{

    let mut moves = Vec::new();

    match board.selected_piece {
        Some((rank, file)) => {
            let piece = board.squares[rank][file].unwrap();
            match piece.ptype {
                PieceType::Pawn => {
                    // Generate pawn moves
                    let direction = match piece.pColor {
                        pColor::White => 1,
                        pColor::Black => -1,
                    };
                    let new_rank = rank as i32 + direction;
                    if new_rank >= 0 && new_rank < 8 {
                        // Move forward
                        if board.squares[new_rank as usize][file].is_none() {
                            moves.push((new_rank as usize, file));
                        }
                        // Move forward two squares
                        if rank == 1 || rank == 6 {
                            let new_rank = rank as i32 + 2 * direction;
                            if board.squares[new_rank as usize][file].is_none() {
                                moves.push((new_rank as usize, file));
                            }
                        }
                        // Capture diagonally to the left
                        if file > 0 {
                            if let Some(captured_piece) = board.squares[new_rank as usize][file - 1] {
                                if captured_piece.pColor != piece.pColor {
                                    moves.push((new_rank as usize, file - 1));
                                }
                            }
                        }
                        // Capture diagonally to the right
                        if file < 7 {
                            if let Some(captured_piece) = board.squares[new_rank as usize][file + 1] {
                                if captured_piece.pColor != piece.pColor {
                                    moves.push((new_rank as usize, file + 1));
                                }
                            }
                        }
                    }
                }
                PieceType::Rook => {
                    // Generate rook moves
                    for direction in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let mut new_rank = rank as i32 + direction.0;
                        let mut new_file = file as i32 + direction.1;
                        while new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                            if board.squares[new_rank as usize][new_file as usize].is_none() {
                                moves.push((new_rank as usize, new_file as usize));
                            } else {
                                if let Some(captured_piece) = board.squares[new_rank as usize][new_file as usize] {
                                    if captured_piece.pColor != piece.pColor {
                                        moves.push((new_rank as usize, new_file as usize));
                                    }
                                }
                                break;
                            }
                            new_rank += direction.0;
                            new_file += direction.1;
                        }
                    }
                }

                PieceType::Knight => {
                    // Generate knight moves
                    for direction in &[(1, 2), (1, -2), (-1, 2), (-1, -2),
                                       (2, 1), (2, -1), (-2, 1), (-2, -1)] {
                        let new_rank = rank as i32 + direction.0;
                        let new_file = file as i32 + direction.1;
                        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                            if board.squares[new_rank as usize][new_file as usize].is_none() {
                                moves.push((new_rank as usize, new_file as usize));
                            } else {
                                if let Some(captured_piece) = board.squares[new_rank as usize][new_file as usize] {
                                    if captured_piece.pColor != piece.pColor {
                                        moves.push((new_rank as usize, new_file as usize));
                                    }
                                }
                            }
                        }
                    }
                }

                PieceType::King => {
                    // Generate king moves
                    for direction in &[(1, 0), (-1, 0), (0, 1), (0, -1),
                                       (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        let new_rank = rank as i32 + direction.0;
                        let new_file = file as i32 + direction.1;
                        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                            if board.squares[new_rank as usize][new_file as usize].is_none() {
                                moves.push((new_rank as usize, new_file as usize));
                            } else {
                                if let Some(captured_piece) = board.squares[new_rank as usize][new_file as usize] {
                                    if captured_piece.pColor != piece.pColor {
                                        moves.push((new_rank as usize, new_file as usize));
                                    }
                                }
                            }
                        }
                    }
                }

                PieceType::Bishop => {
                    // Generate bishop moves
                    for direction in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        let mut new_rank = rank as i32 + direction.0;
                        let mut new_file = file as i32 + direction.1;
                        while new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                            if board.squares[new_rank as usize][new_file as usize].is_none() {
                                moves.push((new_rank as usize, new_file as usize));
                            } else {
                                if let Some(captured_piece) = board.squares[new_rank as usize][new_file as usize] {
                                    if captured_piece.pColor != piece.pColor {
                                        moves.push((new_rank as usize, new_file as usize));
                                    }
                                }
                                break;
                            }
                            new_rank += direction.0;
                            new_file += direction.1;
                        }
                    }
                }

                PieceType::Queen => {
                    // Generate queen moves
                    for direction in &[(1, 0), (-1, 0), (0, 1), (0, -1),
                                       (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        let mut new_rank = rank as i32 + direction.0;
                        let mut new_file = file as i32 + direction.1;
                        while new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                            if board.squares[new_rank as usize][new_file as usize].is_none() {
                                moves.push((new_rank as usize, new_file as usize));
                            } else {
                                if let Some(captured_piece) = board.squares[new_rank as usize][new_file as usize] {
                                    if captured_piece.pColor != piece.pColor {
                                        moves.push((new_rank as usize, new_file as usize));
                                    }
                                }
                                break;
                            }
                            new_rank += direction.0;
                            new_file += direction.1;
                        }
                    }
                }
            }
        },
        None => { return moves;}
    }

    moves

}
impl Board{
    fn new() -> Board{
        let mut board = Board {
            squares: [[None; 8]; 8],
            selected_piece: None,
        };
    
        // Set up the white pieces (top of the board)
        board.squares[0][0] = Some(Piece {
            ptype: PieceType::Rook,
            pColor: pColor::White,
        });
        board.squares[0][1] = Some(Piece {
            ptype: PieceType::Knight,
            pColor: pColor::White,
        });
        board.squares[0][2] = Some(Piece {
            ptype: PieceType::Bishop,
            pColor: pColor::White,
        });
        board.squares[0][3] = Some(Piece {
            ptype: PieceType::Queen,
            pColor: pColor::White,
        });
        board.squares[0][4] = Some(Piece {
            ptype: PieceType::King,
            pColor: pColor::White,
        });
        board.squares[0][5] = Some(Piece {
            ptype: PieceType::Bishop,
            pColor: pColor::White,
        });
        board.squares[0][6] = Some(Piece {
            ptype: PieceType::Knight,
            pColor: pColor::White,
        });
        board.squares[0][7] = Some(Piece {
            ptype: PieceType::Rook,
            pColor: pColor::White,
        });
    
        // Set up the black pieces (bottom of the board)
        board.squares[7][0] = Some(Piece {
            ptype: PieceType::Rook,
            pColor: pColor::Black,
        });
        board.squares[7][1] = Some(Piece {
            ptype: PieceType::Knight,
            pColor: pColor::Black,
        });
        board.squares[7][2] = Some(Piece {
            ptype: PieceType::Bishop,
            pColor: pColor::Black,
        });
        board.squares[7][3] = Some(Piece {
            ptype: PieceType::Queen,
            pColor: pColor::Black,
        });
        board.squares[7][4] = Some(Piece {
            ptype: PieceType::King,
            pColor: pColor::Black,
        });
        board.squares[7][5] = Some(Piece {
            ptype: PieceType::Bishop,
            pColor: pColor::Black,
        });
        board.squares[7][6] = Some(Piece {
            ptype: PieceType::Knight,
            pColor: pColor::Black,
        });

        board.squares[7][7] = Some(Piece {
            ptype: PieceType::Rook,
            pColor: pColor::Black,
        });
    
        for i in 0..8 {
            board.squares[6][i] = Some(Piece {
                ptype: PieceType::Pawn,
                pColor: pColor::Black,
            });

            board.squares[1][i] = Some(Piece {
                ptype: PieceType::Pawn,
                pColor: pColor::White,
            });

        }
        board
    }

    fn display(&mut self){

        const SQUARE_SIZE: u32 = 100;
        const BOARD_SIZE: u32 = 8;

        let mut _moves: Vec<(usize, usize)> = Vec::new();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem
            .window("Chess", 800, 800)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
    
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();
    
        let mut event_pump = sdl_context.event_pump().unwrap();
    
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::MouseButtonDown {x, y, .. } => {
                        
                        // Get the rank and file of the square that was clicked
                        let file = x / 100;
                        let rank = 7 - (y / 100);

                        if let Some((selected_rank, selected_file)) = self.selected_piece {
                            //check if the move is valid
                            if (rank as usize, file as usize) != (selected_rank, selected_file) && _moves.contains(&(rank as usize, file as usize)) {
                                // Move the piece to the new square
                                let piece_to_move = self.squares[selected_rank][selected_file].take();
                                self.squares[rank as usize][file as usize] = piece_to_move;

                                //clear the moves
                                _moves.clear();

                                // Deselect the piece
                                self.selected_piece = None;
                                
                            } else {

                                // Deselect the piece if the same square is clicked and clear the moves
                                self.selected_piece = None;
                                _moves.clear();
                            }
                        } else {

                            // Select the square if no piece is currently selected
                            self.selected_piece = Some((rank as usize, file as usize));
                            _moves = move_generation(&self);
                        }
                    }
                    _ => {}
                }
            }


            // loop to iterate through the 2d array
            for rank in 0..BOARD_SIZE{
                for file in 0..BOARD_SIZE{
                    let x = file * SQUARE_SIZE;
                    let y = (BOARD_SIZE - rank - 1) * SQUARE_SIZE;

                    let square = sdl2::rect::Rect::new(x as i32, y as i32, SQUARE_SIZE, SQUARE_SIZE);
                    
                    let color = if self.selected_piece == Some((rank as usize, file as usize)) {

                        sdl2::pixels::Color::RGB(52, 152, 219)

                    } else if _moves.contains(&(rank as usize, file as usize)) {

                        sdl2::pixels::Color::RGB(41, 128, 185) // Light blue for selected

                    } else if (rank + file) % 2 == 0 {

                        sdl2::pixels::Color::RGB(125, 135, 150) // dark blue for valid moves
                        
                    } else {

                        sdl2::pixels::Color::RGB(233, 236, 239) // Dark square

                    };

                    canvas.set_draw_color(color);
                    canvas.fill_rect(square).unwrap();

                    if let Some(piece) = self.squares[rank as usize][file as usize] {
                        let piece_color = match piece.pColor {
                            pColor::White => sdl2::pixels::Color::RGB(255, 255, 255), // White
                            pColor::Black => sdl2::pixels::Color::RGB(0, 0, 0), // Black
                        };
            
                        // Using Squares to represent pieces 
                        let piece_rect = sdl2::rect::Rect::new(
                            (x + SQUARE_SIZE / 4) as i32,
                            (y + SQUARE_SIZE / 4) as i32,
                            SQUARE_SIZE / 2,
                            SQUARE_SIZE / 2,
                        );
            
                        canvas.set_draw_color(piece_color);
                        canvas.fill_rect(piece_rect).unwrap();
                    }
                }
            }
    
            canvas.present();
        }
    }

}


fn main() {

    let mut board = Board::new();

    board.display();

}