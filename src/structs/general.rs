#![allow(non_snake_case)]

use serde::Serialize;

#[derive(Clone)]
pub struct StaticData {
    pub website_url: String,
    pub storage_url: String,
}

#[derive(Serialize, Clone)]
pub struct FileUnit {
    pub title: String,
    pub link: String,
}

#[derive(Serialize, Clone)]
pub struct FileDetails {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub root: String,
    pub downloads: String,
    pub files: Vec<FileUnit>,
}
