use actix_web::{web, HttpResponse, Result, http};
use actix_web::HttpRequest;
use handlebars::Handlebars;
use serde_json::json;
use std::fs;
use std::io::Read;

use crate::structs::general::*;

fn read_file_contents(file_path: &str) -> Result<String, bool> {
    match fs::File::open(file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(_) = file.read_to_string(&mut contents) {
                return Err(true);
            }
            Ok(contents)
        }
        Err(_) => Err(false),
    }
}

fn get_files(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files: Vec<String> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    files.push(file_name_str.to_string());
                }
            }
        }
    }
    Ok(files)
}

pub async fn open(req: HttpRequest, hb: web::Data<Handlebars<'_>>, statics: web::Data<StaticData>) -> Result<HttpResponse, HttpResponse> {
	let file_id = req.match_info().get("file_id").unwrap();
    let st: StaticData = statics.get_ref().clone();
    let root_path: String = st.storage_url + "/" + file_id + "/";
    let downloads_path: String = root_path.to_owned() + "/downloads";

    let mut root_link: String = st.website_url.to_owned() + "/storage/" + file_id + "/";
    if root_link[root_link.len() - 1..] == "/".to_string() {
        root_link = root_link[..(root_link.len() -1)].to_string();
    }
    let downloads_link: String = root_link.to_owned() + "/downloads";

    let mut file_details = FileDetails {
        title: None,
        description: None,
        image: None,
        root: root_link.to_owned(),
        downloads: downloads_link.to_owned(),
        files: get_files(&downloads_path).unwrap_or(Vec::new()).iter().map(|v| FileUnit {
            title: v.to_owned(),
            link: downloads_link.to_owned() + "/" + v,
        }).collect(),
    };

    let files = get_files(&root_path).unwrap_or(Vec::new());
    for f in files {
        let len = f.len();
        if len < 4 {
            continue;
        }

        let suffix = &f[len - 4..].to_lowercase();
        if suffix == ".png" || suffix == ".jpg" || suffix == ".jpg" {
            file_details.image = Some(root_link.to_owned() + "/" + &f);
        }
        else if suffix == ".txt" {
            file_details.title = Some(f[..(len -4)].to_owned());
            file_details.description = match read_file_contents(&(root_path.to_owned() + &f)) {
                Ok(res) => Some(res),
                Err(_e) => None,
            };
        }
    }

    let data = json!(file_details);
    let body = hb.render("index", &data).unwrap();
    return Ok(HttpResponse::Ok().header(
        http::header::CONTENT_TYPE, "text/html"
    ).body(body));
} 
