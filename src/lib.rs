#[macro_use]
extern crate serde_derive;

pub use self::utils::launch_run;
use serde_json::{json, Value};
use std::fs::File;
pub use structopt::StructOpt;

pub type Error = failure::Error;

pub mod utils;

pub fn parse_csv<I>(url: String, paths: I) -> Result<(), Error>
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
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);
        for rw in rdr.deserialize() {
            let test_row: utils::CsvRow = rw?;
            let query = test_row.query;
            let real_intention = test_row.intention;
            let params = utils::get_params(&query);
            let resp: Value = client.post(&url).json(&params).send()?.json()?;
            let predicted_intention: String = resp["intention"][0][1].to_string();
            println!(
                " --- query: \'{}\', real_intention: \'{}\', predicted_intention: \'{}\'",
                query, real_intention, predicted_intention
            );
            effective_class.push(real_intention);
            predictions.push(predicted_intention);
        }
    }

    println!("predictions: {:#?}", predictions);
    println!("classes: {:#?}", effective_class);
    Ok(())
}
