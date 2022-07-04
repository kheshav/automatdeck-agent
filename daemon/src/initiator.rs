use core_lib::{jobconfiguration::{self, JobStatus}, request::{self, Request}, settings::Settings, feedback, moduleexecutor, cloud::parse_execute};

pub async fn proceede(jobs: Vec<jobconfiguration::Job>, req: request::RequestData) -> Result<(),Box<dyn std::error::Error>>{
    // Proceede with the flow prepared by initiate
   if request::Request::set_status(request::RequestStatus::PROCESSING, req.to_owned()).await{
        // was able to set the status

        // Request Starting module
        moduleexecutor::execute_module(
            moduleexecutor::ExecutionType::RequestStarting, 
            format!(
                    "{{\\\"reqid\\\":{}}}",
                    req.id()
                ).to_string()
        ).await;

        let mut jobfailed: bool = false;
        let mut requestwarning: bool = false;
        for job in jobs{
            // If previous job has failed so mark other sucessive jobs as IGNORED
            if jobfailed{
                job.to_owned().set_status(JobStatus::IGNORED).await;
                continue;
            }
            let _job = job.to_owned();
            job.to_owned().set_status(JobStatus::RUNNING).await;

            // Cloud actions
            // In case one cloud action fails do not proceede with another cloud action
            let cloud = job.cloud().to_owned();
            if cloud.len() > 0 {
                job.to_owned().set_feedback("Cloud config detected..Executing cloud config".to_string(), feedback::FeedbackType::STEP).await;
                for cloudconfig in cloud{
                    job.to_owned().set_feedback(format!("Executing cloud command {:?}",cloudconfig), feedback::FeedbackType::COMMAND).await;
                    match parse_execute(cloudconfig.to_owned()).await{
                        Ok(()) => {
                            #[cfg(debug_assertions)]
                            println!("Success when executing cloud {:?}",cloudconfig);
                            job.to_owned().set_feedback(format!("Successfully executed cloud command {:?}", cloudconfig), feedback::FeedbackType::OUTPUT).await;
                        },
                        Err(e) => {
                            #[cfg(debug_assertions)]
                            println!("Error when executing cloud {:?} ,\nResult: {}",cloudconfig,e.to_string());
                            job.to_owned().set_feedback(e.to_string(), feedback::FeedbackType::ERROR).await;
                            if !job.to_owned().allow_failure() {
                                Request::set_status(request::RequestStatus::FAILED, req.to_owned()).await;
                                jobfailed = true;
                                requestwarning = false;
                            }else{
                                requestwarning = true;
                            }
                            // In any case if one cloud action fails do not proceede with another one
                            break;
                        },
                    }
                }
                if jobfailed {
                    continue;
                }
            }

            //before script
            //pre brefore script
            if *job.to_owned().trigger_module(){
                moduleexecutor::execute_module(
                    moduleexecutor::ExecutionType::PreBeforeScript,
                    format!(
                            "{{\\\"reqid\\\":{},\\\"jobname\\\":\\\"{}\\\"}}",
                            req.id(),job.stage()
                        ).to_string()
                ).await;
            }

            let before_script = job.run_before_command(req.meta().to_string()).await?;
            
            // post before script
            if *job.to_owned().trigger_module(){
                moduleexecutor::execute_module(
                    moduleexecutor::ExecutionType::PostBeforeScript,
                    format!(
                            "{{\\\"reqid\\\":{},\\\"jobname\\\":\\\"{}\\\",\\\"command_status\\\":{}}}",
                            req.id(),job.stage(),before_script
                        ).to_string()
                ).await;
            }

            if !before_script{
                if !job.to_owned().allow_failure(){
                    // Job failed and no allow_failure
                    #[cfg(debug_assertions)]
                    println!("Stopping request {} since stage: {} failed",req.id(),job.to_owned().stage());

                    log::error!("Stopping request {} since stage: {} failed",req.id(),job.to_owned().stage());
                    job.to_owned().set_status(JobStatus::FAILED).await;
                    job.set_feedback("Job Failed".to_string(),feedback::FeedbackType::ERROR).await;
                    Request::set_status(request::RequestStatus::FAILED, req.to_owned()).await;
                    jobfailed = true;
                    requestwarning = false;
                    continue;
                }else{
                    requestwarning = true;
                }
            }

            //main script
            // Pre main script
            if *job.to_owned().trigger_module(){
                moduleexecutor::execute_module(
                    moduleexecutor::ExecutionType::PreMainScript,
                    format!(
                            "{{\\\"reqid\\\":{},\\\"jobname\\\":\\\"{}\\\"}}",
                            req.id(),job.stage()
                        ).to_string()
                ).await;
            }

            let main_script = job.run_main_command(req.meta().to_string()).await?;

            // post main script
            if *job.to_owned().trigger_module(){
                moduleexecutor::execute_module(
                    moduleexecutor::ExecutionType::PostMainScript,
                    format!(
                            "{{\\\"reqid\\\":{},\\\"jobname\\\":\\\"{}\\\",\\\"command_status\\\":{}}}",
                            req.id(),job.stage(),main_script
                        ).to_string()
                ).await;
            }

            if !main_script{
                if !job.to_owned().allow_failure(){
                    #[cfg(debug_assertions)]
                    println!("Stopping request {} since stage: {} failed",req.id(),job.to_owned().stage());

                    log::error!("Stopping request {} since stage: {} failed",req.id(),job.to_owned().stage());
                    job.to_owned().set_status(JobStatus::FAILED).await;
                    job.set_feedback("Job Failed".to_string(),feedback::FeedbackType::ERROR).await;
                    Request::set_status(request::RequestStatus::FAILED, req.to_owned()).await;
                    jobfailed = true;
                    requestwarning = false;
                    continue;
                }else{
                    requestwarning = true;
                }
            }

            //after script
            // Pre after script
            if *job.to_owned().trigger_module(){
                moduleexecutor::execute_module(
                    moduleexecutor::ExecutionType::PreAfterScript,
                    format!(
                            "{{\\\"reqid\\\":{},\\\"jobname\\\":\\\"{}\\\"}}",
                            req.id(),job.stage()
                        ).to_string()
                ).await;
            }

            let after_script = job.run_after_command(req.meta().to_string()).await?;

            // post after script
            if *job.to_owned().trigger_module(){
                moduleexecutor::execute_module(
                    moduleexecutor::ExecutionType::PostAfterScript,
                    format!(
                            "{{\\\"reqid\\\":{},\\\"jobname\\\":\\\"{}\\\",\\\"command_status\\\":{}}}",
                            req.id(),job.stage(),main_script
                        ).to_string()
                ).await;
            }

            if !after_script{
                if !job.to_owned().allow_failure(){
                    #[cfg(debug_assertions)]
                    println!("Stopping request {} since stage: {} failed",req.id(),job.to_owned().stage());

                    log::error!("Stopping request {} since stage: {} failed",req.id(),job.to_owned().stage());
                    job.to_owned().set_status(JobStatus::FAILED).await;
                    job.set_feedback("Job Failed".to_string(),feedback::FeedbackType::ERROR).await;
                    Request::set_status(request::RequestStatus::FAILED, req.to_owned()).await;
                    jobfailed = true;
                    requestwarning = false;
                    continue;
                }else{
                    requestwarning = true;
                }
            }

            job.to_owned().set_status(JobStatus::SUCCESS).await;
            
            // Job finished module
            if *job.to_owned().trigger_module(){
                moduleexecutor::execute_module(
                    moduleexecutor::ExecutionType::JobFinished,
                    format!(
                            "{{\\\"reqid\\\":{},\\\"jobname\\\":\\\"{}\\\"}}",
                            req.id(),job.stage()
                        ).to_string()
                ).await;
            }
        }

        if requestwarning{
            Request::set_status(request::RequestStatus::WARNING, req.to_owned()).await;
        }else{
            if !jobfailed{
                Request::set_status(request::RequestStatus::COMPLETED, req.to_owned()).await;
            }
        }
        // Request finished
        moduleexecutor::execute_module(
            moduleexecutor::ExecutionType::RequestFinished,
            format!(
                    "{{\\\"reqid\\\":{},\\\"status\\\":{}}}",
                    req.id(),!jobfailed
                ).to_string()
        ).await;
        Ok(())
   }else{
        log::error!("Unable to set status for request: {}, hence skipping this request for later",req.id());
        Ok(())
   }

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
                    #[cfg(debug_assertions)]
                    println!("Config invalid");

                    log::info!("Skipping request id {} since execution plan syntax is not valid", req.id());
                    break;
                }

                std::thread::spawn(move ||{
                    log::info!("Processing request id: {}", req.id());
                    //let stages: jobconfiguration::Stages = serde_json::from_str("{\"stage\": [\"stage1\", \"stage2\", \"stage3\"]}").unwrap_or_default();
                    let stages: jobconfiguration::Stages = serde_json::from_str(req.config()).unwrap_or_default();
                    if stages.stages().len() == 0 {
                        log::error!("No stages defined in configuration used for request id: {}",req.id());
                    }else{
                        #[cfg(debug_assertions)]
                        println!("{:?}",stages);
                        let jobs: Vec<jobconfiguration::Job> = jobconfiguration::build_stages(stages.stages(),req.to_owned());
                        #[cfg(debug_assertions)]
                        println!("Cloud {:?}", jobs[0].cloud());
                        //Cloud [{"class": "aws.s3.create_bucket", "bucketname": "toto"}, {"class": "aws.s3.remove_object", "object": "/a.txt", "bucketname": "toto"}]

                        tokio::runtime::Runtime::new().unwrap().block_on(async move{
                            #[allow(unused_variables)]
                            let x = proceede(jobs, req.to_owned()).await;
                        });
                    }
                });

            }
    }
}

