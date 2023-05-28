use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("../../resources/pi_dec_1m.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        for char in line.chars() {
            if char.is_digit(10) {
                println!("{}", char);
            }
        }
    }
    Ok(())
}
