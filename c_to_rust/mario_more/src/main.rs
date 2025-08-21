use my_library::get_input;
const BLOCK: char = '#';
const SPACE: char = ' ';
const GAP_WIDTH: u8 = 2;
const MIN_HEIGHT: u8 = 1;
const MAX_HEIGHT: u8 = 8;

fn main() {
    let mut height: u8;
    loop {
        height = get_input("Pyramid Height: ");
        match height{
            _ if height > MAX_HEIGHT => println!("Pyramid too big! ({height}). Max height: 8"),
            _ if height < MIN_HEIGHT => println!("So, no pyramid?"),
            _ => break
        }
    }
    print_pyramid(height)
}

fn print_pyramid(height: u8) {
    for step in (0..height).rev() {
        // First half
        for i in 0..height {
            if i < step {
                print!("{SPACE}")
            } else {
                print!("{BLOCK}")
            }
        }
        // Gap
        for _ in 0..GAP_WIDTH {
            print!("{SPACE}")
        }
        // Second half
        for _ in 0..height - step {
            print!("{BLOCK}")
        }
        println!("")
    }
}
