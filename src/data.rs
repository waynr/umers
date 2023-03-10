use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use clap::parser::ValuesRef;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::errors::{Error, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Data {
    pub basic: Option<Basic>,
    pub education: Option<Vec<Education>>,
    pub experience: Option<Vec<Experience>>,
    pub skills: Option<Vec<Skills>>,
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

impl TryFrom<ValuesRef<'_, String>> for Data {
    type Error = Error;

    fn try_from(ss: ValuesRef<'_, String>) -> Result<Self> {
        let mut d = Data::new();

        for s in ss {
            let p = PathBuf::from(s);
            let tmp = Data::try_from(p.clone())?;
            d.merge(&tmp)
        }

        Ok(d)
    }
}

impl Data {
    pub fn new() -> Self {
        Data {
            basic: None,
            education: None,
            experience: None,
            skills: None,
        }
    }

    /// Merge aspects from the given Config into the current, return a new Config.
    pub fn merge(&mut self, other: &Self) {
        self.education = merge(&self.education, &other.education, false);
        self.experience = merge(&self.experience, &other.experience, false);
        self.skills = merge(&self.skills, &other.skills, false);

        if let Some(v) = &other.basic {
            self.basic = Some(v.clone());
        }
    }

    pub fn context(&self) -> Result<Context> {
        Ok(Context::from_serialize(self)?)
    }
}

fn merge<T: Clone>(
    left: &Option<Vec<T>>,
    right: &Option<Vec<T>>,
    overwrite: bool,
) -> Option<Vec<T>> {
    let mut new = Vec::new();

    if let Some(v) = &left {
        new = v.clone();
    }

    if let Some(v) = &right {
        if overwrite {
            new = v.clone();
        } else {
            new.append(&mut v.clone());
        }
    }

    match new.len() {
        x if x > 0 => Some(new),
        _ => None,
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Basic {
    pub name: Option<String>,
    pub contact: Option<Contact>,
    pub websites: Option<Vec<Website>>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Contact {
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Website {
    pub text: String,
    pub url: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Education {
    pub school: String,
    pub gpa: String,
    pub startdate: String,
    pub enddate: String,
    pub degrees: Vec<String>,
    pub achievements: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Experience {
    pub company: String,
    pub url: Option<String>,
    pub titles: Vec<Title>,
    pub projects: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Title {
    pub name: String,
    pub startdate: String,
    pub enddate: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Skills {
    pub category: String,
    pub skills: Vec<SkillsOrString>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SkillsOrString {
    String(String),
    Skills(Skills),
}
