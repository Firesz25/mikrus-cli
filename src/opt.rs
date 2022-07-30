use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub struct Opt {
    #[clap(short = 's', long)]
    pub srv: Option<String>,
    #[clap(short = 'k', long)]
    pub key: Option<String>,
}
