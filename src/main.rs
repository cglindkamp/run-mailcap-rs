use std::path::Path;
use std::path::PathBuf;
use std::env;

mod mailcap;
mod mimetype;

fn main() {
    let mut home = PathBuf::from(env::var("HOME").unwrap());
    home.push(".mime.types");

    let mime_paths: [&Path; 4] = [
        &home.as_path(),
        Path::new("/usr/share/etc/mime.types"),
        Path::new("/usr/local/etc/mime.types"),
        Path::new("/etc/mime.types"),
    ];
    let mime_type = mimetype::get_type(&mime_paths, &env::args().nth(1).unwrap()).unwrap();

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
    let mailcap_entries = mailcap::get_entries(&mailcap_paths, &mime_type);

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

