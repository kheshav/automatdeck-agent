extern crate config;
extern crate serde;

use core_lib::{settings::Settings, error, feedback};

use std::time::Duration;
use std::thread;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use std::{process, fs, io::prelude::*};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time;
use signal_hook::flag;
use signal_hook::{consts::TERM_SIGNALS, consts::signal::*, iterator::Signals};
pub mod initiator;


fn get_log_level(level: &str) -> LevelFilter{
    // Get the correct log level
    match level {
        "INFO" => LevelFilter::Info,
        "WARN" => LevelFilter::Warn,
        "DEBUG" => LevelFilter::Debug,
        _ => LevelFilter::Info,
    }
}

fn bootstrap(settings: &config::Config){
    // check of required settings
    
    log::info!("Checking for prerequisits...");

    let mut error: bool = false; 

    if settings.get::<String>("main.url").unwrap_or_default().is_empty(){
        log::error!("url in settings is missing or is empty!!!!");
        feedback::format_display("[ERROR] url in settings is missing or is empty!!!!");
        error = true
    }

    if settings.get::<String>("main.email").unwrap_or_default().is_empty(){
        log::error!("email in settings is missing or is empty!!!!");
        feedback::format_display("[ERROR] email in settings is missing or is empty!!!!");
        error = true
    }
    
    if settings.get::<String>("main.access_key").unwrap_or_default().is_empty(){
        log::error!("access_key in settings is missing or is empty!!!!");
        feedback::format_display("[ERROR] access_key in settings is missing or is empty!!!!");
        error = true
    }

    if settings.get::<String>("main.secret_key").unwrap_or_default().is_empty(){
        log::error!("secret_key in settings cannot be empty!!!!");
        feedback::format_display("[ERROR] secret_key in settings cannot be empty!!!!");
        error = true
    }


    if error{
        feedback::format_display("[ERROR] Prerequisit check failed");
        log::error!("Prerequisits check [KO]");
        process::exit(1);
    }

    log::info!("Prerequisits check [OK]");
    feedback::format_display("Prerequisit check SUCCESSFULL");
    feedback::format_display("Checking for requests...");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    // Error handling
    let mut data = ::std::collections::HashMap::new();
    data.insert("%NAME%", env!("CARGO_PKG_NAME"));
    data.insert("%GITHUB%", env!("CARGO_PKG_REPOSITORY"));

    error::create_hook(Some(data), |path, data| {
        if let Some(path) = path {
            let mut fs = fs::File::create(path)?;
            fs.write_all(data.as_bytes())?;
        }
        Ok(())
    });

    //Settings configuration

    let settings = Settings::new();
    

    let _s = serde_json::to_string(&Settings::deserialize(settings.clone())
                                   .unwrap_or_default()
                                   ).unwrap();
    //println!("{:?}", settings.get::<u64>("main.check_interval").unwrap());
    
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{l}] - {m}\n")))
        .build(settings.get::<String>("main.log_dir").unwrap_or_default() + "/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                   .appender("logfile")
                   .build(
                            get_log_level(
                                            &settings.get::<String>("main.log_level")
                                            .unwrap_or_default()
                                        )
                          )
        )
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    log::info!("Starting ad-agent application");
    feedback::format_display("Starting ad-agent Agent");

    // Check of prerequisits
    bootstrap(&settings);

    let _s: Settings = serde_json::from_str(&_s).unwrap();
    log::debug!("Detected configurations: \n {:#?}", _s);

    // Check for license validity
    //license::check_license().await;

    let term_now = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        // When terminated by a second term signal, exit with exit code 1.
        // This will do nothing the first time (because term_now is false).
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        // But this will "arm" the above for the second time, by setting it to true.
        // The order of registering these is important, if you put this one first, it will
        // first arm and then terminate ‒ all in the first round.
        flag::register(*sig, Arc::clone(&term_now))?;
    }

    tokio::spawn(async move { 
        while !term_now.load(Ordering::Relaxed)
        {

            #[cfg(debug_assertions)]
            println!("Doing work...");
            #[cfg(debug_assertions)]
            println!("OK");

            log::debug!("Sleeping for {} seconds", settings.get::<String>("main.check_interval").unwrap());
            thread::sleep(Duration::from_secs(settings.get::<u64>("main.check_interval").unwrap()));
            
            // Main actions related to business logic
            initiator::initiate().await;
        }

        feedback::format_display("Exiting application");
    });

    /*
    // Our actual work thread
    let t = thread::spawn(move || {
        while !term_now.load(Ordering::Relaxed)
        {
            println!("Doing work...");
            println!("OK");
            log::debug!("Sleeping for {} seconds", settings.get::<String>("main.check_interval").unwrap());
            thread::sleep(Duration::from_secs(settings.get::<u64>("main.check_interval").unwrap()));

            // Check for new requests
            // If new requests then spawn new threads (not execeeding MAX thread)
            // Increment the thread_count variable for each thread created
        }

        println!("\nThread exiting...");
    });
    */

    // Create iterator over signals
    let mut signals = Signals::new(TERM_SIGNALS)?;

    // This loop runs forever, and blocks until a kill signal is received
    'outer: loop {
        thread::sleep(Duration::from_secs(1));
        for signal in signals.pending() {
            thread::sleep(Duration::from_millis(100));
            match signal {
                SIGINT => {
                    println!("\nGot SIGINT");
                    log::warn!("Received SIGINT (Signal 1)");
                    break 'outer;
                },
                SIGTERM => {
                    println!("\nGot SIGTERM");
                    log::warn!("Received SIGTERM (Signal 15)");
                    break 'outer;
                },
                term_sig => {
                    println!("\nGot {:?}", term_sig);
                    log::warn!("Received SIGQUIT (Signal 3)");
                    break 'outer;
                },
            }
        }
    }
    // Wait for thread to exit
    //t.join().unwrap();
    // Cleanup code goes here
    feedback::format_display("Received kill signal. Wait 10 seconds, or hit Ctrl+C again to exit immediately.");
    thread::sleep(time::Duration::from_secs(10));
    feedback::format_display("Application exited");
    log::info!("Application shutdown..");
    Ok(())

    /*
    loop{
        //httpclient::test(String::from("/toto"));
        //let x  = httpclient::get(String::from("https://httpbin.orgq/ip")).await?;
        //println!("{}",x);
        /*
        let x = httpclient::get2("https://httpbin.org/ip").await;
        match x{
            Ok(result) => println!("{}",result),
            Err(_e) => log::error!("Unable to retrive expected info from api"),
        }
        */

        //let query = httpclient::get("/license/").await;
        
        // Method 1
        /*
        match query {
            Ok(response) => {
                                if !response.status().is_success(){
                                    log::error!("Failed to retreive info from {:?}",response.url().path());
                                }else{
                                    println!("{:?}",response.text().await?);
                                }
                            },
            Err(_) => log::error!("Unable to retreive expected info from api"),
        };
        */
        // End of Method 1

        /* Method 2
        if let Ok(x) = query{
            println!("{:?}",x.text().await?);
        }
        */
        println!("OK");
        log::debug!("Sleeping for {} seconds", settings.get::<String>("main.check_interval").unwrap());
        thread::sleep(Duration::from_secs(settings.get::<u64>("main.check_interval").unwrap()));
    }
    */

}
