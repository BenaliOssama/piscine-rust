use std::collections::HashMap;
use json::JsonValue;
use chrono::Datelike;
//
pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    if let JsonValue::Array(commits) = data {
        for commit in commits {
            // author login is commit["author"]["login"]
            if let Some(author) = commit["author"]["login"].as_str() {
                *counts.entry(author.to_string()).or_insert(0) += 1;
            }
        }
    }
    counts
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    if let JsonValue::Array(commits) = data {
        for commit in commits {
            // commit date is commit["commit"]["author"]["date"]
            if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
                if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
                    let iso_week = date.iso_week();
                    let key = format!("{}-W{}", iso_week.year(), iso_week.week());
                    *counts.entry(key).or_insert(0) += 1;
                }
            }
        }
    }
    counts
}

