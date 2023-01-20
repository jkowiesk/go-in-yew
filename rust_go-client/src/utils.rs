

pub fn format_msg(msg_type: &str, content: &str) -> String{
    if content == "" {
        String::from(format!("{{\"message_type\": \"{}\"}}", msg_type))
    } else {
        String::from(format!("{{\"message_type\": \"{}\", {}}}", msg_type, content))
    }
}