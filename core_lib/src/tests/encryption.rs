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

#[test]
fn sign_unsign(){
    let signdata = encryption::sign("uU0nuZNNPgilLlLX2n2r+sSE7+N6U4DukIj3rOLvzek=".to_string());
    assert_eq!(encryption::verify("uU0nuZNNPgilLlLX2n2r+sSE7+N6U4DukIj3rOLvzek=".to_string(), signdata),true);
}
