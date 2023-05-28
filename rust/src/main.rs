use memmap::MmapOptions;
use rayon::prelude::*;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // Use memory mapping to create a view into the file.
    let file = File::open("../resources/pi-billion.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    // The buffer size determines the size of the chunks.
    let buffer_size = 10 * 1024 * 1024; // 10 MB
    let sequence_counts = Arc::new(Mutex::new(vec![0; 200_000_000]));

    let num_chunks = (mmap.len() + buffer_size - 1) / buffer_size;

    // Process each chunk in parallel
    (0..num_chunks)
        .into_par_iter()
        .map(|chunk_index| {
            let start = chunk_index * buffer_size;
            let end = std::cmp::min((chunk_index + 1) * buffer_size + 7, mmap.len());
            start..end
        })
        .for_each(|range| {
            let chunk = &mmap[range];
            let mut local_sequence_counts = vec![0; 200_000_000];

            chunk
                .windows(8)
                // convert to u8 since we know it's just ascii numbers
                .map(|bytes| {
                    let mut number = 0;
                    for &byte in bytes {
                        number = number * 10 + (byte - b'0') as usize;
                    }
                    number
                })
                .filter(|date| (1900_00_00..2100_00_00).contains(date))
                .for_each(|date| {
                    let index = date - 1900_0000;
                    local_sequence_counts[index as usize] += 1;
                });

            // Merge the local counts into the global counts
            let mut global_counts = sequence_counts.lock().unwrap();
            for i in 0..200_000_000 {
                global_counts[i] += local_sequence_counts[i];
            }
        });

    let duration = start.elapsed();
    println!("Finished in {} ms", duration.as_millis());
    let count = sequence_counts.lock().unwrap().iter().sum::<usize>();
    println!("Found {} dates", count);
    Ok(())
}
