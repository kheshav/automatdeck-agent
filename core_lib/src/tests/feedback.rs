#[allow(unused_imports)]
use crate::feedback;

#[test]
fn test_feedback_format(){
    // Test format of feedback
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::OUTPUT),"<p>hello</p>");
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::STEP),"<p>hello</p>");
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::COMMAND),"<p>hello</p>");
    assert_eq!(feedback::format("hello".to_string(), feedback::FeedbackType::ERROR),"<p>hello</p>");
}

