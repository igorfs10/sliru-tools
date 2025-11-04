use std::collections::HashMap;

pub struct RequestResult {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}
