use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::{fs, process};

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct RuntimeArgs {
    #[clap(short, long, help = "Regex filter")]
    pub filter: String,

    #[clap(short, long, help = "File to perform analysis on")]
    pub input: String,

    #[clap(short, long, help = "Consider different cases as different words")]
    pub case_sensitive: bool,

    #[clap(short, long, help = "Print result without using table form")]
    pub raw_print: bool,

    #[clap(short, long, help = "Disable sorting")]
    pub no_sorting: bool,
}

type StatisticBuffer = HashMap<String, u32>;

pub fn read(args: &RuntimeArgs) -> String {
    fs::read_to_string(&args.input).unwrap_or_else(|err| {
        eprintln!("error occurred while reading file: {}", err);
        process::exit(1);
    })
}

pub fn statistics(content: &str, case_sensitive: bool) -> StatisticBuffer {
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

        if !case_sensitive {
            f = f.to_lowercase();
        }
        db.insert(f, db.get(s).unwrap_or(&0) + 1);
    });
    db
}

pub fn sort(db: &StatisticBuffer) -> Vec<(&String, &u32)> {
    let mut sort: Vec<_> = db.iter().collect();
    sort.sort_by(|a, b| b.1.cmp(a.1));

    sort
}

pub fn filter(db: &StatisticBuffer, exp: Regex) -> StatisticBuffer {
    let mut dbc = db.clone();
    dbc.retain(|f, _| exp.is_match(f));
    dbc
}

#[cfg(test)]
mod tests {
    use crate::{filter, sort, StatisticBuffer};
    use regex::Regex;

    #[test]
    fn test_filter() {
        let mut m1 = StatisticBuffer::new();
        m1.insert("test".to_string(), 3);
        m1.insert("hello".to_string(), 2);

        let mut m2 = m1.clone();
        m2.remove("hello");

        assert_eq!(
            filter(&m1, Regex::new(".*es.*").expect("unable to create regex")),
            m2
        );
    }

    #[test]
    fn test_sort() {
        let mut m = StatisticBuffer::new();
        m.insert("test".to_string(), 3);
        m.insert("hello".to_string(), 1);
        m.insert("good".to_string(), 10);
        let s = sort(&m);
        assert_eq!(
            format!("{:?}", s),
            "[(\"good\", 10), (\"test\", 3), (\"hello\", 1)]"
        );
    }
}
