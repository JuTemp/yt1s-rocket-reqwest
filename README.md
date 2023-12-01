# yt1s-rocket-reqwest

pub const TITLE_FILE: &str = "./link.json";
pub const MP4_PATH: &str = "./mp4/";

GET `/` -> root
GET `/<id>` -> parse id
POST `/add` {"id":"<id>", "desc":"<desc>"} -> add
POST `/remove` {"id":"<id>"} -> remove
