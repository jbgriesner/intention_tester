use crate::{json, Error, StructOpt, Value};
use std::process::exit;

#[derive(Debug, Deserialize)]
pub struct CsvRow {
    pub query: String,
    pub intention: String,
}

pub fn get_params(query: &String) -> Value {
    json!({"text": &query,"language":"fr","domain":"poi","count":1})
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

pub fn launch_run<O, F>(run: F)
where
    F: FnOnce(O) -> Result<(), Error>,
    O: StructOpt,
{
    if wrapped_run(run).is_err() {
        exit(1);
    }
}
