
use std::collections::HashMap;
use crate::settings::Settings;



/*
pub fn test(uri: String) -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new();
    let BaseURL = settings.get::<String>("main.url").unwrap();
    let URL = [BaseURL, uri].join("");
    println!("URL {}",URL);
    let resp = reqwest::blocking::get("http://google.com")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
*/

/*
pub async fn get(uri: String) -> Result<String, Box<dyn std::error::Error>>{
    let resp = reqwest::get(uri)
        .await?
        .text()
        .await?;
    Ok(resp)
}
*/

/*
pub async fn get2(uri: &str) ->Result<String,reqwest::Error>{
    let response = reqwest::get(uri).await;
    if let Err(e) = response {
        if e.is_connect() {
            println!("Unable to connect");
        }
        return Err(e)
    }
    let x = response?.text().await?;
    Ok(x)
}
*/


pub async fn get(uri: &str) -> Result<reqwest::Response,reqwest::Error>{
    let settings = Settings::new();
    let mut url = String::new();
    url.push_str(&settings.get::<String>("main.url").unwrap());
    url.push_str(&uri);
    let response = reqwest::get(&url).await;
    if let Err(e) = response {
        if e.is_connect() {
            log::error!("Unable to connect to url: {}",&url);
        } else if e.is_timeout(){
            log::error!("Url: {} Timeout", &url);
        }
        if e.is_status(){
            println!("404");
        }
        return Err(e)
    }


    response
}

pub async fn post(uri: &str , payload: HashMap<&str, &str>) -> Result<reqwest::Response,reqwest::Error>{
    // Post data to a url
    let settings = Settings::new();
    let mut url = String::new();
    url.push_str(&settings.get::<String>("main.url").unwrap());
    url.push_str(&uri);


    let client = reqwest::Client::new();
    let response = client.post(&url).json(&payload).send().await;

    if let Err(e) = response {
        if e.is_connect() {
            log::error!("Unable to connect to url: {}",&url);
        } else if e.is_timeout(){
            log::error!("Url: {} Timeout", &url);
        }
        if e.is_status(){
            println!("404");
        }
        return Err(e)
    }
    response
}
