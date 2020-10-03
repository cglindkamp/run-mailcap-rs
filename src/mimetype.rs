use std::io;
use std::process::Command;

pub fn get_type(filename: &str) -> Result<String, io::Error> {
    if let Ok(output) = Command::new("file")
        .arg("-b")
        .arg("--mime-type")
        .arg(filename)
        .output() {

        if let Ok(content) = String::from_utf8(output.stdout) {
            if let Some(mimetype) = content.lines().next() {
                return Ok(mimetype.to_string());
            }
        }
        Ok(String::from("application/octet-stream"))
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Error executing file command"))
    }
}
