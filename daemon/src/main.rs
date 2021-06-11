extern crate config;
extern crate serde;

use core_lib::settings::Settings;
use core_lib::httpclient;

use std::time::Duration;
use std::thread;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

fn get_log_level(level: &str) -> LevelFilter{
    // Get the correct log level
    match level {
        "INFO" => LevelFilter::Info,
        "WARN" => LevelFilter::Warn,
        "DEBUG" => LevelFilter::Debug,
        _ => LevelFilter::Info,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let settings = Settings::new();
    let _s = serde_json::to_string(&Settings::deserialize(settings.clone())
                                   .unwrap_or_default()
                                   ).unwrap();
    //println!("{}", settings.get::<String>("main.url").unwrap());
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
    log::info!("Starting dd-agent application");
    let _s: Settings = serde_json::from_str(&_s).unwrap();
    log::debug!("Detected configurations: \n {:#?}", _s);
    log::info!("Checking for new requests");

    loop{
        //httpclient::test(String::from("/toto"));
        //let x  = httpclient::get(String::from("https://httpbin.orgq/ip")).await?;
        //println!("{}",x);
        let x = httpclient::get2("https://httpbin.org/ip").await;
        match x{
            Ok(result) => println!("{}",result),
            Err(_e) => log::error!("Unable to retrive expected info from api"),
        }
        //println!("{:?}",x.await?);
        println!("OK");
        log::debug!("Sleeping for {} seconds", settings.get::<String>("main.check_interval").unwrap());
        thread::sleep(Duration::from_secs(settings.get::<u64>("main.check_interval").unwrap()));
    }

}
