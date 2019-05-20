# Tenshi

[![crates.io](https://img.shields.io/crates/v/tenshi.svg)](https://crates.io/crates/tenshi)
[![aur](https://img.shields.io/aur/version/tenshi-rs.svg)](https://aur.archlinux.org/packages/tenshi-rs)

Tenshi is a fast CLI utility that fetches a curated hosts file, generates [Unbound](https://nlnetlabs.nl/projects/unbound/about/) local-zone entries from it, and saves them to `/etc/unbound/local-blocking-data.conf`.

At the moment, only [Steven Black's hosts file](https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts) is used.

Tenshi is primarily written to work on Linux, but it will likely work on the BSDs. It is untested on Macintosh, and will probably not work on Windows.

You will want to add `include: "local-blocking-data.conf"` to your Unbound config to use the result.

## Installation

### Prerequisites

Install either [rustup](https://rustup.rs) or `rust` using your favorite package manager. Tenshi is known to build against Rust 1.34.2.

Tenshi must be run as a user that has write access to `/etc/unbound/local-blocking-data.conf`.

### Build with Cargo

`cargo install tenshi`

### Build from source

```
git clone https://github.com/flacks/tenshi.git
cd tenshi
git checkout $(git tag | tail -n 1)
cargo build --release
cargo install --path .
```

### Arch Linux

```
git clone https://aur.archlinux.org/tenshi-rs.git
cd tenshi-rs
makepkg -i
```

## Synopsis

See note on write access in the **Prerequisites** section.

If installed with Cargo or built from source, ensure Cargo's bin directory is in your PATH, then run `tenshi` and follow prompt.

If built from the AUR, run `tenshi-rs`.

For promptless execution, prepend `true |` to command.

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

A kind Rust Discord server user in `#beginners`
