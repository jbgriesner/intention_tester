use intention_tester::{Error, StructOpt};
use std::fs;

#[derive(StructOpt, Debug)]
struct Args {
    /// Connection to the API
    #[structopt(short = "c", long = "api-connection")]
    nlu_api_url: String,
    /// The path to the test files
    #[structopt(
        short = "i",
        long = "input",
        parse(from_os_str),
        default_value = "./data"
    )]
    path_test_files: std::path::PathBuf,
}

fn run(args: Args) -> Result<(), Error> {
    let url = args.nlu_api_url;

    let (predicted_categories, effective_categories) = if args.path_test_files.is_dir() {
        let paths: std::fs::ReadDir = fs::read_dir(&args.path_test_files)?;
        intention_tester::parse_csv(url, paths.map(|p| p.unwrap().path())).unwrap()
    } else {
        intention_tester::parse_csv(url, std::iter::once(args.path_test_files)).unwrap()
    };

    intention_tester::compute_scores(predicted_categories, effective_categories);
    Ok(())
}

fn main() {
    intention_tester::launch_run(run);
}
