use anyhow::{anyhow, Result};
use flate2::bufread::ZlibDecoder;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Read};
use std::path::Path;

pub fn pretty_cat_file(hash: String) -> Result<()> {
    // Known size of a sha-1 hash, see https://en.wikipedia.org/wiki/SHA-1
    if hash.len() != 40 {
        return Err(anyhow!(
            "object id should be 40 characters but was {}",
            hash.len()
        ))
    }

    let sub_directory: String = hash.chars().take(2).collect();
    let file_name: String = hash.chars().skip(2).collect();
    let path = Path::new(".git").join("objects").join(sub_directory).join(file_name);
    
    let file = BufReader::new(File::open(path)?);
    let decoder = ZlibDecoder::new(file);

    print_file(decoder)
}

fn print_file<R>(reader: R) -> Result<()> where R: Read, {
    let mut reader = BufReader::new(reader);

    let mut buffer = Vec::new();
    reader.read_until(' ' as u8, &mut buffer)?;
    buffer.pop();

    let object_type = String::from_utf8(buffer.clone())?;
    if object_type.as_str() != "blob" {
        return Err(anyhow!("Unsupported object type: {}", object_type))
    }

    buffer.clear();
    reader.read_until(0, &mut buffer)?;
    buffer.pop();

    let size = String::from_utf8(buffer.clone())?.parse::<usize>()?;

    let actual_size = std::io::copy(&mut reader, &mut stdout())?;
    if actual_size != size as u64 {
        return Err(anyhow!(
            "Incorrect content length, expected {} but was {}",
            size,
            actual_size
        ))
    }

    Ok(())
}