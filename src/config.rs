use crate::methods::{AuthenticationMethod, CommunicationMethod, Method};
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize, Clone)]
pub struct Purpose {
    pub tag: String,
    pub attributes: Vec<String>,
    pub allowed_auth: Vec<String>,
    pub allowed_comm: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawCoreConfig {
    auth_methods: Vec<AuthenticationMethod>,
    comm_methods: Vec<CommunicationMethod>,
    purposes: Vec<Purpose>,
}

#[derive(Debug, Deserialize)]
#[serde(from = "RawCoreConfig")]
pub struct CoreConfig {
    pub auth_methods: HashMap<String, AuthenticationMethod>,
    pub comm_methods: HashMap<String, CommunicationMethod>,
    pub purposes: HashMap<String, Purpose>,
}

fn contains_wildcard(target: &Vec<String>) -> bool {
    for val in target {
        if val == "*" {
            return true;
        }
    }
    false
}

fn validate_methods<T>(target: &Vec<String>, options: &HashMap<String, T>) -> bool {
    for val in target {
        if options.get(val).is_none() {
            return false;
        }
    }
    true
}

impl From<RawCoreConfig> for CoreConfig {
    fn from(config: RawCoreConfig) -> Self {
        let mut config = CoreConfig {
            auth_methods: config.auth_methods.iter().map(|m| { (m.tag().clone(), m.clone()) }).collect(),
            comm_methods: config.comm_methods.iter().map(|m| { (m.tag().clone(), m.clone()) }).collect(),
            purposes: config.purposes.iter().map(|m| { (m.tag.clone(), m.clone()) }).collect(),
        };

        // Handle wildcards in purpose auth and comm method lists
        for purpose in config.purposes.values_mut() {
            if contains_wildcard(&purpose.allowed_auth) {
                purpose.allowed_auth = config.auth_methods.keys().map(|x| x.to_string()).collect();
            }
            if contains_wildcard(&purpose.allowed_comm) {
                purpose.allowed_comm = config.comm_methods.keys().map(|x| x.to_string()).collect();
            }
        }

        // check all mentioned auth and comm methods exist
        for purpose in config.purposes.values() {
            if !validate_methods(&purpose.allowed_auth, &config.auth_methods) {
                panic!("Invalid auth method in purpose {}", purpose.tag);
            }
            if !validate_methods(&purpose.allowed_comm, &config.comm_methods) {
                panic!("Invalid comm method in purpose {}", purpose.tag);
            }
        }

        config
    }
}

impl CoreConfig {
    pub fn from_file(filename: &str) -> CoreConfig {
        let contents = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Could not read the config file {}", filename));

        let config: CoreConfig = serde_yaml::from_str(&contents)
            .unwrap_or_else(|e| panic!("Error parsing the config file {}: {:?}", filename, e));

        config
    }
}
