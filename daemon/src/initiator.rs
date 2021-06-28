use core_lib::{settings::Settings,request};
use std::thread;

pub async fn initiate(){
    // Start main logic
    // Check for new requests
    // If new requests then spawn new threads (not execeeding MAX thread)
    // Increment the thread_count variable for each thread created
    let settings = Settings::new();
    let mut thread_count: i8 = 0;
    let max_thread = settings.get::<String>("main.log_dir").unwrap_or_default();

    if let Ok(result) = request::Request::get_request().await{
        println!("Hey {:?}",result);
        result.change_status();
    }
}
