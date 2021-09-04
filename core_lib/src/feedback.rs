use base64;
use chrono::Local;

pub enum FeedbackType{
    STEP,
    COMMAND,
    OUTPUT,
    ERROR
}

pub fn format(message: String, message_type: FeedbackType) -> String {
    // Formater for feedback
    let mut final_message: String = "".to_string();
    let message = htmlescape::encode_minimal(&message);
    match message_type{
        FeedbackType::STEP =>{
            final_message.push_str("<p class=\"step\">");
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

pub fn format_display(message: &str){
    // Display message to console
    let date = Local::now();
    println!("{} - {}", date.format("[%Y-%m-%d %H:%M:%S]"), message);
}
