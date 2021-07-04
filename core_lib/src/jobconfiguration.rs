use serde::{Deserialize,Serialize};
use derive_getters::Getters;
use std::collections::HashMap;

#[derive(Debug,Getters,Serialize,Deserialize)]
pub struct Stages{
    stages: Vec<String>
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

impl Stages {
    #[allow(dead_code)]
    pub fn execute(self){

    }
}

impl Default for Stages{
    fn default() -> Self {
       Stages{
            stages: Vec::new()
       }
    }
}

impl Job{
    
    #[allow(dead_code)]
    fn prepare_commands(){
    }
}
