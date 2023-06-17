use std::{
    format,
    sync::{Arc, Mutex},
};

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
    board: Arc<Mutex<Vec<Entry>>>,
}

impl Scoreboard {
    pub fn new() -> Self {
        Self {
            board: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn new_entry(&self, user: String, ratio: CompressionRatio) {
        let mut locked_board = self.board.lock().unwrap();
        if locked_board
            .iter()
            .find(|entry| entry.user == user)
            .is_none()
        {
            locked_board.push(Entry { user, ratio });
        } else {
            for entry in locked_board.iter_mut() {
                if entry.user == user && ratio.lines > entry.ratio.lines {
                    entry.ratio = ratio;
                    break;
                }
            }
        }
        locked_board.sort();
        locked_board.reverse();
    }
    pub fn get(&self) -> Vec<Entry> {
        let locked_board = self.board.lock().unwrap();
        return locked_board.clone();
    }
}

impl ToString for Scoreboard {
    fn to_string(&self) -> String {
        let locked_board = self.board.lock().unwrap();
        locked_board
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
