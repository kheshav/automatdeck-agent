#[allow(unused_imports)]
use crate::feedback;

#[test]
fn test_feedback_format(){
    // Test format of feedback
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::OUTPUT),"<p class=\\\"output\\\">hello</p>");
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::STEP),"<p class=\\\"step\\\">hello</p>");
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::COMMAND),"<p class=\\\"command\\\">hello</p>");
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::ERROR),"<p class=\\\"error\\\">hello</p>");
}

