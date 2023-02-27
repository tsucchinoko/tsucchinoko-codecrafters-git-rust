use crate::object::GitObject;
use anyhow::{anyhow, Result};
use flate2::bufread::ZlibDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;


pub fn load_object(hash: String) -> Result<GitObject> {
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

    read_object(decoder)
}

fn read_object<R>(reader: R) -> Result<GitObject>
where
    R: Read,
{
    let mut reader = BufReader::new(reader);

    let mut buffer = Vec::new();
    reader.read_until(' ' as u8, &mut buffer)?;
    buffer.pop();

    let object_type = String::from_utf8(buffer.clone())?;

    buffer.clear();
    reader.read_until(0, &mut buffer)?;
    buffer.pop();

    let size = String::from_utf8(buffer.clone())?.parse::<usize>()?;

    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    if content.len() != size {
        return Err(anyhow!(
            "Incorrect content length, expected {} but was {}",
            size,
            content.len()
        ));
    }

    Ok(GitObject {
        object_type,
        content,
    })
}