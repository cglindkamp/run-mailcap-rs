extern crate atty;

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::env;

mod config;
mod mailcap;
mod mimetype;

use config::Config;

fn main() {
    let config = Config::parse(env::args(), env::vars()).unwrap();
    let mut home = PathBuf::from(env::var("HOME").unwrap());
    home.push(".mime.types");

    let mime_paths: [&Path; 4] = [
        &home.as_path(),
        Path::new("/usr/share/etc/mime.types"),
        Path::new("/usr/local/etc/mime.types"),
        Path::new("/etc/mime.types"),
    ];
    let mime_type = mimetype::get_type(&mime_paths, &config.filename).unwrap();

    if config.debug {
        println!("Determined mime type:");
        println!("{}", mime_type);
        println!();
    }

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

    if config.debug {
        println!("Mailcap entries:");
        for entry in &mailcap_entries {
            println!("view: {}", entry.view);
            println!("edit: {}", entry.edit);
            println!("compose: {}", entry.compose);
            println!("print: {}", entry.print);
            println!("test: {}", entry.test);
            println!("needsterminal: {}", entry.needsterminal);
            println!("copiousoutput: {}", entry.copiousoutput);
            println!();
        }
    }

    if let Some(command) = mailcap::get_final_command(&config, atty::is(atty::Stream::Stdout), &mailcap_entries) {
        if config.norun {
            println!("{}", command);
        } else {
            let _status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status();
        }
    }
}

