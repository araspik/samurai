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

import smake.file;

import std.stdio: writeln;

int main(string[] args) {
	if (args.length == 1)
		return 1;
	else
		writeln(File.parse(args[1]).get);

	return 0;
} 
