use rayon::prelude::*;
use std::collections::HashMap;
use std::{fs::File, io::Result, sync::Mutex, time::Instant};
use memmap::MmapOptions;
use std::sync::Arc;

fn main() -> Result<()> {
    let start = Instant::now();

    // Use memory mapping to create a view into the file.
    let file = File::open("../resources/pi-billion.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    // The buffer size determines the size of the chunks.
    let buffer_size = 10 * 1024 * 1024;  // 10 MB

    // Make the sequence_counts HashMap shared and protected by a mutex.
    let sequence_counts: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let num_chunks = mmap.len() / buffer_size;

    // Create a range of chunks, then use rayon to iterate over the range in parallel.
    (0..num_chunks).into_par_iter().for_each(|chunk_idx| {
        let start = chunk_idx * buffer_size;
        let end = std::cmp::min(start + buffer_size, mmap.len());

        // Each thread gets its own sequence and buffer.
        let mut sequence = String::new();
        let buffer = &mmap[start..end];

        for &byte in buffer {
            let char = byte as char;
            sequence.push(char);
            if sequence.len() == 8 {
                // Check if sequence is a valid date and update count
                if is_valid_date(&sequence) {
                    let mut sequence_counts = sequence_counts.lock().unwrap();
                    *sequence_counts.entry(sequence.clone()).or_insert(0) += 1;
                }
                sequence.drain(..1);
            }
        }
    });

    let duration = start.elapsed();
    println!("Finished in {} ms", duration.as_millis());
    Ok(())
}

fn is_valid_date(sequence: &str) -> bool {
    // TODO: Implement proper date validation. This function currently only checks if the sequence does not contain a dot
    // and is 8 characters long.
    !sequence.contains(".") && sequence.len() == 8
}

