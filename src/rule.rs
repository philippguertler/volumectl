extern crate regex;
extern crate simple_error;
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::error::Error;
use simple_error::*;
use lazy_static::*;
use self::regex::Captures;

pub struct Rule {
    pub property: String,
    pub pattern: Regex
}

impl FromStr for Rule {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RULE_REGEX: Regex = Regex::new(r"(.*)\s*=\s*(.*)").unwrap();
        }
        let captures: Captures = RULE_REGEX.captures(s)
            .ok_or(format!("Could not parse Rule {}", s))?;

        Ok(Rule{
            property: captures.get(1).unwrap().as_str().into(),
            pattern: Regex::new(captures.get(2).unwrap().as_str())?
        })
    }
}


pub fn read_rules() -> Result<Vec<Rule>, Box<Error>> {
    let mut path = require_with!(dirs::config_dir(), "Could not find config dir");

    path.push("volumectl.conf");

    if !path.exists() {
        bail!("Please create a volumectl.conf in your config folder ({})", path.to_str().unwrap())
    }

    let file = File::open(path).unwrap();

    BufReader::new(file).lines()
        .map(|line| line.unwrap().as_str().parse())
        .collect()
}