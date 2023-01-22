

pub fn format_msg(msg_type: &str, content: &str) -> String{
    if content == "" {
        String::from(format!("{{\"message_type\": \"{}\"}}", msg_type))
    } else {
        String::from(format!("{{\"message_type\": \"{}\", {}}}", msg_type, content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_msg_without_content() {
        let final_msg = "{\"message_type\": \"test\", \"test\": \"check\"}";
        let formatted_msg = format_msg("test", "\"test\": \"check\"");
        assert!(formatted_msg == final_msg);
    }

    #[test]
    fn test_format_msg_with_content() {
        let final_msg = "{\"message_type\": \"test\"}";
        let formatted_msg = format_msg("test", "");
        assert!(formatted_msg == final_msg);
    }
}
