use reqwest::Client;
use std::fs::File;
use std::io::{self, Cursor};

pub enum DownloadMp4Error {
    ReqwestError(String),
    IoError(String),
}

impl ToString for DownloadMp4Error {
    fn to_string(&self) -> String {
        match self {
            DownloadMp4Error::ReqwestError(err) => err.to_string(),
            DownloadMp4Error::IoError(err) => err.to_string(),
        }
    }
}

pub async fn download_mp4(url: &String, file_name: &String) -> Result<(), DownloadMp4Error> {
    let response = Client::new()
        .get(url)
        .send()
        .await
        .map_err(|err| DownloadMp4Error::ReqwestError(err.to_string()))?;
    let mut file =
        File::create(file_name).map_err(|err| DownloadMp4Error::IoError(err.to_string()))?;
    io::copy(
        &mut Cursor::new(
            response
                .bytes()
                .await
                .map_err(|err| DownloadMp4Error::ReqwestError(err.to_string()))?,
        ),
        &mut file,
    )
    .map_err(|err| DownloadMp4Error::IoError(err.to_string()))?;

    Ok(())
}
