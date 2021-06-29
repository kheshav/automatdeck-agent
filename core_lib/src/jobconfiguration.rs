use serde::Deserialize;
use derive_getters::Getters;
use std::collections::HashMap;

#[derive(Debug,Getters,Deserialize)]
pub struct JobConfiguration{
    stage: Vec<String>
}

#[allow(dead_code)]
pub struct Job{
    stage: String,
    variable: HashMap<String,String>,
    script_execution_strategy: String,
    allow_failure: bool,
    trigger_module: bool,
    timeout: String,
    script_retry: HashMap<String,String>,
    before_script: Vec<String>,
    script: Vec<String>,
    after_script: Vec<String>,
}
