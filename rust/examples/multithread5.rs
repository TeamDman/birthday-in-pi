use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use memmap::MmapOptions;
use rayon::prelude::*;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // Use memory mapping to create a view into the file.
    let file = File::open("../resources/pi-billion.txt")?;
    // let file = File::open("../resources/pi_dec_1m.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let date_presence = Arc::new(
        (0..200_000_000)
            .map(|_| AtomicBool::new(false))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );

    // Choose a chunk size. This can be tuned for performance.
    let mb = 1024 * 1024;
    let chunk_size = 16 * mb;
    let overlap = 7; // size of sliding window minus one

    // Calculate the number of chunks and create an iterator over them
    let num_chunks = (mmap.len() + chunk_size - 1) / chunk_size;
    (0..num_chunks).into_par_iter().for_each(|i| {
        // Calculate the start and end indices of this chunk
        let start = i * chunk_size;
        let end = std::cmp::min((i + 1) * chunk_size + overlap, mmap.len());

        // Calculate sequence counts for this chunk
        let chunk = &mmap[start..end];
        let mut number = 0;
        let power_of_ten = 10_usize.pow(7);

        for &byte in chunk.iter() {
            // Shift existing digits and add the new one
            if byte != b'.' {
                number -= (number / power_of_ten) * power_of_ten;
                number = number * 10 + (byte - b'0') as usize;
            }

            // Check if number is a valid date and update count
            // This will catch the start of the loop where number starts at zero
            if number >= 1900_0000 && number < 2100_0000 {
                let index = number - 1900_0000;
                date_presence[index].store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
    });

    // count the number of unique dates
    let unique_dates = date_presence
        .iter()
        .filter(|&found| found.load(std::sync::atomic::Ordering::Relaxed))
        .count();
    println!("Number of unique dates: {}", unique_dates);

    let duration = start.elapsed();
    println!("Finished in {} ms", duration.as_millis());
    Ok(())
}
