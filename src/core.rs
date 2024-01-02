use ansi_term::Colour;
use dyn_fmt::AsStrFormatExt;
use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Doc {
    pub full: Option<String>,
    pub url: String,
    pub rep: Option<String>,
    pub play: Option<String>,
    pub search: Option<String>,
}

pub struct OpenOption {
    pub rep: bool,
    pub play: bool,
}

impl Doc {
    pub fn open(&self, option: &OpenOption) -> Result<(), Error> {
        if option.rep && self.rep.is_some() {
            webbrowser::open(self.rep.as_ref().unwrap())?;
            return Ok(());
        }

        if option.play && self.play.is_some() {
            webbrowser::open(self.play.as_ref().unwrap())?;
            return Ok(());
        }

        webbrowser::open(&self.url)?;

        Ok(())
    }

    pub fn get_printed_name(&self, name: &str) -> String {
        match &self.full {
            Some(full) => "{}({})".format(&[name, full]),
            None => "{}".format(&[name]),
        }
    }

    pub fn contains(&self, name: &str, str: &str) -> bool {
        self.get_printed_name(name).contains(str)
    }

    pub fn view(&self, name: &str, detail: bool) {
        let Doc {
            full: _,
            url,
            rep,
            play,
            search,
        } = self;

        let s: String = if detail {
            "🔎 {} url: {} rep: {} play: {} search: {}".format(&[
                &Colour::Yellow
                    .paint(self.get_printed_name(name))
                    .to_string(),
                url,
                &rep.to_owned().unwrap_or("--".to_string()),
                &play.to_owned().unwrap_or("--".to_string()),
                &search.to_owned().unwrap_or("--".to_string()),
            ])
        } else {
            "🔎 {}: {}".format([
                &Colour::Yellow
                    .paint(self.get_printed_name(name))
                    .to_string(),
                &url.clone(),
            ])
        };

        println!("{}", s)
    }
}
