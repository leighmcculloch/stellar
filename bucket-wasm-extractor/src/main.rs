use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::{
    env,
    fs::{self, metadata, File},
    io::{Cursor, Write},
    path::Path,
};
use stellar_xdr::curr::{BucketEntry, Frame, LedgerEntryData, Limited, Limits, ReadXdr};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bucket-wasm-extractor <bucket-file> [bucket-file...]");
        std::process::exit(1);
    }

    let files = args.par_iter().skip(1);
    let files_len = files.len();
    files.progress_count(files_len as u64).for_each(|path| {
        if let Err(e) = process_file(Path::new(path)) {
            eprintln!("Error processing {}: {}", path, e);
        }
    });

    Ok(())
}

fn process_file(path: &Path) -> Result<()> {
    let contents = fs::read(path).map_err(|e| format!("Failed to read input file: {}", e))?;
    let mut limited_contents = Limited::new(Cursor::new(contents), Limits::none());
    let iter = Frame::<BucketEntry>::read_xdr_iter(&mut limited_contents);

    for entry in iter {
        let Frame(entry) = match entry {
            Ok(entry) => entry,
            Err(e) => {
                return Err(format!("Failed to read entry: {:?}", e).into());
            }
        };
        match entry {
            BucketEntry::Initentry(entry) | BucketEntry::Liveentry(entry) => {
                if let LedgerEntryData::ContractCode(code) = &entry.data {
                    let hash: &[u8] = code.hash.as_ref();
                    let hash_str = hash
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>();
                    let output_path = format!("{hash_str}.wasm");
                    let mut file = File::create(&output_path)
                        .map_err(|e| format!("Failed to create output file: {}", e))
                        .unwrap();
                    file.write_all(&code.code.as_slice())
                        .map_err(|e| format!("Failed to write WASM bytes: {}", e))
                        .unwrap();
                }
            }
            _ => (),
        }
    }
    Ok(())
}
