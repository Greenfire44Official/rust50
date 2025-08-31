use anyhow::{Result as AnyhowResult, bail};
use std::fs;
use std::io::{BufReader, BufWriter, Read, Write};

fn main() -> AnyhowResult<()> {
    let args: Vec<String> = {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0);
        args
    };
    if args.len() != 1 {
        bail!("Invalid input\n\nUsage: recover input.wav output.wav multiplier\n");
    }

    // Open the recovery file.
    let raw = fs::File::open(&args[0])?;
    let mut reader = BufReader::new(raw);
    let jpeg_signature: Vec<u8> = vec![0xff, 0xd8, 0xff];
    let mut block_buf: Vec<u8> = vec![0; 512];
    let mut file_count = 0;
    let mut file_buf: Vec<u8> = Vec::new();
    let mut writing: bool = false;

    loop {
        match reader.read(&mut block_buf) {
            Ok(x) if x == 0 => {
                if file_buf.len() != 0 {
                    // Write last buffered file.
                    write_file(&file_count, &file_buf)?;
                }
                println!("Recovery completed.");
                break;
            }
            Ok(x) if x < 512 => {
                println!("Slack found at end of file. Number of bytes: {x}");
                break;
            }
            Err(e) => bail!(e),
            _ => {}
        }
        if !writing && block_buf[0..3] != jpeg_signature {
            continue;
        }
        if block_buf[0..3] == jpeg_signature && block_buf[3] >> 4 == 0b1110 {
            if writing {
                write_file(&file_count, &file_buf)?;
                file_buf.clear();
                file_count += 1;
            } else {
                writing = true;
            }
        }
        file_buf.append(&mut block_buf.clone());
    }
    Ok(())
}

fn write_file(file_count: &i32, file_buf: &Vec<u8>) -> AnyhowResult<()> {
    let file_name = format!("{:0>3}.jpg", file_count);
    println!("Writing file: {}", file_name);

    let out = fs::File::create(file_name)?;
    let mut writer = BufWriter::new(out);
    writer.write_all(&file_buf)?;

    Ok(())
}
