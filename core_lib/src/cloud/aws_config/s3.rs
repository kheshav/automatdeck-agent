use std::collections::HashMap;

pub fn check_s3_bucketcreate_params(params: HashMap<String,String>) -> bool{
    vec!["class","bucketname"].iter().all(|&k| params.contains_key(k))
}


pub fn check_s3_copy_object_params(params: HashMap<String,String>) -> bool{
    vec!["class","bucketname","destination_bucket","source_object","destination_object"].iter().all(|&k| params.contains_key(k))
}

pub fn check_s3_remove_object_params(params: HashMap<String,String>) -> bool{
    vec!["class","bucketname","object"].iter().all(|&k| params.contains_key(k))
}


pub fn check_s3_upload_object_params(params: HashMap<String,String>) -> bool{
    vec!["class","bucketname","source_file","destination_file"].iter().all(|&k| params.contains_key(k))
}
