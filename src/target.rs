//! A target is a format-independent method to create outputs from inputs.
//!
//! A target is a method to convert some input files into some output files
//! using a given set of commands. A target may depend upon others to create
//! its input files, such that these dependencies will be run first in order to
//! generate the input files.
//!
//! Formats can create format-dependent extraneous information to be held by
//! targets parsed from files of that format by creating an implementation of
//! `TargetExtra`. This additional information will be included with each
//! target, but by virtue of boxing, targets parsed from different formats can
//! be mixed together.

use custom_error::custom_error;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

/// A uniform interface to format-specific extraneous data.
pub trait TargetExtra {
    /// Returns whether the current target may be referred to by the given
    /// name.
    ///
    /// This is most useful to `Makefile` formats, where targets have multiple
    /// names, corresponding to output files.
    ///
    /// A reasonable default implementation has been provided.
    fn has_name(&self, tgt: &Target, name: &str) -> bool {
        tgt.name == name
    }
}

/// A structure that differentiates mixed dependencies from unmixed (or split)
/// dependencies.
///
/// Useful primarily for `Makefile` formats, where dependencies may be input
/// files or other targets.
pub enum MixedDeps {
    Mixed(Vec<String>),
    UnMixed {
        inputs: Vec<PathBuf>,
        dependencies: Vec<String>,
    },
}

impl MixedDeps {
    /// Converts mixed dependencies to unmixed dependencies, by resolving names
    /// given a predicate that defines whether the dependency exists.
    ///
    /// The predicate returns whether the given name is a dependency as an
    /// optional structure, which, when nothing, represents an input file.
    /// When, however, the name is found to be a dependency, an additional name
    /// is given which is considered a "more correct" reference to it (i.e the
    /// primary name of the matching target). This is useful as it standardizes
    /// names, allowing the result to easily reference dependencies from a hash
    /// map of primary names.
    ///
    /// Panics if a dependency (from split state) is not found by the
    /// predicate.
    fn split<P>(self, mut predicate: P) -> (Vec<PathBuf>, Vec<String>)
    where
        P: FnMut(&str) -> Option<Option<String>>,
    {
        match self {
            MixedDeps::Mixed(deps) => {
                deps.into_iter()
                    .fold((Vec::new(), Vec::new()), |mut res, dep| {
                        if let Some(name) = predicate(&dep) {
                            res.1.push(name.unwrap_or(dep));
                        } else {
                            res.0.push(dep.into());
                        }
                        res
                    })
            }
            MixedDeps::UnMixed {
                inputs,
                dependencies,
            } => {
                // TODO: Convert this to report multiple missing dependencies
                // at a time?
                (
                    inputs,
                    dependencies.into_iter().fold(Vec::new(), |mut res, dep| {
                        if let Some(name) = predicate(&dep) {
                            res.push(name.unwrap_or(dep));
                        } else {
                            panic!("Dependency {} not found!", dep);
                        }
                        res
                    }),
                )
            }
        }
    }
}

/// A format-independent method to create outputs from inputs.
///
/// See the module-level documentation for more info.
pub struct Target {
    /// Name of the target.
    pub name: String,
    /// Files produced by the target.
    pub outputs: Vec<PathBuf>,
    /// Inputs and dependencies, mixed or unmixed.
    pub dependencies: MixedDeps,
    /// Commands to run.
    ///
    /// Due to the fact that executing a command needs to be done mutably, a
    /// whole bunch of errors come up because of the way updates are laid out.
    /// As such, a command is created and executed at the time of update, not
    /// created beforehand.
    pub commands: Vec<String>,
    /// Extraneous format-specific data.
    pub extra: Box<TargetExtra>,
}

/// An error type for updates.
custom_error! {pub UpdateErr
    Io{source: io::Error} = "I/O Error",
    Status{status: i32} = "Process exited with error code {status}",
    Signal = "Process exited with signal",
}

/// Creates a command from a string.
///
/// The command will be wrappped in a platform-specific shell.
fn string_to_command(command: &str) -> Command {
    let mut cmd = Command::new(if cfg!(windows) { "cmd" } else { "sh" });
    cmd.arg(if cfg!(windows) { "/C" } else { "-c" });
    cmd.arg(command);
    cmd
}

impl Target {
    /// Creates a new target.
    pub fn new(
        name: String,
        outputs: Vec<String>,
        dependencies: MixedDeps,
        commands: Vec<String>,
        extra: Box<TargetExtra>,
    ) -> Target {
        Target {
            name,
            outputs: outputs.into_iter().map(|p| p.into()).collect(),
            dependencies,
            commands,
            extra,
        }
    }

    /// Returns input files of the target, if known.
    ///
    /// Panics if the input files are unknown.
    /// This is done as these functions are only expected to be called after
    /// finalization is completed, at which point they are known for sure.
    pub fn inputs(&self) -> &Vec<PathBuf> {
        if let MixedDeps::UnMixed { inputs, .. } = &self.dependencies {
            inputs
        } else {
            panic!("Input files are still mixed!");
        }
    }

    /// Returns dependencies, if known.
    ///
    /// Panics if the dependencies are unknown.
    /// It panics as these functions are only expected to be called after
    /// finalization is complete, at which point they are known for sure.
    pub fn dependencies(&self) -> &Vec<String> {
        if let MixedDeps::UnMixed { dependencies, .. } = &self.dependencies {
            dependencies
        } else {
            panic!("Dependencies are still mixed!");
        }
    }

    /// Updates the target.
    ///
    /// Returns `None` if it failed.
    /// Otherwise, returns a boolean indicating whether an update was needed.
    /// The commands are executed sequentially and synchronously.
    ///
    /// Returns any errors that may have occurred during updating, including if
    /// the commands failed to run.
    pub fn update(&self, list: &HashMap<String, Target>) -> Result<bool, UpdateErr> {
        // First, update dependencies, stopping on failure.
        if self.dependencies().iter()
            .try_fold(false, |res, dep| {
                list.get(dep).unwrap().update(list).map(|r| res || r)
            })?
           // If a dependency was updated, force update.
           // Otherwise, check modification times.
        || self.inputs().iter() // TODO: Better error messages
                .map(|p| fs::metadata(p).unwrap().modified().unwrap())
                .max() // If no inputs, force update
                .map_or(true, |latest| self.outputs.iter()
                    .map(|o| fs::metadata(o).and_then(|md| md.modified()).ok())
                    // If missing output, update
                    // If output updated earlier than input, update
                    .any(|o| o.map_or(true, |o| o < latest)))
        {
            // Update: Run all commands, printing exit status on failure of
            // any.
            self.commands
                .iter()
                .map(|cmd| string_to_command(&cmd))
                .try_for_each(|mut cmd| {
                    cmd.status()?
                        .code()
                        .map_or(Err(UpdateErr::Signal), |status| {
                            if status == 0 {
                                Ok(())
                            } else {
                                Err(UpdateErr::Status { status })
                            }
                        })
                })?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Finalizes a whole list of targets.
    ///
    /// Handles some external bookkeeping required by `finalize`.
    pub fn finalize_list(mut list: Vec<Target>) -> HashMap<String, Target> {
        let mut post = HashMap::with_capacity(list.len());
        let mut path = Vec::new();

        // Loop over the targets. Keep popping, since we cannot iterate
        // normally (because recursiveness may absorb multiple elements).
        while let Some(elem) = list.pop() {
            elem.finalize(&mut list, &mut post, &mut path);
        }

        post
    }

    /// Finalizes the target.
    ///
    /// Finalization involves verifying dependencies, differentiating inputs
    /// from dependencies (if necessary), translating dependencies into primary
    /// names for the referred-to targets, finalizing dependencies, and putting
    /// the target into the given output hash map.
    ///
    /// This function is recursive - it further finalizes all of its
    /// dependencies. In order to prevent circular dependencies, which would
    /// cause the application to hang, a "path" is taken, which describes which
    /// targets called each other (in a stack-like list) until they reached
    /// this call. If a dependency of the current function is found which
    /// already exists on the path, then this function panics.
    ///
    /// Additionally, this function panics if a dependency is not found or if a
    /// target with the same primary name already exists in the output hashmap.
    pub fn finalize(
        mut self,
        list: &mut Vec<Target>,
        post: &mut HashMap<String, Target>,
        path: &mut Vec<String>,
    ) {
        // First, we resolve (not finalize) dependencies.
        let (inputs, dependencies) = self.dependencies.split(|dep| {
            list.iter()
                .chain(post.values())
                .find(|tgt| tgt.extra.has_name(tgt, &dep))
                .map(|target| {
                    if target.name == dep {
                        None
                    } else {
                        Some(target.name.clone())
                    }
                })
        });

        // Then, we finalize each dependency, checking for cyclic or missing
        // dependencies.
        // Note that we push the name onto the path stack, and pop it off
        // afterwards. This means that the path will be modified, but in the
        // same state as how it was passed to the function.
        path.push(self.name);
        for dep in dependencies.iter() {
            if path.contains(dep) {
                panic!("Cyclic dependency found for {}!", dep);
            }

            // Now, we check to see if we have to finalize the dependency.
            if let Some(loc) = list.iter().position(|t| &t.name == dep) {
                // We remove it (ownership) and then finalize it.
                list.remove(loc).finalize(list, post, path);
            }

            // Note that all dependencies exist, since the `MixedDeps::split`
            // function checked it for all dependencies. As such, any
            // dependencies not in `list` are in the output hash map already.
        }
        self.name = path.pop().unwrap();

        // Now, the target is stored on the output hash map.
        // NOTE: At the moment, the key is cloned from the name. If possible,
        // this should be prevented.
        self.dependencies = MixedDeps::UnMixed {
            inputs,
            dependencies,
        };
        if let Some(tgt) = post.insert(self.name.clone(), self) {
            // Duplicate found! Panic.
            panic!("Duplicate target {} found!", tgt.name);
            // Note that tgt.name == key == self.name
        }
    }
}
