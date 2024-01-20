mod encryption;
mod file_io;

use std::path::{Path, PathBuf};
use encryption::{encrypt, decrypt};
use file_io::{read_file_to_string, write_string_to_file};
use aes::{Aes256, NewBlockCipher};
use generic_array::GenericArray;
use rand::{thread_rng, RngCore};
use base64::{encode, decode}; 
use std::env;
use dotenv::dotenv;

enum Mode {
    Encrypt,
    Decrypt,
    GenerateKey,
}

impl Mode {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "encrypt" => Ok(Mode::Encrypt),
            "decrypt" => Ok(Mode::Decrypt),
            "generate_key" => Ok(Mode::GenerateKey),
            _ => Err("Invalid mode. Must be either 'encrypt' or 'decrypt'"),
        }
    }
}

fn encrypt_file(input_path : &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    
    let encrypted_path = PathBuf::from("encrypted.txt");

    let input_data = read_file_to_string(input_path)?;

    let key = env::var("KEY").expect("KEY must be set");    
    let key_bytes = decode(&key)?;
    if key_bytes.len() != 32 {
        return Err("KEY must be 32 bytes long".into());
    }
    let key = GenericArray::from_slice(&key_bytes);
    let key = Aes256::new(&key);

    let encrypted_data = encrypt(input_data.as_bytes(), key)?;
    let encrypt_str = encode(&encrypted_data);

    write_string_to_file(&encrypted_path, &encrypt_str)?;
    println!("Encrypted file created successfully.");
    Ok(encrypted_path)
}

fn decrypt_file(input_path : &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    
    let decrypted_path = PathBuf::from("decrypted.txt");

    let encrypted_base64 = read_file_to_string(input_path)?;
    let encrypted_data = decode(&encrypted_base64)?;
    let key = env::var("KEY").expect("KEY must be set");
    let key_bytes = decode(&key)?;
    if key_bytes.len() != 32 {
        return Err("KEY must be 32 bytes long".into());
    }
    let key = GenericArray::from_slice(&key_bytes);
    let key = Aes256::new(&key);

    let decrypt_data = decrypt(&encrypted_data, key)?;

    match write_string_to_file(&decrypted_path, std::str::from_utf8(&decrypt_data)?) {
        Ok(_) => println!("Decrypted file created successfully."),
        Err(e) => println!("Error writing file: {}", e),
    }

    Ok(decrypted_path)
}

fn generate_key() -> Result<(), Box<dyn std::error::Error>> {
    // generate 32 bytes long key
    let mut key = [0u8; 32];
    thread_rng().fill_bytes(&mut key);
    let key = encode(&key);
    println!("Generated key (Base64): {}", key);
    println!("Generated key length (Base64): {}", key.len());

    // Decoding back to check the length
    let decoded_key = decode(&key)?;
    println!("Decoded key length: {}", decoded_key.len());

    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Invalid number of arguments. Must be both <'encrypt' or 'decrypt'> and <path>".into());
    }

    let mode = Mode::from_str(&args[1])?;
    let path = Path::new(&args[2]);
    match mode {
        Mode::Encrypt => {
            let encrypted_path = encrypt_file(path)?;
            println!("Encrypted file is at {}", encrypted_path.display());
        },
        Mode::Decrypt => {
            let decrypted_path =  decrypt_file(path)?;
            println!("Decrypted file is at {}", decrypted_path.display());
        },
        Mode::GenerateKey => generate_key()?,
    }

    Ok(())
}
