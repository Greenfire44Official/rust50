use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::Path;

use rust50::get_string;

use anyhow::{Result as AnyhowResult, bail};

const HEADER_SIZE: usize = 44;
fn main() -> AnyhowResult<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        bail!("Invalid input\n\nUsage: volume input.wav output.wav multiplier\n");
    }
    let src_path = Path::new(&args[1]);
    let out_path = &args[2];
    let volume_mult = match args[3].parse::<f32>()? {
        x if x >= 0.0 => x,
        _ => bail!("Multiplier must be positive"),
    };

    // Open the files.
    let src = std::fs::File::open(src_path)?;

    let out = match std::fs::File::create_new(out_path) {
        std::io::Result::Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            println!(
                "File {} already exists. Do you want to overwrite? [y/N]",
                &args[2]
            );
            if !["y", "yes"].contains(&get_string("").to_lowercase().as_str()) {
                println!("Canceled");
                return Ok(());
            }
            std::fs::File::create(out_path)?
        }
        Err(e) => bail!(e),
    };

    let mut reader = BufReader::new(src);
    let mut writer = BufWriter::new(out);
    let mut header_buf: Vec<u8> = vec![0; HEADER_SIZE];
    let mut sample_buf: Vec<u8> = vec![0; 2];

    
    reader.read_exact(&mut header_buf)?;
    writer.write_all(&header_buf)?;

    let mut step = 0;
    loop {
        print!("\rSample {step}");
        std::io::stdout().flush().unwrap();
        step += 1;

        match reader.read(&mut sample_buf) {
            Ok(b) if b != 2 => break,
            Ok(_) => {}
            Err(e) => bail!(e),
        };
        let bytes = [sample_buf[0], sample_buf[1]];
        let mut sample = i16::from_le_bytes(bytes);

        sample = (sample as f32 * volume_mult).round() as i16;

        writer.write_all(&sample.to_le_bytes())?;
    }
    println!("");
    Ok(())
}
