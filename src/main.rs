use clap::{Parser, Subcommand};
use figment::providers::Env;
use figment::providers::Format;
use figment::providers::Serialized;
use figment::providers::Toml;
use figment::Figment;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, env = "FLAG", default_value = "false")]
    flag: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
enum Commands {
    Test {
        #[arg(short, long, env = "NUMBER", default_value = "1")]
        number: u16,
    },
}

fn main() {
    let config: Cli = Figment::new()
        .merge(Serialized::defaults(Cli::parse()))
        .merge(Toml::file("config.toml"))
        .merge(Env::raw())
        .extract()
        .expect("Failed to load config");

    println!("{config:#?}");
}
