use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blog {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogPost {
    pub url: String,
    pub title: String,
    pub content: String,
    pub blog: Blog,
    pub categories: Vec<String>,
    #[serde(with = "ts_seconds_option")]
    pub published: Option<DateTime<Utc>>,
}