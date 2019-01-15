/**** Frontend main execution point.
  * 
  * Here the main code is executed.
  * This whole frontend layer simply acts as a glue between the CLI and the
  * `smake` library.
  * 
  * Author: ARaspiK
  * License: MIT
  */
module front.app;

import smake.file: File;

import std.getopt;
import std.stdio;

int main(string[] args) {
	// Output types for verbosity enabling options.
	enum OutputTypes: size_t {
		rules,
		// Last two are special cases.
		all,
		none,
	}

	string path = "./SMakefile";
	OutputTypes[] verbosity = [OutputTypes.none];

	auto helpInfo = args.getopt(config.caseSensitive,
			config.bundling, config.passThrough,
		"file|f", "The SMakefile to read from", &path,
		"verbose|v", "Verbosity options (all|none|rules)", &verbosity,
	);

	// Translate verbosity opts into usable data
	bool[OutputTypes.max - 1] verbose;
	foreach (v; verbosity) switch (v) with (OutputTypes) {
		case all:
			verbose[] = true;
			break;
		case none:
			verbose[] = false;
			break;
		default:
			verbose[v] = true;
	}

	if (helpInfo.helpWanted)
		defaultGetoptPrinter("A Make program designed for SDLang-formatted makefiles",
				helpInfo.options);
	else
		File.parse(path).get.toString(verbose[OutputTypes.rules]).writeln;

	return 0;
} 
