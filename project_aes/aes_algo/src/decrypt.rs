use std::fmt::Error;
use std::fs;
use super::*;

pub fn decrypt(key:&[u8],iv:&[u8],encrypted_file: &str) -> Result<(),Error>{
    let mut ciphertext = base64::decode(encrypted_file).unwrap();
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt(&mut ciphertext).unwrap();
    println!("{}", str::from_utf8(decrypted_ciphertext).unwrap());
    Ok(())
}