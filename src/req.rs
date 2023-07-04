use std::io;
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

const GO_VERSION_ENDPOINT: &'static str = "https://go.dev/dl/?mode=json";
const GO_DOWNLOAD_ENDPOINT: &'static str = "https://go.dev/dl/";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileItem {
    pub filename: String,
    pub os: String,
    pub arch: String,
    pub version: String,
    pub sha256: String,
    pub size: i32,
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionItem {
    pub version: String,
    pub stable: bool,
    pub files: Vec<FileItem>,
}

pub fn get() -> Result<Vec<VersionItem>, reqwest::Error> {
    let response = reqwest::blocking::get(GO_VERSION_ENDPOINT)?
        .json::<Vec<VersionItem>>()?;
    Ok(response)
}

#[derive(Debug)]
pub struct FileDownload {
    pub filename: String,
    pub download_to: String, 
}

impl FileDownload {
    pub fn new(download_dir: &str, filename: &str) -> Self {
        let download_to: PathBuf = [download_dir, filename].iter().collect();
        Self {
            filename: String::from(filename),
            download_to: String::from(download_to.to_str().unwrap()),
        }
    }

    pub fn download(&self) {
        let resp = reqwest::blocking::get(GO_DOWNLOAD_ENDPOINT.to_owned() + self.filename.as_str()).expect("[Download] request failed");
        let mut body = std::io::Cursor::new(resp.bytes().expect("[Download] body invalid"));
        let mut out = File::create(self.download_to.as_str()).expect("[Download] failed to create file");
        io::copy(&mut body, &mut out).expect("[Download] failed to copy content");
    }
}
