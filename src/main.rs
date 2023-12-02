#[macro_use]
extern crate lazy_static;

use structopt::Opt;
use util::file_reqwest::{download_mp4, DownloadMp4Error};

// mod request_yt1s;
mod start_server;
pub mod structopt;
mod util;

lazy_static! {
    static ref OPTS: Opt = structopt::get_opt();
    pub static ref TITLE_FILE: &'static str = OPTS.title_file.as_str();
    pub static ref MP4_PATH: &'static str = OPTS.mp4_path.as_str();
}

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server::start().unwrap();
    Ok(())
}

#[derive(Debug)]
pub enum DescError {
    OpenDescFileError(String),
    ParseDescFileError(String),
    IoError(String),
    FindIndexNone(String),
}

impl ToString for DescError {
    fn to_string(&self) -> String {
        match self {
            DescError::OpenDescFileError(err) => err.to_string(),
            DescError::ParseDescFileError(err) => err.to_string(),
            DescError::IoError(err) => err.to_string(),
            DescError::FindIndexNone(err) => err.to_string(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Link {
    id: String,
    desc: String,
}

pub fn get_desc(id: &String) -> Result<String, DescError> {
    let data = util::json_parse::parse::<Vec<Link>>(
        util::file_io::open(*TITLE_FILE)
            .map_err(|err| DescError::OpenDescFileError(err.to_string()))?
            .as_str(),
    )
    .map_err(|err| DescError::ParseDescFileError(err.to_string()))?;
    Ok(data
        .iter()
        .find(|x| &x.id == id)
        .ok_or(DescError::FindIndexNone(String::from(
            "DescError::FindIndexNone",
        )))?
        .desc
        .clone())
}

pub fn add_desc(id: &String, desc: &String) -> Result<(), DescError> {
    let mut data = util::json_parse::parse::<Vec<Link>>(
        util::file_io::open(*TITLE_FILE)
            .map_err(|err| DescError::OpenDescFileError(err.to_string()))?
            .as_str(),
    )
    .map_err(|err| DescError::ParseDescFileError(err.to_string()))?;
    if let Some(index) = data.iter().position(|x| &x.id == id) {
        data.remove(index);
    }
    data.append(
        vec![Link {
            id: id.to_string(),
            desc: desc.to_string(),
        }]
        .as_mut(),
    );
    Ok(util::file_io::write(
        *TITLE_FILE,
        util::json_parse::stringify::<Vec<Link>>(&data)
            .map_err(|err| DescError::ParseDescFileError(err.to_string()))?
            .as_str(),
    )
    .map_err(|err| DescError::IoError(err.to_string()))?)
}

pub fn remove_desc(id: &String) -> Result<(), DescError> {
    let mut data = util::json_parse::parse::<Vec<Link>>(
        util::file_io::open(*TITLE_FILE)
            .map_err(|err| DescError::OpenDescFileError(err.to_string()))?
            .as_str(),
    )
    .map_err(|err| DescError::ParseDescFileError(err.to_string()))?;
    data.remove(
        data.iter()
            .position(|x| &x.id == id)
            .ok_or(DescError::FindIndexNone(String::from(
                "DescError::FindIndexNone",
            )))?,
    );
    Ok(util::file_io::write(
        *TITLE_FILE,
        util::json_parse::stringify::<Vec<Link>>(&data)
            .map_err(|err| DescError::ParseDescFileError(err.to_string()))?
            .as_str(),
    )
    .map_err(|err| DescError::IoError(err.to_string()))?)
}

pub async fn add_mp4(id: &String, url: &String) -> Result<(), DownloadMp4Error> {
    download_mp4(url, &(MP4_PATH.to_string() + &id + ".mp4")).await
}
