#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate toml;

mod config;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // let configuration = config::print_config("../SimpleNexus/database_sanitizer/simplenexus.toml");
    config::print_config();
}

fn process() {
    let mut line = String::with_capacity(67108864);
    let mut count = 0;

    let f = File::open("../SimpleNexus/database_sanitizer/temp.sql").unwrap();
    let mut reader = BufReader::new(f);

    let sql_line_regex_set = regex::RegexSet::new(&[
        // SQL_COMMENT_REGEX: /^[ \t]*(--.*|\/\*.*\*\/;)$/im;
        r"(?smi)^[ \t]*(--.*|/\*.*\*/;)$",
        // SQL_EMPTY_LINE_REGEX: /^\s*$/im;
        r"(?si)^\s*$",
        // SQL_COMPLETE_STATEMENT_REGEX: /^.*;[ \t]*$/im
        r"(?smi)^.*;[ \t]*$"
    ]).unwrap();

    loop {
        let length = reader.read_line(&mut line).unwrap();
        count += 1;

        if count == 500 || length == 0 {
            break;
        }

        let matches = sql_line_regex_set.matches(&line);

        match matches.into_iter().nth(0) {
            Some(0) => {
                println!("{}: Comment", count);
                line.clear();
            },
            Some(1) => {
                println!("{}: Empty line", count);
                line.clear();
            },
            Some(2) => {
                process_statement(&line);
                line.clear();
            },
            Some(_) => {
                println!("{}: Error", count);
                break;
            },
            None => {
                println!("{}: Error", count);
                break;
            }
        }
    }
}

fn process_statement(line: &String) {
    let sql_statement_regex_set = regex::RegexSet::new(&[
        // SQL_CREATE_DATEBASE_REGEX: /^create\s+database.*;$/im;
        r"(?im)^create\s+database.*;$",
        // SQL_CREATE_TABLE_REGEX: /^create\s+table\s+`(\w+)`/im;
        r"(?im)^create\s+table\s+`(\w+)`",
        // SQL_INSERT_REGEX: /^insert\s+into\s+`(\w+)`\s+values\s+(.*);$/im;
        r"(?im)^insert\s+into\s+`(\w+)`\s+values\s+(.*);$",
        // SQL_USE_STATEMENT_REGEX: /^use\s+`(\w+)`;\s*$/im;
        r"(?im)^use\s+`(\w+)`;\s*$"
    ]).unwrap();

    let matches = sql_statement_regex_set.matches(line);

    match matches.into_iter().nth(0) {
        Some(0) => println!("Create database statement"),
        Some(1) => println!("Create table statement"),
        Some(2) => println!("Insert statement"),
        Some(3) => println!("Use database statement"),
        Some(_) => {
            panic!("Error: Unknown statement");
        },
        None => {
            panic!("Error: No matching statements");
        }
    }
}

    // SQL_TABLE_COLUMNS_REGEX: /(^create\s+table\s+`\w+`\s+\(\s+|,\s+)`(\w+)`/im;
    // SQL_VALUE_SETS_REGEX: /(?:values)?\s*\((.*?)\)(?:,|;\s*$)/im;
    // SQL_VALUES_REGEX: /('.*?'|[^']+?)(?:,|$)/im;
