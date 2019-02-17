use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::env;

fn get_mime_type(extension: &str) -> Result<String, io::Error> {
    let path = Path::new("/etc/mime.types");

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let file = BufReader::new(file);

    for line in file.lines() {
        match line {
            Ok(line) => {
                let mut items = line.split_whitespace();
                let mime = items.nth(0);
                if let Some(mime) = mime {
                    for s in line.split_whitespace() {
                        if extension == s {
                            return Ok(String::from(mime));
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(String::from("application/octet-stream"))
}

fn main() {
    println!("{}", get_mime_type(&env::args().nth(1).unwrap()).unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_types() {
        assert_eq!(get_mime_type("mp4").unwrap(), "video/mp4");
        assert_eq!(get_mime_type("txt").unwrap(), "text/plain");
        assert_eq!(get_mime_type("").unwrap(), "application/octet-stream");
    }
}
