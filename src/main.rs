mod config;
mod model;
mod opt;
mod view;

use clap::Parser;
use view::HELLO_MSG;
use model::ApiClient;
use model::get_config;
use config::create_config_file;

use crate::config::Config;

#[tokio::main]
async fn main() {
    create_config_file().unwrap();
    let opts = opt::Opt::parse();
    let cfg = get_config(opts).unwrap();
    println!("{}", HELLO_MSG);
    let client = ApiClient::new();
    loop {
        let input = view::promt().unwrap();
        match input.as_str() {
            "exit" => break,
            "help" => view::print_help(),
            "info" => println!("{}", client.info(cfg.srv.clone(), cfg.key.clone()).await.unwrap()),
            "serwery" => println!("{}", client.serwery(cfg.srv.clone(), cfg.key.clone()).await.unwrap()),
            "restart" => println!("{}", client.restart(cfg.srv.clone(), cfg.key.clone()).await.unwrap()),
            "logs" => println!("{}", client.logs(cfg.srv.clone(), cfg.key.clone()).await.unwrap()),
            "amfetamina" => println!("{}", client.amfetamina(cfg.srv.clone(), cfg.key.clone()).await.unwrap()),
            "db" => println!("{}", client.db(cfg.srv.clone(), cfg.key.clone()).await.unwrap()),
            "stats" => println!("{}", client.stats(cfg.srv.clone(), cfg.key.clone()).await.unwrap()),
            "porty" => println!("{}", client.porty(cfg.srv.clone(), cfg.key.clone()).await.unwrap()),
            "login" => {
                println!("Please input server number");
                let srv = view::promt().unwrap();
                println!("Please input key");
                let key = view::promt().unwrap();
                let config = Config::new(srv, key);
                println!("save config to file? (y/n)");
                let save = view::promt().unwrap();
                if save == "y" {
                    config.save().unwrap();
                }
            }
            input if input.starts_with("log") => {
                let mut args = input.split_whitespace();
                let log_id = args.nth(1).unwrap();
                println!("{}", client.log_id(cfg.srv.clone(), cfg.key.clone(), log_id).await.unwrap());
            },
            input if input.starts_with("exec") => {
                let cmd = input[4..].trim();
                println!("{}", client.exec(cfg.srv.clone(), cfg.key.clone(), cmd).await.unwrap());
            },
            _ => println!("use help for more info {}", input),
        }
    }
}
