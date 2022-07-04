use async_trait::async_trait;

pub struct AwsFactory;

use std::collections::HashMap;
use crate::cloud::aws_config::s3 as s3Config;
use cloud_lib::aws::s3::S3Client;
use std::io::{Error, ErrorKind};



#[async_trait]
trait Factory<S3: cloud_lib::aws::s3::S3Client>{
    async fn generate_s3(&self,bucketname: String, region:Option<String>) -> S3;
}

#[async_trait]
impl Factory<cloud_lib::aws::s3::CLIENT> for AwsFactory{
    async fn generate_s3(&self,bucketname: String, region:Option<String>) -> cloud_lib::aws::s3::CLIENT {
       return cloud_lib::aws::s3::generate_client(bucketname, region).await; 
    }
}


pub async fn parse_execute_aws(class_params: Vec<&str>, conf: HashMap<String,String>) -> Result<(),Error>{
    // aws parser
    let factory = AwsFactory{};
    match class_params[1] {
        "s3" => {
            if conf.contains_key("bucketname"){
                let s3 = factory.generate_s3(conf.get("bucketname").unwrap().to_string(), Some("".to_string())).await;
                match class_params[2] {
                    "create_bucket" =>{
                       if ! s3Config::check_s3_bucketcreate_params(conf){
                           return Err(Error::new(
                                ErrorKind::Other,
                                format!("[Cloud] Invalid parameters for s3 create_bucket"),
                            ));
                       }
                       let response = s3.create_bucket().await;
                       match response {
                            Ok(()) => { return Ok(());},
                            Err(_err) => {
                                #[cfg(debug_assertions)]
                                println!("Cloud: {}",_err.to_string());
                               return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("[Cloud] {}",_err.to_string()),
                                ));
                            }
                       }
                    },
                    "remove_object" => {
                       if ! s3Config::check_s3_remove_object_params(conf.to_owned()){
                           //return false;
                           return Err(Error::new(
                                ErrorKind::Other,
                                format!("[Cloud] Invalid parameters for S3 remove_object."),
                            ));
                       }
                       let response = s3.remove_object(conf.get("object").unwrap().to_string()).await;
                       match response {
                            Ok(()) => { return Ok(());},
                            Err(_err) => {
                               return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("[Cloud] {}",_err.to_string()),
                                ));
                            }
                       }
                    },
                    "copy_object" => {
                       if ! s3Config::check_s3_copy_object_params(conf.to_owned()){
                           //return false;
                           return Err(Error::new(
                                ErrorKind::Other,
                                format!("[Cloud] Invalid parameters for s3 copy_object."),
                            ));
                       }
                       let response = s3.copy_object(conf.get("destination_bucket").unwrap().to_string(), conf.get("source_object").unwrap().to_string(), conf.get("destination_object").unwrap().to_string()).await;
                       match response {
                            Ok(()) => { return Ok(());},
                            Err(_err) => {
                               return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("[Cloud] {}",_err.to_string()),
                                ));
                            }
                       }
                    },
                    _ => {} 
                }
            }
        },
        _ => {}
    }
    return Err(Error::new(
        ErrorKind::Other,
        format!("[Cloud] Provider resource invalid"),
    ));
}
