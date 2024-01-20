mod encryption;
mod file_io;

use std::path::Path;
use encryption::{encrypt, decrypt, Aes256Cbc};
use file_io::{read_file_to_string, write_string_to_file};
use aes::{Aes256, NewBlockCipher};
use generic_array::GenericArray;
use rand::{thread_rng, RngCore};
use base64::{encode, decode}; // Import decode function as well

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // File paths
    let path = Path::new("bitcoin-svgrepo-com.svg");
    let encrypted_path = Path::new("encrypted.svg");
    let decrypted_path = Path::new("decrypted.svg");

    
    let input_data = read_file_to_string(path)?;

    // Generate encryption key
    let mut key = GenericArray::default();
    thread_rng().fill_bytes(&mut key);
    let key = Aes256::new(&key);
    let copykey = key.clone(); // Clone the key for decryption

    // Encrypt data
    let encrypted_data = encrypt(input_data.as_bytes(), key)?;
    let encrypt_str = encode(&encrypted_data); // Encode to Base64 string

    // Write encrypted data to file
    write_string_to_file(encrypted_path, &encrypt_str)?;

    // Read the Base64 encoded encrypted data back from the file
    let encrypted_base64 = read_file_to_string(encrypted_path)?;

    // Decode the Base64 string back to a byte array
    let encrypted_data = decode(&encrypted_base64)?;

    // Decrypt data
    let decrypt_data = decrypt(&encrypted_data, copykey)?;

    // Write decrypted data to file
    match write_string_to_file(decrypted_path, std::str::from_utf8(&decrypt_data)?) {
        Ok(_) => println!("Decrypted file created successfully."),
        Err(e) => println!("Error writing file: {}", e),
    }

    Ok(())
}
