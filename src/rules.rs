use crate::docker::DockerConfig;
use crate::sarif::MultiformatMessageString;
use crate::sarif::ReportingDescriptor;

pub(crate) trait Rule {
    fn get_id(&self) -> String;
    fn emit_config(&mut self, config: &DockerConfig);
    fn get_reporting_descriptor(&self) -> ReportingDescriptor;
}

pub struct RuleUserRoot {
    pub status: String,
}

impl Rule for RuleUserRoot {
    fn get_id(&self) -> String {
        String::from("PL007")
    }

    fn emit_config(&mut self, config: &DockerConfig) {
        println!("emit_config RuleUserRoot");
        if "" == config.config.user {
            self.status = "fail".to_string()
        } else {
            self.status = "pass".to_string()
        }
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
}
