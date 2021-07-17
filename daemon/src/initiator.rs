use core_lib::{settings::Settings,request,jobconfiguration};

pub async fn test(id: i64){
    println!("AYnc called: {}",id);
}

pub async fn proceede(stages: Vec<jobconfiguration::Job>, req: request::RequestData){
    // Proceede with the flow prepared by initiate
    request::Request::set_status(request::RequestStatus::PROCESSING, req).await;

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
            for req in message{
                log::debug!("Detected request id: {}",req.id());
                #[cfg(debug_assertions)]
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
                    //println!("LOL {}",req.id());
                    //test().await;
                    //executor::run(test(req.id().to_owned()));
                    //println!("A child thread borrowing `var`: {:?}", req.id());
                    log::info!("Processing request id: {}", req.id());
                    //let stages: jobconfiguration::Stages = serde_json::from_str("{\"stage\": [\"stage1\", \"stage2\", \"stage3\"]}").unwrap_or_default();
                    let stages: jobconfiguration::Stages = serde_json::from_str(req.config()).unwrap_or_default();
                    /*
                    let parse = json::parse(req.config()).unwrap();
                    //println!("Parsed : {:?}",parse);
                    //println!("KKKK {:?}",parse["job1"]);
                    for x in parse.entries(){
                        println!("Entries: {:?}",x.1);
                        if x.0 != "stages"{
                            //println!("{:?}",x.1.to_string());
                            let y: jobconfiguration::Job = serde_json::from_str(&x.1.to_string()).unwrap();
                            //let y: jobconfiguration::Job = serde_json::from_str("{\"stage\":\"stage2\",\"variables\":{\"HTTP_HOST\":\"http://127.0.0.1\",\"AGENT\":\"Firefox/1.2.3\"},\"script_execution_strategy\":\"solo\",\"allow_failure\":true,\"trigger_module\":false,\"timeout\":\"1h\",\"script_retry\":{\"retry\":true,\"max\":1,\"when\":\"script_failure\"},\"before_script\":[\"echo \\\"Executing job 2\\\"\"],\"script\":[\"mkdir -p  /tmp/test\",\"curl ${HTTP_HOST} -H \\\\\\\"Authorization:\\\\ Bearer ${token}\\\\\\\" -H \\\\\\\"Agent:\\\\ ${AGENT}\\\\\\\" -o /tmp/test/$(date +\\\\\\\"%Y_%m_%d_%I_%M_%p\\\\\\\").out\"],\"after_script\":[\"echo \\\"Script executed\\\"\"]}").unwrap();
                            println!("{:?}",y);
                        }

                    }
                    */
                    if stages.stages().len() == 0 {
                        log::error!("No stages defined in configuration used for request id: {}",req.id());
                    }else{
                        #[cfg(debug_assertions)]
                        println!("{:?}",stages);
                        let jobs: Vec<jobconfiguration::Job> = jobconfiguration::build_stages(stages.stages(),req.to_owned());
                        tokio::runtime::Runtime::new().unwrap().block_on(async move{
                            proceede(jobs, req.to_owned()).await;
                        });
                    }
                });

            }
    }
}

