use std::collections::HashMap;
use std::hash::Hash;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::{fs, process, thread};

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
    pub case_insensitive: bool,

    #[clap(short, long)]
    pub raw_print: bool,

    #[clap(short, long, default_value_t = 100000)]
    pub limit: usize,
}

type StatisticBuffer = HashMap<String, u32>;

pub fn read(args: &RuntimeArgs) -> String {
    fs::read_to_string(&args.input).unwrap_or_else(|err| {
        eprintln!("error occurred while reading file: {}", err);
        process::exit(1);
    })
}

fn merge_noref<K: Eq + Hash, V>(map1: HashMap<K, V>, map2: HashMap<K, V>) -> HashMap<K, V> {
    map1.into_iter().chain(map2).collect()
}

pub fn statistics(content: &String, case_insensitive: bool, limit: usize) -> StatisticBuffer {
    let mut hdls: Vec<JoinHandle<()>> = Vec::new();
    let mut splited: Vec<String> = Vec::new();
    content.split_whitespace().for_each(|f| {
        let mut f = f.to_string();

        if !case_insensitive {
            f = f.to_lowercase();
        }
        splited.push(f.to_string());
    });

    let (tsm, rcv) = mpsc::channel();

    let mut times = splited.len() / limit;
    if times == 0 {
        times = 1;
    }

    for n in 0..times {
        let sliced = splited[n * limit..(n + 1) * limit].to_vec();

        hdls.push(thread::spawn(move || {
            let mut tmp = StatisticBuffer::new();
            sliced.iter().for_each(|f| {
                let cpt = Regex::new("[a-zA-Z]*")
                    .unwrap()
                    .captures(f)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str();
                tmp.insert(String::from(cpt), tmp.get(cpt).unwrap_or_else(|| &0) + 1);
            });
            tsm.send(tmp).unwrap();
        }));
    }

    let mut rst = StatisticBuffer::new();

    for hdl in hdls {
        hdl.join().unwrap();
        rst = merge_noref(rst, rcv.recv().unwrap());
    }

    rst
}

pub fn filter(db: &StatisticBuffer, exp: Regex) -> StatisticBuffer {
    let mut dbc = db.clone();
    dbc.retain(|f, _| exp.is_match(&f));
    dbc
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::{filter, StatisticBuffer};

    #[test]
    fn test_filter() {
        let mut m1: StatisticBuffer = HashMap::new();
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

/*

let b = true;
let mut f = "AWA";

if b {
    f = &f.to_lowercase();
}

println!("{}", f);

 */
