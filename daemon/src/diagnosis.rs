use crate::{args::Diagnose, bootstrap};
use std::process;
use std::io::Result;
use colored::*;
use core_lib::{httpclient, request,settings::Settings,feedback};
use cli_table::{
    format::{Align, Justify},
    print_stdout, Color, Table, WithTitle,
};


#[derive(Debug, Table)]
struct RequestResult {
    #[table(
        title = "Requestid",
        justify = "Justify::Right",
        align = "Align::Top",
        color = "Color::Green",
        bold
    )]
    id: i64,
    #[table(title = "Title")]
    title: String,
    #[table(title = "Status")]
    status: String,
    #[table(title = "Valid Configuration")]
    valid: bool,

}

fn border(){
    // Print border
    println!("{}","--------------------------------------------------".bright_blue());
}

fn check_conf(){
    // Check config file
    let settings = Settings::new();
    bootstrap(&settings,false);
    
}

fn print_table(title: &str,data: Vec<RequestResult>) -> Result<()>{
    let mut new_title = String::from("");
    new_title.push_str("\n-------------------");
    new_title.push_str(title);
    new_title.push_str("-------------------");
    println!("{}",new_title.cyan().bold());
    print_stdout(data.with_title())
}

pub async fn diagnose(arguments: &Diagnose){
    // Diagnose command parsing and logic
    #[cfg(debug_assertions)]
    println!("Launched diagnose with following Arguments: \n{:?}", arguments);
    let mut error_code: i32 = 0;

    if *arguments.debug(){
        feedback::format_display("Executing in debug mode", feedback::FedbackDisplayType::DEBUG);
    } else {
        feedback::format_display("Executing in normal mode", feedback::FedbackDisplayType::INFO);
    }
    check_conf();

    border();

    /*
    let mut y = HashMap::new();
    y.insert("class".to_string(),"aws.s3.create_bucket".to_string());
    y.insert("bucketname".to_string(), "ksetest2".to_string());
    parse_execute(y.to_owned()).await;
    */

    let query = httpclient::get("/test/").await;
    let mut query_result: String = "[KO]".red().bold().to_string();
    let mut connection_status: bool = false;
    #[allow(unused_assignments)]
    let mut connection_status_http_code: String = "".to_string();

    match query{
        Ok(response) =>{
            if response.status().is_success(){
                query_result = "[OK]".green().bold().to_string();
                connection_status = true;
            } else {
                error_code = 100;
            }
            if response.status() == 403 {
                error_code = 101;
            }
            connection_status_http_code = response.status().to_string();
        },
        Err(_e) => {
            error_code = 100;
            connection_status_http_code = _e.to_string();
        }
    };
    println!("Testing connection with API \t{}",query_result);
    if *arguments.debug(){
        println!("{} {} {}","DEBUG:".bright_magenta().bold(),"Test connection Response code",connection_status_http_code.bright_yellow());
    }
    if error_code == 101{
        println!("{}","Invalid access_key or secret_key".red().bold());
    }
    if connection_status{
        // Proceede only if connection was able to be established
        println!("Testing credentials \t\t{}", "[OK]".green().bold());
        if *arguments.debug(){
            println!("{} {} {}","DEBUG:".bright_magenta().bold(),"Test credentials Response code",connection_status_http_code.bright_yellow());
        }

        if *arguments.list_new_requests(){
            if let Ok(result) = request::Request::get_request().await{
                let message = result.message().to_owned();
                if message.is_empty(){
                    println!("{}","No detected request found".yellow().bold());
                }
                let mut list_req_new : Vec<RequestResult> = Vec::new();
                for req in message{
                    if *arguments.list_new_requests(){
                        if req.status() == "N"{
                            list_req_new.push(RequestResult{ 
                                id: req.id().to_owned(),
                                title: req.title().to_string(),
                                status: "New".to_string(), 
                                valid: req.valid().to_owned() 
                            });
                        }
                    }
                }
                if !list_req_new.is_empty(){
                    let _ = print_table("New requests",list_req_new);
                }else{
                    println!("{}","No New Request found".yellow().bold());
                }
            }
        }
    }
    process::exit(error_code);
}
