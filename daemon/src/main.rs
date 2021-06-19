extern crate config;
extern crate serde;

use config::ConfigError;
use core_lib::settings::Settings;
use core_lib::httpclient;
use core_lib::license;
use core_lib::error;

use std::time::Duration;
use std::thread;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use std::{process, fs, io::prelude::*};


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
    
    if settings.get::<String>("main.access_key").unwrap().is_empty(){
        log::error!("access_key in settings cannot be empty!!!!");
        error = true
    }

    if settings.get::<String>("main.secret_key").unwrap().is_empty(){
        log::error!("secret_key in settings cannot be empty!!!!");
        error = true
    }


    if error{
        process::exit(1);
    }

    log::info!("Prerequisits checked [OK]");
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
        .build(settings.get::<String>("main.log_dir").unwrap() + "/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(get_log_level(&settings.get::<String>("main.log_level")
                                        .unwrap_or_default()
                                        )
                          )
        )
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    // Check of prerequisits
    bootstrap(&settings);

    log::info!("Starting dd-agent application");
    let _s: Settings = serde_json::from_str(&_s).unwrap();
    log::debug!("Detected configurations: \n {:#?}", _s);

    // Check for license validity
    license::check_license().await;

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

}
