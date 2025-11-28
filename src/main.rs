
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

use clap::{Parser};

mod algo;
mod docker;
mod rules;
mod sarif;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Docker image file to scan
    #[arg(short, long, default_value = "FILE.tar")]
    file: Option<PathBuf>,

    /// Report file path
    #[arg(short, long, default_value = "FILE.tar.sarif")]
    output: Option<PathBuf>,
}

fn generate_message(message: &sarif::ResultMessage) -> String {
    if None != message.arguments {
        let mut res = message.text.to_string();
        while let Some(args) = &message.arguments {
            res = res.replace("{0}", &args[0])
        }
        return res;
    }
    message.text.to_string()
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let myfile = cli.file.unwrap();

    // boostrap context
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    //let author = env!("CARGO_PKG_AUTHORS");
    let about = env!("CARGO_PKG_DESCRIPTION");
    let uri = env!("CARGO_PKG_REPOSITORY");
    let driver = algo::ToolInfo {
        name: name.to_string(),
        information_uri: uri.to_string(),
        full_name: about.to_string(),
        version: version.to_string(),
    };
    // do one run
    println!("Analysing file {}...", myfile.display());
    if !Path::new(&myfile).exists() {
        panic!("Problem opening the file: {}", myfile.display());
    }
    let log = algo::analyze_one_archive(driver, myfile);
    // do some pretty print
    println!("rule\tkind\tlevel\tmessage");
    println!("----\t----\t-----\t-------");
    for result in &log.runs[0].results {
        let generated_message = generate_message(&result.message);
        println!(
            "{}\t{}\t{}\t{}",
            result.rule_id, result.kind, result.level, generated_message
        );
    }
    println!("");

    // export as a SARIF file
    let output = cli.output.unwrap();
    println!("generating report file {}...", output.display());
    let mut file = File::create(output)?;
    let json = serde_json::to_string_pretty(&log).unwrap();
    file.write_all(json.as_bytes())?;

    Ok(())
}
