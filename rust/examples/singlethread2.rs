use memmap::MmapOptions;
use std::fs::File;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // Use memory mapping to create a view into the file.
    let file = File::open("../resources/pi-billion.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let mut sequence_counts = vec![0; 200_000_000];
    let mut number = 0;

    // Go through the memory map in sliding windows of 8 bytes
    for window in mmap.windows(8) {
        // Transform the window into a number
        for &byte in window {
            number = number * 10 + (byte - b'0') as usize;
        }

        // Check if number is a valid date and update count
        if number >= 1900_0000 && number < 2100_0000 {
            let index = number - 1900_0000;
            sequence_counts[index] += 1;
        }

        // Shift the number to prepare for the next digit
        number /= 10;
    }

    let duration = start.elapsed();
    println!("Finished in {} ms", duration.as_millis());
    Ok(())
}
