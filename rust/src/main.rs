use memmap::MmapOptions;
use std::fs::File;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let file = File::open("../resources/pi-billion.txt")?;
    let map = unsafe { MmapOptions::new().map(&file)? };

    let mut sequence_counts = vec![0; 2_000_000];
    let mut sequence = Vec::with_capacity(8);

    for &byte in map.iter().skip(2) {
        sequence.push(byte - b'0');
        if sequence.len() == 8 {
            let date = sequence.iter().fold(0, |acc, &digit| acc * 10 + (digit as usize));
            sequence.remove(0);
            if date >= 1_900_0000 && date < 2_100_0000 {
                sequence_counts[date - 1_900_0000] += 1;
            }
        }
    }

    let total_unique_dates = sequence_counts.into_iter().filter(|&count| count > 0).count();

    let duration = start.elapsed();
    println!("Counted {} unique dates, took {} ms", total_unique_dates, duration.as_millis());

    Ok(())
}

// took 484224