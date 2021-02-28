use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::time;

use anyhow::{Context, Error, Result};

use libcond::{Condition, Rule};

use shellexpand::tilde;

use walkdir::WalkDir;

use super::file::{File, FileField};
use super::manifest::{MonitorDefinition, RuleDefinition};

pub struct Monitor {
    pub root_directory: PathBuf,
    pub recursive: bool,
    pub check_frequency: time::Duration,
    rules: Vec<Rule<File, FileField>>,
}

impl Monitor {
    fn try_apply<P>(&self, p: P) -> Result<bool>
    where
        P: AsRef<Path>,
        P: fmt::Debug,
        P: PartialEq<PathBuf>,
    {
        if p == self.root_directory {
            return Ok(false);
        }
        let f = File::new(&p)?;

        let mut at_least_one = false;
        for rule in self.rules.iter() {
            at_least_one |= rule.apply(&f)?
        }

        Ok(at_least_one)
    }

    pub fn check(&self) -> Result<()> {
        if !self.root_directory.exists() {
            log::debug!("monitor directory does not exist - skipping");
            return Ok(());
        }

        if self.recursive {
            let wk = WalkDir::new(&self.root_directory);
            for pos_path in wk.into_iter().filter_map(|f| f.ok()) {
                let _applied = self.try_apply(pos_path.path())?; // TODO: Do something with.
            }
        } else {
            // Non-recursive implementation.
            let data = fs::read_dir(&self.root_directory)?;
            for dir_entry in data.filter_map(|f| f.ok()) {
                let _applied = self.try_apply(dir_entry.path())?; // TODO: Use.
            }
        }
        Ok(())
    }
}

impl TryFrom<MonitorDefinition> for Monitor {
    type Error = Error;

    fn try_from(other: MonitorDefinition) -> Result<Monitor> {
        let check_frequency =
            parse_duration::parse(&other.check_frequency).context("Invalid check frequency")?;

        let rules: Result<Vec<Rule<File, FileField>>> =
            other.rules.into_iter().map(Rule::try_from).collect();

        Ok(Monitor {
            root_directory: PathBuf::from(
                tilde(&other.root_directory.to_str().unwrap()).to_string(),
            ),
            recursive: other.recursive,
            check_frequency,
            rules: rules?,
        })
    }
}

impl TryFrom<RuleDefinition> for Rule<File, FileField> {
    type Error = Error;

    fn try_from(other: RuleDefinition) -> Result<Rule<File, FileField>> {
        Ok(Rule::new(
            //other.name,
            Condition::parse(&other.condition)?,
            other.action.action_obj()?,
        ))
    }
}
