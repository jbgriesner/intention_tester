#[derive(Debug, Deserialize)]
pub struct CsvRow {
    pub query: String,
    pub intention: String,
    pub other: Option<String>,
}
