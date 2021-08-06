pub enum FeedbackType{
    STEP,
    COMMAND,
    OUTPUT,
    ERROR
}

pub fn format(message: String, message_type: FeedbackType) -> String {
    // Formater for feedback
    let mut final_message: String = "".to_string();
    match message_type{
        FeedbackType::STEP =>{
            final_message.push_str("<p>");
            final_message.push_str(&message);
            final_message.push_str("</p>");
        },
        FeedbackType::COMMAND =>{
            final_message.push_str("<p>");
            final_message.push_str(&message);
            final_message.push_str("</p>");
        },
        FeedbackType::OUTPUT =>{
            final_message.push_str("<p>");
            final_message.push_str(&message);
            final_message.push_str("</p>");
        },
        FeedbackType::ERROR =>{
            final_message.push_str("<p>");
            final_message.push_str(&message);
            final_message.push_str("</p>");
        },
    }
    return final_message;
}
