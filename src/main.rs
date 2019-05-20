use std::error::Error;
use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    println!("Tenshi 0.2.1 :: Jean Lucas <jean@4ray.co>

    Tenshi fetches Steven Black's adware + malware hosts file and generates Unbound local-zone entries from it

    Data will be saved to /etc/unbound/local-blocking-data.conf

    Run Tenshi as a user with write access to that file

    Add 'include: \"local-blocking-data.conf\"' to your Unbound config to use the result\n");

    // Prompt to continue
    loop {
        print!("Enter to continue, \"q\" to quit: ");
        stdout().flush().unwrap();

        let mut confirm = String::new();
        stdin()
            .read_line(&mut confirm)
            .expect("Failed to read line");
        match confirm.trim() {
            "" => {
                println!();
                break;
            }
            "q" => std::process::exit(0),
            _ => println!("\n    Invalid response\n"),
        }
    }

    // Confirm write access
    print!("    Confirming write access... ");
    stdout().flush().unwrap();
    let path = std::path::Path::new("/etc/unbound/local-blocking-data.conf");
    let display = path.display();
    let file = match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)
    {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(file) => {
            println!("Success");
            file
        }
    };

    // Fetch hosts file
    let client = reqwest::Client::new();
    print!("    Fetching hosts file... ");
    stdout().flush().unwrap();
    let mut request = client
        .get("https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts")
        .send()
        .unwrap();
    println!("Success");
    let hosts = request.text().unwrap();

    // Gather results using regex and write local-zone entries to file buffer
    let re = regex::Regex::new(r"(?m)^0.0.0.0 (\S+)").unwrap();
    let mut hosts_buf = BufWriter::new(file);
    print!("    Generating local-zone entries... ");
    stdout().flush().unwrap();
    for cap in re.captures_iter(&hosts) {
        writeln!(&mut hosts_buf, "local-zone: \"{}\" refuse", &cap[1]).unwrap();
    }
    println!("Success\n")
}
