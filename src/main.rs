extern crate rusty_machine as rm;
extern crate reqwest;
extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs;
use std::fs::File;
use std::process::exit;
pub type Error = failure::Error;

use structopt::StructOpt;
use serde_json::{Value};

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
struct TestRow {
    query: String,
    intention: String,
    info_poi: Option<String>,
}


#[derive(Deserialize)]
struct NLUresponse {
    _query: String,
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
            let test_row: TestRow = rw?;
//            let test_row: TestRow = rw.unwrap();
//            let test_row: TestRow = row?;
//            println!("--- {:?}", test_row);
            let query = test_row.query;
            let intention = test_row.intention;

            println!("query: {}, intention: {}", query, intention);
            let params = [("q", query)];
            let resp = client.post(&url).query(&params).send()?.text()?;
            let v: Value = serde_json::from_str(&resp)?;
            println!("{:#?}", v);
            println!("{:#?}", v["features"]);

            let predicted = v["class"].to_string();

            effective_class.push(intention);
            predictions.push(predicted);
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
//http://smallcultfollowing.com/babysteps/blog/2016/04/27/non-lexical-lifetimes-introduction/
