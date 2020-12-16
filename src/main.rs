use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tar::Archive;

#[derive(Serialize, Deserialize, Debug)]
struct SarifLog {
    #[serde(rename = "$schema")]
    schema: String,
    version: String,
    runs: Vec<Run>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Run {
    tool: Tool,
    artifacts: Vec<Artifact>,
    results: Vec<Result>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tool {
    driver: Driver,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Driver {
    name: String,
    information_uri: String,
    full_name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artifact {
    location: ArtifactLocation,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArtifactLocation {
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Result {
    rule_id: String,
    message: ResultMessage,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultMessage {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReportingDescriptor {
    message: String,
}

fn rule_pl001(ar: &mut Archive<File>) -> Vec<Result> {
    let rule_id = "PL001".to_string();
    let text = "User root pb!".to_string();
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
        if Path::new("manifest.json") == file.header().path().unwrap() {
            // files implement the Read trait
            let mut s = String::new();

            file.read_to_string(&mut s).unwrap();
            println!("{}", s);
        }
    }

    vec![Result {
        rule_id: rule_id,
        message: ResultMessage { text: text },
    }]
}

fn analyze_one_archive(driver: Driver, input: &str) -> SarifLog {
    let mut ar = Archive::new(File::open(input).unwrap());

    // create a run
    let tool = Tool { driver: driver };
    let mut run = Run {
        tool: tool,
        artifacts: Vec::new(),
        results: Vec::new(),
    };
    // add docker image as artifact
    run.artifacts.push(Artifact {
        location: ArtifactLocation { uri: (&input).to_string() },
    });
    // test PL001 and aggregate results
    run.results.extend(rule_pl001(&mut ar));

    // create a report with only one run
    let mut log = SarifLog {
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
    let author = "Damien Carol <damien.carol@gmail.com>";
    let about = env!("CARGO_PKG_DESCRIPTION");
    let uri = env!("CARGO_PKG_HOMEPAGE");
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
    let driver = Driver {
        name: name.to_string(),
        information_uri: uri.to_string(),
        full_name: about.to_string(),
        version: version.to_string(),
    };
    // do one run
    println!("Analysing file {}...", myfile);
    let log = analyze_one_archive(driver, &myfile);

    // manage ouput
    if matches.is_present("output") {
        // export as a SARIF file
        let mut file = File::create(matches.value_of("output").unwrap())?;
        let json = serde_json::to_string_pretty(&log).unwrap();
        file.write_all(json.as_bytes())?;
    }
    Ok(())
}
