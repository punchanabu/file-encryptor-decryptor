use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{thread_rng, RngCore};
use std::string::String;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>,String> {
    
    let mut iv = [0u8, 16];
    thread_rng().fill_bytes(&mut iv);


    let cipher = Aes256Cbc::new_var(key, &iv).unwrap();

    let mut buffer = [iv.to_vec(), data.to_vec()].concat();

    let cipher_text = cipher.encrypt_vec(&mut buffer);

    Ok(cipher_text)

}

pub fn decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>,String> {
    
    if data.len() < 16 {
        return Err(String::from("Data is too short to be encrypted"));
    }

    let iv = &data[0..16];
    let encrypted_data = &data[16..];

    let cipher = Aes256Cbc::new_var(key, iv).unwrap();

    cipher.decrypt_vec(encrypted_data).map_err(|_| String::from("Decryption failed"))

}