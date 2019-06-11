mod check;
mod fetch;
mod parse;
mod prompt;

fn main() {
    prompt::confirm();

    let dest = check::write_access();

    let hosts = fetch::steven_black();

    parse::generate(dest, hosts);
}
