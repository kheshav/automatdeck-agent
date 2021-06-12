use crate::httpclient;

struct License{
    result: String,
}

pub fn is_valid(access_key: String, secret_key: String) -> bool{
    // Check if a license is valid or not
    let query = httpclient::get("/license/");
    false
}
