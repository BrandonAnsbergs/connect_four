use std::io;

// Constants for board dimensions
const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

// ANSI escape codes for terminal colors
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[;31m";
const GREEN: &str = "\x1b[32m";

// Type alias for the game board
type Board = [[u8; BOARD_WIDTH]; BOARD_HEIGHT];

// Enum representing players
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum Player {
    One = 1,
    Two = 2,
    None = 0,
}

impl Player {
    // Convert an integer to a Player
    fn from_int(int: u8) -> Player {
        match int {
            1 => Player::One,
            2 => Player::Two,
            _ => Player::None,
        }
    }
}

// Enum for possible move errors
#[derive(Debug)]
enum MoveError {
    GameFinished,
    InvalidColumn,
    ColumnFull,
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::ColumnFull => write!(f, "Column is full"),
            MoveError::InvalidColumn => write!(f, "Column must be between 1 and 7"),
            MoveError::GameFinished => write!(f, "Game is already finished "),
        }
    }
}

// Struct representing the game state
struct Game {
    current_move: u8,
    current_player: Player,
    board: Board,
    is_finished: bool,
    winner: Player,
}

impl Game {
    // Initialize a new game with default values
    fn default() -> Game {
        Game {
            current_move: 0,
            current_player: Player::One,
            board: [
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ],
            is_finished: false,
            winner: Player::None,
        }
    }

    // Clear the terminal screen
    fn clear_screen(&self) {
        // Sends the ANSI escape code to clear the screen
        print!("{}[27", 27 as char);
    }

    // Display the game board
    fn display_board(&self) {
        // Clears the screen before displaying the board
        self.clear_screen();
        println!("\n");
        println!("{}--------------------{}", GREEN, RESET);
        println!("{}CONNECT 4 (Move {}){}", GREEN, self.current_move, RESET);
        println!("{}--------------------{}", GREEN, RESET);
        
        // Iterate over each row in the board and print it
        for row in self.board {
            let row_str: String = row.iter()
                .map(|&cell| match cell {
                    1 => "🔴", // Red for Player One
                    2 => "🔵", // Blue for Player Two
                    _ => "⚫", // Black for empty cell
                })
                .collect::<Vec<&str>>()
                .join(" ");
            println!("{}", row_str);
        }
        println!("{}--------------------{}", GREEN, RESET);

        // Display the winner if the game is finished
        if self.is_finished {
            match self.winner {
                Player::One => println!("{} 🔴 Player 1 has won!{}", GREEN, RESET),
                Player::Two => println!("{} 🔵 Player 2 has won!{}", GREEN, RESET),
                Player::None => println!("{} It's a draw!{}", GREEN, RESET),
            }
        }
        println!("{}--------------------{}", GREEN, RESET);
    }

    // Display an error message
    fn display_error(&self, error: String) {
        // Display the board and then the error message
        self.display_board();
        println!("{}Error: {}{}", RED, error, RESET);
    }

    // Calculate the winner of the game
    fn calculate_winner(&mut self) -> Player {
        // Early return if not enough moves have been made to win
        if self.current_move < 7 {
            return Player::None;
        }

        // Directions to check for a connect four
        let directions = [
            (0, 1),  // horizontal
            (1, 0),  // vertical
            (1, 1),  // diagonal (top-left to bottom-right)
            (-1, 1), // diagonal (bottom-left to top-right)
        ];

        // Iterate over each cell in the board
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let cell = self.board[row][col];

                // Skip empty cells
                if cell != 0 {
                    // Check each direction
                    for (row_step, col_step) in directions {
                        let mut consecutive_count = 1;
                        let mut r = row as isize + row_step;
                        let mut c = col as isize + col_step;

                        // Check for consecutive cells in the current direction
                        while r >= 0
                            && r < BOARD_HEIGHT as isize
                            && c >= 0
                            && c < BOARD_WIDTH as isize
                        {
                            if self.board[r as usize][c as usize] == cell {
                                consecutive_count += 1;

                                // If four consecutive cells are found, the current player wins
                                if consecutive_count == 4 {
                                    self.is_finished = true;
                                    return Player::from_int(cell);
                                }
                            } else {
                                break;
                            }

                            r += row_step;
                            c += col_step;
                        }
                    }
                }
            }
        }
        
        // Check for a draw (board is full)
        if self.current_move >= BOARD_HEIGHT as u8 * BOARD_WIDTH as u8 {
            self.is_finished = true;
        }

        Player::None
    }

    // Play a move in the specified column
    fn play_move(&mut self, column: usize) -> Result<(), MoveError> {
        // Check if the game is already finished
        if self.is_finished {
            return Err(MoveError::GameFinished);
        }

        // Check if the column is valid
        if column >= BOARD_WIDTH {
            return Err(MoveError::InvalidColumn);
        }

        // Find the first empty cell in the column
        if let Some(row) = (0..BOARD_HEIGHT).rev().find(|&row| self.board[row][column] == 0) {
            // Place the current player's piece in the cell
            self.board[row][column] = self.current_player as u8;
            self.current_move += 1;
        } else {
            // If the column is full, return an error
            return Err(MoveError::ColumnFull);
        }

        // Calculate the winner after the move
        let calculated_winner = self.calculate_winner();

        if calculated_winner != Player::None {
            // If there's a winner, update the winner field
            self.winner = calculated_winner;
        } else {
            // Otherwise, switch to the other player
            self.current_player = match self.current_player {
                Player::One => Player::Two,
                _ => Player::One,
            };
        }

        Ok(())
    }
}

fn main() {
    let mut game = Game::default();

    game.display_board();

    loop {
        while !game.is_finished {
            println!("\n");

            // Display the current player's turn
            match game.current_player {
                Player::One => println!("Player 1"),
                Player::Two => println!("Player 2"),
                _ => (),
            }

            println!("Enter a column between 1 and 7:");

            let mut user_move = String::new();

            // Read user input
            io::stdin().read_line(&mut user_move).expect("Failed to read line");
            
            // Parse the user input
            let user_move: usize = match user_move.trim().parse() {
                Ok(num) => {
                    if num < 1 || num > 7 {
                        game.display_error(MoveError::InvalidColumn.to_string());
                        continue;
                    } else {
                        num
                    }
                }
                Err(err) => {
                    game.display_error(err.to_string());
                    continue;
                }
            };

            // Attempt to play the move
            match game.play_move(user_move - 1) {
                Ok(_) => {
                    game.display_board();
                }
                Err(err) => {
                    game.display_error(err.to_string());
                }
            }
        }
        
        println!("Press 'R' to restart or 'Q' to quit the game.");

        let mut user_input = String::new();

        // Read user input to restart or quit
        io::stdin().read_line(&mut user_input).expect("failed to read line");

        // Handle the user input
        match user_input.trim() {
            "R" | "r" => {
                game = Game::default();
                game.display_board();
            }
            "Q" | "q" => {
                println!("Quitting...");
                break;
            }
            _ => game.display_error("Invalid input".to_string()),
        }
    }
}
