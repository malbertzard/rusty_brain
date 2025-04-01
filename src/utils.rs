use std::fs::File;
use std::io::{self, Write};

pub fn write_assembly_to_file(assembly_code: &str, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(assembly_code.as_bytes())?;
    Ok(())
}
