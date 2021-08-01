use serde::{Deserialize,Serialize};
use derive_getters::Getters;
use derive_getters::Dissolve;
use std::collections::HashMap;
use crate::httpclient;
use crate::request::Request;
use crate::request::RequestData;
use crate::request::RequestStatus;
use json;
use std::env;
use os_pipe::pipe;
use std::io::prelude::*;
use std::process::Command;

#[derive(Debug)]
pub enum JobStatus{
    PENDING,
    RUNNING,
    FAILED,
    IGNORED,
    SUCCESS,
}

#[derive(Debug)]
pub enum ScriptType{
    BEFORE,
    AFTER,
    SCRIPT,
}

#[derive(Debug,Getters,Serialize,Deserialize)]
pub struct Stages{
    stages: Vec<String>
}


#[derive(Debug,Getters,Serialize,Deserialize,Dissolve,Clone)]
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
#[derive(Debug,Getters,Serialize,Deserialize,Clone)]
pub struct Job{
    #[serde(default = "default_reqid")]
    reqid: i64, // no in json to be used as ref for the corresponding rquest id
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

fn default_reqid() -> i64{
    0
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

impl JobStatus{

    #[allow(dead_code)]
    fn value(&self) -> &str{
        match *self{
            JobStatus::PENDING => "P",
            JobStatus::RUNNING => "R",
            JobStatus::FAILED => "F",
            JobStatus::IGNORED => "I",
            JobStatus::SUCCESS => "S"
        }
    }

    #[allow(dead_code)]
    fn value_real(&self) -> &str{
        match *self{
            JobStatus::PENDING => "PENDING",
            JobStatus::RUNNING => "RUNNING",
            JobStatus::FAILED => "FAILED",
            JobStatus::IGNORED => "IGNORED",
            JobStatus::SUCCESS => "SUCCESS"
        }
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

    #[allow(dead_code)]
    pub async fn set_status(self, status: JobStatus){
        // Set status of the job
        log::debug!("Setting status of requestid: {}, job: {} to {}", self.reqid,self.stage, status.value_real());
        let uri = "/job/unknown/".to_string();
        let mut data = HashMap::new();

        let mut toupdate: String = "".to_string();
        toupdate.push_str("{\"status\":");
        toupdate.push_str("\"");
        toupdate.push_str(&status.value());
        toupdate.push_str("\"}");

        let _reqid = self.reqid.to_string().to_owned();
        data.insert("requestid", _reqid.as_str());
        data.insert("stage", &self.stage);
        data.insert("toupdate", &toupdate);

        let query = httpclient::patch(&uri, data).await;
    }

    fn replace_templatevariable(&mut self, command: &mut String){
        // Replace script variables by template variables

        for (variable, value) in self.variables.to_owned(){
            let _script = command.to_owned();
            let mut to_replace = "${".to_string();
            to_replace.push_str(&variable);
            to_replace.push_str("}");
            *command = _script.replace(&to_replace,&value);
        }

    }

    fn replace_sysvariable(&mut self,command: &mut String){
        // Replace scripts variables by sys variables   
        for (sys_variable, sys_value) in env::vars(){
            let _command = command.to_owned();
            let mut to_replace = "${".to_string();
            to_replace.push_str(&sys_variable);
            to_replace.push_str("}");
            *command = _command.replace(&to_replace, &sys_value);
        }
    }
    
    fn prepare_commands(&mut self){
        // Prepare command; replacing defined variables

        log::debug!("Preparing commands for execution");

        // For Script
        let mut final_scripts: Vec<String> = Vec::new();
        for script in self.script.to_owned(){
            let mut _script: String = script.to_owned();
            self.replace_templatevariable(&mut _script);
            self.replace_sysvariable(&mut _script);
            _script = _script.replace("\\","");
            final_scripts.push(_script);
        }
        self.script = final_scripts;

        // For After Script
        let mut final_scripts: Vec<String> = Vec::new();
        for script in self.after_script.to_owned(){
            let mut _script: String = script.to_owned();
            self.replace_templatevariable(&mut _script);
            self.replace_sysvariable(&mut _script);
            _script = _script.replace("\\","");
            final_scripts.push(_script);
        }
        self.after_script = final_scripts;

        // For Before Script
        let mut final_scripts: Vec<String> = Vec::new();
        for script in self.before_script.to_owned(){
            let mut _script: String = script.to_owned();
            self.replace_templatevariable(&mut _script);
            self.replace_sysvariable(&mut _script);
            _script = _script.replace("\\","");
            final_scripts.push(_script);
        }
        self.before_script = final_scripts;
    }

    #[allow(dead_code)]
    fn prepare_inherit_command(&self, script_type:ScriptType) -> String{
        // Prepare inherit commands
        #[allow(unused_assignments)]
        let mut scripts : Vec<String> = Vec::new();
        match script_type{
            ScriptType::AFTER => {
                scripts = self.after_script.to_owned();
            },
            ScriptType::BEFORE => {
                scripts = self.before_script.to_owned();
            },
            ScriptType::SCRIPT => {
                scripts = self.script.to_owned();
            }
        }
        let mut count: i8 = 0;
        let mut final_scripts: String = "".to_string();
        for script in scripts{
            if count > 0{
                final_scripts.push_str("&&");
                final_scripts.push_str(&script);
            } else {
                final_scripts.push_str(&script);
            }
            count += 1;
        }
        return final_scripts;
    }


    #[allow(dead_code)]
    fn execute_commands(&self,command: String) -> bool{
        // Generic executor of command

        if command.is_empty(){
            return true;
        }
        let (shell, flag) = if cfg!(windows) { ("cmd.exe", "/C") } else { ("sh", "-c") };
        let mut child = Command::new(shell);
        child.arg(flag);
        child.arg(command.to_owned());

        // Here's the interesting part. Open a pipe, copy its write end, and
        // give both copies to the child.
        let (mut reader, writer) = pipe().unwrap();
        let writer_clone = writer.try_clone().unwrap();
        child.stdout(writer);
        child.stderr(writer_clone);

        // Now start the child running.
        //let mut handle = child.spawn().unwrap();
        let handle = child.status().unwrap();

        // Very important when using pipes: This parent process is still
        // holding its copies of the write ends, and we have to close them
        // before we read, otherwise the read end will never report EOF. The
        // Command object owns the writers now, and dropping it closes them.
        #[cfg(debug_assertions)]
        println!("Command {} handle: {:?}",command.to_owned(),handle);
        
        drop(child);

        // Finally we can read all the output and clean up the child.
        let mut output = String::new();
        reader.read_to_string(&mut output).unwrap();
        #[cfg(debug_assertions)]
        println!("Output of command {} : \n {}",command.to_owned(),output);
        //handle.wait().unwrap();
        return handle.success();
    }

    #[allow(dead_code)]
    pub fn run_before_command(self) -> bool{
        // Run before script

        if self.before_script.len() == 0 {
            return true;
        }
        let mut count = 0;
        if self.script_execution_strategy.eq(&"inherit"){
            let mut result: bool = self.execute_commands(self.prepare_inherit_command(ScriptType::BEFORE));
            if self.script_retry.retry{
                while !result {
                    count += 1;
                    if count <= self.script_retry.max{
                        log::warn!("Before_script for request: {} failed, retrying command..",self.reqid());
                        result = self.execute_commands(self.prepare_inherit_command(ScriptType::BEFORE));
                    }
                }
            }
            return result;
            
        }else{
            // Solo strategy
            let mut result: bool = false;
            for command in self.before_script.to_owned(){
                result = self.execute_commands(command.to_owned());
                if self.script_retry.retry{
                    while !result {
                        count += 1;
                        if count <= self.script_retry.max{
                            log::warn!("Before_script for request: {} failed, retrying command..",self.reqid());
                            result = self.execute_commands(command.to_owned());
                        }
                    }
                }
            }
            return result;
        }
    }

    #[allow(dead_code)]
    fn run_after_command(self) -> bool{
        // Run after script
        return true;
    }

    #[allow(dead_code)]
    pub fn run_main_command(self) -> bool{
        // Run script
        return true;
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
                job.reqid = request.id().to_owned();
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
