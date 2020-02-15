use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::time;

use libcond::{Condition, Rule};
use rood::{Cause, CausedResult, Error};
use shellexpand::tilde;
use walkdir::{DirEntry, WalkDir};

use super::file::{File, FileField};
use super::manifest::{MonitorDefinition, RuleDefinition};

pub struct Monitor {
    root_directory: PathBuf,
    recursive: bool,
    check_frequency: time::Duration,
    rules: Vec<Rule<File, FileField>>,
}

impl Monitor {
    fn try_apply<P>(&self, p: P) -> CausedResult<bool>
    where
        P: AsRef<Path>,
        P: fmt::Debug,
        P: PartialEq<PathBuf>,
    {
        if p == self.root_directory {
            return Ok(false);
        }
        println!("Trying: {:?}", &p);
        let f = File::new(&p)?;

        let mut at_least_one = false;
        for rule in self.rules.iter() {
            at_least_one |= rule.apply(&f)?
        }

        Ok(at_least_one)
    }

    pub fn check(&self) -> CausedResult<()> {
        if self.recursive {
            let wk = WalkDir::new(&self.root_directory);
            for pos_path in wk.into_iter() {
                match pos_path {
                    Ok(d_entry) => {
                        let applied = self.try_apply(d_entry.path())?; // TODO: Do something with.
                    }
                    Err(e) => return Err(Error::new(Cause::IOError, &format!("{}", e))),
                }
            }
        } else {
            // Non-recursive implementation.
            let data = fs::read_dir(&self.root_directory)?;
            for f in data.into_iter() {
                let dir_entry = f?;
                let applied = self.try_apply(dir_entry.path())?; // TODO: Use.
            }
        }
        Ok(())
    }
}

impl TryFrom<MonitorDefinition> for Monitor {
    type Error = Error;

    fn try_from(other: MonitorDefinition) -> CausedResult<Monitor> {
        let check_frequency = match parse_duration::parse(&other.check_frequency) {
            Ok(f) => Ok(f),
            Err(_) => Err(Error::new(Cause::InvalidData, "Invalid check frequency")),
        }?;

        let rules: CausedResult<Vec<Rule<File, FileField>>> =
            other.rules.into_iter().map(|r| Rule::try_from(r)).collect();

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

    fn try_from(other: RuleDefinition) -> CausedResult<Rule<File, FileField>> {
        Ok(Rule::new(
            other.name,
            Condition::parse(&other.condition)?,
            other.action.action_obj()?,
        ))
    }
}
