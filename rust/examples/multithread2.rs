use std::fs::File;
use std::time::Instant;
use memmap::MmapOptions;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // Use memory mapping to create a view into the file.
    let file = File::open("../resources/pi-billion.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let sequence_counts = Arc::new((0..200_000_000).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>().into_boxed_slice());

    // Choose a chunk size. This can be tuned for performance.
    let mb = 1024 * 1024;
    let chunk_size = 16 * mb;
    let overlap = 7;  // size of sliding window minus one

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

        for (i, &byte) in chunk.iter().enumerate() {
            // Shift existing digits and add the new one
            number = number * 10 + (byte - b'0') as usize;

            if i >= 8 {
                // Once we have 8 digits, remove the leftmost digit from the previous number
                let leftmost_digit = (chunk[i - 8] - b'0') as usize;
                number -= leftmost_digit * power_of_ten;
            }

            // Check if number is a valid date and update count
            if i >= 7 && number >= 1900_0000 && number < 2100_0000 {
                let index = number - 1900_0000;
                sequence_counts[index].fetch_add(1, Ordering::Relaxed);
            }
        }
    });

    let duration = start.elapsed();
    println!("Finished in {} ms", duration.as_millis());
    Ok(())
}
