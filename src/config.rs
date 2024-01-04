use crate::core::{Doc, OpenOption};
use crate::error::DocError;
use crate::log::Log;
use crate::tpl::INIT_CONFIG;
use ansi_term::Colour;
use dyn_fmt::AsStrFormatExt;
use serde::{Deserialize, Serialize};

use std::fs;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{Error, ErrorKind, Read},
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub loc: Option<PathBuf>,
    pub map: BTreeMap<String, Doc>,
}

impl Config {
    pub fn new(loc: Option<String>) -> Result<Self, Error> {
        match loc {
            Some(loc) => {
                let path = Path::new(&loc).to_path_buf();

                if path.is_file() {
                    read(&path)
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        DocError::new(
                            &Log::Warn(
                                &"Can not find config file by {}"
                                    .format(&[path.display().to_string()]),
                            )
                            .to_string(),
                        ),
                    ))
                }
            }
            None => {
                let home = dirs::home_dir();

                if let Some(home) = home {
                    read(&home.join(".doc.toml"))
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        DocError::new(&Log::Err("Can not find home dir.").to_string()),
                    ))
                }
            }
        }
    }

    pub fn new_empty() -> Config {
        Config {
            loc: None,
            map: BTreeMap::new(),
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        let Config { loc, map } = self;
        let mut content = String::from(INIT_CONFIG);

        if !map.is_empty() {
            content = self.to_string()?
        }

        if let Some(loc) = loc {
            fs::write(loc, content)?
        }

        Ok(())
    }

    pub fn to_string(&self) -> Result<String, Error> {
        toml::to_string(self).map_err(|err| Error::new(ErrorKind::InvalidData, err))
    }

    pub fn open(&self, name: &str, option: &OpenOption) -> Result<(), Error> {
        match self.find(name) {
            Some(doc) => {
                doc.open(option)?;
                Ok(())
            }
            None => {
                Log::Warn(
                    &"Can not find any doc by '{}', similar docs found below:".format(&[name]),
                )
                .println();

                self.walk_config(|n, doc| {
                    if doc.contains(n, &name[0..1]) {
                        println!("{}", Colour::Green.paint(doc.get_printed_name(n)));
                    }
                });
                Ok(())
            }
        }
    }

    pub fn view(&self, name: &str, detail: bool) {
        if let Some(doc) = self.map.get(name) {
            doc.view(name, detail);
        }
    }

    // TODO remove it ??
    pub fn walk_config<T>(&self, mut call: T)
    where
        T: FnMut(&String, &Doc),
    {
        // BTree has already sorted
        for (n, doc) in &self.map {
            call(n, doc)
        }
    }

    pub fn find(&self, name: &str) -> Option<&Doc> {
        let mut doc_name = String::new();

        match self.map.get(name) {
            Some(_) => doc_name = name.to_string(),
            None => {
                self.walk_config(|n, doc| {
                    if let Doc {
                        full: Some(full), ..
                    } = doc
                    {
                        if full == name {
                            doc_name = n.to_string()
                        }
                    }
                });
            }
        }
        if doc_name.is_empty() {
            None
        } else {
            Some(self.map.get(&doc_name).unwrap())
        }
    }
}

impl From<(PathBuf, String)> for Config {
    fn from(value: (PathBuf, String)) -> Self {
        let (loc, str) = value;
        Config {
            loc: Some(loc),
            map: if let Ok(config) = toml::from_str(&str) {
                config
            } else {
                BTreeMap::new()
            },
        }
    }
}

pub fn parse_config(cont: &str) -> Result<Config, Error> {
    let parse_result = toml::from_str::<Config>(cont);

    match parse_result {
        Ok(value) => Ok(value),
        Err(err) => {
            Log::Err(&"Parse Error, error content is: {}".format(&[cont]));
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }
}

pub fn read(loc: &PathBuf) -> Result<Config, Error> {
    let mut cont = String::new();
    File::open(loc)?.read_to_string(&mut cont)?;

    parse_config(&cont)
}

pub fn save(loc: &PathBuf, content: &str) -> Result<(), Error> {
    fs::write(
        loc,
        if content.is_empty() {
            INIT_CONFIG
        } else {
            content
        },
    )?;

    Ok(())
}
