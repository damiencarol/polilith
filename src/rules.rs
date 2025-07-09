use crate::docker::DockerConfig;
use crate::sarif::MultiformatMessageString;
use crate::sarif::ReportingDescriptor;
use crate::sarif::Result;
use crate::sarif::ResultMessage;
use crate::sarif::*;

pub(crate) trait Rule {
    fn new() -> Self;
    fn get_id(&self) -> String;
    fn emit_config(&mut self, config: &DockerConfig);
    fn get_reporting_descriptor(&self) -> ReportingDescriptor;
    fn get_result(&self, location: &ArtifactLocation) -> Vec<Result>;
}

pub struct RuleUserRoot {
    pub user_detected: std::option::Option<String>,
}

impl Rule for RuleUserRoot {
    fn new() -> RuleUserRoot {
        RuleUserRoot {
            user_detected: None,
        }
    }

    fn get_id(&self) -> String {
        String::from("PL007")
    }

    fn emit_config(&mut self, config: &DockerConfig) {
        self.user_detected = Some(config.config.user.clone())
    }

    fn get_reporting_descriptor(&self) -> ReportingDescriptor {
        ReportingDescriptor{
            id: self.get_id(),
            short_description: Some(MultiformatMessageString{
                text: "Do not run as root".to_string()
            }),
            full_description: Some(MultiformatMessageString{
                text: "Root in a container is the same root as on the host machine, but restricted by the docker daemon configuration. No matter the limitations, if an actor breaks out of the container he will still be able to find a way to get full access to the host.".to_string()
            }),
        }
    }

    fn get_result(&self, location: &ArtifactLocation) -> Vec<Result> {
        let dd = self.user_detected.as_ref().unwrap();
        if dd.len() == 0 {
            return vec![Result {
                rule_id: "PL007".to_string(),
                kind: "fail".to_string(),
                level: "error".to_string(),
                message: ResultMessage {
                    text: "Process in image run as root".to_string(),
                    arguments: None,
                },
                locations: vec![ResultLocation {
                    physical_location: PhysicalLocation {
                        artifact_location: ArtifactLocation {
                            uri: location.uri.clone(),
                        },
                    },
                }],
            }];
        } else {
            return vec![Result {
                rule_id: "PL007".to_string(),
                kind: "pass".to_string(),
                level: "none".to_string(),
                message: ResultMessage {
                    text: "Process doesn't run as root".to_string(),
                    arguments: None,
                },
                locations: vec![ResultLocation {
                    physical_location: PhysicalLocation {
                        artifact_location: ArtifactLocation {
                            uri: location.uri.clone(),
                        },
                    },
                }],
            }];
        }
    }
}

pub struct RuleEnv {
    suspicious_envs: Vec<String>,
}

impl Rule for RuleEnv {
    fn new() -> RuleEnv {
        RuleEnv {
            suspicious_envs: Vec::new(),
        }
    }

    fn get_id(&self) -> String {
        String::from("PL001")
    }

    fn emit_config(&mut self, config: &DockerConfig) {
        let suspicious_tokens = [
            "passwd", "password", "pass", //  "pwd", can't use this one
            "secret", "key", "access", "api_key", "apikey", "token", "tkn",
        ];
        for var in &config.config.env {
            let dc: Vec<&str> = var.split("=").collect();
            println!("analyzing environment variable {:#?}...", dc[0]);
            for it in suspicious_tokens.iter() {
                if dc[0].contains(it) {
                    self.suspicious_envs.push(dc[0].to_string());
                }
            }
        }
    }

    fn get_reporting_descriptor(&self) -> ReportingDescriptor {
        ReportingDescriptor{
            id: self.get_id(),
            short_description: Some(MultiformatMessageString{
                text: "Do not store secrets in environment variables".to_string()
            }),
            full_description: Some(MultiformatMessageString{
                text: r#"The first docker security issue to prevent is including plaintext secrets in the Dockerfile.
Secrets distribution is a hairy problem and it’s easy to do it wrong. For containerized applications one can surface them either from the filesystem by mounting volumes or more handily through environment variables.
Unfortunately using ENV to store tokens, password or credentials is a bad practice: because Dockerfiles are usually distributed with the application, so there is no difference from hard coding secrets in code."#.to_string()
            }),
        }
    }

    fn get_result(&self, location: &ArtifactLocation) -> Vec<Result> {
        if self.suspicious_envs.len() > 0 {
            let mut res = Vec::new();
            for sus in self.suspicious_envs.iter() {
                res.push(Result {
                    rule_id: "PL001".to_string(),
                    kind: "fail".to_string(),
                    level: "error".to_string(),
                    message: ResultMessage {
                        text: "Potential secret in ENV key found: '{0}'".to_string(),
                        arguments: Some(vec![sus.to_string()]),
                    },
                    locations: vec![ResultLocation {
                        physical_location: PhysicalLocation {
                            artifact_location: ArtifactLocation {
                                uri: location.uri.clone(),
                            },
                        },
                    }],
                });
            }
            return res;
        } else {
            return vec![Result {
                rule_id: "PL001".to_string(),
                kind: "pass".to_string(),
                level: "none".to_string(),
                message: ResultMessage {
                    text: "No suspicious environment variables found".to_string(),
                    arguments: None,
                },
                locations: vec![ResultLocation {
                    physical_location: PhysicalLocation {
                        artifact_location: ArtifactLocation {
                            uri: location.uri.clone(),
                        },
                    },
                }],
            }];
        }
    }
}
