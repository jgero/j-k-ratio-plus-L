use std::format;

use serde::Serialize;

use crate::CompressionRatio;

#[derive(PartialEq, Eq, Clone, Serialize)]
pub struct Entry {
    user: String,
    ratio: CompressionRatio,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ratio.chars.total_cmp(&other.ratio.chars)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub struct Scoreboard {
    board: Vec<Entry>,
}

impl Scoreboard {
    pub fn new() -> Self {
        Self { board: Vec::new() }
    }
    pub fn new_entry(&mut self, user: String, ratio: CompressionRatio) {
        if self.board.iter().find(|entry| entry.user == user).is_none() {
            self.board.push(Entry { user, ratio });
        } else {
            for entry in self.board.iter_mut() {
                if entry.user == user && ratio.lines > entry.ratio.lines {
                    entry.ratio = ratio;
                    break;
                }
            }
        }
        self.board.sort();
        self.board.reverse();
    }
    pub fn get(&self) -> Vec<Entry> {
        self.board.clone()
    }
}

impl ToString for Scoreboard {
    fn to_string(&self) -> String {
        self.board
            .iter()
            .map(|entry| {
                format!(
                    "[{}]: chars: {} lines: {}",
                    entry.user, entry.ratio.chars, entry.ratio.lines
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
