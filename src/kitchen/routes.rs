use std::{env, fs};
use infer::MatcherType;
use rocket::{FromForm, get, post};
use rocket::form::Form;
use rocket::fs::TempFile;
use serde_json::json;
use crate::errors::ApiError;
use crate::kitchen::util::unique_file_name;

const STORAGE_PATH: &str = "storage/kitchen/";

#[get("/kitchen")]
pub fn get_files() -> String {
    let files = fs::read_dir(STORAGE_PATH).unwrap();
    let mut paths = Vec::new();
    for file in files {
        let path = file.unwrap().path();

        let ii = infer::get_from_path(&path).unwrap();
        if let Some(ii) = ii {
            if ii.matcher_type() == MatcherType::Image {
                paths.push(path);
            }
        }
    }
    json!(paths).to_string()
}

#[derive(FromForm)]
pub struct UploadForm<'v> {
    password: &'v str,
    files: Vec<TempFile<'v>>,
}

#[post("/kitchen/upload", data = "<data>")]
pub async fn upload_file(mut data: Form<UploadForm<'_>>) -> Result<(), ApiError> {
    let token = env::var("KITCHEN_TOKEN").unwrap();
    if data.password != token {
        return Err(ApiError::InvalidToken);
    }

    if data.files.is_empty() {
        return Err(ApiError::NoFileUploaded);
    }

    for file in data.files.iter_mut() {
        // TODO: check if file type is image using infer crate

        let extension = file.content_type().unwrap().extension().unwrap().as_str();
        let name = unique_file_name(extension);
        let path = format!("{STORAGE_PATH}{name}");

        file.persist_to(&path).await.unwrap();
    }

    Ok(())
}
