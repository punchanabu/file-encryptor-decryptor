mod file_io;
mod encryption;

use std::path::Path;
use encryption::{encrypt, decrypt, Aes256Cbc}; // Assuming Aes256Cbc is the type you're using for your key.
use file_io::{read_file_to_string, write_string_to_file};
use aes::{Aes256, NewBlockCipher};
use generic_array::GenericArray;
use rand::{thread_rng, RngCore};
use base64::{encode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // file path
    let path = Path::new("bitcoin-svgrepo-com.svg");
    let enccrypted_path = Path::new("encrypted.txt");
    let input_data = read_file_to_string(path)?;

    let mut key = GenericArray::default();
    thread_rng().fill_bytes(&mut key);
    let key = Aes256::new(&key);

    let encrypted_data = encrypt(input_data.as_bytes(), key)?;
    let encrypt_str = encode(encrypted_data);

    write_string_to_file(enccrypted_path, &encrypt_str)?;

    Ok(())
}

