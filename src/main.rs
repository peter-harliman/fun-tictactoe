
use std::io;

// Main function
fn main() {
    println!("Enter");

    let mut my_box = [0; 9];
    init_box(&mut my_box);
    draw_box(my_box);

    let mut turn = -1;
    loop {
        println!("Turn is {}", turn);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: usize = guess.trim()
            .parse()
            .expect("Please type a number!");

        // update box
        if my_box[guess] == 0 {
            my_box[guess] = turn;
        } else {
            println!("Box {} is already filled!", guess);
            continue;
        }

        draw_box(my_box);

        // check winner
        if check_win(my_box, turn) == 1 {
            println!("Player {} wins!", turn);
            break;
        }

        if turn == -1 {
            turn = 1;
        } else {
            turn = -1;
        }
    }

    println!("Exit");
}

// Initialize box
fn init_box(input_box: &mut [i32; 9]) {
    println!("Initializing box");
    for i in 0..9 {
        input_box[i] = 0;
    }
    println!("Finished initializing");
}

// Draw box
fn draw_box(input_box: [i32; 9]) {
    println!("Draw box");
    for i in 0..3 {
        for j in 0..3 {
            match input_box[3 * i + j] {
                0 => print!("| "),
                1 => print!("|O"),
                -1 => print!("|X"),
                _ => print!("|?"),
            }
        }
        println!("|");
    }
}

fn check_win(input_box: [i32; 9], turn: i32) -> i32 {
    let mut win;

    for i in 0..3 {
        win = 1;
        for j in 0..3 {
            if input_box[3 * i + j] != turn {
                win = 0;
                break;
            }
        }
        if win == 1 {
            return win;
        }
    }

    for j in 0..3 {
        win = 1;
        for i in 0..3 {
            if input_box[3 * i + j] != turn {
                win = 0;
                break;
            }
        }
        if win == 1 {
            return win;
        }
    }

    // return
    0
}
