/**** File: First layer in reading in SMakefiles.
  * 
  * This is the interface for reading in SMakefiles into what the rest of the
  * library can understand.
  * 
  * Author: ARaspiK
  * License: MIT
  */
module smake.file;

import smake.rule;

import std.typecons;

import sdlang;

struct File {
	Rule[] rules;

	static Nullable!File parse(string path) {
		import std.algorithm;
		import std.array;
		import std.range;

		return path.parseFile.tags
			.filter!(t => t.name == "rule")
			.map!(t => Rule.parse(t))
			.fold!((a, e) {
					if (e.isNull)
						a.nullify;
					else
						a.get ~= e.get;
					return a;
				})(appender!(Rule[]).nullable)
			.apply!(rs => File(rs.data));
	}

	/// Stringifier.
	string toString() const {
		import std.format;

		return rules.format!"%-(%s%|\n%)";
	}
}
