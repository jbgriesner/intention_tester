#[macro_use]
extern crate serde_derive;

pub use self::utils::launch_run;
use rusty_machine::analysis::score::accuracy;
use serde_json::{json, Value};
use std::fs::File;
pub use structopt::StructOpt;

pub type Error = failure::Error;

mod csvrow;
mod utils;

use csvrow::CsvRow;

pub fn compute_scores(predictions: Vec<String>, effective_class: Vec<String>) -> () {
    let acc = accuracy(predictions.iter(), effective_class.iter());
    println!("Final accuracy: {}", acc);
}

fn parse_row(
    url: &String,
    rw: Result<CsvRow, csv::Error>,
    client: &reqwest::Client,
    predictions: &mut Vec<String>,
    effective_class: &mut Vec<String>,
) -> Result<(), Error> {
    let test_row = rw?;
    let query = test_row.query;
    let real_intention = test_row.intention;
    let params = utils::get_params(&query);
    let resp: Value = client.post(url).json(&params).send()?.json()?;
    let predicted_intention: String = resp["intention"][0][1]
        .to_string()
        .trim_matches('\"')
        .to_string();
    println!(
        " --- query: \'{}\', real_intention: \'{}\', predicted_intention: \'{}\'",
        query, real_intention, predicted_intention
    );
    effective_class.push(real_intention);
    predictions.push(predicted_intention);
    Ok(())
}

pub fn parse_csv<I>(url: String, paths: I) -> Result<(Vec<String>, Vec<String>), Error>
where
    I: Iterator<Item = std::path::PathBuf>,
{
    let mut predictions: Vec<String> = Vec::new();
    let mut effective_class: Vec<String> = Vec::new();

    let client = reqwest::Client::new();

    for path in paths {
        println!(
            "Loading test file: \'{}\'...",
            path.clone().into_os_string().into_string().unwrap()
        );
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => continue,
        };
        let mut rdr = csv::Reader::from_reader(file);

        for rw in rdr.deserialize::<CsvRow>() {
            match parse_row(&url, rw, &client, &mut predictions, &mut effective_class) {
                Ok(_) => (),
                Err(_) => continue,
            }
        }
    }
    Ok((predictions, effective_class))
}
