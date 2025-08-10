use std::collections::HashMap;
use json::JsonValue;
use chrono::{DateTime, Utc, Datelike};

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    if let JsonValue::Array(commits) = data {
        for commit in commits {
        if let Some(author_login) = commit["author"]["login"].as_str() {
                *counts.entry(author_login.to_string()).or_insert(0) += 1;
        }
        }
    }
    counts
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    if let JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
                if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                    let iso_year = date.iso_week().year();
                    let iso_week = date.iso_week().week();

                    let key = format!("{}-W{}", iso_year, iso_week);
                    *counts.entry(key).or_insert(0) += 1;
                }
            }
        }
    }
    counts
}
