extern crate atty;
extern crate regex;

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::env;

mod config;
mod mailcap;
mod mimetype;

use config::Config;

fn print_usage() {
    println!("Usage: run-mailcap-rs [OPTION]... [MIME-TYPE:]FILE");
    println!();
    println!("Options:");
    println!("    --action=<action>");
    println!("        Specify the action performed on the file. Valid actions are:");
    println!("        view, see (same as view), cat (same as view, but only handle");
    println!("        entries with copiousoutput and don't use a pager), edit,");
    println!("        change (same es edit), compose, create (same as compose)");
    println!("        and print.");
    println!("    --debug");
    println!("        Print some debugging statements. Its more of a tool during");
    println!("        development but may also help to determine whats wrong, when");
    println!("        unexpected actions are performend.");
    println!("    --nopager");
    println!("        Ignore \"copiousoutput\" in mailcap files and call the corresponding");
    println!("        command without invoking a pager");
    println!("    --norun");
    println!("        Do not execute the found command, but just print it. The \"test\"");
    println!("        commands in the mailcap entries are still executed.");
}

fn main() {
    let config = Config::parse(env::args(), env::vars());

    if let Err(_err) = config {
        print_usage();
        return;
    }
    let mut config = config.unwrap();

    if config.mimetype == "" {
        config.mimetype = mimetype::get_type(&config.filename).unwrap();

        if config.debug {
            println!("Determined mime type:");
            println!("{}", config.mimetype);
            println!();
        }
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
    let mailcap_entries = mailcap::get_entries(&mailcap_paths, &config.mimetype).unwrap();

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

