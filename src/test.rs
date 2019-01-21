//! Tests
//!
//! Tests for everything SMake.
//!
//!
//! Author: ARaspiK
//! License: MIT

use crate as smake;

#[test]
fn test_parser() {
    let test_str = "main:
    cmds:
        - \"gcc -c hello.c\"
    ins:
        - \"hello.c\"
    outs:
        - \"hello.o\"";

    let rules = smake::File::from_str(test_str);

    assert!(rules.is_ok());
}
