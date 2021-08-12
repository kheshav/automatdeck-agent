use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey, pkcs1::FromRsaPrivateKey, pkcs8::FromPublicKey};
use rand::rngs::OsRng;
use std::path::Path;

pub fn encrypt(message: String) -> String{
    // Encrypt message using the public key
    let mut rng = OsRng;
    let public_key = RsaPublicKey::read_public_key_pem_file(Path::new("/Users/kheshavsewnundun/Projects/automatdeck/agent/keys/public-key.pem")).expect("Error reading public key");
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
