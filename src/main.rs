

// Main function
fn main() {
    println!("Enter");

    let mut my_box = [0; 9];
    init_box(&mut my_box);
    draw_box(my_box);

    let mut turn = 0;
    loop {

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
                _ => print!("|?")
            } 
        }
        println!("|");
    }
}
