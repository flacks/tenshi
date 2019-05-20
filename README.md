# tenshi

This utility fetches a curated hosts file, generates Unbound local-data from it, and saves the result in `/etc/unbound/local-blocking-data.conf`.

At the moment, only Steven Black's hosts file is used. Source: https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts.

This utility is meant to work on Linux. It is untested on Mac, and will probably not work on Windows.

You will want to add `include: "local-blocking-data.conf"` to your Unbound config to use the result.

This package is available on crates.io at https://crates.io/crates/tenshi, and the Arch User Respository at https://aur.archlinux.org/packages/tenshi-rs.

## Installation

### Prerequisites

Install either [rustup](http://rustup.rs/) or `rust` using your favorite package manager.

This utility has been tested to work with Rust 1.34.2.

`sudo` is needed if you do not have write access to `/etc/unbound/local-blocking-data.conf`.

### Build with Cargo

`cargo install tenshi`

### Arch Linux

```
git clone https://aur.archlinux.org/tenshi-rs.git
cd tenshi
makepkg -i
```

### Build from source

```
git clone https://github.com/flacks/tenshi.git
git checkout $(git tag | tail -n 1) # Optional
cd tenshi
cargo build --release
cargo install --path .
```

## Synopsis

If installed with Cargo or built from source, run `sudo tenshi`, excluding `sudo` if you have write access to `/etc/unbound/local-blocking-data.conf`.

If installed from the Arch User Repository, run `sudo tenshi-rs`, heeding the same `sudo` exception above.

Utility will prompt to continue. For promptless execution, prepend `echo y |` to command.

## To do

- [ ] *Actually* support different hosts files
  * Upstream variants
- [ ] Custom hosts files support
- [ ] Argument support
- [ ] Quiet mode
- [ ] Support changing data destination
- [ ] Modularize code

## Credits

Inspired by https://github.com/gbxyz/unbound-block-hosts

A kind Rust Discord server user on #beginners
