# Encryption and Decryption Tools
This tools provide a simple way to encrypt and decrypt files using AES-256 encryption. it's written in Rust and use envoriment variables for managing encryption keys.

## Installation

1. Clone this repository
```bash
git clone https://github.com/punchanabu/file-encryptor-decryptor
```
2. Navigate to the project folder
```bash
cd file-encryptor-decryptor
```

## Usage
### Generating a Key
Basically you have to generate a encryption_key I provided you a function for that
```bash
cargo run generate_key
```
Then it will log a key in the console, copy it and save it in a .env file in the root of the project
```makefile
KEY=your_generated_key
```
### Encrypting a File
To encrypt a file you have to run the following command
```bash
cargo run encrypt <file_path> <output_path>
```
This will create a encrypted file in output_path directory.

### Decrypting a File
To decrypt a file:
```bash
cargo run decrypt <encrypted_file_path> <output_path>
```
This will create a decrypted file in output_path directory.

## Notes 
- This is a simple tool for encrypting and decrypting files, it's not intended to be used in production environments and this is my first rust project so it's not perfect.
