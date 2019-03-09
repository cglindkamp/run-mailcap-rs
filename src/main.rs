use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::env;

mod config;
mod mailcap;
mod mimetype;

fn run_mailcap(action: &str, filename: &str, mailcap_entries: &Vec<mailcap::MailcapEntry>) {
    for entry in mailcap_entries {
        let command = match action {
            "view" => &entry.view,
            "edit" => &entry.edit,
            "compose" => &entry.compose,
            "print" => &entry.print,
            _ => &entry.view,
        };
        if command != "" {
            let command = command.replace("%s", filename);
            let _status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status();
            return;
        }
    }
}

fn main() {
    let config = config::Config::parse(env::args()).unwrap();
    let mut home = PathBuf::from(env::var("HOME").unwrap());
    home.push(".mime.types");

    let mime_paths: [&Path; 4] = [
        &home.as_path(),
        Path::new("/usr/share/etc/mime.types"),
        Path::new("/usr/local/etc/mime.types"),
        Path::new("/etc/mime.types"),
    ];
    let mime_type = mimetype::get_type(&mime_paths, &config.filename).unwrap();

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
    let mailcap_entries = mailcap::get_entries(&mailcap_paths, &mime_type).unwrap();

    for entry in &mailcap_entries {
        println!("");
        println!("view: {}", entry.view);
        println!("edit: {}", entry.edit);
        println!("compose: {}", entry.compose);
        println!("print: {}", entry.print);
        println!("test: {}", entry.test);
        println!("needsterminal: {}", entry.needsterminal);
        println!("copiousoutput: {}", entry.copiousoutput);
    }

    run_mailcap(&config.action, &config.filename, &mailcap_entries);
}

