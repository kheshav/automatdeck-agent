use core_lib::{settings::Settings,request};
use crossbeam_utils::thread;

pub async fn initiate(){
    // Start main logic
    // Check for new requests
    // If new requests then spawn new threads (not execeeding MAX thread)
    // Increment the thread_count variable for each thread created
    let settings = Settings::new();
    let mut thread_count: u8 = 0;
    let max_thread = settings.get::<u8>("main.max_thread").unwrap_or_default();

    if let Ok(result) = request::Request::get_request().await{
        let x = result.message().iter();
        for request in x{
            if thread_count >= max_thread{
                log::warn!("Max thread reached skipping unprocessed request for later run");
                break;
            }
            thread_count += 1;
            log::info!("Detected request id: {}",request.id());

            thread::scope(|s| {
                s.spawn(|_| {
                    println!("A child thread borrowing `var`: {:?}", request.id());
                    log::info!("Processing request id: {}", request.id());
                });
            }).unwrap_or_default();
        }
    }
}
