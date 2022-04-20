use base64;
use chrono::Local;
use colored::*;

pub enum FeedbackType{
    // For job feedback formating
    STEP,
    COMMAND,
    OUTPUT,
    ERROR
}

pub enum FedbackDisplayType{
    // For console display
    ERROR,
    INFO,
    DEBUG,
    WARN,
}

pub fn format(message: String, message_type: FeedbackType) -> String {
    // Formater for feedback
    let mut final_message: String = "".to_string();
    let date = Local::now();
    let message = htmlescape::encode_minimal(&message);
    match message_type{
        FeedbackType::STEP =>{
            final_message.push_str("<p class=\"step\">");
            final_message.push_str(&date.format("[%Y-%m-%d %H:%M:%S%.6f %Z] - ").to_string());
            final_message.push_str(&message);
            final_message.push_str("</p>");
        },
        FeedbackType::COMMAND =>{
            final_message.push_str("<p class=\"command\">");
            final_message.push_str(&message);
            final_message.push_str("</p>");
        },
        FeedbackType::OUTPUT =>{
            final_message.push_str("<p class=\"output\">");
            final_message.push_str(&message);
            final_message.push_str("</p>");
        },
        FeedbackType::ERROR =>{
            final_message.push_str("<p class=\"error\">");
            final_message.push_str(&message);
            final_message.push_str("</p>");
        },
    }
    return base64::encode(final_message.as_bytes());
}


pub fn format_display(message: &str, message_type: FedbackDisplayType){
    // Display message to console
    let date = Local::now();
    #[allow(unused_assignments)]
    let mut display_level: ColoredString = "".bold();
    let mut rendered_message: ColoredString = format!("{}",message).normal();
    match message_type {
        FedbackDisplayType::ERROR =>{
            display_level = "[ERROR]".red().bold();
            rendered_message = rendered_message.bold();
        },
        FedbackDisplayType::INFO =>{
            display_level = "[INFO]".blue().bold();
        },
        FedbackDisplayType::WARN =>{
            display_level = "[WARNING]".yellow().bold();
        },
        FedbackDisplayType::DEBUG =>{
            display_level = "[DEBUG]".magenta().bold();
        },
    }
    println!("{} - {} {}", date.format("[%Y-%m-%d %H:%M:%S]"),display_level, rendered_message);
}
