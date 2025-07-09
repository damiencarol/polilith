use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use tar::Archive;

// use polilith::docker;
// mod sarif;
// mod rules;
//use rules;

use crate::docker::DockerManifest;
use crate::docker::*;
use crate::rules::Rule;
use crate::rules::RuleEnv;
use crate::rules::RuleUserRoot;
use crate::sarif::ArtifactLocation;
use crate::sarif::*;

pub(crate) struct ToolInfo {
    pub(crate) name: String,
    pub(crate) information_uri: String,
    pub(crate) full_name: String,
    pub(crate) version: String,
}

fn rule_pl007(
    ar: &mut Archive<File>,
    manifest: &DockerManifest,
    artifact_location: &ArtifactLocation,
    rule1: &mut RuleEnv,
    rule: &mut RuleUserRoot,
) -> Vec<Result> {
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
            // deserialize the config file
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            //println!("{}", s);
            let config: DockerConfig = serde_json::from_str(&s).unwrap();

            // emit config event
            rule1.emit_config(&config);
            rule.emit_config(&config);
        }
    }
    let mut res = Vec::new();
    res.extend(rule1.get_result(&artifact_location));
    res.extend(rule.get_result(&artifact_location));
    res
}

pub(crate) fn analyze_one_archive(infos: ToolInfo, input: PathBuf) -> SarifLog {
    // get the manifest
    let mut ar = Archive::new(File::open(&input).unwrap());
    let manifest = get_manifest(&mut ar);

    // add some rules
    let mut rule1 = RuleEnv::new();
    let mut rule7 = RuleUserRoot::new();

    // create a run
    let tool = Tool {
        driver: Driver {
            name: infos.name,
            information_uri: infos.information_uri,
            full_name: infos.full_name,
            version: infos.version,
            rules: vec![
                rule1.get_reporting_descriptor(),
                rule7.get_reporting_descriptor(),
            ],
        },
    };

    // add docker image as artifact
    let archive_artifact = Artifact {
        location: ArtifactLocation {
            uri: (&input).display().to_string(),
        },
    };

    // emit Docker config
    // for r in rules {
    //     r.emit_config(&manifest[0]);
    // }

    // let locations = vec![ResultLocation {
    //     physical_location: PhysicalLocation {
    //         artifact_location: ArtifactLocation {
    //             uri: archive_artifact.location.uri.clone(),
    //         },
    //     },
    // }];

    // test PL001 and aggregate results
    let mut ar2 = Archive::new(File::open(input).unwrap());
    let res_run = rule_pl007(
        &mut ar2,
        &manifest[0],
        &archive_artifact.location,
        &mut rule1,
        &mut rule7,
    );

    let run = Run {
        tool: tool,
        artifacts: vec![archive_artifact],
        results: res_run,
    };

    // create a report with only one run
    let mut log = SarifLog {
        schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json".to_string(),
        version: "2.1.0".to_string(),
        runs: Vec::new(),
    };
    log.runs.push(run);
    log
}
