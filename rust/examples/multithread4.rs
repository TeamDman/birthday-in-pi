use std::collections::{BTreeMap, HashMap};
use std::fs::File;

use chrono::NaiveDate;
use memmap::MmapOptions;
use rayon::prelude::*;
use serde_json::to_writer_pretty;
use std::io::BufWriter;
use std::time::Instant;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // Use memory mapping to create a view into the file.
    let file = File::open("../resources/pi-billion.txt")?;
    // let file = File::open("../resources/pi_dec_1m.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let sequence_counts = Arc::new(
        (0..1_000_000_000)
            .map(|_| AtomicUsize::new(0))
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
            sequence_counts[number].fetch_add(1, Ordering::Relaxed);
        }
    });

    // count the number of unique dates
    let unique_dates = sequence_counts[1900_00_00..2100_00_00]
        .iter()
        .filter(|count| count.load(Ordering::Relaxed) > 0)
        .count();
    println!("Number of unique dates: {}", unique_dates);

    // find the date with the most occurrences, return a tuple of the date and the count
    let max_count = sequence_counts[1900_00_00..2100_00_00]
        .iter()
        .enumerate()
        .map(|(index, count)| (1900_0000 + index, count.load(Ordering::Relaxed)))
        .max_by_key(|&(_, count)| count)
        .unwrap();
    println!("Max count: {}: {}", max_count.0, max_count.1);

    // Create a HashMap to hold the dates and counts
    let mut date_counts = HashMap::new();

    // Convert sequence_counts into dates
    for (index, count) in sequence_counts.iter().enumerate() {
        let count = count.load(Ordering::Relaxed);
        if count > 0 {
            let sequence_number = 1900_0000 + index;
            let year: i32 = (sequence_number / 10_000) as i32;
            let month: u32 = ((sequence_number / 100) % 100) as u32;
            let day: u32 = (sequence_number % 100) as u32;
            if NaiveDate::from_ymd_opt(year, month, day).is_some() {
                date_counts.insert(format!("{year:04}-{month:02}-{day:02}"), count);
            }
        }
    }

    // Convert HashMap to JSON and write to a file
    let ordered: BTreeMap<_, _> = date_counts.into_iter().collect();
    let file = File::create("date_counts.json")?;
    to_writer_pretty(BufWriter::new(file), &ordered).expect("couldn't write json");

    let duration = start.elapsed();
    println!("Finished in {} ms", duration.as_millis());
    Ok(())
}
