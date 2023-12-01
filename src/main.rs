use util::file_reqwest::{download_mp4, DownloadMp4Error};

// mod request_yt1s;
mod start_server;
mod util;

pub const TITLE_FILE: &str = "./link.json";
pub const MP4_PATH: &str = "./mp4/";

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server::start().unwrap();
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Link {
    id: String,
    desc: String,
}

pub fn get_desc(id: &String) -> Result<String, std::io::Error> {
    let data = util::json_parse::parse::<Vec<Link>>(util::file_io::open(TITLE_FILE)?.as_str())?;
    Ok(data.iter().find(|x| &x.id == id).unwrap().desc.clone())
}

pub fn add_desc(id: &String, desc: &String) -> Result<(), std::io::Error> {
    let mut data = util::json_parse::parse::<Vec<Link>>(util::file_io::open(TITLE_FILE)?.as_str())?;
    data.append(
        vec![Link {
            id: id.to_string(),
            desc: desc.to_string(),
        }]
        .as_mut(),
    );
    Ok(util::file_io::write(
        TITLE_FILE,
        util::json_parse::stringify::<Vec<Link>>(&data)?.as_str(),
    )?)
}

pub fn remove_desc(id: &String) -> Result<(), std::io::Error> {
    let mut data = util::json_parse::parse::<Vec<Link>>(util::file_io::open(TITLE_FILE)?.as_str())?;
    data.remove(data.iter().position(|x| &x.id == id).unwrap());
    Ok(util::file_io::write(
        TITLE_FILE,
        util::json_parse::stringify::<Vec<Link>>(&data)?.as_str(),
    )?)
}

pub async fn add_mp4(id: &String, url: &String) -> Result<(), DownloadMp4Error> {
    download_mp4(url, &(MP4_PATH.to_string() + &id + ".mp4")).await
}
