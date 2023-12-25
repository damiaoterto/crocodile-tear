use std::{
    fs::File,
    io::{Error, Read}
};

use crypto::{
    symmetriccipher::SynchronousStreamCipher,
    aes::{KeySize, ctr},
    blockmodes::CtrMode,
    buffer::{BufferResult, ReadBuffer, WriteBuffer},
};

pub fn encrypt_file(
    path: &str, 
    key: &[u8; 32], 
    nonce: &[u8; 24],
) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let mut plaintext = Vec::new();

    file.read_to_end(&mut plaintext)?;
    
    let mut output_data = Vec::new();
    let mut cipher = ctr(KeySize::KeySize128, key, &[0; 16]);

    
}