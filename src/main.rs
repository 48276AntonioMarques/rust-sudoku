use game::board::Board;

mod game;

mod exceptions;

fn main() {
    // Generate/Fetch board
    println!("\x1B[2J");
    println!("Generating board...");
    let board = match game::board::Board::new(
        [
            0, 0, 9, /* */ 0, 0, 0, /* */ 0, 0, 0,
            0, 0, 0, /* */ 0, 0, 0, /* */ 0, 0, 0,
            0, 0, 0, /* */ 0, 0, 0, /* */ 0, 0, 0,
            /*--------------------------------- */
            0, 0, 0, /* */ 0, 0, 0, /* */ 0, 0, 0,
            0, 0, 0, /* */ 0, 0, 0, /* */ 0, 0, 0,
            0, 0, 0, /* */ 0, 0, 0, /* */ 0, 0, 0,
            /*--------------------------------- */
            0, 0, 0, /* */ 0, 0, 0, /* */ 0, 0, 0,
            0, 0, 0, /* */ 0, 0, 0, /* */ 0, 0, 0,
            0, 0, 0, /* */ 0, 0, 0, /* */ 0, 0, 0,
            ]
    ) {
        Err(exception) => {
            // Say that there was an error an halt
            panic!("Error!");
        },
        Ok(board) => board
    };
    println!("\x1B[2J");
    println!("Board generated!");
    // TODO: Create a TUI that handles UI Management
    // Print board
    let line = "-------------------------------";
    println!("{}", line);
    for i in 0..9 {
        print!("|");
        let row = match Board::get_row(&board, i) {
            Ok(row) => row,
            _ => panic!("Invalid Row")
        };
        for j in 0..9 {
            let pos = row.content[j];
            let mut m_pos = pos & 0x03FE;
            let mut number = 0;
            loop {
                m_pos >>= 1;
                if m_pos <= 0 { break; }
                number += 1;
            }
            print!(" {} ", number);
            if ((j + 1) % 3) == 0 {
                print!("|");
            }
        }
        println!("");
        if ((i + 1) % 3) == 0 {
            println!("{}", line);
        }

    }
    // Solve board
}
