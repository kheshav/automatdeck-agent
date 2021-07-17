use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use crate::httpclient;
use derive_getters::Getters;


#[derive(Debug)]
#[allow(dead_code)]
pub enum RequestStatus{
    COMPLETED,
    FAILED,
    PROCESSING,
    WARNING,
}

#[derive(Debug,Getters,Deserialize,Serialize,Clone)]
pub struct Request{
    status: String,
    message: Vec<RequestData>
}


#[derive(Debug,Getters,Deserialize,Serialize,Clone)]
pub struct  RequestData{
   id: i64,
   config: String,
   title: String,
   valid: bool,
   status: String,
   meta: String,
}


impl RequestStatus{

    #[allow(dead_code)]
    fn value(&self) -> &str{
        match *self{
            RequestStatus::PROCESSING => "P",
            RequestStatus::COMPLETED => "C",
            RequestStatus::FAILED => "F",
            RequestStatus::WARNING => "W"
        }
    }
}


impl Request{

    #[allow(dead_code)]
    pub async fn get_request() -> Result<Request, String>{
        // Check for new requests
        log::info!("Checking new requests");
        let query = httpclient::get("/requests/").await;
        match query {
            Ok(response) => {
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
    

    pub async fn set_status(status: RequestStatus,req: RequestData) -> bool{
        // Set the status of the request 
        log::debug!("Updating Status of request id: {} to {}", req.id(), status.value());
        let mut uri = "/requests/".to_string();
        uri.push_str(&req.id().to_string());
        uri.push_str("/");
        let mut data = HashMap::new();
        data.insert("status", status.value());
        let query = httpclient::patch(&uri, data).await;
        match query {
            Ok(response) => {
                match response.error_for_status(){
                    Ok(_) => {
                        return true;
                    },
                    Err(_) => {
                        log::error!("Unable to set status for request: {}",req.id());
                        return false;
                    }
                }
            },
            Err(_) => {
                log::error!("An unexpected error occured while setting status for request id: {}",req.id());
            }
        };
        return true;
    }
}

