use std::collections::HashMap;

use crate::settings::Settings;


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

pub async fn get(uri: String) -> Result<String, Box<dyn std::error::Error>>{
    let resp = reqwest::get(uri)
        .await?
        .text()
        .await?;
    Ok(resp)
}


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
