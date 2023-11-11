extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::render::Texture;

#[derive(Copy, Clone)]
pub struct Piece {
    p_type: PieceType,
    p_color: PColor,
}

impl Piece {
    fn new_white(p_type: PieceType) -> Piece {
        Piece { p_type: (p_type), p_color: (PColor::White)}
    }
    
    fn new_black(p_type: PieceType) -> Piece {
        Piece { p_type: (p_type), p_color: (PColor::Black)}
    }
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

#[derive(Copy , Clone, PartialEq, Debug)]
enum PColor{
    White,
    Black,
} 

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    selected_piece: Option<(usize, usize)>,
    turn: PColor,

}

fn print_board(board: &Board) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            match board.squares[rank][file] {
                Some(piece) => {
                    let piece_symbol = match piece.p_type {
                        PieceType::Pawn => "♙",
                        PieceType::Rook => "♖",
                        PieceType::Knight => "♘",
                        PieceType::Bishop => "♗",
                        PieceType::Queen => "♕",
                        PieceType::King => "♔",
                    };
                    let color_symbol = match piece.p_color {
                        PColor::White => "W",
                        PColor::Black => "B",
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
            match piece.p_type {
                PieceType::Pawn => {
                    // Generate pawn moves
                    let direction = match piece.p_color {
                        PColor::White => 1,
                        PColor::Black => -1,
                    };
                    let new_rank = rank as i32 + direction;
                    if new_rank >= 0 && new_rank < 8 {
                        // Move forward
                        if board.squares[new_rank as usize][file].is_none() {
                            moves.push((new_rank as usize, file));
                        }
                        // Move forward two squares
                        if (rank == 1 && direction == 1) || (rank == 6 && direction == -1) {
                            let new_rank = rank as i32 + 2 * direction;
                            if board.squares[new_rank as usize][file].is_none() && board.squares[(new_rank - direction) as usize][file].is_none() {
                                moves.push((new_rank as usize, file));
                            }
                        }
                        // Capture diagonally to the left
                        if file > 0 {
                            if let Some(captured_piece) = board.squares[new_rank as usize][file - 1] {
                                if captured_piece.p_color != piece.p_color {
                                    moves.push((new_rank as usize, file - 1));
                                }
                            }
                        }
                        // Capture diagonally to the right
                        if file < 7 {
                            if let Some(captured_piece) = board.squares[new_rank as usize][file + 1] {
                                if captured_piece.p_color != piece.p_color {
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
                                    if captured_piece.p_color != piece.p_color {
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
                                    if captured_piece.p_color != piece.p_color {
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
                                    if captured_piece.p_color != piece.p_color {
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
                                    if captured_piece.p_color != piece.p_color {
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
                                    if captured_piece.p_color != piece.p_color {
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
    pub fn new() -> Board{
        let mut board = Board {
            squares: [[None; 8]; 8],
            selected_piece: None,
            turn: PColor::White,
        };

        //Starting position
        //Set up white pieces

        for file in 0..8{
            board.new_piece(Piece::new_white(PieceType::Pawn), 1, file);
        }
        board.new_piece(Piece::new_white(PieceType::Rook), 0, 0);
        board.new_piece(Piece::new_white(PieceType::Rook), 0, 7);
        board.new_piece(Piece::new_white(PieceType::Knight), 0, 1);
        board.new_piece(Piece::new_white(PieceType::Knight), 0, 6);
        board.new_piece(Piece::new_white(PieceType::Bishop), 0, 2);
        board.new_piece(Piece::new_white(PieceType::Bishop), 0, 5);
        board.new_piece(Piece::new_white(PieceType::Queen), 0, 3);
        board.new_piece(Piece::new_white(PieceType::King), 0, 4);
        
        //Set up black pieces
        for file in 0..8{
            board.new_piece(Piece::new_black(PieceType::Pawn), 6, file);
        }
        board.new_piece(Piece::new_black(PieceType::Rook), 7, 0);
        board.new_piece(Piece::new_black(PieceType::Rook), 7, 7);
        board.new_piece(Piece::new_black(PieceType::Knight), 7, 1);
        board.new_piece(Piece::new_black(PieceType::Knight), 7, 6);
        board.new_piece(Piece::new_black(PieceType::Bishop), 7, 2);
        board.new_piece(Piece::new_black(PieceType::Bishop), 7, 5);
        board.new_piece(Piece::new_black(PieceType::Queen), 7, 3);
        board.new_piece(Piece::new_black(PieceType::King), 7, 4);


  
        board
    }

    pub fn display(&mut self){

        const WINDOW_SIZE: u32 = 1000;
        const SQUARE_SIZE: u32 = WINDOW_SIZE / BOARD_SIZE;
        const BOARD_SIZE: u32 = 8;
        const PIECE_SIZE: u32 = SQUARE_SIZE *  9 / 10 ;

        let mut _moves: Vec<(usize, usize)> = Vec::new();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
        .expect("Couldn't initialize image context");
        let window = video_subsystem
            .window("Chess", WINDOW_SIZE, WINDOW_SIZE)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        // canvas.clear();
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
                        let file = x as u32 / SQUARE_SIZE;
                        let rank = 7 - (y as u32 / SQUARE_SIZE);



                        if let Some((selected_rank, selected_file)) = self.selected_piece {
                            if let Some(piece) = self.squares[selected_rank][selected_file] {
                                if piece.p_color != self.turn {
                                    continue;
                                }
                            }
                            //check if the move is valid
                            if (rank as usize, file as usize) != (selected_rank, selected_file) && _moves.contains(&(rank as usize, file as usize)) {
                                // Move the piece to the new square
                                let piece_to_move = self.squares[selected_rank][selected_file].take();
                                self.squares[rank as usize][file as usize] = piece_to_move;

                                //clear the moves
                                _moves.clear();

                                // Deselect the piece
                                self.selected_piece = None;

                                //Change the turn
                                self.turn = match self.turn {
                                    PColor::White => PColor::Black,
                                    PColor::Black => PColor::White,
                                };
                                
                            } else {

                                // Deselect the piece if the same square is clicked and clear the moves
                                self.selected_piece = None;
                                _moves.clear();
                            }
                        } else {
                            //check if the clicked piece is the same color as the turn
                            if let Some(piece) = self.squares[rank as usize][file as usize] {
                                if piece.p_color != self.turn {
                                    continue;
                                }
                            }
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

                        let piece_rect = sdl2::rect::Rect::new(
                            (x + (SQUARE_SIZE - PIECE_SIZE) / 2) as i32,
                            (y + (SQUARE_SIZE - PIECE_SIZE) / 2) as i32,
                            PIECE_SIZE,
                            PIECE_SIZE,
                        );
            
                        let texture_string = format!("textures/{}-{}.svg",format!("{:?}", piece.p_type).split("::").last().unwrap().to_lowercase(),format!("{:?}", piece.p_color).split("::").last().unwrap().to_lowercase());
                        let texture_creator = canvas.texture_creator();
                        let texture = texture_creator.load_texture(texture_string).unwrap();
                    
                        canvas.copy(&texture, None, piece_rect)
                        .expect("Failed to copy image.");
                    }
                }
            }
            
            canvas.present();
        }
    }

    pub fn new_piece(&mut self, piece: Piece, rank: usize, file: usize){
        self.squares[rank][file] = Some(piece);
    }
}
                   