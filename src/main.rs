pub mod chess;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::chess::{PColor , PieceType};

fn main() {
    let mut board = chess::Board::new();

    const WINDOW_SIZE: u32 = 1000;
    const SQUARE_SIZE: u32 = WINDOW_SIZE / BOARD_SIZE;
    const BOARD_SIZE: u32 = 8;
    const PIECE_SIZE: u32 = SQUARE_SIZE * 9 / 10;

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
                Event::MouseButtonDown { x, y, .. } => {
                    // Get the rank and file of the square that was clicked
                    let file = x as u32 / SQUARE_SIZE;
                    let rank = 7 - (y as u32 / SQUARE_SIZE);

                    if let Some((selected_rank, selected_file)) = board.selected_piece {
                        if let Some(piece) = board.squares[selected_rank][selected_file] {
                            if piece.p_color != board.turn {
                                continue;
                            }
                        }
                        //check if the move is valid
                        if (rank as usize, file as usize) != (selected_rank, selected_file)
                            && _moves.contains(&(rank as usize, file as usize))
                        {
                            // Move the piece to the new square
                            let piece_to_move = board.squares[selected_rank][selected_file].take();
                            //check for en passant
                            if piece_to_move.unwrap().p_type == PieceType::Pawn
                                && selected_file != file as usize
                                && board.squares[rank as usize][file as usize].is_none()
                            {
                                if board.turn == PColor::White {
                                    board.squares[rank as usize - 1][file as usize] = None;
                                } else {
                                    board.squares[rank as usize + 1][file as usize] = None;
                                }
                            }
                            board.squares[rank as usize][file as usize] = piece_to_move;

                            board.selected_piece = Some((rank as usize, file as usize));

                            //clear the moves
                            _moves.clear();

                            // Deselect the piece
                            board.selected_piece = None;

                            //Change the turn
                            board.turn = match board.turn {
                                PColor::White => PColor::Black,
                                PColor::Black => PColor::White,
                            };
                        } else {
                            // Deselect the piece if the same square is clicked and clear the moves
                            board.selected_piece = None;
                            _moves.clear();
                        }
                    } else {
                        //check if the clicked piece is the same color as the turn
                        if let Some(piece) = board.squares[rank as usize][file as usize] {
                            if piece.p_color != board.turn {
                                continue;
                            }
                        }
                        // Select the square if no piece is currently selected
                        board.selected_piece = Some((rank as usize, file as usize));
                        _moves = board.legal_moves();
                    }
                }
                _ => {}
            }
        }

        // loop to iterate through the 2d array
        for rank in 0..BOARD_SIZE {
            for file in 0..BOARD_SIZE {
                let x = file * SQUARE_SIZE;
                let y = (BOARD_SIZE - rank - 1) * SQUARE_SIZE;

                let square = sdl2::rect::Rect::new(x as i32, y as i32, SQUARE_SIZE, SQUARE_SIZE);

                let color = if board.selected_piece == Some((rank as usize, file as usize)) {
                    sdl2::pixels::Color::RGB(172, 172, 172) // Light Square
                } else if _moves.contains(&(rank as usize, file as usize)) {
                    sdl2::pixels::Color::RGB(172, 172, 172) // Light blue for selected
                } else if (rank + file) % 2 == 0 {
                    sdl2::pixels::Color::RGB(101, 97, 92) // dark blue for valid moves
                } else {
                    sdl2::pixels::Color::RGB(198, 193, 170) // Dark square
                };

                canvas.set_draw_color(color);
                canvas.fill_rect(square).unwrap();

                if let Some(piece) = board.squares[rank as usize][file as usize] {
                    let piece_rect = sdl2::rect::Rect::new(
                        (x + (SQUARE_SIZE - PIECE_SIZE) / 2) as i32,
                        (y + (SQUARE_SIZE - PIECE_SIZE) / 2) as i32,
                        PIECE_SIZE,
                        PIECE_SIZE,
                    );

                    let texture_string = format!(
                        "textures/{}-{}.png",
                        format!("{:?}", piece.p_type)
                            .split("::")
                            .last()
                            .unwrap()
                            .to_lowercase(),
                        format!("{:?}", piece.p_color)
                            .split("::")
                            .last()
                            .unwrap()
                            .to_lowercase()
                    );
                    let texture_creator = canvas.texture_creator();
                    let texture = texture_creator.load_texture(texture_string).unwrap();

                    canvas
                        .copy(&texture, None, piece_rect)
                        .expect("Failed to copy image.");
                }
            }
        }

        canvas.present();
    }
}
