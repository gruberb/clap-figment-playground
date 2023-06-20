use clap::Args;
use clap::{Parser, Subcommand};
use figment::error::Kind;
use figment::providers::Env;
use figment::providers::Format;
use figment::providers::Serialized;
use figment::providers::Toml;
use figment::providers::Yaml;
use figment::Figment;
use serde::{Deserialize, Serialize};

const DEFAULT_DB_PATH: &str = "./default-db/";

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[arg(short, long, env = "FLAG")]
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<bool>,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
enum SubCommand {
    Tce(Tce),
}

#[derive(Args, Debug, Serialize, Deserialize)]
struct Tce {
    #[arg(short, long, env = "TCE_LOCAL_PORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    local_port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Validator {
    #[serde(default = "default_db_path")]
    db_path: String,

    #[serde(default)]
    boots_peers: Vec<String>,

    #[serde(default = "validator_default_port")]
    local_port: u16,
}

fn default_db_path() -> String {
    DEFAULT_DB_PATH.to_string()
}

fn validator_default_port() -> u16 {
    4000
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    tce: Validator,
    peer_id: String,

    #[serde(default = "default_name")]
    name: String,
    #[deprecated]
    #[serde(default)]
    flag: bool,
}

fn default_name() -> String {
    "name default".to_string()
}

fn main() {
    let cli = Cli::parse();

    println!("{:#?}", cli);
    let mut figment = Figment::new()
        .merge(Toml::file("base.toml"))
        .merge(Yaml::file("config.yaml"))
        .merge(Env::prefixed("TOPOS_").split("__"))
        .merge(Serialized::defaults(Cli::parse()));

    figment = match cli.subcmd {
        SubCommand::Tce(values) => figment.merge(Serialized::defaults(values).key("tce")),
    };

    let config: Config = match figment.extract() {
        Ok(config) => config,
        Err(figment::Error {
            kind: Kind::MissingField(name),
            ..
        }) => {
            println!("Missing configuration value: {}", name);
            std::process::exit(1);
        }
        Err(figment::Error {
            kind: Kind::InvalidType(actual, expected),
            ..
        }) => {
            println!(
                "Missing configuration value type, expecting {}, found {}",
                expected, actual
            );
            std::process::exit(1);
        }

        Err(e) => panic!("{:#?}", e),
    };

    // let config = Cli::parse();
    println!("{config:#?}");

    let y = toml::to_string(&config).unwrap();

    // println!("{}", y);
}
