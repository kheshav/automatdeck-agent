use serde::Deserialize;
use std::collections::HashMap;
use crate::httpclient;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Request{
    status: String,
    message: Vec<RequestData>
}


#[derive(Deserialize)]
#[derive(Debug)]
struct Config{
    title: String,
    config: String
}

#[derive(Deserialize)]
#[derive(Debug)]
struct  RequestData{
   id: i32,
   //config: Vec<Config>,
   config: HashMap<String, String>,
   status: String,
   meta: String,
}


/*
#[derive(Debug)]
pub struct Request{
    requests: Vec<RequestData>
}
*/
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

    #[allow(dead_code)]
    pub fn change_status(self){
        println!("{}",self.status);

    }
}


