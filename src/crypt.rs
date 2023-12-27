extern crate crypto;

use crypto::aes::{ ctr, KeySize };

use std::{
    fs::{File, remove_file},
    io::{Read, Write},
    error::Error
};

pub const ENCRYPTED_FILE_EXT: &str = "block";

pub fn encrypt_file(path: &str, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open(path)?;

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let mut output_data = vec![0; input_data.len()];

    let mut cipher = ctr(
        KeySize::KeySize128, 
        key,
        &[0; 16]
    );

    cipher.process(&mut input_data, &mut output_data);

    let mut output_file = File::create(format!("{}.{}", path, ENCRYPTED_FILE_EXT))?;
    output_file.write_all(&output_data)?;

    match remove_file(path) {
        Ok(_) => (),
        Err(e) => println!("Erro ao deletar o arquivo: {}", e),
    }

    Ok(())
}
