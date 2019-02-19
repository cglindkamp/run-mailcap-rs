use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::env;

fn get_mime_type(mime_paths: &[&Path], extension: &str) -> Result<String, io::Error> {
    for path in mime_paths {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_e) => continue,
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
    }

    Ok(String::from("application/octet-stream"))
}

fn main() {
    let mut home = PathBuf::from(env::var("HOME").unwrap());
    home.push(".mime.types");

    let mime_paths: [&Path; 4] = [
        &home.as_path(),
        Path::new("/usr/share/etc/mime.types"),
        Path::new("/usr/local/etc/mime.types"),
        Path::new("/etc/mime.types"),
    ];
    println!("{}", get_mime_type(&mime_paths, &env::args().nth(1).unwrap()).unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_types() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mime.types");

        let mime_paths: [&Path; 1] = [&path.as_path()];

        assert_eq!(get_mime_type(&mime_paths, "mp4").unwrap(), "video/mp4");
        assert_eq!(get_mime_type(&mime_paths, "txt").unwrap(), "text/plain");
        assert_eq!(get_mime_type(&mime_paths, "").unwrap(), "application/octet-stream");
    }
}
