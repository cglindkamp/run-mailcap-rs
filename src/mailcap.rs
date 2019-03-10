use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

use super::config::*;

#[derive(Debug)]
pub struct MailcapEntry {
    pub view: String,
    pub edit: String,
    pub compose: String,
    pub print: String,
    pub test: String,
    pub needsterminal: bool,
    pub copiousoutput: bool,
}

impl Default for MailcapEntry {
    fn default() -> Self {
        MailcapEntry {
            view: String::new(),
            edit: String::new(),
            compose: String::new(),
            print: String::new(),
            test: String::new(),
            needsterminal: false,
            copiousoutput: false,
        }
    }
}

fn mime_types_match(mailcap_mime_type: &str, mime_type: &str) -> bool {
    let mut mailcap_mime_parts = mailcap_mime_type.split('/');
    let mime_parts = mime_type.split('/');
    let matches;
    {
        let mut parts = mailcap_mime_parts.by_ref().take(2).zip(mime_parts);
        matches = parts.all(|part| part.0 == "*" || part.0 == part.1);
    }
    matches && mailcap_mime_parts.count() == 0
}

fn parse_line(line: &str, mime_type: &str) -> Option<MailcapEntry> {
    let mut items = line.split(';');
    if let Some(mime) = items.next() {
        if mime_types_match(mime, mime_type) {
            if let Some(command) = items.next() {
                let mut entry: MailcapEntry = MailcapEntry {
                    view: String::from(command.trim()),
                    ..Default::default()
                };
                for item in items {
                    let mut keyvalue = item.splitn(2, '=');
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

pub fn get_entries(mailcap_paths: &[&Path], mime_type: &str) -> Result<Vec<MailcapEntry>, io::Error> {
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
            if fullline.ends_with('\\') {
                fullline.pop();
                continue;
            }
            if fullline.starts_with('#') {
                fullline = String::new();
                continue;
            }
            if let Some(entry) = parse_line(&fullline, mime_type) {
                entries.push(entry);
            }
            fullline = String::new();
        }
    }

    if !file_opened {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No usable mailcap file found"));
    }
    Ok(entries)
}

pub fn get_final_command<'a, I>(config: &Config, isatty: bool, mailcap_entries: I) -> Option<String>
where
    I: IntoIterator<Item = &'a MailcapEntry>,
{
    for entry in mailcap_entries {
        let command = match config.action {
            Action::View => &entry.view,
            Action::Cat => &entry.view,
            Action::Edit => &entry.edit,
            Action::Compose => &entry.compose,
            Action::Print => &entry.print,
        };
        if command != "" {
            let mut command = command.replace("%s", &config.filename);

            if entry.copiousoutput && config.action != Action::Print {
                command = command + "|" + &config.pager;
            }

            if entry.needsterminal && config.action != Action::Print {
                if isatty {
                    return Some(command);
                } else if config.running_in_x {
                    return Some(format!("{} -T \"{}\" -e sh -c \"{}\"", config.xtermcmd, command, command));
                } else {
                    return None
                }
            } else {
                return Some(command);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_mailcap_nonexistantfile() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mailcap.");

        let mime_paths: [&Path; 1] = [&path.as_path()];
        let results = get_entries(&mime_paths, "text/plain").unwrap_err();
        assert_eq!(results.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_mailcap_nonexistantentry() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mailcap");

        let mime_paths: [&Path; 1] = [&path.as_path()];
        let results = get_entries(&mime_paths, "text/foo").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_mailcap_singleentry() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mailcap");

        let mime_paths: [&Path; 1] = [&path.as_path()];
        let results = get_entries(&mime_paths, "text/plain").unwrap();
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
        let results = get_entries(&mime_paths, "text/plain").unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].view, "less '%s'");
        assert_eq!(results[0].edit, "vi '%s'");
        assert_eq!(results[0].test, "test \"$DISPLAY\" != \"\"");
        assert_eq!(results[0].needsterminal, true);
        assert_eq!(results[1].view, "cat '%s'");
        assert_eq!(results[2].view, "hexdump '%s'");

        let results = get_entries(&mime_paths, "video/x-matroska").unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].view, "mpv '%s'");
        assert_eq!(results[1].view, "mplayer '%s'");
        assert_eq!(results[2].view, "hexdump '%s'");
    }

    #[test]
    fn test_mailcap_ignorecomments() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mailcap");

        let mime_paths: [&Path; 1] = [&path.as_path()];
        let results = get_entries(&mime_paths, "#text/plain").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_final_command() {
        let entries: [MailcapEntry; 2] = [
            MailcapEntry{
                view: String::from("cat '%s'"),
                edit: String::new(),
                compose: String::new(),
                print: String::new(),
                test: String::new(),
                copiousoutput: false,
                needsterminal: true,
            },
            MailcapEntry{
                view: String::new(),
                edit: String::from("vim '%s'"),
                compose: String::new(),
                print: String::from("lpr '%s'"),
                test: String::new(),
                copiousoutput: false,
                needsterminal: true,
            },
        ];

        let config = Config {
            filename: String::from("test.txt"),
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, true, &entries).unwrap(), "cat 'test.txt'");

        let config = Config {
            filename: String::from("test.txt"),
            action: Action::Edit,
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, true, &entries).unwrap(), "vim 'test.txt'");

        let config = Config {
            filename: String::from("test.txt"),
            action: Action::Compose,
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, true, &entries), None);

        let config = Config {
            filename: String::from("test.txt"),
            action: Action::Edit,
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, false, &entries), None);

        let config = Config {
            filename: String::from("test.txt"),
            action: Action::Edit,
            running_in_x: true,
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, false, &entries).unwrap(), "xterm -T \"vim 'test.txt'\" -e sh -c \"vim 'test.txt'\"");

        let config = Config {
            filename: String::from("test.txt"),
            action: Action::Edit,
            xtermcmd: String::from("urxvt"),
            running_in_x: true,
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, false, &entries).unwrap(), "urxvt -T \"vim 'test.txt'\" -e sh -c \"vim 'test.txt'\"");

        let config = Config {
            filename: String::from("test.txt"),
            action: Action::Print,
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, false, &entries).unwrap(), "lpr 'test.txt'");
    }

    #[test]
    fn test_final_command_copiousoutput() {
        let entries: [MailcapEntry; 1] = [
            MailcapEntry{
                view: String::from("cat '%s'"),
                edit: String::new(),
                compose: String::new(),
                print: String::from("lpr '%s'"),
                test: String::new(),
                copiousoutput: true,
                needsterminal: true,
            },
        ];

        let config = Config {
            filename: String::from("test.txt"),
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, true, &entries).unwrap(), "cat 'test.txt'|less");

        let config = Config {
            filename: String::from("test.txt"),
            running_in_x: true,
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, false, &entries).unwrap(), "xterm -T \"cat 'test.txt'|less\" -e sh -c \"cat 'test.txt'|less\"");

        let config = Config {
            filename: String::from("test.txt"),
            action: Action::Print,
            ..Default::default()
        };
        assert_eq!(get_final_command(&config, false, &entries).unwrap(), "lpr 'test.txt'");
    }
}
