use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration, Delete, ObjectIdentifier};
use aws_sdk_s3::{Client, Error, Region, PKG_VERSION};
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::Endpoint;
use http::Uri;
use std::path::Path;
use std::fmt;
use async_trait::async_trait;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl std::error::Error for MyError {}

pub struct CLIENT{
    client: aws_sdk_s3::Client,
    region: String,
    bucket: String
}

#[async_trait]
pub trait S3Client{
    async fn create_bucket(&self) ->Result<(), Error>;
    async fn copy_object(&self, destination_bucket: String,source_object: String, destination_object: String) -> Result<(), Error>;

    async fn remove_objects(&self, objects: Vec<String>) -> Result<(), Error>;
    async fn remove_object(&self,key:String) -> Result<(), Error>;
    async fn upload_object(&self, source_file: String, destination_file: String) -> Result<() , Box<dyn std::error::Error>>;
    fn test(&self);
}

#[async_trait]
impl S3Client for CLIENT{

    fn test(&self){
        println!("Test S3CLIENT");
    }

    async fn copy_object(&self, destination_bucket: String,source_object: String, destination_object: String) -> Result<(), Error>{
        // Copy object from one bucket to another
        let mut source_bucket_and_object: String = "".to_owned();
        source_bucket_and_object.push_str(&self.bucket);
        source_bucket_and_object.push('/');
        source_bucket_and_object.push_str(&source_object);

        self.client
        .copy_object()
        .copy_source(source_bucket_and_object)
        .bucket(destination_bucket)
        .key(destination_object)
        .send()
        .await?;

        Ok(())
    }

    async fn create_bucket(&self)->Result<(), Error>{
        // Create Bucket
        let constraint = BucketLocationConstraint::from(self.region.as_str());
        let cfg = CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();

        self.client
            .create_bucket()
            .create_bucket_configuration(cfg)
            .bucket(&self.bucket)
            .send()
            .await?;
        println!("Created bucket.");

        Ok(()) 
    }

    async fn remove_object(&self,key:String) -> Result<(), Error>{
        // Delete Object
        self.client
        .delete_object()
        .bucket(&self.bucket)
        .key(key)
        .send()
        .await?;

        Ok(())
    }

    async fn remove_objects(&self, objects: Vec<String>) -> Result<(), Error>{
        // Delete multiple objects
        let mut delete_objects: Vec<ObjectIdentifier> = vec![];

        for obj in objects {
            let obj_id = ObjectIdentifier::builder().set_key(Some(obj)).build();
            delete_objects.push(obj_id);
        }

        let delete = Delete::builder().set_objects(Some(delete_objects)).build();

        self.client
            .delete_objects()
            .bucket(&self.bucket)
            .delete(delete)
            .send()
            .await?;
        Ok(())
    }

    async fn upload_object(&self, source_file: String, destination_file: String) -> Result<() , Box<dyn std::error::Error>>{
        // Upload file to s3
        let body = ByteStream::from_path(Path::new(&source_file)).await;

        match body {
            Ok(b) => {
                let _resp = self.client
                    .put_object()
                    .bucket(&self.bucket)
                    .key(destination_file)
                    .body(b)
                    .send()
                    .await?;

                //println!("Upload success. Version: {:?}", _resp.version_id);
            }
            Err(_e) => {
                return Err(Box::new(MyError(format!("Cannot read file {}",&source_file).into())));
            }
        }
        return Ok(());
    }

}

pub async fn generate_client(bucket:String,region: Option<String>) -> CLIENT{
    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-1"));

    let r_rgr = region_provider.region().await.unwrap();
    let r_str = r_rgr.as_ref();


    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    // DEBUG
    /*
    let conf = aws_config::load_from_env().await;
    let ep = Endpoint::immutable(Uri::from_static("https://s3.us-west-004.backblazeb2.com"));
    let s3_conf = aws_sdk_s3::config::Builder::from(&conf)
        .endpoint_resolver(ep)
        .build();
    let client = Client::from_conf(s3_conf);
    */
    // End of debug
    CLIENT {
        client,
        region: r_str.to_string(),
        bucket
    }
}
