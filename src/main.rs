// mod request_yt1s;
mod start_server;
mod util;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server::start().unwrap();
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Link {
    name: String,
    desc: String,
}

pub fn get_desc(name: &String) -> Result<String, std::io::Error> {
    let data = util::json_parse::parse::<Vec<Link>>(
        util::file_io::open("/home/jtp/Temp/link.json")?.as_str(),
    )?;
    Ok(data.iter().find(|x| &x.name == name).unwrap().desc.clone())
}

pub fn add_desc(name: &String, desc: &String) -> Result<(), std::io::Error> {
    let mut data = util::json_parse::parse::<Vec<Link>>(
        util::file_io::open("/home/jtp/Temp/link.json")?.as_str(),
    )?;
    data.append(
        vec![Link {
            name: name.to_string(),
            desc: desc.to_string(),
        }]
        .as_mut(),
    );
    Ok(util::file_io::write(
        "/home/jtp/Temp/link.json",
        util::json_parse::stringify::<Vec<Link>>(&data)?.as_str(),
    )?)
}

pub fn remove_desc(name: &String) -> Result<(), std::io::Error> {
    let mut data = util::json_parse::parse::<Vec<Link>>(
        util::file_io::open("/home/jtp/Temp/link.json")?.as_str(),
    )?;
    data.remove(data.iter().position(|x| &x.name == name).unwrap());
    Ok(util::file_io::write(
        "/home/jtp/Temp/link.json",
        util::json_parse::stringify::<Vec<Link>>(&data)?.as_str(),
    )?)
}
