use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct HighScoreEntry {
    name: String,
    score: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HighScores {
    entries: Vec<HighScoreEntry>,
}

impl HighScores {
    const FILE_PATH: &'static str = "high_scores.json";

    pub fn new() -> Self {
        HighScores { entries: Vec::new() }
    }

    pub fn load(&mut self) -> io::Result<()> {
        if Path::new(Self::FILE_PATH).exists() {
            let mut file = File::open(Self::FILE_PATH)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            *self = serde_json::from_str(&contents)?;
        }
        Ok(())
    }

    fn save(&self) -> io::Result<()> {
        let mut file = File::create(Self::FILE_PATH)?;
        let contents = serde_json::to_string(&self)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn qualifies(&self, score: i32) -> bool {
        self.entries.len() < 10 || score > self.entries.last().unwrap().score
    }

    pub fn add_score(&mut self, name: String, score: i32) {
        let trimmed_name: String = name.trim().chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect();

        self.entries.push(HighScoreEntry { name: trimmed_name, score });
        self.entries.sort_by(|a, b| b.score.cmp(&a.score));
        self.entries.truncate(10);

        if let Err(e) = self.save() {
            eprintln!("Error saving high scores: {}", e);
        }
    }

    pub fn display(&self) -> Vec<String> {
        let mut strings: Vec<String> = Vec::new();
        for (i, entry) in self.entries.iter().enumerate() {
            strings.push(format!("{}: {} - {}", i + 1, entry.name, entry.score));
        }
        strings
    }
}