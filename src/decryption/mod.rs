use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::string::String;
use generic_array::GenericArray;

pub type Aes256Cbc = Cbc<Aes256, Pkcs7>;


pub fn decrypt(data: &[u8], key: Aes256) -> Result<Vec<u8>,String> {
    
    if data.len() < 16 {
        return Err(String::from("Data is too short to be encrypted"));
    }

    let iv = GenericArray::from_slice(&data[0..16]);
    let encrypted_data = &data[16..];

    let cipher = Aes256Cbc::new(key, &iv);

    cipher.decrypt_vec(encrypted_data).map_err(|_| String::from("Decryption failed"))

}