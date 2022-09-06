mod args;
// use args::{UserSubCommand, CLIAPP};
use args::{CLIAPP,UserSubCommand};
use clap::Parser;
use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
mod secret;
use secret::Secret;
mod encrypt;
use encrypt::*;
mod decrypt;
use decrypt::*;
pub type Aes128Cbc = Cbc<Aes128, Pkcs7>;
use hex::*;
use hex_literal::*;
use rand::*;
use std::*;
use std::fs::File;
use std::io::Write;
use anyhow::anyhow;
use base64::*;

fn main() {
    let args = CLIAPP::parse();
    match args.actions {
    UserSubCommand::Encrypt{ file_path , key } => {
    let key_file = fs::read(key).unwrap();
    let secret : Secret = serde_json::from_slice(&key_file).unwrap();
    let key = base64::decode(secret.keys()).unwrap();
    let iv = base64::decode(secret.iv()).unwrap();
    let result = encrypt_file(file_path.as_str() , &key , &iv);
    match result {
        Ok(()) => {
            println!("File Encrypted")
        },
        Err(err) => println!("Encryption failed : {}" , err),

    }
},
UserSubCommand::Decrypt { file_path, key } => {
    let file = fs::read_to_string(file_path).unwrap();
   println!("{}",file);
    let key_file = fs::read(key).unwrap();
    let secret : Secret = serde_json::from_slice(&key_file).unwrap();
    let key = base64::decode(secret.keys()).unwrap();
    let iv = base64::decode(secret.iv()).unwrap();
    let result = decrypt( &key , &iv,file.as_str() );
    match result {
        Ok(()) => {
            println!("File Decrypted")
        },
        Err(err) => println!("Decryption failed : {}" , err),

    }
        
},

UserSubCommand::Generate => {
    let secret = Secret::generate_secret();
        let secret_bytes = serde_json::to_vec(&secret).unwrap();
        let mut file = File::create("secret.txt").unwrap();
        file.write_all(&secret_bytes);
        println!("Secret Generated");
       }
    }
}