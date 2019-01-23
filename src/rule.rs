/*! Rule: A basic rule to execute.
  * 
  * They consist of commands to execute, along with inputs and outputs.
  *
  * They have the special condition that all of their inputs must exist. As
  * such, attempting to create one returns as a io::Result, and update_mtimes()
  * takes ownership (since the inputs may no longer exist, and so the rule may
  * be invalidated). The Result (should!) provide information about any I/O
  * errors that popped up.
  *
  * Author: ARaspiK
  * License: MIT
  */

use crate as smake;
use std::{io, fs};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::fmt;
use serde_yaml;
use serde_derive::{Serialize, Deserialize};

/// A basic rule to execute.
pub struct Rule {
    /// Commands to run to execute the rule.
    cmds: Vec<String>,
    /// Input files (paths and modification times).
    inps: Vec<(PathBuf, SystemTime)>,
    /// Output files (paths and optional modification times, it may not exist).
    outs: Vec<(PathBuf, Option<SystemTime>)>,
}

/// Information about an output's update requirements.
pub struct UpdateReq<'a> {
    /// The path to the output.
    pub path: &'a Path,
    /// The modification time of the output (if it existed)
    time: Option<SystemTime>,
    /// The path to an input (if an input was modified after the output).
    inps: Option<Vec<&'a Path>>,
}

/// A Rule created for (de)serialization purposes.
#[derive(Serialize, Deserialize)]
pub(crate) struct RuleData {
    pub cmds: Vec<String>,
    #[serde(alias = "ins")]
    pub inputs: Vec<String>,
    #[serde(alias = "outs")]
    pub outputs: Vec<String>,
}

impl Rule {
    /**
     * Creates a Rule given the commands, inputs and outputs.
     *
     * The inputs and outputs are searched for and (if found) their last
     * modification time is recorded.
     */
    pub fn new(cmds: Vec<String>, inps: Vec<String>, outs: Vec<String>)
            -> smake::Result<Rule> {
        Ok(Rule {
            cmds, // Same command list
            // Look for inputs that can't be accessed and fail on them.
            // Simultaneously, grab their latest modification time.
            inps: inps.iter()
                .map(move |s| PathBuf::from(s))
                .map(|path| fs::metadata(path.as_path())
                    .and_then(|m| m.modified())
                    .map(|mt| (path.clone(), mt))
                    .map_err(|e| match e.kind() {
                        io::ErrorKind::NotFound => smake::Error::NoFile{path},
                        _ => smake::Error::Other{source:e}
                    }))
                .collect::<smake::Result<Vec<_>>>()?,
            // Outputs don't have to exist, but grab their modification time if
            // they do.
            outs: outs.iter()
                .map(move |s| PathBuf::from(s))
                .map(|p| (p.clone(), fs::metadata(p).ok().map(|m|
                    m.modified().ok()).unwrap_or(None)))
                .collect::<Vec<_>>(),
        })
    }

    /**
     * Updates modification times for inputs and outputs.
     *
     * It takes ownership as the rule may be invalidated if an input no longer
     * exists.
     */
    pub fn update_mtimes(mut self) -> io::Result<Self> {
        // Modify each input
        for (ref path, ref mut time) in self.inps.iter_mut() {
            // by grabbing its modification time and stopping on failure.
            *time = fs::metadata(path)?.modified()?;
        }
        // Modify each output
        for (ref path, ref mut time) in self.outs.iter_mut() {
            // by trying to grab its timestamp and ignoring failure.
            *time = fs::metadata(path).ok().map(|m| m.modified().ok())
                .unwrap_or(None);
        }
        // If successful (w.r.t finding inputs) return the rule.
        Ok(self)
    }

    /**
     * Whether an update is necessary for the rule or not.
     *
     * It calculates the latest that an input has been modified and compares
     * that to the earliest that an output was modified. It assumes that all
     * inputs map to all outputs, and so any input that was modified _after_ the
     * latest any output has been modified means that an update is required to
     * update that output file.
     */
    pub fn needs_update(&self) -> bool {
        // Get latest input modification time.
        self.inps.iter().map(|e| e.1).max()
            // If no inputs, always update
            // Otherwise, update if any output can be found that
            .map_or(true, |inp_mt| self.outs.iter().any(|o|
                // either doesn't exist
                // or was modified before the latest input modification time.
                o.1.map_or(true, |out_mt| out_mt < inp_mt)))
    }

    /**
     * Returns specific information about the update requirements of each
     * output.
     *
     * Returns detailed information suitable for verbose reasoning to why a rule
     * must be executed.
     */
    pub fn update_reqs<'a>(&'a self) -> Vec<UpdateReq<'a>> {
        let mut res = Vec::with_capacity(self.outs.len());
        for (ref path, ref time) in self.outs.iter() {
            res.push(UpdateReq {
                path: path.as_path(),
                time: *time,
                inps: time.map(|mt| self.inps.iter()
                    .filter(|(_, ref intime)| *intime > mt)
                    .map(|(ref inpath, _)| inpath.as_path())
                    .collect::<Vec<_>>()),
            });
        }
        res
    }

    /**
     * Converts a RuleData into a Rule.
     *
     * For serialization purposes.
     */
    pub(crate) fn from_data(data: RuleData) -> smake::Result<Rule> {
        Self::new(data.cmds, data.inputs, data.outputs)
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?} via {:?}",
            self.inps.iter().map(|i| i.0.to_str().unwrap())
                .collect::<Vec<_>>(),
            self.outs.iter().map(|o| o.0.to_str().unwrap())
                .collect::<Vec<_>>(),
            self.cmds)
    }
}

impl From<Rule> for RuleData {
    fn from(data: Rule) -> Self {
        Self {
            cmds: data.cmds,
            inputs: data.inps.iter()
                .map(|i| i.0.to_str().unwrap().to_string())
                .collect::<Vec<_>>(),
            outputs: data.outs.iter()
                .map(|o| o.0.to_str().unwrap().to_string())
                .collect::<Vec<_>>(),
        }
    }
}

impl<'a> UpdateReq<'a> {
    /**
     * Whether the output file requires an update.
     *
     * An output requires an update when one of the following are true:
     * * It does not exist.
     * * Input files exist which are newer than it.
     * * No input files were associated with the rule.
     */
    pub fn needs_update(&self) -> bool {
        self.time.is_none() || self.inps.is_some()
    }
}

impl<'a> fmt::Display for UpdateReq<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(inps) = self.inps.as_ref() {
            write!(f, "\"{}\" older than {}, needs update.",
                self.path.to_str().unwrap(),
                inps.iter().map(|i| i.to_str().unwrap())
                    .fold(String::new(), |s, i| s + i + ", "))
        } else if self.time.is_some() {
            write!(f, "\"{}\" is newer than all inputs, does not need update.",
                self.path.to_str().unwrap())
        } else {
            write!(f, "\"{}\" does not exist, needs update.",
                self.path.to_str().unwrap())
        }
    }
}
