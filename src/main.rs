#[macro_use]
extern crate prettytable;

use std::time::{Duration, Instant};

use clap::Parser;
use prettytable::{format, row, Table};
use regex::Regex;

use rfreq::RuntimeArgs;

fn main() {
    let args: RuntimeArgs = RuntimeArgs::parse();

    let start = Instant::now();

    let content = rfreq::read(&args);

    let time_cost_read = start.elapsed();
    let start = Instant::now();

    let stas = rfreq::statistics(content.as_str(), args.case_sensitive);

    let time_cost_statstics = start.elapsed();
    let start = Instant::now();

    let mut exp = args.filter;
    if !args.case_sensitive {
        exp = exp.to_lowercase();
    }
    let filter = rfreq::filter(&stas, Regex::new(&exp).expect("illegal regular expression"));

    let time_cost_analyse = start.elapsed();

    let mut time_cost_sort = Duration::from_millis(0);

    let result_non_sort = &filter;
    let mut result_sort = vec![];
    let mut sorted = false;

    if !args.no_sorting {
        let start = Instant::now();
        result_sort = rfreq::sort(&filter);
        time_cost_sort = start.elapsed();
        sorted = true;
    }

    if stas.is_empty() {
        println!("nothing found");
        return;
    }

    if args.raw_print {
        println!("word\tcount");
        if sorted {
            for i in result_sort {
                println!("{}\t{}", i.0, i.1);
            }
        } else {
            result_non_sort.iter().for_each(|(word, times)| {
                println!("{}\t{}", word, times);
            });
        }
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.add_row(row![b -> "word", b -> "count"]);
        if sorted {
            for i in result_sort {
                table.add_row(row![i.0, i.1]);
            }
        } else {
            result_non_sort.iter().for_each(|(word, times)| {
                table.add_row(row![word, times]);
            });
        }

        table.printstd();
    }

    println!(
        "\ntime costs:\n - read: {}ms\n - statistics: {}ms\n - filter: {}ms\n - sort: {}ms",
        time_cost_read.as_millis(),
        time_cost_statstics.as_millis(),
        time_cost_analyse.as_millis(),
        time_cost_sort.as_millis(),
    );
}
