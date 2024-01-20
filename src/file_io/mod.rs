use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_string_to_file<P: AsRef<Path>>(path : P, data: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}