use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::process::Command;

pub fn get_type_by_magic(filename: &str) -> Result<String, io::Error> {
    if let Ok(output) = Command::new("file")
        .arg("-E")
        .arg("-b")
        .arg("--mime-type")
        .arg(filename)
        .output() {

        if output.status.success() {
            if let Ok(content) = String::from_utf8(output.stdout) {
                if let Some(mimetype) = content.lines().next() {
                    return Ok(mimetype.to_string());
                }
            }
            Ok(String::from("application/octet-stream"))
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Error executing file command"))
        }
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Error executing file command"))
    }
}

pub fn get_type_by_extension(mime_paths: &[&Path], filename: &str) -> Result<String, io::Error> {
    let mut file_opened = false;

    if !filename.contains('.') {
        return Ok(String::from("application/octet-stream"));
    }

    let extension = filename.rsplit('.').next().unwrap();

    for path in mime_paths {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_e) => continue,
        };
        file_opened = true;

        let file = BufReader::new(file);

        for line in file.lines() {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }
            let mut items = line.split_whitespace();
            if let Some(mime) = items.next() {
                if mime.matches('/').count() != 1 {
                    continue;
                }
                for item in items {
                    if extension.to_lowercase() == item {
                        return Ok(String::from(mime));
                    }
                }
            }
        }
    }

    if file_opened {
        Ok(String::from("application/octet-stream"))
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No usable mime.types file found"))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_mime_types() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mime.types");

        let mime_paths: [&Path; 1] = [&path.as_path()];

        assert_eq!(get_type_by_extension(&mime_paths, "test.mp4").unwrap(), "video/mp4");
        assert_eq!(get_type_by_extension(&mime_paths, "test.MP4").unwrap(), "video/mp4");
        assert_eq!(get_type_by_extension(&mime_paths, "test.txt").unwrap(), "text/plain");
        assert_eq!(get_type_by_extension(&mime_paths, "test").unwrap(), "application/octet-stream");
        assert_eq!(get_type_by_extension(&mime_paths, "test.html").unwrap(), "application/octet-stream");
    }

    #[test]
    fn test_mime_types_nonexistant_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mime.types.");

        let mime_paths: [&Path; 1] = [&path.as_path()];

        assert_eq!(get_type_by_extension(&mime_paths, "test.txt").unwrap_err().kind(), io::ErrorKind::NotFound);
    }
}
