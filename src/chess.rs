extern crate sdl2;

#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    pub p_type: PieceType,
    pub p_color: PColor,
}

impl Piece {
    fn new_white(p_type: PieceType) -> Piece {
        Piece {
            p_type: (p_type),
            p_color: (PColor::White),
        }
    }

    fn new_black(p_type: PieceType) -> Piece {
        Piece {
            p_type: (p_type),
            p_color: (PColor::Black),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PColor {
    White,
    Black,
}

pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub selected_piece: Option<(usize, usize)>,
    pub turn: PColor,
    pub king_under_attack: [bool; 2],
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
impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            squares: [[None; 8]; 8],
            selected_piece: None,
            turn: PColor::White,
            king_under_attack: [false; 2],
        };

        //Starting position
        //Set up white pieces

        for file in 0..8 {
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
        for file in 0..8 {
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

    pub fn move_generation(&mut self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        match self.selected_piece {
            Some((rank, file)) => {
                let piece = self.squares[rank][file].unwrap();
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
                            if self.squares[new_rank as usize][file].is_none() {
                                moves.push((new_rank as usize, file));
                            }
                            // Move forward two squares
                            if (rank == 1 && direction == 1) || (rank == 6 && direction == -1) {
                                let new_rank = rank as i32 + 2 * direction;
                                if self.squares[new_rank as usize][file].is_none()
                                    && self.squares[(new_rank - direction) as usize][file].is_none()
                                {
                                    moves.push((new_rank as usize, file));
                                }
                            }
                            // Capture diagonally to the left
                            if file > 0 {
                                if let Some(captured_piece) =
                                    self.squares[new_rank as usize][file - 1]
                                {
                                    if captured_piece.p_color != piece.p_color {
                                        moves.push((new_rank as usize, file - 1));
                                    }
                                }
                            }
                            // Capture diagonally to the right
                            if file < 7 {
                                if let Some(captured_piece) =
                                    self.squares[new_rank as usize][file + 1]
                                {
                                    if captured_piece.p_color != piece.p_color {
                                        moves.push((new_rank as usize, file + 1));
                                    }
                                }
                            }

                            //en passant for black and white
                            if (rank == 4 && direction == 1) || (rank == 3 && direction == -1) {
                                if file > 0 {
                                    if let Some(captured_piece) = self.squares[rank][file - 1] {
                                        if captured_piece.p_color != piece.p_color
                                            && captured_piece.p_type == PieceType::Pawn
                                        {
                                            moves.push((new_rank as usize, file - 1));
                                        }
                                    }
                                }
                                if file < 7 {
                                    if let Some(captured_piece) = self.squares[rank][file + 1] {
                                        if captured_piece.p_color != piece.p_color
                                            && captured_piece.p_type == PieceType::Pawn
                                        {
                                            moves.push((new_rank as usize, file + 1));
                                        }
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
                                if self.squares[new_rank as usize][new_file as usize].is_none() {
                                    moves.push((new_rank as usize, new_file as usize));
                                } else {
                                    if let Some(captured_piece) =
                                        self.squares[new_rank as usize][new_file as usize]
                                    {
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
                        for direction in &[
                            (1, 2),
                            (1, -2),
                            (-1, 2),
                            (-1, -2),
                            (2, 1),
                            (2, -1),
                            (-2, 1),
                            (-2, -1),
                        ] {
                            let new_rank = rank as i32 + direction.0;
                            let new_file = file as i32 + direction.1;
                            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                                if self.squares[new_rank as usize][new_file as usize].is_none() {
                                    moves.push((new_rank as usize, new_file as usize));
                                } else {
                                    if let Some(captured_piece) =
                                        self.squares[new_rank as usize][new_file as usize]
                                    {
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
                        for direction in &[
                            (1, 0),
                            (-1, 0),
                            (0, 1),
                            (0, -1),
                            (1, 1),
                            (1, -1),
                            (-1, 1),
                            (-1, -1),
                        ] {
                            let new_rank = rank as i32 + direction.0;
                            let new_file = file as i32 + direction.1;
                            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                                if self.squares[new_rank as usize][new_file as usize].is_none() {
                                    moves.push((new_rank as usize, new_file as usize));
                                } else {
                                    if let Some(captured_piece) =
                                        self.squares[new_rank as usize][new_file as usize]
                                    {
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
                                if self.squares[new_rank as usize][new_file as usize].is_none() {
                                    moves.push((new_rank as usize, new_file as usize));
                                } else {
                                    if let Some(captured_piece) =
                                        self.squares[new_rank as usize][new_file as usize]
                                    {
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
                        for direction in &[
                            (1, 0),
                            (-1, 0),
                            (0, 1),
                            (0, -1),
                            (1, 1),
                            (1, -1),
                            (-1, 1),
                            (-1, -1),
                        ] {
                            let mut new_rank = rank as i32 + direction.0;
                            let mut new_file = file as i32 + direction.1;
                            while new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                                if self.squares[new_rank as usize][new_file as usize].is_none() {
                                    moves.push((new_rank as usize, new_file as usize));
                                } else {
                                    if let Some(captured_piece) =
                                        self.squares[new_rank as usize][new_file as usize]
                                    {
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
            }
            None => {
                return moves;
            }
        }

        moves
    }

    pub fn new_piece(&mut self, piece: Piece, rank: usize, file: usize) {
        self.squares[rank][file] = Some(piece);
    }

    pub fn make_move(&mut self, rank: usize, file: usize) {
        if let Some((selected_rank, selected_file)) = self.selected_piece {
            let piece_to_move = self.squares[selected_rank][selected_file].take();
            
            self.squares[rank as usize][file as usize] = piece_to_move;

            self.selected_piece = Some((rank as usize, file as usize));

            // Deselect the piece
            self.selected_piece = None;

            //Change the turn
            self.turn = match self.turn {
                PColor::White => PColor::Black,
                PColor::Black => PColor::White,
            };
        }
    }

    fn copy(&mut self) -> Board {
        let mut board_copy = Board::new();
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.squares[rank][file] {
                    board_copy.new_piece(piece, rank, file);
                } else {
                    board_copy.squares[rank][file] = None;
                }
            }
        }

        board_copy.selected_piece = self.selected_piece;
        board_copy.turn = self.turn;

        board_copy
    }

    pub fn legal_moves(&mut self) -> Vec<(usize, usize)> {
        //generate moves
        let moves = self.move_generation();
        let mut legalmoves = Vec::new();
        //perform the move on a copy of the board
        for (rank, file) in moves {
            let mut board_copy = self.copy();
            board_copy.make_move(rank, file);

            if !board_copy.in_check() {
                legalmoves.push((rank, file));
            }
        }
        legalmoves
    }

    pub fn find(&mut self, piece_to_find: Piece) -> Option<(usize, usize)> {
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.squares[rank][file] {
                    if piece == piece_to_find {
                        return Some((rank, file));
                    }
                }
            }
        }
        None
    }

    pub fn in_check(&mut self) -> bool {
        let color = match self.turn {
            PColor::White => PColor::Black,
            PColor::Black => PColor::White,
        };

        //check if the king is under attack by any piece
        if let Some(king_position) = self.find(Piece {
            p_type: (PieceType::King),
            p_color: (color),
        }) {
            for rank in 0..8 {
                for file in 0..8 {
                    if let Some(piece) = self.squares[rank][file] {
                        if piece.p_color != color {
                            self.selected_piece = Some((rank, file));
                            let moves = self.move_generation();
                            if moves.contains(&king_position) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }
}
