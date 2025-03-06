//  SHA-1 is a hash function used by a lot of old websites to store the passwords of the users. In
//  theory, a hashed password can’t be recovered from its hash. Thus by storing the hash in their
//  database, a website can assert that a given user has the knowledge of its password without
//  storing the password in cleartext, by comparing the hashes. So if the website’s database is
//  breached, there is no way to recover the passwords and access the users’ data.
//  Reality is quite different. Let’s imagine a scenario where we just breached such a website, and
//  we now want to recover the credentials of the users in order to gain access to their accounts.

// This is where a hash cracker is useful. A hash cracker is a program that will try many different hashes in order to find the original password.

// Let's get some cli arguments.
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

// Get The sha1 crate
use sha1::{ Digest};

// Default length for the hashed  cli argument
const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the CLI args.
    let args: Vec<String> = env::args().collect(); // env::args() returns an iterator that can be collected with the .collec() method.

    if args.len() != 3 {
        panic!(
            "
            Error, to little CLI args. 
            sha1_cracker: <wordlist.txt> <sha1_hash>
        "
        );
    };

    let hash_to_crack = args[2].trim(); // Access the third element as the HashToCrack then remove white spaces with .trim()

    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("SHA 1 Hash not valid".into());
    }

    // Open the file passed as the first cli arg.
    let word_list = File::open(&args[1])?;
    let reader = BufReader::new(&word_list);

    for line in reader.lines() {
        let line = line?.trim().to_string();
        let common_password = line.trim();
        // Compare the hash to the current common_password
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password Found: {common_password}");
            return  Ok(());
        }
        
    }

    println!("Password Not Found"); 

    Ok(())
}

// Example: cargo run -- wordlist.txt 8d6e34f987851aa599257d3831a1af040886842f
// Output: Password Found: sunshine
