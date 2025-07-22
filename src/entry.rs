use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EntryType {
    File,
    Folder,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EntryContent {
    Many(Box<Vec<Entry>>),
    Data(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub ty: EntryType,
    pub path: String,
    pub contents: EntryContent,
    pub created_at: SystemTime,
}