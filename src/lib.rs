use std::collections::HashMap;
use std::{fs, process};
use clap::Parser;
use regex::Regex;

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
    pub case_sensitive: bool,

    #[clap(short, long)]
    pub raw_print: bool,

}

pub fn read(args: &RuntimeArgs) -> String {
    fs::read_to_string(&args.input).unwrap_or_else(|err| {
        eprintln!("error occurred while reading file: {}", err);
        process::exit(1);
    })
}

pub fn statistics(content: &String, case_sensitive: &bool) -> HashMap<String, u32> {
    let mut db: HashMap<String, u32> = HashMap::new();
    content.split_whitespace().for_each(|f| {
        let f = Regex::new("[a-zA-Z]*").expect("unknown error").captures(f).unwrap().get(0).unwrap().as_str();

        if *case_sensitive {
            db.insert(String::from(f), db.get(f).unwrap_or_else(|| &0) + 1);
        } else {
            let f = &f.to_lowercase();
            db.insert(String::from(f), db.get(f).unwrap_or_else(|| &0) + 1);
        }
    });
    db
}

pub fn filter(db: &HashMap<String, u32>, exp: Regex) -> HashMap<String, u32> {
    let mut dbc = db.clone();
    dbc.retain(|f, _| {
        exp.is_match(&f)
    });
    dbc
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use regex::Regex;
    use crate::filter;

    #[test]
    fn test_filter() {
        let mut m1: HashMap<String, u32> = HashMap::new();
        m1.insert("test".to_string(), 3);
        m1.insert("hello".to_string(), 2);

        let mut m2: HashMap<String, u32> = m1.clone();
        m2.remove("hello");

        assert_eq!(filter(&m1, Regex::new(".*es.*").expect("unable to create regex")), m2);
    }
}
