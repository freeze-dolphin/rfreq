use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::{fs, process};

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct RuntimeArgs {
    /*
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
     */
    #[clap(short, long)]
    pub filter: String,

    #[clap(short, long)]
    pub input: String,

    #[clap(short, long)]
    pub case_insensitive: bool,

    #[clap(short, long)]
    pub raw_print: bool,
}

type StatisticBuffer = HashMap<String, u32>;

pub fn read(args: &RuntimeArgs) -> String {
    fs::read_to_string(&args.input).unwrap_or_else(|err| {
        eprintln!("error occurred while reading file: {}", err);
        process::exit(1);
    })
}

pub fn statistics(content: &str, case_insensitive: bool) -> StatisticBuffer {
    let mut db: StatisticBuffer = StatisticBuffer::new();
    content.split_whitespace().for_each(|f| {
        let s = Regex::new("[a-zA-Z]*")
            .unwrap()
            .captures(f)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();

        let mut f = s.to_string();

        if !case_insensitive {
            f = f.to_lowercase();
        }
        db.insert(f, db.get(s).unwrap_or(&0) + 1);
    });
    db
}

pub fn filter(db: &StatisticBuffer, exp: Regex) -> StatisticBuffer {
    let mut dbc = db.clone();
    dbc.retain(|f, _| exp.is_match(f));
    dbc
}

#[cfg(test)]
mod tests {
    use crate::{filter, StatisticBuffer};
    use regex::Regex;

    #[test]
    fn test_filter() {
        let mut m1: StatisticBuffer = StatisticBuffer::new();
        m1.insert("test".to_string(), 3);
        m1.insert("hello".to_string(), 2);

        let mut m2: StatisticBuffer = m1.clone();
        m2.remove("hello");

        assert_eq!(
            filter(&m1, Regex::new(".*es.*").expect("unable to create regex")),
            m2
        );
    }
}
