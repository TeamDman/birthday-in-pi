use std::fs::File;
use std::time::Instant;
use memmap::MmapOptions;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // Use memory mapping to create a view into the file.
    let file = File::open("../resources/pi-billion.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let mut sequence_counts = vec![0; 200_000_000];
    let mut number = 0;
    let power_of_ten = 10_usize.pow(7); // Used to remove leftmost digit from number

    for (i, &byte) in mmap.iter().enumerate() {
        // Shift existing digits and add the new one
        number = number * 10 + (byte - b'0') as usize;

        if i >= 8 {
            // Once we have 8 digits, remove the leftmost digit from the previous number
            let leftmost_digit = (mmap[i - 8] - b'0') as usize;
            number -= leftmost_digit * power_of_ten;
        }

        // Check if number is a valid date and update count
        if i >= 7 && number >= 1900_0000 && number < 2100_0000 {
            let index = number - 1900_0000;
            sequence_counts[index] += 1;
        }
    }


    // count the number of unique dates
    let unique_dates = sequence_counts.iter().filter(|&c| *c > 0).count();
    println!("Number of unique dates: {}", unique_dates);

    // find the date with the most occurrences, return a tuple of the date and the count
    let max_count = sequence_counts.iter().enumerate().map(|(index, count)| (1900_0000 + index, count)).max_by_key(|&(_, count)| count).unwrap();
    println!("Max count: {}: {}", max_count.0, max_count.1);

    let duration = start.elapsed();
    println!("Finished in {} ms", duration.as_millis());
    Ok(())
}
