# SMake #
---------
[SMake][smake] is a simple Make program to run multiple commands easily. It is
programmer-oriented, but can be used in minimal form with little to no
understanding.

This is a functional/technical specification, which is _not_ complete. It is a
live document, and changes will keep coming.

## Examples ##
--------------

## Goals ##
-----------
* [ ] New syntax (SDLang) while providing backward compatability for typical
      Makefiles
* [ ] Machine-parseable versions of commands
* [ ] Functions as a library (so that other programs can wrap core
      functionality without having to call the application as a program)
* [ ] Ability to ignore errors that occur in a rule, which then recalculates
      which rules can still be run and runs them
* [ ] Rule visibility (public, with descriptions, and private, without);
      Private rules can only be executed w/ a special option

## Nongoals ##
--------------
* A GUI. This application is meant to be user-friendly from the console, and as
  such building up a full graphical user interface provides no advantage.

## Terminology / Definitions ##
-------------------------------

### `SMakefile`
A `SMakefile` is a file which contains descriptions of targets in such a format
that SMake can understand it. By default, SMake looks for this file as being
named `SMakefile` and residing in the current directory.

### Target / Rule
A target is a method to convert some input files into some output files. It
takes in a set of input files, runs a given set of commands, and expects the
output files to be created. A target may depend upon another one for input
files, such that the depended-upon target will be run first to generate files
which are used as input files to the depending target.

## Process Flowchart ##
-----------------------
* Begin
  * Build
  * Help
  * Info
    * General
    * Dependencies
    * Reverse dependencies

### Begin
The application begins with a command, a set of targets, and some options.
These are taken from the command line.
A list of commands which can be used are:
* `b(uild)` (def.): Builds the given targets.
* `i(nfo)`: Provides build information.
* `h(elp)`: Returns help information.

### Build
The given targets are built (executed). Without a given list of targets, the
utilized target depends on whether compatability mode is in place. If it is,
the first target (if any) is used. If no targets are defined, an error occurs.

When executing targets, checks are made to ensure that all required input files
exist. If they do not, an error occurs, the user is alerted, and processing
halts.

If the SMakefile does not exist, an error occurs.

### Info
A subcommand can be specified:
* `g(eneral)` (def.): Returns general information.
* `d(epends)`: Returns a list of targets which the selected target(s) depend
               upon.
* `r(evdeps)`: Returns a list of targets which depend upon the given targets.

When no targets are given,
* `g(eneral)` returns a list of targets
* `d(epends)` and `r(evdeps)` both return a graph of targets by their
                              dependencies.

Missing input files are marked (by prepending a `!` in front of invalid target
names) in all subcommands, but in `g(eneral)` the missing input files are
named.

If the SMakefile does not exist, an error occurs.

### Help
Provides help information about either SMake in general (global options,
available commands, invocation examples, etc.) or provides information about a
specific command (with options, subcommands if any, examples, etc.). Target
names are not recognized, and any build configurations are ignored.
The SMakefile is not required, and is never used, by this command.

<!-- Links --!>
[smake]: https://github.com/araspik/smake
