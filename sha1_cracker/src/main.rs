/* Simple hash_cracker logic;
 * In real world it would be better to use a hashcat or John the Ripper in order to
 * crack things with better performance; And it would also be a good idea to load the whole
 * passwords file in memory instead of cursoring it.
 */

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();

        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }
    println!("Password not found in wordlist :(");

    Ok(())
}
