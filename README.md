# Stop Wasting Time!

A program for when you need to lock-in and stop wasting time on pointless websites. This simple executable for Linux temporarily blocks domains of choice. 

This is my first Rust project and was created so that I can practise the language. Use with caution.

## Compilation
Compile with Rustup and Cargo.
```
cargo build --release
```
The binary will be created as `target/release/stopwastingtime`.

## Usage
Blacklist domains by editing /etc/hosts.
This program needs root priveleges to change the hostfile, so you can use sudo.

Create a backup of your hostfile before first time use.
```
sudo cp /etc/hosts /etc/hosts.backup
```

Add to a blacklist with
```
sudo stopwastingtime -a "www.domain.com"
```
Turn it on/off with
```
sudo stopwastingtime on
sudo stopwastingtime off
```
For more options, run with `--help`.

## Setuid Bit
**Disclaimer: Only follow this step if you understand the security risks.**

To skip manual authentication with sudo, you can change the permissions and add a setuid bit.
```
sudo chown root <binary_name>
sudo chgrp root <binary_name>
sudo chmod 4755 <binary_name>
```
Since rust is memory-safe, the program should be safe with these priveleges.
