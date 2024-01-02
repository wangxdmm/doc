use std::{
    fs,
    io::{Error, ErrorKind},
};

use crate::config::Config;
use crate::{core::*, tpl::INIT_CONFIG};
use ansi_term::Colour;
use clap::{Parser, Subcommand};
use dirs;
use dyn_fmt::AsStrFormatExt;
use isahc::prelude::*;

#[derive(Parser)]
#[command(
    author = "wangxd@lovingskymm@foxmail.com",
    version,
    about,
    long_about = "'doc' is a command line tool which can open document url or repository easily eg: doc rust"
)]
pub struct Cli {
    #[arg(help = "Doc name you want to use")]
    pub name: Option<String>,
    #[arg(short, long, default_value_t = false, help = "Open Repository")]
    pub rep: bool,
    #[arg(short, long, default_value_t = false, help = "Open playground")]
    pub play: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Search docs in config file"
    )]
    pub search: bool,
    #[arg(
        help = "Search content by 'config.search' which will be replace by google if it is empty"
    )]
    content: Option<String>,

    #[arg(short, long)]
    pub dir: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(long_about = "See all config", alias = "ls")]
    List {
        #[arg(short, long)]
        filter: Option<String>,
        #[arg(short, long, default_value_t = false)]
        detail: bool,
    },
    #[command(long_about = "Init config, you can use -f to update config forcely, -u to get remote config")]
    Init {
        #[arg(short, long, default_value_t = false)]
        force: bool,
        #[arg(short, long)]
        url: Option<String>,
    },
}

pub fn get_remote_config(url: &str) -> Result<String, Error> {
    let mut response = isahc::get(url)?;
    response.text()
}

pub fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    if let Some(name) = cli.name {
        let config = Config::new(cli.dir)?;
        if cli.search {
            let mut st = "https://www.google.com/search?q={}";
            let mut sc = "{} ".format(&[&name]);

            if let Some(doc) = config.map.get(&name) {
                if let Some(s) = &doc.search {
                    st = &s;
                    sc.clear();
                }

                if let Some(c) = cli.content {
                    sc.push_str(&c);
                }
            };
            webbrowser::open(&st.format(&[sc.trim()]))?;
            return Ok(());
        }

        config.open(
            &name,
            &OpenOption {
                rep: cli.rep,
                play: cli.play,
            },
        )?;
        return Ok(());
    }

    match &cli.command {
        Some(Commands::List { filter, detail }) => {
            let config = Config::new(cli.dir)?;
            println!("{}", Colour::Yellow.paint("docs:"));

            config.walk(|n, doc| {
                if let Some(f) = filter {
                    if let Some(doc) = doc {
                        if doc.contains(n, f) {
                            config.view(n, *detail)
                        }
                    }
                } else {
                    config.view(n, *detail)
                }
            })
        }
        Some(Commands::Init { force, url }) => {
            let home = dirs::home_dir();
            let is_force = *force;
            let mut config = String::from(INIT_CONFIG);

            if let Some(home) = home {
                let config_path = home.join(".doc.toml");
                if let Some(url) = url {
                    config = get_remote_config(url)?
                }
                if config_path.exists() {
                    if !is_force {
                        println!(
                            "❗Config file already exist, If you want to overwrite it, add '-f'"
                        );
                        return Ok(());
                    } else {
                        fs::write(&config_path, config)?
                    }
                } else {
                    fs::write(&config_path, config)?
                }

                println!("Success init config in {}", &config_path.display())
            } else {
                return Err(Error::new(ErrorKind::NotFound, "❗Can not find home dir."));
            }
        }
        None => {
            let config = Config::new(cli.dir)?;
            if cli.name.is_none() {
                println!("❓Please select one of the names below to see, or just use '-s' to search by google \neg: doc xx -s 👇:");
                config.walk(|n, doc| {
                    print!(
                        "{} ",
                        if doc.is_some() {
                            Colour::Green
                                .paint(doc.unwrap().get_printed_name(n))
                                .to_string()
                        } else {
                            String::new()
                        }
                    )
                });
                println!("\n");
            }
        }
    }

    Ok(())
}
