#[macro_use]
extern crate serde_derive;
//extern crate serde_json;

use std::fs;
use std::fs::File;
use std::process::exit;
use structopt::StructOpt;
use serde_json::{json,Value};

pub type Error = failure::Error;

#[derive(StructOpt, Debug)]
struct Args {
    /// Connection to the API
    #[structopt(
        short = "c",
        long = "api-connection")
    ]
    nlu_api_url: String,
    /// The path to the test files
    #[structopt(
        short = "i",
        long = "input",
        parse(from_os_str),
        default_value="./data")
    ]
    path_test_files: std::path::PathBuf,
}

#[derive(Debug,Deserialize)]
struct CsvRow {
    query: String,
    intention: String,
    info_poi: Option<String>,
}

fn get_params(query: &String) -> Value {
    json!({"text": &query,"language":"fr","domain":"poi","count":1})
}

fn parse_csv<I>(url: String, paths: I) -> Result<(), Error>
where
    I: Iterator<Item = std::path::PathBuf>,
{
    let mut predictions: Vec<String> = Vec::new();
    let mut effective_class: Vec<String> = Vec::new();

    println!("api url: {}", url);
    let client = reqwest::Client::new();
    for path in paths {
        println!("Loading test file: \'{}\'...", path.clone().into_os_string().into_string().unwrap());
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for rw in rdr.deserialize() {
            let test_row: CsvRow = rw?;
            let query = test_row.query;
            let real_intention = test_row.intention;
            let params = get_params(&query);
            let resp: Value = client.post(&url).json(&params).send()?.json()?;

            let predicted_intention: String = resp["intention"][0][1].to_string();
            println!(" --- query: \'{}\', real_intention: \'{}\', predicted_intention: \'{}\'", query, real_intention, predicted_intention);
            effective_class.push(real_intention);
            predictions.push(predicted_intention);
        }
        println!("finished!");
    }

    println!("predictions: {:#?}", predictions);
    println!("classes: {:#?}", effective_class);
    Ok(())
}

fn run(args: Args) -> Result<(), Error> {
        let url = args.nlu_api_url;

        if args.path_test_files.is_dir() {
            let paths: std::fs::ReadDir = fs::read_dir(&args.path_test_files)?;
            parse_csv(url, paths.map(|p| p.unwrap().path()))
        } else {
            parse_csv(url, std::iter::once(args.path_test_files))
        }
}

fn wrapped_run<O, F>(run: F) -> Result<(), Error>
where
    F: FnOnce(O) -> Result<(), Error>,
    O: StructOpt,
{
    if let Err(e) = run(O::from_args()) {
        for cause in e.iter_chain() {
            println!("{}", cause);
        }
        Err(e)
    } else {
        Ok(())
    }
}

fn launch_run<O, F>(run: F)
where
    F: FnOnce(O) -> Result<(), Error>,
    O: StructOpt,
{
    if wrapped_run(run).is_err() {
        exit(1);
    }
}

fn main() {
    launch_run(run);
}
