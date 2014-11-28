use std::io;

pub fn main() {
    loop {
        print!("Reading: ");
        let input = io::stdin()
                        .read_line()
                        .ok()
                        .expect("Nothing to read");
        print!("Writing: {}", input)
    }
}
