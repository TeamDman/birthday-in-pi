use std::fs::File;
use std::time::Instant;
use memmap::MmapOptions;

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // Use memory mapping to create a view into the file.
    let file = File::open("../resources/pi-billion.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    // The buffer size determines the size of the chunks.
    let buffer_size = 10 * 1024 * 1024;  // 10 MB
    let mut buffer = vec![0; buffer_size];

    let mut sequence = String::new();
    let mut sequence_counts = vec![0; 200_000_000];
    
    let mut i = 0;
    while i < mmap.len() {
        // Read into the buffer.
        let end = std::cmp::min(i + buffer_size, mmap.len());
        buffer[..end-i].copy_from_slice(&mmap[i..end]);

        // Append any remaining characters from the previous chunk to ensure we don't split sequences.
        if !sequence.is_empty() {
            let remaining = 8 - sequence.len();
            for &byte in &buffer[0..remaining] {
                sequence.push(byte as char);
            }
            // Check if sequence is a valid date and update count
            if is_valid_date(&sequence) {
                let sequence_number = sequence.parse::<usize>().unwrap();
                if sequence_number >= 1900_0000 && sequence_number < 2100_0000 {
                    let index = sequence_number - 1900_0000;
                    sequence_counts[index] += 1;
                }
            }
            
            sequence.clear();
        }

        for &byte in &buffer[..end-i] {
            let char = byte as char;
            sequence.push(char);
            if sequence.len() == 8 {
                // Check if sequence is a valid date and update count
                if is_valid_date(&sequence) {
                    let sequence_number = sequence.parse::<usize>().unwrap();
                    if sequence_number >= 1900_0000 && sequence_number < 2100_0000 {
                        let index = sequence_number - 1900_0000;
                        sequence_counts[index] += 1;
                    }
                }
                
                sequence.drain(..1);
            }
        }
        i += buffer_size;
    }

    let duration = start.elapsed();
    println!("Finished in {} ms", duration.as_millis());
    Ok(())
}

fn is_valid_date(sequence: &str) -> bool {
    // Here you'd implement the date validation logic.
    // This is a placeholder implementation that always returns true.
    !sequence.contains(".") && sequence.len() == 8
}
