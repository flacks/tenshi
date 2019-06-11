use std::io::{stdout, Write};

pub fn steven_black() -> String {
    print!("    Fetching hosts file... ");
    stdout().flush().unwrap();

    let client = reqwest::Client::new();
    let mut request = client
        .get("https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts")
        .send()
        .unwrap();

    println!("✔");

    // Return the response text as a string
    request.text().unwrap()
}
