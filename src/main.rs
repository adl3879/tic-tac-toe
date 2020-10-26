use std::io;

struct Game<'a> {
    board: Vec<&'a str>,
    player: bool,
    win: bool,
    rounds: usize,
}

impl Game<'static> {
    fn new() -> Game<'static> {
        Game {
            board: vec!["0", "1", "2", "3", "4", "5", "6", "7", "8"],
            player: true,
            win: false,
            rounds: 0,
        }
    }

    fn print_board(&self) {
        println!(
            "
                {}  {}  {}
                {}  {}  {}
                {}  {}  {}
            ",
            self.board[0],
            self.board[1],
            self.board[2],
            self.board[3],
            self.board[4],
            self.board[5],
            self.board[6],
            self.board[7],
            self.board[8]
        );
    }

    fn check_win(&mut self) {
        if self.board[0] == "X" && self.board[1] == "X" && self.board[2] == "X" {
            self.win = true;
        }
        if self.board[0] == "O" && self.board[1] == "O" && self.board[2] == "O" {
            self.win = true;
        }
        if self.board[0] == "X" && self.board[3] == "X" && self.board[6] == "X" {
            self.win = true;
        }
        if self.board[0] == "O" && self.board[3] == "O" && self.board[6] == "O" {
            self.win = true;
        }
        if self.board[3] == "X" && self.board[4] == "X" && self.board[5] == "X" {
            self.win = true;
        }
        if self.board[3] == "O" && self.board[4] == "O" && self.board[5] == "O" {
            self.win = true;
        }
        if self.board[6] == "X" && self.board[7] == "X" && self.board[8] == "X" {
            self.win = true;
        }
        if self.board[6] == "O" && self.board[7] == "O" && self.board[8] == "O" {
            self.win = true;
        }
        if self.board[1] == "X" && self.board[4] == "X" && self.board[7] == "X" {
            self.win = true;
        }
        if self.board[1] == "O" && self.board[4] == "O" && self.board[7] == "O" {
            self.win = true;
        }
        if self.board[2] == "X" && self.board[5] == "X" && self.board[8] == "X" {
            self.win = true;
        }
        if self.board[2] == "O" && self.board[5] == "O" && self.board[8] == "O" {
            self.win = true;
        }
        if self.board[0] == "X" && self.board[4] == "X" && self.board[8] == "X" {
            self.win = true;
        }
        if self.board[0] == "O" && self.board[4] == "O" && self.board[8] == "O" {
            self.win = true;
        }
        if self.board[2] == "X" && self.board[4] == "X" && self.board[6] == "X" {
            self.win = true;
        }
        if self.board[2] == "O" && self.board[4] == "O" && self.board[6] == "O" {
            self.win = true;
        }

        if self.win {
            println!("PLAYER {} WINS", if !self.player { "X" } else { "O" });
        }

        if self.rounds == 9 {
            println!("GAME OVER, NO WINNER!!!");
            self.win = true;
        }
    }
}

fn main() {
    let mut game = Game::new(); //new game instance.

    println!("\t\nWELCOME TO X AND O version 1.0");
    println!(
        "
            0   1   2
            3   4   5
            6   7   8
        "
    );

    loop {
        println!("\nPLAYER {} NEXT", if game.player { "X" } else { "O" });

        let mut ans = String::new();

        match io::stdin().read_line(&mut ans) {
            Ok(ans) => ans,
            Err(_) => panic!("error"),
        };

        let parsed_ans: usize = ans.trim().parse::<usize>().unwrap();

        if game.board[parsed_ans] != "X" || game.board[parsed_ans] != "O" {
            if game.player && game.board[parsed_ans] != "O" {
                game.board[parsed_ans] = "X";
            } else if !game.player && game.board[parsed_ans] != "X" {
                game.board[parsed_ans] = "O";
            } else {
                println!("ALREADY PICKED!!!");
            }
        }

        game.rounds = game.rounds + 1;
        game.player = !game.player;
        game.print_board();
        game.check_win();

        if game.win {
            break;
        }
    }
}
