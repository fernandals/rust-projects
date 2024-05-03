use std::io;

fn print_board(board: [[char; 3]; 3]) {
    let sep = "-".repeat(11);
    let mut r = 0;
    let mut c : u32;

    for row in board {
        r += 1;
        c = 0;
        for e in row {
            c += 1;
            
            if c <= 2 {
                print!(" {e} |");
            } else {
                println!(" {e}")
            }
        }
        
        if r <= 2 {
            println!("{}", sep);
        }
    }
}

fn convert_idx(i: u32) -> (usize, usize) {
    let p = if (i % 3) == 0 { (i/3 - 1, 2) }
            else { (i/3, (i%3 - 1)) };
    return (p.0 as usize, p.1 as usize);
}

fn make_play(play: char, idx: (usize, usize), board: &mut [[char; 3]; 3]) -> bool {
    if board[idx.0][idx.1] != ' ' {
        return false;
    } else {
        board[idx.0][idx.1] = play;
        return true;
    }
}

fn win_check(board: [[char; 3]; 3]) -> bool {
    let mut prev : char; 
    let mut count : u32;
    
    // horizontal victory
    for row in board {
        prev = row[0];
        count = 0;
        for e in row {
            if (prev == e) & (prev != ' ') {
                count += 1;
                prev = e;
            } else {
                break; // next row
            }
        }
        
        if count == 3 {
            return true;
        }
    }

    // vertical victory
    for col in 0..3 {
        prev = board[0][col];
        count = 0;
        for row in 0..3 {
            if (prev == board[row][col]) & (prev != ' ') {
                count += 1;
                prev = board[row][col];
            } else {
                break; // next column
            }
        }

        if count == 3 {
            return true;
        }
    }

    // diagonal victory
    if (board[0][0] == board[1][1]) & 
        (board[1][1] == board[2][2]) &
         (board[0][0] != ' ') {
        return true;
    } else if (board[2][0] == board[1][1]) &
        (board[1][1] == board[0][2]) &
        (board[2][0] != ' '){
        return true;
    }

    return false;   
}

fn main() {
    println!("Welcome to Tic-tac-toe.");
    
    let mut board : [[char; 3]; 3] = [[' ', ' ', ' '],
                                      [' ', ' ', ' '],
                                      [' ', ' ', ' ']];
    let mut player : char = 'O';
    let mut c = 0;

    loop {
        print_board(board);

        if win_check(board) {
            println!("Congrats {player}, you won!");
            break;
        }

        if c == 9 {
            println!("Draw!");
            break;
        }

        match player {
            'X' => player = 'O',
            _ => player = 'X',
        };

        println!("It's {player} turn. Select a position on the board: ");
        
        // reading input
        let mut play = String::new();
        io::stdin().read_line(&mut play)
            .expect("Failed to read line...");
        
        // transforming input
        let play : u32 = match play.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Please type a number!");
                        continue},
        };

        if (play < 1) | (play > 9) {
            println!("Please type a number between 1 and 9.");
            continue;
        }
        
        let idx = convert_idx(play);

        if !make_play(player, idx, &mut board) {
            println!("Please type a valid position of the board.");
            continue;
        }

        c += 1; 
    }
}
