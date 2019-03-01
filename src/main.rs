extern crate csv;
extern crate hyper;
#[macro_use]
extern crate serde_derive;

use std::fs;
use std::fs::File;
use std::process::exit;
pub type Error = failure::Error;

use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};

use structopt::StructOpt;
use log::{info, warn, error};


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

fn process_file(path: std::path::PathBuf, url: hyper::Uri) -> impl Future<Item=(), Error=()> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for row in rdr.deserialize() {
        let test_row: TestRow = row.unwrap();
    //            let test_row: TestRow = row?;
        println!("--- {:?}", test_row);
    }
}

fn parse_csv<I>(url: hyper::Uri, paths: I) -> Result<(), Error>
where
    I: Iterator<Item = std::path::PathBuf>,
{

    for path in paths {
        println!("Loading test file: \'{}\'...", path.clone().into_os_string().into_string().unwrap());
        rt::run(process_file(path, url));
        println!("finished!");
    }

    rt::run(rt::lazy(|| {
        let client = Client::new();

        let uri = "http://httpbin.org/ip".parse().unwrap();


        client
            .get(uri)
            .and_then(|res| {
                println!("Response: {}", res.status());
                res
                    .into_body()
                    .for_each(|chunk| {
                        io::stdout()
                            .write_all(&chunk)
                            .map_err(|e| {
                                panic!("example expects stdout is open, error={}", e)
                            })
                    })
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
    }));



    Ok(())
}

fn run(args: Args) -> Result<(), Error> {
        let url = args.nlu_api_url.parse::<hyper::Uri>().unwrap();
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
            error!("{}", cause);
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
