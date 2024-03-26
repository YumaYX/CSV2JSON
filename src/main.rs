use clap::{App, Arg};
use rayon::prelude::*;
use serde_json::{self, Map, Value};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Config {
    filename: String,
    allow_null: bool,
    key_arg: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn get_args() -> MyResult<Config> {
    let matches = App::new("csv2json")
        .version("0.1.0")
        .author("Yuma")
        .about("Convert CSV into JSON")
        .arg(
            Arg::with_name("csv_file")
                .value_name("CSVFILE")
                .help("CSV File Name")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("null_arg")
                .short("n")
                .value_name("NULL")
                .help("\"\" is null?")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("key_arg")
                .short("k")
                .value_name("KEY HASH")
                .help("Generate JSON with Key and Value")
                .takes_value(false),
        )
        .get_matches();
    Ok(Config {
        filename: matches.value_of("csv_file").unwrap().to_string(),
        allow_null: matches.is_present("null_arg"),
        key_arg: matches.is_present("key_arg"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn Read>> {
    match filename {
        "-" => Ok(Box::new(io::stdin())),
        _ => Ok(Box::new(File::open(filename)?)),
    }
}

fn json_parse(element: &str, allow_null: bool) -> Value {
    if element == "true" || element == "false" {
        Value::Bool(element.parse().unwrap())
    } else if let Ok(parsed_number) = element.parse::<serde_json::Number>() {
        Value::Number(parsed_number)
    } else if allow_null && element.is_empty() {
        Value::Null
    } else {
        Value::String(element.into())
    }
}

fn run(config: Config) -> MyResult<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(open(&config.filename)?);

    let headers = rdr.headers()?.clone();

    if config.key_arg {
        let json_records = Arc::new(Mutex::new(Map::new()));
        rdr.records()
            .map(Result::unwrap)
            .collect::<Vec<_>>()
            .par_iter()
            .for_each(|record| {
                let mut json_map = Map::new();

                if let Some(key) = record.get(0) {
                    for (header, value) in headers.iter().zip(record.iter()) {
                        let result = json_parse(value, config.allow_null);
                        json_map.insert(header.to_string(), result);
                    }
                    let mut json_records = json_records.lock().unwrap();
                    json_records.insert(key.to_string(), Value::Object(json_map));
                }
            });
        let json_output = serde_json::to_string_pretty(&*json_records.lock().unwrap())?;
        println!("{}", json_output);
    } else {
        let json_output = Arc::new(Mutex::new(Vec::<Value>::new()));

        rdr.records()
            .map(Result::unwrap)
            .collect::<Vec<_>>()
            .par_iter()
            .for_each(|record| {
                let mut json_map = Map::new();
                if record.get(0).is_some() {
                    for (header, value) in headers.iter().zip(record.iter()) {
                        let result = json_parse(value, config.allow_null);
                        json_map.insert(header.to_string(), result);
                    }
                    let mut json_output = json_output.lock().unwrap();
                    json_output.push(Value::Object(json_map));
                }
            });
        let json_output = serde_json::to_string_pretty(&*json_output.lock().unwrap())?;
        println!("{}", json_output);
    }

    Ok(())
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
