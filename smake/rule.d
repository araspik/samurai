/**** Rule: A basic rule to execute.
  * 
  * Rules consist of commands to execute, along with a (optional) set of inputs
  * and outputs.
  * 
  * Author: ARaspiK
  * License: MIT
  */
module smake.rule;

import std.datetime;
import std.file;
import std.typecons;

import sdlang;

/// A basic rule to execute.
struct Rule {
	/// Name of rule.
	string name;
	/// Commands to run.
	string[] commands;
	/// Input and output file names.
	string[] inputs, outputs;
	/// Cached modification timestamps for inputs.
	SysTime[] inputTimes;
	/// Whether update is needed or not.
	bool updateNeeded;

	/// Constructor.
	this(string name, string[] commands, string[] inputs, string[] outputs) {
		import std.algorithm, std.array;

		this.name = name;
		this.commands = commands;
		this.inputs = inputs;
		this.outputs = outputs;

		// Initialize inputTimes with current timestamps.
		this.inputTimes = inputs.map!timeLastModified.array;
		// Keep temporary latest modification timestamp.
		auto lastMod = inputTimes.maxElement;

		// Check for first output needing update, and set updateNeeded.
		this.updateNeeded = !outputs
			.until!(o => !o.exists || o.timeLastModified < lastMod)
			.empty;
	}

	/**** Returns verbose information about update requirements.
		* 
		* The information is returned as a lazy range of output-update information.
		* Each element represents one output.
		* The properties of each element are:
		* * `.output`: Name of output file.
		* * `.needsUpdate`: Whether the output needs to be updated.
		* * `.exists`: Whether the file exists.
		* * `.input`: An input file which the output is older than.
		*             If this doesn't make sense then it is 'null', which occurs
		*             when the output is newer than all input files or it just
		*             doesn't exist at all.
		* * `.toString`: A human-readable string output.
		* 
		*/
	auto getUpdateInfo() const {
		static struct OutputUpdateInfo {
			string output;
			string input;
			bool needsUpdate;
			bool exists;

			string toString() const {
				import std.format;

				if (!exists)
					return output.format!`"%s" nonexistent, needs update.`;
				else if (needsUpdate)
					return output.format!`"%s" is older than "%s", needs update.`(input);
				else
					return output.format!`"%s" is newest, does not need update.`;
			}
		}

		import std.algorithm: map;

		return outputs.map!((o) {
			import std.algorithm: countUntil;

			OutputUpdateInfo res = OutputUpdateInfo(o, null, true, o.exists);

			if (!res.exists) {}
			else if (auto j = inputTimes.countUntil!"a > b"(o.timeLastModified) + 1)
				res.input = inputs[j-1];
			else
				res.needsUpdate = false;

			return res;
		});
	}

	/// Stringifier.
	string toString() const {
		import std.format;

		return format!`{%(%s%|, %)} -> {%(%s%|, %)} via "%-(%s%|; %)" (%s update)`
			(inputs, outputs, commands, updateNeeded ? "needs" : "does not need");
	}

	/// Verbose stringifier.
	string toString(bool verbose) const {
		import std.format;

		if (verbose)
			return toString ~ getUpdateInfo.format!"%-(\n* %s%)";
		else
			return toString;
	}

	/// Reads from a SDLang Tag.
	/// Returns nonexistent if parsing failed.
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
