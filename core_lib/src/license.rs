use std::{panic::panic_any, process};
use crate::httpclient;
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
struct License{
    status: String,
    message: HashMap<String,String>,
}

pub async fn check_license(){
    // Check if a license is valid or not
    log::info!("Checking license validity..");
    let query = httpclient::get("/license/").await;
    match query {
        Ok(response) => {
                            if !response.status().is_success(){
                                log::error!("Unable to check license!!..exiting application");
                                process::exit(1);
                            } else {
                                log::info!("Detected valid license.");
                                match response.json::<License>().await{
                                    Ok(result) =>{
                                               match result.message.get("expiry_date"){
                                                    Some(expiry_date) => log::info!("Detected license expiry_date (YYYY-MM-DD): {}",expiry_date),
                                                    None => log::warn!("Could not detect license expiry_date"), 
                                               }
                                            },

                                    Err(_) => log::error!("Unable to detect expiry")
                                };

                            }
                        },
        Err(e) => {
                    log::error!("An unexpected error occured exiting application...");
                    panic_any(e.to_string());
                  }
    };
}
