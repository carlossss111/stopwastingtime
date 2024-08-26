# Stop Wasting Time!

A program for when you need to lock-in and stop wasting time on pointless websites. This simple executable for Linux temporarily blocks domains of choice. 

This is my first Rust project and was created so that I can practise the language.

## Compilation
Compile with Rustup and Cargo.
```
cargo build
```

## Usage
Blacklist domains by editing /etc/hosts.
```
Usage: stopwastingtime [OPTIONS] [state]

Arguments:
  [state]  Desired state of the program [possible values: on, off]

Options:
  -l, --list             Lists blacklisted domains
  -a, --add <domain>     Add domain to blacklist
  -r, --remove <domain>  Remove domain from blacklist
```
