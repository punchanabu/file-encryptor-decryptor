mod encryption;
mod file_io;

use std::path::Path;
use encryption::{encrypt, decrypt};
use file_io::{read_file_to_string, write_string_to_file};
use aes::{Aes256, NewBlockCipher};
use generic_array::GenericArray;
use rand::{thread_rng, RngCore};
use base64::{encode, decode, write}; 
use std::env;
use dotenv::dotenv;

enum Mode {
    Encrypt,
    Decrypt,
}

impl Mode {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "encrypt" => Ok(Mode::Encrypt),
            "decrypt" => Ok(Mode::Decrypt),
            _ => Err("Invalid mode. Must be either 'encrypt' or 'decrypt'"),
        }
    }
}

fn encrypt_file() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("bitcoin-svgrepo-com.svg");
    let encrypted_path = Path::new("encrypted.svg");

    let input_data = read_file_to_string(path)?;

    let key = env::var("KEY").expect("KEY must be set");    
    if key.len() != 32 {
        return Err("KEY must be 32 bytes long".into());
    }
    let key = GenericArray::from_slice(key.as_bytes());
    let key = Aes256::new(&key);

    let encrypted_data = encrypt(input_data.as_bytes(), key)?;
    let encrypt_str = encode(&encrypted_data);

    write_string_to_file(encrypted_path, &encrypt_str)?;
    println!("Encrypted file created successfully.");
    Ok(())
}

fn decrypt_file() -> Result<(), Box<dyn std::error::Error>> {
    let encrypted_path = Path::new("encrypted.svg");
    let decrypted_path = Path::new("decrypted.svg");

    let encrypted_base64 = read_file_to_string(encrypted_path)?;
    let encrypted_data = decode(&encrypted_base64)?;
    let key = env::var("KEY").expect("KEY must be set");
    if key.len() != 32 {
        return Err("KEY must be 32 bytes long".into());
    }
    let key = GenericArray::from_slice(key.as_bytes());
    let key = Aes256::new(&key);

    let decrypt_data = decrypt(&encrypted_data, key)?;

    match write_string_to_file(decrypted_path, std::str::from_utf8(&decrypt_data)?) {
        Ok(_) => println!("Decrypted file created successfully."),
        Err(e) => println!("Error writing file: {}", e),
    }

    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Invalid number of arguments. Must be either 'encrypt' or 'decrypt'".into());
    }

    let mode = Mode::from_str(&args[1])?;

    match mode {
        Mode::Encrypt => encrypt_file(),
        Mode::Decrypt => decrypt_file(),
    }
}
