#[derive(Debug, Clone)]
pub struct Game {
    pub code: String,
    pub name: String,
    pub console: String,
    pub region: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub path: String,
    pub code: Option<String>,
    pub game_name: Option<String>,
    pub console: Option<String>,
}