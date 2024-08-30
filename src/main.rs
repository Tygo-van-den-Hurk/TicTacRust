/*
//  Tic Tac Toe game in Rust
//  The game is played on a 3x3 board
//  The players take turns to place their symbol (X or O) on the board
//  The player who places their symbol in a row, column, or diagonal wins the game
//  The game ends in a draw if the board is full and there is no winner
*/

use std::io::{ self, Write };

/*
// derive is used to automatically implement some traits for the enum

> Debug is used to print the enum in a debug format
> Clone is used to make a deep copy of the enum
> Copy is used to make a shallow copy of the enum
> PartialEq is used to implement the equality operator for the enum

// this allows us to compare two instances of the enum using the == operator
*/

//  player enum to represent the two players in the game
#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    PlayerX,
    PlayerO,
}

//  cell enum to represent the state of a cell on the board
#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Occupied(Player),
}

//  player implementation to display the player's name instead of the enum variant
//  so it displays
//      "Player 1 (X)" instead of "PlayerX"
//  and
//      "Player 2 (O)" instead of "PlayerO"
impl Player {
    fn display_name(&self) -> &str {
        match self {
            Player::PlayerX => "Player 1 (X)",
            Player::PlayerO => "Player 2 (O)",
        }
    }
}

//  board struct with a 3x3 grid of cells
struct Board {
    cells: [[Cell; 3]; 3],
}

//  board implementation
impl Board {
    fn new() -> Board {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    //  clears the console
    fn clear(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        io::stdout().flush().unwrap();
    }

    //  prints the board to the console
    fn print_board(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                let symbol = match cell {
                    Cell::Empty => ' ',
                    Cell::Occupied(Player::PlayerX) => 'X',
                    Cell::Occupied(Player::PlayerO) => 'O',
                };
                print!("{} | ", symbol);
            }
            println!("\n---------");
        }
    }

    //  checks if the board is full
    fn is_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }

    //  checks for a winner in rows, columns, and diagonals
    fn check_winner(&self) -> Option<Player> {
        for i in 0..3 {
            if self.cells[i][0] == self.cells[i][1] && self.cells[i][1] == self.cells[i][2] {
                if let Cell::Occupied(player) = self.cells[i][0] {
                    return Some(player);
                }
            }
            if self.cells[0][i] == self.cells[1][i] && self.cells[1][i] == self.cells[2][i] {
                if let Cell::Occupied(player) = self.cells[0][i] {
                    return Some(player);
                }
            }
        }
        if self.cells[0][0] == self.cells[1][1] && self.cells[1][1] == self.cells[2][2] {
            if let Cell::Occupied(player) = self.cells[0][0] {
                return Some(player);
            }
        }
        if self.cells[0][2] == self.cells[1][1] && self.cells[1][1] == self.cells[2][0] {
            if let Cell::Occupied(player) = self.cells[0][2] {
                return Some(player);
            }
        }
        None
    }

    //  makes a move on the board if the cell is empty
    fn make_move(&mut self, row: usize, col: usize, player: Player) -> Result<(), &str> {
        if self.cells[row][col] == Cell::Empty {
            self.cells[row][col] = Cell::Occupied(player);
            Ok(())
        } else {
            Err("cell is already occupied!")
        }
    }
}

fn main() {
    //  initialize the board and the current player
    let mut board = Board::new();
    let mut current_player = Player::PlayerX;

    //  game loop
    loop {
        board.clear();
        board.print_board();

        //  get the player's move
        println!("{}, enter your move (row and column): ", current_player.display_name());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let mut coords = input.trim().split_whitespace();

        let row: usize = match coords.next().and_then(|s| s.parse().ok()) {
            Some(num) if num < 3 => num,
            _ => {
                println!("enter a number between 0 and 2.");
                continue;
            }
        };

        //  check if the input is valid
        let col: usize = match coords.next().and_then(|s| s.parse().ok()) {
            Some(num) if num < 3 => num,
            _ => {
                println!("invalid input. please enter a number between 0 and 2.");
                continue;
            }
        };

        //  make the move
        if let Err(e) = board.make_move(row, col, current_player) {
            println!("{}", e);
            continue;
        }

        //  check for a winner
        if let Some(winner) = board.check_winner() {
            board.print_board();
            println!("{} wins!", winner.display_name());
            break;
        }

        //  check for a draw
        if board.is_full() {
            board.print_board();
            println!("it's a draw!");
            break;
        }

        //  switch whos turn it is
        current_player = match current_player {
            Player::PlayerX => Player::PlayerO,
            Player::PlayerO => Player::PlayerX,
        };
    }
}
