use std::collections::HashMap;

pub struct RequestData {
    pub method: Option<String>,
    pub url: Option<String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout_secs: Option<u64>,
    pub follow_redirects: Option<bool>,
}
