use regex::Regex;
use walkdir::WalkDir;
use crate::core::ScanResult;
use crate::db::GameDB;

pub struct Scanner {
    db: GameDB,
    patterns: Vec<(String, Regex)>,
}

impl Scanner {
    pub fn new(db: GameDB) -> Self {
        let patterns = vec![
            ("PS4".to_string(), Regex::new(r"CUSA\\d{5}").unwrap()),
            ("PS3".to_string(), Regex::new(r"BLES\\d{5}|BLUS\\d{5}").unwrap()),
            ("PSVita".to_string(), Regex::new(r"PCSE\\d{5}|PCSB\\d{5}").unwrap()),
            //("PSP".to_string(), Regex::new(r"ULUS\\d{5}|ULES\\d{5}").unwrap()),
            ("PSP".to_string(),Regex::new(r"[A-Z]{4}\d{5}").unwrap(),),
            ("Nintendo Switch".to_string(), Regex::new(r"0100[0-9A-F]{12}").unwrap()),
            ("3DS".to_string(), Regex::new(r"CTR-P-[A-Z0-9]{4}").unwrap()),
            ("Nintendo DS".to_string(), Regex::new(r"NTR-[A-Z0-9]{4}").unwrap()),
        ];

        Self { db, patterns }
    }

    fn extract_code(&self, text: &str) -> Option<(String, String)> {
        let upper = text.to_uppercase();

        for (console, regex) in &self.patterns {
            if let Some(mat) = regex.find(&upper) {
                return Some((mat.as_str().to_string(), console.clone()));
            }
        }

        None
    }

    pub fn scan(&self, path: &str) -> Vec<ScanResult> {
        let mut results = vec![];

        println!("=== Starting scan ===");
        println!("Path: {}", path);

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {

            println!("Entry: {}", entry.path().display());

            let file_name = entry.file_name().to_string_lossy();

            let (code, console) = match self.extract_code(&file_name) {
                Some(v) => {
                    println!(
                        "Found code '{}' in '{}'",
                        v.0,
                        file_name
                    );
                    v
                }
                None => {
                    println!(
                        "No code found in '{}'",
                        file_name
                    );
                    continue;
                }
            };

            let game = self.db.get_game_by_code(&code);

            match &game {
                Some(g) => {
                    println!(
                        "Matched {} -> {}",
                        code,
                        g.name
                    );
                }
                None => {
                    println!(
                        "No DB match for {}",
                        code
                    );
                }
            }

            results.push(ScanResult {
                path: entry.path().display().to_string(),
                code: Some(code),
                game_name: game.as_ref().map(|g| g.name.clone()),
                console: Some(console),
            });
        }

        println!("Results found: {}", results.len());

        results
    }
}