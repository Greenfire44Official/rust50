use std::{collections::HashMap, fs};

use anyhow::{Result as AnyhowResult, bail};

struct Person {
    name: String,
    dna: HashMap<String, usize>,
}

fn main() -> AnyhowResult<()> {
    let args: Vec<String> = {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0);
        args
    };
    if args.len() != 2 {
        bail!("Invalid input\n\nUsage: dna database.csv sequence.txt\n");
    }

    // Load files
    let sequence = fs::read_to_string(&args[1])?;
    let database = load_csv(&args[0])?;

    // Parse database into a vector of people.
    let mut people: Vec<Person> = Vec::new();
    for record in database.records {
        let mut person = Person {
            name: record[0].to_owned(),
            dna: HashMap::new(),
        };
        for (i, dna_str_count) in record[1..].iter().enumerate() {
            person
                .dna
                .insert(database.headers[i + 1].to_string(), dna_str_count.parse()?);
        }
        people.push(person);
    }

    // Count STRs in the sequence provided.
    let mut sequence_hashmap: HashMap<String, usize> = HashMap::new();
    for dna_str in database.headers[1..].iter() {
        let dna_str = dna_str.to_string();
        let mut highest_repeats = 0;
        for i in 0..sequence.len() {
            let current_slice = sequence[i..match i + dna_str.len() {
                x if x >= sequence.len() => break,
                x => x,
            }]
                .to_string();
            if dna_str != current_slice {
                continue;
            }
            let mut repeats = 1;
            loop {
                let slice_start = i + dna_str.len() * repeats;
                let slice_end = match slice_start + dna_str.len() {
                    x if x >= sequence.len() => break,
                    x => x,
                };
                if sequence[slice_start..slice_end].to_string() != dna_str {
                    break;
                }
                repeats += 1;
            }
            if repeats > highest_repeats {
                highest_repeats = repeats;
            }
        }
        sequence_hashmap.insert(dna_str.to_string(), highest_repeats);
    }

    // Search database for an exact match
    for person in people {
        // println!("{}:\n{:?}", person.name, person.dna);
        if person.dna == sequence_hashmap {
            println!("{}\n", person.name);
            return Ok(());
        }
    }
    println!("No match");
    Ok(())
}

struct CsvData {
    headers: Vec<String>,
    records: Vec<Vec<String>>,
}

// Simple csv data loader. Assumes file provided IS a valid database csv and will not check for csv validity.
fn load_csv(path: &String) -> AnyhowResult<CsvData> {
    let file = fs::read_to_string(&path)?;
    let lines: Vec<&str> = file.split("\r\n").collect();
    let mut records: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        let values: Vec<&str> = line.split(",").collect();
        records.push(values);
    }
    let headers = records.remove(0);

    // Turn the data into String vectors to guarantee lifetime.
    let headers: Vec<String> = headers.iter().filter_map(|h| Some(h.to_string())).collect();
    let records: Vec<Vec<String>> = records
        .iter()
        .filter_map(|v| Some(v.iter().filter_map(|h| Some(h.to_string())).collect()))
        .collect();

    Ok(CsvData {
        headers: headers,
        records: records,
    })
}
