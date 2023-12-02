use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short="t", long="title", help="The filename of title.json, eg. `./title.json`")]
    pub title_file: String,

    #[structopt(short="m", long="mp4", help="The path of mp4s, eg. `./mp4`")]
    pub mp4_path: String,
}

pub fn get_opt() -> Opt {
    Opt::from_args()
}
