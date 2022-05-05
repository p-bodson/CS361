use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub fn read(file_path: &str) -> Result<String, Box<dyn Error>> {
    let utf8_vector = fs::read(file_path)?;
    let foo = String::from_utf8_lossy(&utf8_vector).to_string();

    Ok(foo)
}

pub fn write(file_path: &str, thing_to_write: &str) -> Result<(), Box<dyn Error>> {
    let mut f = OpenOptions::new().write(true).append(true).open(file_path)?;

    f.write(thing_to_write.as_bytes())?;
    f.write("\n".as_bytes())?;
    
    Ok(())
}

pub fn truncate(file_path: &str) -> Result<(), Box<dyn Error>> {
    OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
    Ok(())
}