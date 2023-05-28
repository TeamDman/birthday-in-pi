use memmap::MmapOptions;
use std::fs::File;
use std::time::Instant;

fn is_valid_date(date: usize) -> bool {
    let year = date / 10000;
    let month = (date % 10000) / 100;
    let day = date % 100;

    year >= 1900 && year < 2100 && month >= 1 && month <= 12 && day >= 1 && day <= 31
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // let file = File::open("../resources/pi-billion.txt")?;
    let file = File::open("../resources/pi_dec_1m.txt")?;
    let map = unsafe { MmapOptions::new().map(&file)? };

    let mut sequence_counts = vec![0; 1_000_000_000]; // Adjust size to accommodate all 8-digit numbers
    let mut sequence: Vec<u8> = map[2..10].to_vec();
    for byte in sequence.iter_mut() {
        *byte -= b'0';
    }

    for &byte in map.iter().skip(10) {
        sequence.push(byte - b'0');
        let date = sequence.iter().fold(0, |acc, &digit| acc * 10 + (digit as usize));
        sequence.remove(0);
        sequence_counts[date] += 1;
    }

    sequence_counts = sequence_counts.into_iter().enumerate()
        .filter(|&(date, _)| is_valid_date(date + 1_900_0000))
        .map(|(_, count)| count)
        .collect();

    let total_unique_dates = sequence_counts.into_iter().filter(|&count| count > 0).count();

    let duration = start.elapsed();
    println!("Counted {} unique dates, took {} ms", total_unique_dates, duration.as_millis());

    Ok(())
}
