use std::io::{self, Write};

struct TicTacToe {
    board: Vec<Vec<String>>,
    player_x: bool,
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            board: vec![
                vec![" ".to_string(), " ".to_string(), " ".to_string()],
                vec![" ".to_string(), " ".to_string(), " ".to_string()],
                vec![" ".to_string(), " ".to_string(), " ".to_string()],
            ],
            player_x: true,
        }
    }

    fn get_x_or_o(&self) -> String {
        if self.player_x {
            "X".to_string()
        } else {
            "O".to_string()
        }
    }

    fn print_board(&self) {
        println!(" c: 0 1 2");
        println!(
            "r0: {}|{}|{}",
            self.board[0][0], self.board[0][1], self.board[0][2]
        );
        println!("    -+-+-");
        println!(
            "R1: {}|{}|{}",
            self.board[1][0], self.board[1][1], self.board[1][2]
        );
        println!("    -+-+-");
        println!(
            "R2: {}|{}|{}",
            self.board[2][0], self.board[2][1], self.board[2][2]
        );
        println!("");
    }

    fn make_move(&mut self, row: usize, col: usize) -> Result<(), String> {
        if row >= 3 || col >= 3 {
            return Err("Invalid move! Row and column should be between 0 and 2.".to_string());
        }
        if self.board[row][col] != " ".to_string() {
            return Err("Invalid move! Cell already occupied.".to_string());
        }
        self.board[row][col] = self.get_x_or_o();

        self.player_x = !self.player_x;

        Ok(())
    }

    fn check_winner(&self) -> String {
        for i in 0..3 {
            // Check rows
            if self.board[i][0] == self.board[i][1]
                && self.board[i][1] == self.board[i][2]
                && self.board[i][0] != "".to_string()
            {
                return self.board[i][0].clone();
            }

            // Check columns
            if self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
                && self.board[0][i] != "".to_string()
            {
                return self.board[0][i].clone();
            }
        }

        // Check diagonals
        if self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
            && self.board[0][0] != "".to_string()
        {
            return self.board[0][0].clone();
        }
        if self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
            && self.board[0][2] != "".to_string()
        {
            return self.board[0][2].clone();
        }

        " ".to_string()
    }
}

fn main() {
    let mut game = TicTacToe::new();

    loop {
        println!("\nCurrent player: {:?}", game.player_x);
        game.print_board();

        print!("Enter row (0-2): ");
        io::stdout().flush().unwrap();
        let mut row_input = String::new();
        io::stdin()
            .read_line(&mut row_input)
            .expect("Failed to read row input.");
        let row: usize = row_input.trim().parse().unwrap();

        print!("Enter column (0-2): ");
        io::stdout().flush().unwrap();
        let mut col_input = String::new();
        io::stdin()
            .read_line(&mut col_input)
            .expect("Failed to read column input.");
        let col: usize = col_input.trim().parse().unwrap();

        if let Err(msg) = game.make_move(row, col) {
            println!("{}", msg);
            continue;
        }

        let winner = game.check_winner();
        if winner != " ".to_string() {
            game.print_board();
            println!("\nPlayer {:?} wins!", winner);
            break;
        }

        let board_full = game
            .board
            .iter()
            .all(|row| row.iter().all(|cell| *cell != " ".to_string()));

        if board_full {
            game.print_board();
            println!("\nIt's a draw!");
            break;
        }
    }
}
