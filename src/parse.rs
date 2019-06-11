use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};

pub fn generate(dest: File, hosts: String) {
    print!("    Generating local-zone entries... ");
    stdout().flush().unwrap();

    // Gather results using regex and write local-zone entries to destination file buffer
    let re = regex::Regex::new(r"(?m)^0.0.0.0 (\S+)").unwrap();
    let mut hosts_buf = BufWriter::new(&dest);
    for cap in re.captures_iter(&hosts) {
        writeln!(&mut hosts_buf, "local-zone: \"{}\" refuse", &cap[1]).unwrap();
    }

    println!("✔\n")
}
