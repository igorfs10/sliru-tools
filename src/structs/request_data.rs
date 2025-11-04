use std::collections::HashMap;

pub struct RequestData {
    pub method: Option<String>,
    pub url: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
