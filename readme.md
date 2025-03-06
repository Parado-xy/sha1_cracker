# SHA-1 Password Cracker

A simple SHA-1 hash cracker written in Rust that attempts to find the original password from a SHA-1 hash using a dictionary attack.

## Overview

This tool takes a SHA-1 hash and attempts to crack it by trying passwords from a provided wordlist. It's a demonstration of how hash cracking works and why using strong, unique passwords is important.

## Prerequisites

- Rust and Cargo installed on your system
- A wordlist file (provided as `wordlist.txt`)

## Dependencies

- `sha-1`: For SHA-1 hash computation
- `hex`: For hexadecimal encoding/decoding

## Usage

Run the program with a wordlist and the SHA-1 hash you want to crack:

```sh
cargo run -- wordlist.txt <sha1_hash>
```

### Example

```sh
cargo run -- wordlist.txt 8d6e34f987851aa599257d3831a1af040886842f
```

Output:
```
Password Found: sunshine
```

## How It Works

1. The program takes a wordlist file and a SHA-1 hash as command-line arguments
2. It reads the wordlist file line by line
3. For each password candidate, it:
   - Converts the password to bytes
   - Computes the SHA-1 hash
   - Compares it with the target hash
4. If a match is found, it displays the cracked password
5. If no match is found in the wordlist, it reports that the password wasn't found

## Security Note

This tool is for educational purposes only. SHA-1 is considered cryptographically broken and should not be used for secure applications.

## License

MIT