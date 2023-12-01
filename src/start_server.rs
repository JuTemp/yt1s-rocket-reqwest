use rocket::{
    catch, catchers,
    fs::FileServer,
    get, post,
    response::{content::RawHtml, status},
    routes,
};
use serde::Deserialize;

use crate::util;

#[get("/")]
async fn root() -> RawHtml<&'static str> {
    RawHtml::<&str>(
        r#"<input id="id"><button onclick="window.location.pathname=document.querySelector('#id').value">Enter</button>"#,
    )
}

#[get("/<id>")]
async fn parse_link(id: u32) -> RawHtml<String> {
    RawHtml(format!(
        r#"<h1>{}</h1><video controls src="static/{}.mp4" style="width: 100%; height: 100%; object-fit: contain;"></video>"#,
        crate::get_desc(&id.to_string()).unwrap(),
        id
    ))
}

#[derive(Deserialize)]
struct AddDesc {
    id: String,
    desc: String,
}

#[post("/add_desc", format = "json", data = "<body>")]
fn add(body: String) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    println!("{}", "add");
    let body = util::json_parse::parse::<AddDesc>(body.as_str())
        .map_err(|_| status::BadRequest(String::from("Err")))?;
    crate::add_desc(&body.id, &body.desc).map_err(|_| status::BadRequest(String::from("Err")))?;
    Ok(status::Accepted(String::from("Ok")))
}

#[derive(Deserialize)]
struct RemoveDesc {
    id: String,
}

#[post("/remove_desc", format = "json", data = "<body>")]
fn remove(body: String) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    let body = util::json_parse::parse::<RemoveDesc>(body.as_str())
        .map_err(|_| status::BadRequest(String::from("Err")))?;
    crate::remove_desc(&body.id).map_err(|_| status::BadRequest(String::from("Err")))?;
    Ok(status::Accepted(String::from("Ok")))
}

#[derive(Deserialize)]
struct AddMp4struct {
    id: String,
    url: String,
}

/// @income {"id":"<id>", "url":"<URL>"}
#[post("/add_mp4", format = "json", data = "<body>")]
async fn add_mp4(body: String) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    let body = util::json_parse::parse::<AddMp4struct>(body.as_str())
        .map_err(|_| status::BadRequest(String::from("Err")))?;
    crate::add_mp4(&body.id, &body.url)
        .await
        .map_err(|_| status::BadRequest(String::from("Err")))?;

    Ok(status::Accepted(String::from("Ok")))
}

#[catch(default)]
async fn not_found() -> &'static str {
    "catcher"
}

#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 20833)))
        .mount("/static/", FileServer::from(crate::MP4_PATH))
        .mount("/", routes![root, parse_link, add, remove, add_mp4])
        .register("/", catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
