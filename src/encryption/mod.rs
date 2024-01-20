use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{thread_rng, RngCore};
use std::string::String;

pub type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt(data: &[u8], key: Aes256) -> Result<Vec<u8>,String> {
    
    let mut iv = generic_array::GenericArray::default();
    thread_rng().fill_bytes(&mut iv);


    let cipher = Aes256Cbc::new(key, &iv);

    let mut buffer = [iv.to_vec(), data.to_vec()].concat();

    let cipher_text = cipher.encrypt_vec(&mut buffer);

    Ok(cipher_text)

}
