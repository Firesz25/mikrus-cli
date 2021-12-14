use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "firesz25 <olek.cz@outlook.com>")]
pub struct Opt {
    #[clap(short = 's', long)]
    pub srv: Option<String>,
    #[clap(short = 'k', long)]
    pub key: Option<String>,
}
