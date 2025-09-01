use anyhow::Result;
use clap::{arg, value_parser, Command};
use csv::Reader;
use std::{fs, path::Path};

fn main() -> Result<()> {
    let matches = clap::Command::new("rcli")
        .version("0.1.0")
        .author("songever")
        .about("A Rust CLI tool for various tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(subcommand_csv())
        .get_matches();

    match matches.subcommand() {
        Some(("csv", sub_m)) => {
            let input = sub_m.get_one::<String>("input").expect("required");
            let output = sub_m
                .get_one::<String>("output")
                .expect("default ensures this");
            let delimiter = *sub_m
                .get_one::<char>("delimiter")
                .expect("default ensures this");
            let has_header = *sub_m
                .get_one::<bool>("header")
                .expect("default ensures this");

            println!("Input: {}", input);
            println!("Output: {}", output);
            println!("Delimiter: {}", delimiter);
            println!("Has Header: {}", has_header);
            process_csv(input, output)?;
        }
        _ => println!("No valid subcommand was used"),
    }

    Ok(())
}

fn subcommand_csv() -> Command {
    Command::new("csv")
        .about("Show CSV or convert to other formats")
        .arg(arg!(-i --input <INPUT> "Input CSV file").value_parser(verify_input_file))
        .arg(arg!(-o --output [OUTPUT] "Outpit file name").default_value("output.json"))
        .arg(
            arg!(delimiter: -d --delimit [delimit] "Delimiter, default is comma")
                .value_parser(value_parser!(char))
                .default_value(","),
        )
        .arg(
            arg!(--header "Indicates if the CSV has a header row")
                .value_parser(value_parser!(bool))
                .default_value("true"),
        )
        .arg_required_else_help(true)
}
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file does not exist")
    }
}

#[cfg(feature = "normal")]
fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;

        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }
    let content = serde_json::to_string_pretty(&ret)?;
    fs::write(output, content)?;
    Ok(())
}

#[cfg(feature = "experimental")]
fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let ret = reader
        .records()
        .map(|record| {
            record.map(|rec| {
                headers
                    .iter()
                    .zip(rec.iter())
                    .collect::<serde_json::Value>()
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let content = serde_json::to_string_pretty(&ret)?;
    fs::write(output, content)?;
    Ok(())
}
