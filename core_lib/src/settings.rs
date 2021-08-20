use config::{Config, ConfigError, File};
use serde::{Serialize, Deserialize};
use std::process;


#[derive(Debug,Serialize, Deserialize)]
struct Main {
    url: String,
    check_interval: u16,
    email: String,
    access_key: String,
    secret_key: String,
    log_dir: String,
    log_level: String,
    max_thread: u16,
}

#[derive(Debug,Serialize, Deserialize)]
struct Security{
    enable_encryption: bool,
    key_path: String,
}

#[derive(Debug,Serialize, Deserialize)]
struct Module{
    python_path: String,
    module_dir: String,
    enabled_modules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Settings {
    main: Main,
    security: Security,
    modules: Module,
}

impl Default for Main{
    // Default for Main
    fn default() -> Self {
        Main{
            url: String::from(""),
            check_interval:300,
            email:String::from(""),
            access_key:String::from(""),
            secret_key:String::from(""),
            log_dir:String::from("/etc/ad-agent/log"),
            log_level:String::from("INFO"),
            max_thread: 4,
        }
    }
}

impl Default for Security{
    // Default for Security
    fn default() -> Self {
        Security{
                enable_encryption: false,
                key_path: String::from(""),
        }
    }
}

impl Default for Module{
    // Default for Module
    fn default() -> Self {
        Module{
            python_path: String::from("/usr/bin/python"),
            module_dir: String::from("/etc/ad-agent/modules"),
            enabled_modules: Vec::new(),
        }
    }
}

impl Settings{

    pub fn new() -> Config{
        let mut s = Config::default();
        let x = s.merge(File::with_name("config/settings"));
        if x.is_err(){
            println!("Invalid Configuration in settings!");
            process::exit(101);
        }else{
            x.unwrap();
        }
        s
    }
    

    pub fn deserialize(s: Config) -> Result<Settings, ConfigError>{
        s.try_into()
    }
}
