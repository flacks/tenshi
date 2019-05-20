use std::error::Error;
use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    println!("tenshi 0.1.4 :: Jean Lucas <jean@4ray.co>\n");

    println!("This utility will fetch Steven Black's adware + malware hosts file and generate Unbound local-data from it\n");

    println!("Data will be saved to /etc/unbound/local-blocking-data.conf");
    println!("You must have permission to write to that file\n");

    println!(
        "Add `include: \"local-blocking-data.conf\"` to your Unbound config to use the result\n"
    );

    // Prompt to continue
    loop {
        print!("Enter \"y\" to continue, \"q\" to quit: ");
        stdout().flush().unwrap();

        let mut confirm = String::new();
        stdin()
            .read_line(&mut confirm)
            .expect("Failed to read line");
        match confirm.trim() {
            "y" => {
                println!();
                break;
            }
            "q" => std::process::exit(0),
            _ => println!("\nInvalid response\n"),
        }
    }

    // Check write permission
    print!("Checking write permission... ");
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
    print!("Fetching hosts file... ");
    stdout().flush().unwrap();
    let mut request = client
        .get("https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts")
        .send()
        .unwrap();
    println!("Success");
    let hosts = request.text().unwrap();

    // Gather results using regex and write local-data entries to file buffer
    let re = regex::Regex::new(r"(?m)^0.0.0.0 (\S+)").unwrap();
    let mut hosts_buf = BufWriter::new(file);
    print!("Generating local-data... ");
    stdout().flush().unwrap();
    for cap in re.captures_iter(&hosts) {
        write!(
            &mut hosts_buf,
            "local-data: \"{} A 0.0.0.0\"\nlocal-data: \"{} AAAA ::\"\n",
            &cap[1], &cap[1]
        )
        .unwrap();
    }

    println!("Success")
}
