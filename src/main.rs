use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::env;
use std::vec::Vec;

fn get_mime_type(mime_paths: &[&Path], extension: &str) -> Result<String, io::Error> {
    let mut file_opened = false;

    for path in mime_paths {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_e) => continue,
        };
        file_opened = true;

        let file = BufReader::new(file);

        for line in file.lines() {
            let line = line?;
            let mut items = line.split_whitespace();
            if let Some(mime) = items.next() {
                for item in items {
                    if extension == item {
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

#[derive(Debug)]
struct MailcapEntry {
    view: String,
    edit: String,
    compose: String,
    print: String,
    test: String,
    needsterminal: bool,
    copiousoutput: bool,
}

fn mailcap_mime_types_match(mailcap_mime_type: &str, mime_type: &str) -> bool {
    let mailcap_mime_parts = mailcap_mime_type.split('/');
    let mime_parts = mime_type.split('/');

    let mut parts = mailcap_mime_parts.zip(mime_parts);

    parts.all(|part| part.0 == "*" || part.0 == part.1)
}

fn mailcap_parse_line(line: &str, mime_type: &str) -> Option<MailcapEntry> {
    let mut items = line.split(";");
    if let Some(mime) = items.next() {
        if mailcap_mime_types_match(mime, mime_type) {
            if let Some(command) = items.next() {
                let mut entry = MailcapEntry {
                    view: String::from(command.trim()),
                    edit: String::new(),
                    compose: String::new(),
                    print: String::new(),
                    test: String::new(),
                    needsterminal: false,
                    copiousoutput: false,
                };
                for item in items {
                    let mut keyvalue = item.splitn(2, "=");
                    let key = keyvalue.next();
                    let value = keyvalue.next();

                    match value {
                        Some(value) => {
                            match key.unwrap().trim() {
                                "edit" => entry.edit = String::from(value),
                                "compose" => entry.compose = String::from(value),
                                "print" => entry.print = String::from(value),
                                "test" => entry.test = String::from(value),
                                _ => continue,
                            }
                        }
                        None => {
                            match key.unwrap().trim() {
                                "needsterminal" => entry.needsterminal = true,
                                "copiousoutput" => entry.copiousoutput = true,
                                _ => continue,
                            }
                        }
                    }
                }
                return Some(entry);
            }
        }
    }
    None
}

fn mailcap_get_entries(mailcap_paths: &[&Path], mime_type: &str) -> Result<Vec<MailcapEntry>, io::Error> {
    let mut file_opened = false;
    let mut entries = Vec::new();

    for path in mailcap_paths {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_e) => continue,
        };
        file_opened = true;

        let file = BufReader::new(file);

        let mut fullline = String::new();
        for line in file.lines() {
            let line = line?;
            fullline.push_str(&line);
            if fullline.ends_with("\\") {
                fullline.pop();
                continue;
            }
            match mailcap_parse_line(&fullline, mime_type) {
                Some(entry) => entries.push(entry),
                None => {},
            }
            fullline = String::from("");
        }
    }

    if !file_opened {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No usable mailcap file found"));
    }
    Ok(entries)
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
    let mime_type = get_mime_type(&mime_paths, &env::args().nth(1).unwrap()).unwrap();

    println!("{}", mime_type);

    let mut home = PathBuf::from(env::var("HOME").unwrap());
    home.push(".mailcap");

    let mailcap_paths: [&Path; 5] = [
        &home.as_path(),
        Path::new("/etc/mailcap"),
        Path::new("/usr/share/etc/mailcap"),
        Path::new("/usr/local/etc/mailcap"),
        Path::new("/usr/etc/mailcap"),
    ];
    let mailcap_entries = mailcap_get_entries(&mailcap_paths, &mime_type);

    for entry in mailcap_entries.unwrap() {
        println!("");
        println!("view: {}", entry.view);
        println!("edit: {}", entry.edit);
        println!("compose: {}", entry.compose);
        println!("print: {}", entry.print);
        println!("test: {}", entry.test);
        println!("needsterminal: {}", entry.needsterminal);
        println!("copiousoutput: {}", entry.copiousoutput);
    }
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

    #[test]
    fn test_mime_types_nonexistant_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mime.types.");

        let mime_paths: [&Path; 1] = [&path.as_path()];

        assert_eq!(get_mime_type(&mime_paths, "txt").unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_mailcap_nonexistantfile() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mailcap.");

        let mime_paths: [&Path; 1] = [&path.as_path()];
        let results = mailcap_get_entries(&mime_paths, "text/plain").unwrap_err();
        assert_eq!(results.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_mailcap_nonexistantentry() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mailcap");

        let mime_paths: [&Path; 1] = [&path.as_path()];
        let results = mailcap_get_entries(&mime_paths, "text/foo").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_mailcap_singleentry() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mailcap");

        let mime_paths: [&Path; 1] = [&path.as_path()];
        let results = mailcap_get_entries(&mime_paths, "text/plain").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].view, "less '%s'");
        assert_eq!(results[0].edit, "vi '%s'");
        assert_eq!(results[0].test, "test \"$DISPLAY\" != \"\"");
        assert_eq!(results[0].needsterminal, true);
    }

    #[test]
    fn test_mailcap_wildcardentry() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mailcap-wildcard");

        let mime_paths: [&Path; 1] = [&path.as_path()];
        let results = mailcap_get_entries(&mime_paths, "text/plain").unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].view, "less '%s'");
        assert_eq!(results[0].edit, "vi '%s'");
        assert_eq!(results[0].test, "test \"$DISPLAY\" != \"\"");
        assert_eq!(results[0].needsterminal, true);
        assert_eq!(results[1].view, "cat '%s'");
        assert_eq!(results[2].view, "hexdump '%s'");

        let results = mailcap_get_entries(&mime_paths, "video/x-matroska").unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].view, "mpv '%s'");
        assert_eq!(results[1].view, "mplayer '%s'");
        assert_eq!(results[2].view, "hexdump '%s'");
    }
}
