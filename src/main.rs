use std::path::Path;
use std::fs::File;
use tar::Archive;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct SarifLog {
    version: String,
    runs: Vec<Run>
}

#[derive(Serialize, Deserialize, Debug)]
struct Run {
    tool: Tool,
    artifacts:  Vec<Artifact>,
    results: Vec<Result>
}

#[derive(Serialize, Deserialize, Debug)]
struct Tool {
    driver: Driver
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Driver {
    name: String,
    information_uri: String,
    full_name: String,
    version: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Artifact {
    location: ArtifactLocation
}

#[derive(Serialize, Deserialize, Debug)]
struct ArtifactLocation {
    uri: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Result {
    rule_id: String,
    message: ResultMessage
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultMessage {
    text: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ReportingDescriptor {
    message: String
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

    vec![Result{ 
        rule_id: rule_id,
        message: ResultMessage{ text: text }
    }]
}


fn main() -> std::io::Result<()> {
    let image_name = "busybox:latest";
    let mut ar = Archive::new(File::open("busybox-latest.tar").unwrap());

    // create a run
    let tool = Tool{ driver: Driver{ 
        name: "polilith".to_string(),
        information_uri: "".to_string(),
        full_name: "Polilith Docker image quality tool".to_string(),
        version: "0.0.1".to_string()
    } };
    let mut run = Run{ tool : tool, artifacts: Vec::new(), results: Vec::new() };
    // add docker image as artifact
    run.artifacts.push(Artifact{
        location: ArtifactLocation{ uri: image_name.to_string() }
    });
    // test PL001 and aggregate results
    run.results.extend(rule_pl001(&mut ar));

    // create a report with only one run
    let mut log = SarifLog{ version : "2.1.0".to_string(), runs : Vec::new()};
    log.runs.push(run);
    
    // export as a SARIF file
    let mut file = File::create("report.sarif")?;
    let json = serde_json::to_string_pretty(&log).unwrap();
    file.write_all(json.as_bytes())?;
    Ok(())
}
