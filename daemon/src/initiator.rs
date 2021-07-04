use core_lib::{settings::Settings,request,jobconfiguration};
//use crossbeam_utils::thread;

pub async fn test(id: i64){
    println!("AYnc called: {}",id);
}


pub async fn initiate(){
    // Start main logic
    // Check for new requests
    // If new requests then spawn new threads (not execeeding MAX thread)
    // Increment the thread_count variable for each thread created
    let settings = Settings::new();
    let mut thread_count: u8 = 0;
    let max_thread = settings.get::<u8>("main.max_thread").unwrap_or_default();

    if let Ok(result) = request::Request::get_request().await{
        let message = result.message().to_owned();
//        thread::scope(|s| {
            for req in message{
                log::debug!("Detected request id: {}",req.id());
                println!("{:?}",req.config());
                if thread_count >= max_thread{
                    log::warn!("Max thread reached skipping unprocessed request for later run");
                    break;
                }
                thread_count += 1;
                if !req.valid(){
                    log::info!("Skipping request id {} since execution plan syntax is not valid", req.id());
                    break;
                }

                std::thread::spawn(move ||{
                    println!("LOL {}",req.id());
                    //test().await;
                    executor::run(test(req.id().to_owned()));
                    println!("A child thread borrowing `var`: {:?}", req.id());
                    log::info!("Processing request id: {}", req.id());
                    //let stages: jobconfiguration::Stages = serde_json::from_str("{\"stage\": [\"stage1\", \"stage2\", \"stage3\"]}").unwrap_or_default();
                    let stages: jobconfiguration::Stages = serde_json::from_str(req.config()).unwrap_or_default();
                    if stages.stages().len() == 0 {
                        log::error!("No stages defined in configuration used for request id: {}",req.id());
                    }
                    println!("{:?}",stages);
                });

               /*
               tokio::spawn(async move{
                    println!("LOL {}",req.id());
                    //test().await;
                    println!("A child thread borrowing `var`: {:?}", req.id());
                    log::info!("Processing request id: {}", req.id());
                    //let stages: jobconfiguration::Stages = serde_json::from_str("{\"stage\": [\"stage1\", \"stage2\", \"stage3\"]}").unwrap_or_default();
                    let stages: jobconfiguration::Stages = serde_json::from_str(req.config()).unwrap_or_default();
                    if stages.stages().len() == 0 {
                        log::error!("No stages defined in configuration used for request id: {}",req.id());
                    }
                    println!("{:?}",stages);
               });
               */
/*               
                s.spawn(move |_|{
                    println!("A child thread borrowing `var`: {:?}", req.id());
                    log::info!("Processing request id: {}", req.id());
                    //let stages: jobconfiguration::Stages = serde_json::from_str("{\"stage\": [\"stage1\", \"stage2\", \"stage3\"]}").unwrap_or_default();
                    let stages: jobconfiguration::Stages = serde_json::from_str(req.config()).unwrap_or_default();
                    if stages.stages().len() == 0 {
                        log::error!("No stages defined in configuration used for request id: {}",req.id());
                    }
                    if *req.id() == 1 as i64 {
                        println!("skippign 1");
                    }
                    println!("{:?}",stages);
                });
                

*/
            }
//        }).unwrap_or_default();
    }
}

