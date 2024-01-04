use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

use crate::{
    config::{self, parse_config, Config},
    log::Log,
};
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
    #[command(
        long_about = "Init config, you can use -f to update config forcely, -u to get remote config"
    )]
    Init {
        #[arg(short, long, default_value_t = false)]
        force: bool,
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long, default_value_t = false)]
        merge: bool,
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

            config.walk_config(|n, doc| {
                if let Some(f) = filter {
                    if doc.contains(n, f) {
                        config.view(n, *detail)
                    }
                } else {
                    config.view(n, *detail)
                }
            })
        }
        Some(Commands::Init { force, url, merge }) => {
            let home = dirs::home_dir();
            let is_force = *force;
            let should_merge = *merge;
            let mut write_config = String::new();
            let mut remote_config = String::new();

            if let Some(home) = home {
                let config_path = home.join(".doc.toml");
                if let Some(url) = url {
                    remote_config = get_remote_config(url)?;
                }

                if config_path.exists() {
                    File::open(&config_path)?.read_to_string(&mut write_config)?;

                    let mut user_config = if write_config.is_empty() {
                        Config::new_empty()
                    } else {
                        parse_config(&write_config)?
                    };

                    if should_merge {
                        let rc_config = parse_config(&remote_config)?;
                        rc_config.map.keys().for_each(|s| {
                            if let Some(doc) = rc_config.map.get(s) {
                                user_config.map.insert(s.to_string(), doc.clone());
                            }
                        });

                        write_config = user_config.to_string()?;
                    }

                    if is_force && !remote_config.is_empty() {
                        config::save(&config_path, &remote_config)?;
                        return Ok(());
                    }

                    config::save(&config_path, &write_config)?;
                } else {
                    write_config = String::from(if remote_config.is_empty() {
                        INIT_CONFIG
                    } else {
                        &remote_config
                    });
                    config::save(&config_path, &write_config)?;
                }

                Log::Suc(&"Success init config in {}".format([&config_path.display()])).println();
            } else {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    Log::Err("Can not find home dir.").to_string(),
                ));
            }
        }
        None => {
            let config = Config::new(cli.dir)?;
            if cli.name.is_none() {
                Log::Warn("Please select one of the names below to see, or just use '-s' to search by google \neg: doc xx -s 👇:").println();
                config.walk_config(|n, doc| {
                    print!("{} ", Colour::Green.paint(doc.get_printed_name(n)))
                });
                println!("\n");
            }
        }
    }

    Ok(())
}
