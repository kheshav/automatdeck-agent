use serde::{Deserialize,Serialize};
use derive_getters::Getters;
use derive_getters::Dissolve;
use std::collections::HashMap;
use crate::httpclient;
use crate::request::Request;
use crate::request::RequestData;
use crate::request::RequestStatus;
use json;


#[derive(Debug,Getters,Serialize,Deserialize)]
pub struct Stages{
    stages: Vec<String>
}


#[derive(Debug,Getters,Serialize,Deserialize,Dissolve)]
#[allow(dead_code)]
pub struct ScriptRetry{
    #[serde(default)]
    retry: bool,
    #[serde(default)]
    max: i32,
    #[serde(default)]
    when: String,
}


#[allow(dead_code)]
#[derive(Debug,Getters,Serialize,Deserialize)]
pub struct Job{
    stage: String,
    #[serde(default = "default_variables")]
    variables: HashMap<String,String>,
    #[serde(default = "default_script_execution_strategy")]
    script_execution_strategy: String,
    #[serde(default = "default_allow_failure")]
    allow_failure: bool,
    #[serde(default = "default_trigger_module")]
    trigger_module: bool,
    #[serde(default = "default_timeout")]
    timeout: String,
    #[serde(flatten)]
    script_retry: ScriptRetry,
    #[serde(default = "default_scripts")]
    before_script: Vec<String>,
    script: Vec<String>,
    #[serde(default = "default_scripts")]
    after_script: Vec<String>,
}


fn default_scripts() -> Vec<String>{
    Vec::new()
}

fn default_timeout() -> String {
    "5m".to_string()
}

fn default_trigger_module() -> bool{
    false
}

fn default_script_execution_strategy() -> String{
    "solo".to_string()
}

fn default_allow_failure() -> bool{
    false
}

fn default_variables() -> HashMap<String,String> {
    let mut variables: HashMap<String,String> = HashMap::new();
    variables.insert(String::from("1234"), String::from("1234"));
    variables
}

impl Stages {
    #[allow(dead_code)]
    pub fn execute(self){

    }
}

impl Default for Stages{
    // Default for Stages
    fn default() -> Self {
       Stages{
            stages: Vec::new()
       }
    }
}

impl Default for ScriptRetry{
    fn default() -> Self{
        ScriptRetry{
            retry: false,
            max: 0,
            when: String::from("always"),
        }
    }
}


impl Job{
    
    fn prepare_commands(&mut self){
        // Prepare command; replacing defined variables

        log::debug!("Preparing commands for execution");

        // For Script
        let mut final_scripts: Vec<String> = Vec::new();
        for script in &self.script{
            //println!("Script: {}",script);
            let mut _script: String = script.to_owned();
            for (variable, value) in self.variables.to_owned(){
                let mut to_replace = "${".to_string();
                to_replace.push_str(&variable);
                to_replace.push_str("}");
                _script = _script.replace(&to_replace,&value);
            }
            _script = _script.replace("\\","");
            final_scripts.push(_script);
        }
        self.script = final_scripts;

        // For After Script
        let mut final_scripts: Vec<String> = Vec::new();
        for script in &self.after_script{
            //println!("Script: {}",script);
            let mut _script: String = script.to_owned();
            for (variable, value) in self.variables.to_owned(){
                let mut to_replace = "${".to_string();
                to_replace.push_str(&variable);
                to_replace.push_str("}");
                _script = _script.replace(&to_replace,&value);
            }
            _script = _script.replace("\\","");
            final_scripts.push(_script);
        }
        self.after_script = final_scripts;

        // For Before Script
        let mut final_scripts: Vec<String> = Vec::new();
        for script in &self.before_script{
            //println!("Script: {}",script);
            let mut _script: String = script.to_owned();
            for (variable, value) in self.variables.to_owned(){
                let mut to_replace = "${".to_string();
                to_replace.push_str(&variable);
                to_replace.push_str("}");
                _script = _script.replace(&to_replace,&value);
            }
            _script = _script.replace("\\","");
            final_scripts.push(_script);
        }
        self.before_script = final_scripts;
    }

    #[allow(dead_code)]
    pub fn run_command(self){

    }
}

#[allow(dead_code)]
pub fn build_stages(stages: &Vec<String>, request: RequestData) -> Vec<Job>{
    // Build the correct flow based on defined stages
    let mut flow: Vec<Job> = Vec::new();
    let parsed_config = json::parse(request.config()).unwrap();
    let mut valid_stages: bool = false;
    let mut stages_to_create: Vec<String> = Vec::new();
    for stage in stages.iter(){
        valid_stages = false;
        log::debug!("Stage detected :{:?} in request id {}",stage,request.id());
        log::debug!("Checking for corresponding stage job {} in request id: {}",stage,request.id());
        for conf in parsed_config.entries(){
            if conf.0 != "stages"{
                let mut job: Job = serde_json::from_str(&conf.1.to_string()).unwrap();
    //            let mut job: Job = serde_json::from_str("{\"stage\":\"stage2\",\"script_execution_strategy\":\"solo\",\"trigger_module\":false,\"timeout\":\"1h\",\"script\":[\"mkdir -p  /tmp/test\",\"curl ${HTTP_HOST} -H \\\\\\\"Authorization:\\\\ Bearer ${token}\\\\\\\" -H \\\\\\\"Agent:\\\\ ${AGENT}\\\\\\\" -o /tmp/test/$(date +\\\\\\\"%Y_%m_%d_%I_%M_%p\\\\\\\").out\"],\"after_script\":[\"echo \\\"Script executed\\\"\"]}").unwrap();
                //println!("stage: {} , job.stage: {}",stage,job.stage);
                if stage.to_owned() == job.stage{
                    valid_stages = true;
                    job.prepare_commands();
                    flow.push(job);
                    stages_to_create.push(stage.to_string());
                    break;
                }
            }

        }
    }
    if !valid_stages{
        log::error!("Skipping request id: {}, and marking it as failed since one or more defined stages are missing.", request.id());
        flow.clear();
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            Request::set_status(RequestStatus::WARNING, request.to_owned()).await;
        });
    }
    #[cfg(debug_assertions)]
    println!("Stages Valid: {}",valid_stages);
    #[cfg(debug_assertions)]
    println!("Flow: {:?}", flow);
    tokio::runtime::Runtime::new().unwrap().block_on(async{
        create_stages(&stages_to_create,request.id().to_owned()).await;
    });
    return flow;
}

#[allow(dead_code)]
async fn create_stages(stages: &Vec<String>,reqid: i64){
    // Create stages on server
    // if stage cannot or wasnot created when stage will run will create it on server
    if ! stages.is_empty(){
        let mut stage_data: String = "".to_string();
        let size: usize = stages.len();
        let mut count: usize = 0;
        for stage in stages.iter(){
            count += 1;
            stage_data.push_str(&stage);
            if count != size{
                stage_data.push_str(",");
            }
        }
        let reqid_string = reqid.to_string();
        let mut payload = HashMap::new();
        payload.insert("stages",stage_data.as_str());
        payload.insert("request_id",&reqid_string);
        log::debug!("Creating initial stages for request: {}",reqid);
        let create_stage_query = httpclient::post("/job/", payload).await;
        match create_stage_query {
            Ok(response) => {
                match response.error_for_status(){
                    Ok(_) => {
                        log::debug!("Initial stage(s) successfully created for request: {}",reqid);
                    }
                    Err(_) =>{
                        log::warn!("Unable to create initial stage(s) for request: {}",reqid);
                    }
                }
            }
            Err(_) =>{
                log::error!("Unable to create initial stage(s) for request: {}",reqid);
            }
        }
    }
}
