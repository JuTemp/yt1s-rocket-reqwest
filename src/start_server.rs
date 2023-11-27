use std::collections::HashMap;

use rocket::{
    catch, catchers, fs::FileServer, get, http::Status, post, response::content::RawHtml, routes,
    Response,
};

use crate::util;

#[get("/")]
async fn root() -> &'static str {
    "Enter id."
}

#[get("/<id>")]
async fn parse_link(id: u32) -> RawHtml<String> {
    RawHtml(format!(
        r#"<h1>{}</h1><video controls src="static/{}.mp4" style="width: 100%; height: 100%; object-fit: contain;"></video>"#,
        crate::get_desc(&id.to_string()).unwrap(),
        id
    ))
}

#[post("/add", data = "<body>")]
fn add(body: String) {
    let body = util::json_parse::parse::<HashMap<String, String>>(body.as_str());
    if body.is_err()
        || body.as_ref().unwrap().get("id").is_none()
        || body.as_ref().unwrap().get("name").is_none()
    {
        Response::build()
            .status(Status::BadRequest)
            .ok::<()>()
            .unwrap();
    };
    let body = body.unwrap();
    crate::add_desc(body.get("name").unwrap(), body.get("desc").unwrap()).unwrap();
}

#[post("/remove", data = "<body>")]
fn remove(body: String) {
    let body = util::json_parse::parse::<HashMap<String, String>>(body.as_str());
    if body.is_err() || body.as_ref().unwrap().get("id").is_none() {
        Response::build()
            .status(Status::BadRequest)
            .ok::<()>()
            .unwrap();
    };
    let body = body.unwrap();
    crate::remove_desc(body.get("name").unwrap()).unwrap();
}

#[catch(404)]
async fn not_found() -> &'static str {
    "404 Not Found"
}

#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 20833)))
        .mount("/static/", FileServer::from("./"))
        .mount("/", routes![root, parse_link, add, remove])
        .register("/", catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
