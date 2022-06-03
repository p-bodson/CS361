use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io;
use std::path::Path;

pub fn read<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let utf8_vector = fs::read(file_path.as_ref())?;
    let stringy = String::from_utf8_lossy(&utf8_vector).to_string();

    Ok(stringy)
}

pub fn write(file_path: &str, thing_to_write: &str) -> io::Result<()> {
    let mut f = OpenOptions::new().write(true).append(true).open(file_path)?;

    f.write(thing_to_write.as_bytes())?;
    f.write("\n".as_bytes())?;
    
    Ok(())
}

pub fn truncate(file_path: &str) -> io::Result<()> {
    OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
    Ok(())
}