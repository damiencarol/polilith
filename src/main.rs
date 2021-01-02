use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

mod algo;
mod docker;
mod rules;
mod sarif;

fn main() -> std::io::Result<()> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let about = env!("CARGO_PKG_DESCRIPTION");
    let uri = env!("CARGO_PKG_REPOSITORY");
    // parse command line
    let args = App::new(name)
        .version(version)
        .author(author)
        .about(about)
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Docker image file"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("out")
                .takes_value(true)
                .help("Report file"),
        );
    let matches = args.get_matches();

    let myfile = matches
        .value_of("file")
        .unwrap_or("busybox-latest.tar")
        .to_string();

    // boostrap context
    let driver = algo::ToolInfo {
        name: name.to_string(),
        information_uri: uri.to_string(),
        full_name: about.to_string(),
        version: version.to_string(),
    };
    // do one run
    println!("Analysing file {}...", myfile);
    let log = algo::analyze_one_archive(driver, &myfile);
    // do some pretty print
    println!("rule\tkind\tlevel\tmessage");
    println!("----\t----\t-----\t-------");
    for result in &log.runs[0].results {
        println!(
            "{}\t{}\t{}\t{}",
            result.rule_id, result.kind, result.level, result.message.text
        );
    }
    println!("");
    // manage ouput
    if matches.is_present("output") {
        // export as a SARIF file
        let output = matches.value_of("output").unwrap();
        println!("generating report file {}...", output);
        let mut file = File::create(output)?;
        let json = serde_json::to_string_pretty(&log).unwrap();
        file.write_all(json.as_bytes())?;
    }
    Ok(())
}
