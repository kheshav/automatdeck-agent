use serde::Deserialize;
use std::collections::HashMap;
use crate::httpclient;
use derive_getters::Getters;


#[derive(Debug,Getters,Deserialize)]
pub struct Request{
    status: String,
    message: Vec<RequestData>
}


#[derive(Debug,Getters,Deserialize)]
pub struct Config{
    title: String,
    config: String
}

#[derive(Debug,Getters,Deserialize)]
pub struct  RequestData{
   id: i64,
   config: HashMap<String, String>,
   status: String,
   meta: String,
}


impl Request{

    #[allow(dead_code)]
    pub async fn get_request() -> Result<Request, String>{
        // Check for new requests
        log::info!("Checking new requests");
        let query = httpclient::get("/requests/").await;
        match query {
            Ok(response) => {
                println!("OK");
                match response.json::<Request>().await{
                    Ok(result) => {
                        return Ok(result);
                    },
                    Err(_) => {
                        log::error!("Unable to parse request data");
                        return Err(String::from("Error parsing json"));
                    }
                }
            },
            Err(_) => {
                log::error!("An unexpected error occured while getting new requests");
            }
        };
        return Err(String::from("Failed"));
    }
}

