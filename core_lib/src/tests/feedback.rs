#[allow(unused_imports)]
use crate::feedback;

#[test]
fn test_feedback_format(){
    // Test format of feedback
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::OUTPUT),base64::encode("<p class=\"output\">hello</p>".as_bytes()));
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::COMMAND),base64::encode("<p class=\"command\">hello</p>".as_bytes()));
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::ERROR),base64::encode("<p class=\"error\">hello</p>".as_bytes()));
}

