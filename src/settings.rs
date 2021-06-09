use config::{Config, ConfigError, File};
use serde::{Serialize, Deserialize};


#[derive(Debug,Serialize, Deserialize)]
struct Main {
    url: String,
    check_interval: u16,
    access_key: String,
    secret_key: String,
    log_dir: String,
    log_level: String,
}

#[derive(Debug,Serialize, Deserialize)]
struct Security{
    enable_encryption: bool,
    key_path: String,
}

#[derive(Debug,Serialize, Deserialize)]
struct Module{
    module_dir: String,
    enabled_modules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    main: Main,
    security: Security,
    modules: Module,
}


impl Settings{

    pub fn new() -> Config{
        let mut s = Config::default();
        s.merge(File::with_name("config/settings")).unwrap();
        s
    }
    

    pub fn deserialize(s: Config) -> Result<Settings, ConfigError>{
        s.try_into()
    }
}
