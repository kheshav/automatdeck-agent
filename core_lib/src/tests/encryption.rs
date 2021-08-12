#[allow(unused_imports)]
use crate::encryption;


#[test]
fn encrypt_decrypt(){
    // Test encryption and decrypt
    let encryptdata = encryption::encrypt("loll".to_string());
    let decrypt = encryption::decrypt(encryptdata.to_owned());
    assert_eq!("loll",decrypt);

    let encryptdata = encryption::encrypt("".to_string());
    let decrypt = encryption::decrypt(encryptdata.to_owned());
    assert_eq!("",decrypt);
}
