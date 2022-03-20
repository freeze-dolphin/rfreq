use clap::Parser;
use prettytable::{format, row, Table};
use regex::Regex;
use rfreq::RuntimeArgs;
use std::time::Instant;

#[macro_use]
extern crate prettytable;

fn main() {
    let args: RuntimeArgs = RuntimeArgs::parse();

    let start = Instant::now();

    let content = rfreq::read(&args);

    let time_cost_read = start.elapsed();
    let start = Instant::now();

    let stas = rfreq::statistics(&content, args.case_insensitive, args.limit);

    let time_cost_statstics = start.elapsed();
    let start = Instant::now();

    let mut exp = args.filter;
    if !args.case_insensitive {
        exp = exp.to_lowercase();
    }

    let filter = rfreq::filter(&stas, Regex::new(&exp).expect("illegal regular expression"));

    let time_cost_analyse = start.elapsed();

    if stas.is_empty() {
        println!("nothing found");
        return;
    }

    if args.raw_print {
        println!("word\tcount");
        filter.iter().for_each(|(word, times)| {
            println!("{}\t{}", word, times);
        });
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.add_row(row![b -> "word", b -> "count"]);
        filter.iter().for_each(|(word, times)| {
            table.add_row(row![word, times]);
        });

        table.printstd();
    }

    println!(
        "\ntime costs:\n - read: {}ms\n - statistics: {}ms\n - filter: {}ms",
        time_cost_read.as_millis(),
        time_cost_statstics.as_millis(),
        time_cost_analyse.as_millis()
    );
}
