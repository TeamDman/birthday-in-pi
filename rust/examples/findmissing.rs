use std::{collections::HashMap, fs::File};

use chrono::NaiveDate;

pub fn main() -> std::io::Result<()> {
    let file = File::open(std::env::args().nth(1).expect("missing file argument"))?;
    let rdr = std::io::BufReader::new(file);
    let json: HashMap<String, usize> = serde_json::from_reader(rdr).expect("couldn't parse json");
    let mut missing: Vec<String> = Vec::new();
    // check each date from 1900_0000 to 2100_0000
    for i in 1900_0000..2100_0000 {
        // convert i to a string
        let year: i32 = i / 10_000;
        let month: u32 = ((i % 10_000) / 100) as u32;
        let day: u32 = (i % 100) as u32;
        if NaiveDate::from_ymd_opt(year, month, day).is_none() {
            continue;
        }
        let s = format!("{}-{:02}-{:02}", year, month, day);
        // check if s is in the json
        if !json.contains_key(&s) {
            // if not, add it to the missing vector
            missing.push(s);
        }
    }

    // print the missing dates
    for s in missing {
        println!("{}", s);
    }

    return Ok(());
}
