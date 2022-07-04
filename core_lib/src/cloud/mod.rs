pub mod aws;
pub mod aws_config;

use std::collections::HashMap;
use aws::parse_execute_aws;
use std::io::{Error,ErrorKind};

pub async fn parse_execute(conf: HashMap<String,String>) -> Result<(),Error>{
    println!("{:?}", conf);
    // Parse cloud config
    if !conf.contains_key("class"){
       return Err(Error::new(
            ErrorKind::Other,
            format!("No Cloud class Specified"),
        ));
    }
    let class_params: Vec<&str> = conf.get("class").unwrap().trim().split(".").collect();
    if class_params.len() < 3{
       return Err(Error::new(
            ErrorKind::Other,
            format!("Invalid cloud class."),
        ));
    }
    match class_params[0] {
        "aws" => {
            return parse_execute_aws(class_params, conf.to_owned()).await;
        },
        _ => {
           return Err(Error::new(
                ErrorKind::Other,
                format!("Invalid provider"),
            ));
        }
    }
}
