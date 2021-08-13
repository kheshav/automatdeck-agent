use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey, pkcs1::FromRsaPrivateKey, pkcs8::FromPublicKey};
use rand::rngs::OsRng;
use std::path::Path;
#[allow(unused_imports)]
use crate::settings::Settings;

use sha2::{Sha512, Digest};


pub fn encrypt(message: String) -> String{
    // Encrypt message using the public key
    let mut rng = OsRng;
    
    let public_key;
    if cfg!(debug_assertions) {
        public_key = RsaPublicKey::read_public_key_pem_file(Path::new("/Users/kheshavsewnundun/Projects/automatdeck/agent/keys/public-key.pem")).expect("Error reading public key");
    } else {
        let settings = Settings::new();
        public_key = RsaPublicKey::read_public_key_pem_file(Path::new(&settings.get::<String>("security.key_path").unwrap_or_default())).expect("Error reading public key");
    }
    
    let data = message.as_bytes();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key.encrypt(&mut rng, padding, &data[..]).expect("failed to encrypt");

    let mut final_data = "".to_string();
    let mut total_data = enc_data.len();
    // Transform the return output to string bytes separated by space
    for byte_data in enc_data.to_owned(){
        total_data -= 1;
        final_data.push_str(&format!("{}",&byte_data));
        if total_data >0 {
            final_data.push_str(" ");
        }
    }
    return final_data;
}

pub fn decrypt(message: String) -> String{
    // Decrypt the message using private key
    let private_key = RsaPrivateKey::read_pkcs1_pem_file(Path::new("/Users/kheshavsewnundun/Projects/automatdeck/agent/keys/private-key.pem")).expect("Error reading private key");
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let mut vec_data: Vec<u8> = Vec::new();

    // Convert string of bytes to bytes
    for hex in message.split_whitespace(){
        vec_data.push(hex.parse::<u8>().unwrap());
    }
    let dec_data = private_key.decrypt(padding, &vec_data).expect("failed to decrypt");
    return String::from_utf8(dec_data).expect("Found invalid UTF-8");
}

pub fn sign(message: String) -> Vec<u8>{
    // Sign message
    let private_key = RsaPrivateKey::read_pkcs1_pem_file(Path::new("/Users/kheshavsewnundun/Projects/automatdeck/agent/keys/private-key.pem")).expect("Error reading private key");
    let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::hash::Hash::SHA2_512));
    let data = message.as_bytes();
    let mut hasher = Sha512::new();
    hasher.update(data);
    let data = hasher.finalize();


    let enc_data = private_key.sign(padding, &data[..]).expect("failed to sign");
    /*
    let mut final_data = "".to_string();
    let mut total_data = enc_data.len();
    // Transform the return output to string bytes separated by space
    for byte_data in enc_data.to_owned(){
        total_data -= 1;
        final_data.push_str(&format!("{}",&byte_data));
        if total_data >0 {
            final_data.push_str(" ");
        }
    }
    return final_data;
    */
    return enc_data;
}

pub fn verify(message: String, sig: Vec<u8>) -> bool{
    // Verify a message with signature
    let public_key;
    if cfg!(debug_assertions) {
        public_key = RsaPublicKey::read_public_key_pem_file(Path::new("/Users/kheshavsewnundun/Projects/automatdeck/agent/keys/public-key.pem")).expect("Error reading public key");
    } else {
        public_key = RsaPublicKey::read_public_key_pem_file(Path::new("/etc/ad-agent/keys/ag-agent-public-key.pem")).expect("Error reading public key");
    }
    
    let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::hash::Hash::SHA2_512));

    let mut hasher = Sha512::new();
    hasher.update(message.as_bytes());
    let data = hasher.finalize();
    let result = public_key.verify(padding, &data[..], &sig);
    if let Ok(_) = result{
        return true;
    }
    return false;
}
