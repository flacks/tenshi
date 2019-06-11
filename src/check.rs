use std::{
    error::Error,
    fs::File,
    io::{stdout, Write},
};

pub fn write_access() -> File {
    print!("    Confirming write access... ");
    stdout().flush().unwrap();

    let path = std::path::Path::new("/etc/unbound/local-blocking-data.conf");
    let display = path.display();

    // Return writeable destination file
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)
    {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(file) => {
            println!("✔");
            file
        }
    }
}
