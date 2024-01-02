use crate::core::{Doc, OpenOption};
use crate::error::Error as DocError;
use ansi_term::Colour;
use serde::Deserialize;

use std::{
    collections::HashMap,
    fs::File,
    io::{Error, ErrorKind, Read},
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub loc: Option<PathBuf>,
    pub map: HashMap<String, Doc>,
}

impl Config {
    pub fn new(loc: Option<String>) -> Result<Self, Error> {
        match loc {
            Some(loc) => {
                let path = Path::new(&loc).to_path_buf();

                if path.is_file() {
                    read(path)
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        DocError::new(&format!(
                            "❗ Can not find config file by {}",
                            path.display()
                        )),
                    ))
                }
            }
            None => {
                let home = dirs::home_dir();

                if let Some(home) = home {
                    read(home.join(".doc.toml"))
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        DocError::new("❗ Can not find home dir."),
                    ))
                }
            }
        }
    }

    pub fn open(&self, name: &str, option: &OpenOption) -> Result<(), Error> {
        match self.find(name) {
            Some(doc) => {
                doc.open(option)?;
                Ok(())
            }
            None => {
                println!(
                    "❗Can not find any doc by '{}', similar docs find below:",
                    name
                );

                self.walk(|n, doc| {
                    if let Some(doc) = doc {
                        if doc.contains(n, &name[0..1]) {
                            println!("{}", Colour::Yellow.paint(doc.get_printed_name(n)));
                        }
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

    pub fn walk<T>(&self, mut call: T)
    where
        T: FnMut(&String, Option<&Doc>),
    {
        let mut ks: Vec<_> = self.map.keys().collect();

        ks.sort();

        for n in ks {
            call(n, self.map.get(n))
        }
    }

    pub fn find(&self, name: &str) -> Option<&Doc> {
        let mut doc_name = String::new();

        match self.map.get(name) {
            Some(_) => doc_name = name.to_string(),
            None => {
                self.walk(|n, doc| {
                    if let Some(Doc {
                        full: Some(full), ..
                    }) = doc
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

pub fn read(loc: PathBuf) -> Result<Config, Error> {
    let mut cont = String::new();
    let mut config = Config {
        loc: Some(loc.clone()),
        map: HashMap::new(),
    };
    File::open(&loc)?.read_to_string(&mut cont)?;

    let parse_result = toml::from_str::<Config>(&cont);

    match parse_result {
        Ok(value) => {
            config.map = value.map;
            Ok(config)
        }
        Err(_) => Err(Error::new(
            ErrorKind::InvalidData,
            DocError::new(&format!(
                "❌It seems that some errors occur in your toml config, in {}",
                loc.display()
            )),
        )),
    }
}
