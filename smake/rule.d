/**** Rule: A basic rule to execute.
  * 
  * Rules consist of commands to execute, along with a (optional) set of inputs
	* and outputs.
  * 
  * Author: ARaspiK
  * License: MIT
  */
module smake.rule;

import std.typecons;

import sdlang;

/// A basic rule to execute.
struct Rule {
	string name;
	string[] commands, inputs, outputs;

	/// Stringifier.
	string toString() const {
		import std.format;

		return format!"{%(%s%|, %)} -> {%(%s%|, %)} via \"%-(%s%|; %)\""
			(inputs, outputs, commands);
	}

	/// Reads from a SDLang Tag.
	/// Returns nonexistent if failed.
	static Nullable!Rule parse(Tag tag) {
		import std.algorithm;
		import std.array;
		import std.range;

		if (tag.name != "rule"
				|| tag.values.length != 1
				|| tag.values[0].peek!string is null)
			return typeof(return)();

		string name = tag.values[0].get!string;

		string[] cmds = tag.tags
			.filter!(t => t.name == "cmd")
			.map!(t => t.values
				.map!(v => v.peek!string !is null ? v.get!string : null))
			.join;

		if (!cmds.length)
			return typeof(return)();

		string[] inputs = tag.tags
			.filter!(t => t.name == "in")
			.map!(t => t.values
					.map!(v => v.peek!string !is null ? v.get!string : null))
			.join;

		string[] outputs = tag.tags
			.filter!(t => t.name == "out")
			.map!(t => t.values
					.map!(v => v.peek!string !is null ? v.get!string : null))
			.join;

		return chain(cmds, inputs, outputs).canFind(null)
			? typeof(return)()
			: Rule(name, cmds, inputs, outputs).nullable;
	}
}
