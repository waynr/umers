use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::errors::{Error, Result};

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub education: Vec<Education>,
    pub experience: Vec<Experience>,
    pub skills: Vec<Skills>,
}

impl TryFrom<PathBuf> for Data {
    type Error = Error;

    fn try_from(pb: PathBuf) -> Result<Self> {
        // verify path exists
        let _ = pb.symlink_metadata()?;

        let mut file = File::open(pb)?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents)?;
        Ok(serde_yaml::from_str(&contents)?)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Education {
    pub school: String,
    pub gpa: String,
    pub startdate: String,
    pub enddate: String,
    pub degrees: Vec<String>,
    pub achievements: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Experience {
    pub company: String,
    pub url: Option<String>,
    pub titles: Vec<Title>,
    pub projects: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Title {
    pub name: String,
    pub startdate: String,
    pub enddate: String,
}

#[derive(Deserialize, Serialize)]
pub struct Skills {
    pub category: String,
    pub skills: Vec<SkillsOrString>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum SkillsOrString {
    String(String),
    Skills(Skills),
}
