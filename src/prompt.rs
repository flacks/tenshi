use std::io::{stdin, stdout, Write};

pub fn confirm() {
    println!("Tenshi 0.2.2 :: Jean Lucas <jean@4ray.co>

    Tenshi fetches Steven Black's adware + malware hosts file and generates Unbound local-zone entries from it

    Data will be saved to /etc/unbound/local-blocking-data.conf

    Run Tenshi as a user with write access to that file

    Add 'include: \"local-blocking-data.conf\"' to your Unbound config to use the result\n");

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
}
