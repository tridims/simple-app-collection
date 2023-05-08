use crate::server::model::Note;

// Get id from request URL
pub fn get_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

// deserialize note from request body without id
pub fn get_user_request_body(request: &str) -> Result<Note, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
