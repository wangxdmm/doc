use ansi_term::Colour;

pub enum Log<'a> {
    Err(&'a str),
    Warn(&'a str),
    Suc(&'a str),
    Info(&'a str),
}

impl<'a> Log<'a> {
    pub fn to_string(self) -> String {
        match self {
            Self::Err(str) => Colour::Red.paint(String::from("❌") + str).to_string(),
            Self::Warn(str) => Colour::Yellow.paint(String::from("❗") + str).to_string(),
            Self::Suc(str) => Colour::Green.paint(String::from("✅") + str).to_string(),
            Self::Info(str) => Colour::White.paint(String::from("📢") + str).to_string(),
        }
    }

    pub fn println(self) {
        if let Self::Info(str) = self {
            println!("{}", str)
        } else {
            println!("{}", self.to_string())
        }
    }
}
