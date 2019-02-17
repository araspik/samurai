# `SMake` #

[`SMake`][smake] is a simple Make program to run multiple commands easily. It
is programmer-oriented, but can be used in minimal form with little to no
understanding.

This is a functional/technical specification, which is _not_ complete. It is a
live document, and changes will keep coming.

## Examples ##

## Goals ##
* [ ] New format while providing backward compatibility for typical `Makefile`s
* [ ] Machine-parseable versions of commands
* [ ] Functions as a library (so that other programs can wrap core
      functionality without having to call the application as a program)
* [ ] Ability to ignore errors that occur in a rule, which then recalculates
      which rules can still be run and runs them
* [ ] Rule visibility (public, with descriptions, and private, without);
      Private rules can only be executed w/ a special option

## Nongoals ##

* A GUI. This application is meant to be user-friendly from the console, and as
  such building up a full graphical user interface provides no advantage.

## Terminology / Definitions ##

### `SMakefile`
A `SMakefile` is a file which contains descriptions of targets in such a format
that `SMake` can understand it. By default, `SMake` looks for this file as
being named `SMakefile` and residing in the current directory.

### Format
A format specifies which Makefile format to use when parsing the makefile.
Different formats have different features, and this allows `SMake` to work
differently for different targets.

### Target / Rule
A target is a method to convert some input files into some output files. It
takes in a set of input files, runs a given set of commands, and expects the
output files to be created. A target may depend upon another one for input
files, such that the depended-upon target will be run first to generate files
which are used as input files to the depending target.

### Dependency
A dependency of a target A runs before A, to ensure that A will run
successfully. For example, A may require an input file `foo.o` that is only
produced by another target. In this case, A will (should) declare its
dependency on the target so that that target runs before A.

### Virtual dependency
Target `B` is a virtual dependency of target `A` if one of `B`'s input files is
one of `A`s output files.

The difference between a dependency and a _virtual_ dependency is that virtual
dependencies are _not_ declared by targets, even though they should be. `SMake`
detects virtual dependencies and warns about them automatically, since a
virtual dependency is generally a sign of a dependency that the programmer
forgot to declare.

### Cyclic dependency
This is a situation where two targets depend on each other. Since `SMake` will
(by default) update the dependencies of a target before updating the target
itself, this leads to an infinite loop, and so is not allowed.

## Process Flowchart ##
* Begin
  - Parse options
  - Parse command
  * Build
    - Find, parse `SMakefile`
      - Not found: Print error and fail
    - Recursively add 'virtual dependencies' of targets
      + Ignore declared dependencies
      + Use hash table keyed by target name to prevent infinite recursion in
        case of cyclic dependencies
      - Warn user about virtual dependencies
      - Look for cyclic dependecies
        - Fail if found
    - Per target:
      - Update (virtual) dependencies
      - Check input files
        - Force update if no inputs
        - Fail on missing
      - Check output files
        - Force update if no outputs
        - Missing outputs forces updates
      - If all outputs are newer than all inputs then skip
      - Execute update
      - Update file modification times
  * Info
    - Find, parse `SMakefile`
      - Not found: Print error and fail
    - For each target:
      - Collect parsed info
      - Get target status:
        - Check input files for existence and modification times
        - Check output files (for existence and modification times)
        - Missing input files marks the target as 'invalid'
        - Missing output files marks the target as needing update
        - If an output file is older than an input then mark as needing update
        - Otherwise mark as up-to-date
      - Retrieve virtual dependencies
        - Warn if any found
        - Warn on cyclic dependency
      - Store information
    * General
      - Print stored information
        + Marking
        + Name
        + Inputs
        + Outputs
        + Commands
        + Dependencies
    * Dependencies
      - Print dependency info
        + Marking
        + Name
        + Dependency list (one level deep only)
    * Reverse dependencies
      - Build rev-dependency list:
        - For each target:
          - Look for (virtual) dependencies on this target
          - Save targets (by ref)
      - Print out list
        + Marking
        + Name
        + Dependers (one level deep only)
  * Help
    - Find command
    - Print help
      + Name
      + Brief Description
      + Detailed Description
      + Options
      + Examples

## Commands

### Begin
The application begins with some options, a command, and additional arguments
to that command (if any).  
First, options are parsed (using `getopt`). Some global options are:
* `-f|--file PATH`: Selects the path to the `SMakefile` to be used.
* `-v|--verbose [SEC]`: Increases amount of output, optionally for a specific
                        section (type) of output.
* `-q|--quiet [SEC]`: Reduces the amount of output, optionally for a specific
                      section (type) of output.

A list of commands which can be used are:
* `b(uild)`: Builds the given targets. Default when in compatibility mode.
* `i(nfo)`: Provides build information.
* `h(elp)`: Returns help information

### Build
The given targets are built (executed). Without a given list of targets, the
utilized target depends on whether compatibility mode is in place. If it is,
the first target (if any) is used. Otherwise, an 'all' target is used. If no
such targets are defined, an error occurs.

When executing targets, checks are made to ensure that all required input files
exist. If they do not, an error occurs, the user is alerted, and processing
halts.

If the `SMakefile` does not exist, an error occurs.

### Info
A subcommand can be specified:
* `g(eneral)` (def.): Returns general information.
* `d(epends)`: Returns a list of targets which the selected target(s) depend
               upon.
* `r(evdeps)`: Returns a list of targets which depend upon the given targets.

When no targets are given,
* `g(eneral)` returns a list of targets
* `d(epends)` and `r(evdeps)` both return a graph of all targets by their
                              dependencies.

Missing input files are marked (by prepending a `!` in front of invalid target
names) in all subcommands, but in `g(eneral)` the missing input files are
named.

If the `SMakefile` does not exist, an error occurs.

### Help
Provides help information about either `SMake` in general (global options,
available commands, invocation examples, etc.) or provides information about a
specific command (with options, subcommands if any, examples, etc.). Target
names are not recognized, and any build configurations are ignored.
The `SMakefile` is not required, and is never used, by this command.

## Parsing
Parsing is the act of converting inputted text of some sort (in our case from
a `Makefile`) into some internal representation (the internal `Target` type).
Since `SMake` supports multiple formats (mostly for backward compatibility), a
description of the process is added here. This is for technical purposes only.

Different formats (POSIX, GNU, and `SMake`) each have different file and target
types. All target types implement a special `Target` trait that provides
uniform access to them, and all file types implement a `File` trait for the
same reason.

Parsing works by getting each format to parse to a single global target type,
which hosts extra data in the form of a trait. This makes parsing
format-independent (so different formats can be used simultaneously), and
simplifies the process significantly. However, the immediately-parsed targets
are not ready to use - their dependencies must be resolved. This occurs in a
process called finalization, wherein the list of targets is converted into a
hash map, and dependencies, stored by name, are "standardized" such that they
refer to the dependency's primary name (which is used as the key to the hash
map). The finalization process is recursive, and automagically fails on missing
dependencies, cyclic dependencies, as well as duplicate target names.

TODO: Virtual dependency checking

### Parsing Flowchart
* For every file:
  * Match file to format
  * Parse file into unfinalized target list
* Create a final hash map of targets (size is the size of the target list)
* For each unfinalized target (pop off list, since each call removes multiple)
  * Remove it from the list
  * Finalize it
    * Split dependencies if not already done so
      * Standardizes dependency names into the primary name of the dependency
      * Checks for missing dependencies
    * For every dependency, find matching target
      * Fail if the dependency creates a cyclic dependency
      * No missing dependencies exist here!
      * Finalize that target (recursive)!
    * Store the now-finalized target in the output hash map
    * TODO: Find virtual dependencies here
* Return target hash map

[smake]: https://github.com/araspik/smake
