use std::process::Command;
use crate::settings::Settings;

#[derive(Debug)]
pub enum ExecutionType{
    PreBeforeScript,
    PostBeforeScript,
    PreMainScript,
    PostMainScript,
    PreAfterScript,
    PostAfterScript,
    JobStarting,
    JobFinished,
    RequestStarting,
    RequestFinished,
}

impl ExecutionType{
    
    #[allow(dead_code)]
    fn value(&self) -> &str{
        match *self{
            ExecutionType::RequestStarting => "-rs",
            ExecutionType::RequestFinished => "-rf",
            ExecutionType::JobStarting => "-js",
            ExecutionType::JobFinished => "-jf",
            ExecutionType::PreMainScript => "-pre_ms",
            ExecutionType::PostMainScript => "-post_ms",
            ExecutionType::PreAfterScript => "-pre_as",
            ExecutionType::PostAfterScript => "-post_as",
            ExecutionType::PreBeforeScript => "-pre_bs",
            ExecutionType::PostBeforeScript => "-post_bs",
        }
    }
}

pub async fn execute_module(executiontype: ExecutionType, data: String){
    // Execute the modules
    let settings = Settings::new();
    let modules = settings.get::<Vec<String>>("modules.enabled_modules").unwrap_or_default();
    let module_dir = settings.get::<String>("modules.module_dir").unwrap_or_default();
    let python_path = settings.get::<String>("modules.python_path").unwrap_or_default();
    let (shell, flag) = ("sh","-c");

    for module in modules.iter(){
        let command = format!("{} {}/{} {} {}",python_path,module_dir,module,executiontype.value(),data);
        
        #[cfg(debug_assertions)]
        println!("Module command {}",command);

        log::info!("Executing module {} section {} \n data:{}",module,executiontype.value(), data);
        log::debug!("Executing module command {}",command);
        let mut child = Command::new(shell);
        child.arg(flag);
        child.arg(command);
        child.output().expect("OK");
        log::info!("Finished executing module {} section {} \n data:{}",module,executiontype.value(), data);
    }
}
