pub enum FormatConverter {
    Json,
    Csv,
    Yaml,
}

impl From<i32> for FormatConverter {
    fn from(value: i32) -> Self {
        match value {
            0 => FormatConverter::Json,
            1 => FormatConverter::Csv,
            2 => FormatConverter::Yaml,
            _ => FormatConverter::Json,
        }
    }
}
