use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tar::Archive;

mod docker;
mod sarif;

fn rule_pl007(
    ar: &mut Archive<File>,
    manifest: &docker::DockerManifest,
    artifact_location: &sarif::ArtifactLocation,
) -> Vec<sarif::Result> {
    let rule_id = "PL007".to_string();
    let locations = vec![sarif::ResultLocation {
        physical_location: sarif::PhysicalLocation {
            artifact_location: sarif::ArtifactLocation {
                uri: artifact_location.uri.clone(),
            },
        },
    }];
    //let file = ar.unpack("manifest.json").unwrap();
    //let file = ar.unpack("manifest.json");//.unwrap();
    // get the manifest
    // let file = ar.unpack("manifest.json");

    // let serialized = file.unwrap();
    // println!("serialized = {}", serialized);

    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized);

    // get all files
    // for (i, file) in ar.entries().unwrap().enumerate() {
    //     let mut file = file.unwrap();
    //     print!("{:?}", file.path());
    //     file.unpack(format!("file-{}", i)).unwrap();
    // }

    for file in ar.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        // Inspect metadata about the file
        //println!("{:?}", file.header().path().unwrap());
        //println!("{}", file.header().size().unwrap());

        // check if we have manifest
        if Path::new(&manifest.config) == file.header().path().unwrap() {
            println!("config found...");
            // files implement the Read trait
            let mut s = String::new();

            file.read_to_string(&mut s).unwrap();
            //println!("{}", s);
            let config: docker::DockerConfig = serde_json::from_str(&s).unwrap();
            println!("user detected is rule 007: {}", config.config.user);

            if config.config.user == "" {
                return vec![sarif::Result {
                    rule_id: rule_id,
                    kind: "fail".to_string(),
                    level: "error".to_string(),
                    message: sarif::ResultMessage {
                        text: "Process in image run as root".to_string(),
                    },
                    locations: locations,
                }];
            } else {
                return vec![sarif::Result {
                    rule_id: rule_id,
                    kind: "pass".to_string(),
                    level: "none".to_string(),
                    message: sarif::ResultMessage {
                        text: "Process doesn't run as root".to_string(),
                    },
                    locations: locations,
                }];
            }
        }
    }

    vec![sarif::Result {
        rule_id: rule_id,
        kind: "fail".to_string(),
        level: "error".to_string(),
        message: sarif::ResultMessage {
            text: "Can't find configuration file from manifest".to_string(),
        },
        locations: locations,
    }]
}

fn analyze_one_archive(driver: sarif::Driver, input: &str) -> sarif::SarifLog {
    // get the manifest
    let mut ar = Archive::new(File::open(input).unwrap());
    let manifest = docker::get_manifest(&mut ar);

    // create a run
    let tool = sarif::Tool { driver: driver };
    let mut run = sarif::Run {
        tool: tool,
        artifacts: Vec::new(),
        results: Vec::new(),
    };
    // add docker image as artifact
    let archive_artifact = sarif::Artifact {
        location: sarif::ArtifactLocation {
            uri: (&input).to_string(),
        },
    };
    run.artifacts.push(archive_artifact);
    // test PL001 and aggregate results
    let mut ar2 = Archive::new(File::open(input).unwrap());
    run.results.extend(rule_pl007(
        &mut ar2,
        &manifest[0],
        &run.artifacts[0].location,
    ));

    // create a report with only one run
    let mut log = sarif::SarifLog {
        schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json".to_string(),
        version: "2.1.0".to_string(),
        runs: Vec::new(),
    };
    log.runs.push(run);

    log
}

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
    let driver = sarif::Driver {
        name: name.to_string(),
        information_uri: uri.to_string(),
        full_name: about.to_string(),
        version: version.to_string(),
    };
    // do one run
    println!("Analysing file {}...", myfile);
    let log = analyze_one_archive(driver, &myfile);
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
