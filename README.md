### morpheus-sys
This crate provides a Rust wrapper around the Morpheus morphology parser/lemmatizer library.  Morpheus is an important tool used by many textual databases and analysis systems, but it is coded in the C89 dialect of C which can be difficult to work with.  This crate provides a safe and idiomatic Rust interface to the Morpheus library, making it easier to use in Rust projects.

To use include this crate in your project add
```
[dependencies]
morpheus-sys = { git = "https://github.com/jeremymarch/morpheus-sys.git", branch = "main" }
```
to your Cargo.toml file.  

You will also want to copy the Morpheus stem library to a location accessible to your program.  

Call `morpheus_check("FE/RW", Some("path/to/morpheus/stemlib"))` to analyze the betacode encoded data in the first parameter.  It will return the result as an Option&lt;String&gt;.  Alternatively, you can set the second parameter to None and specify the path to the Morpheus stem library using the `MORPHLIB` environment variable.

See the .github workflow for details on how to test and build this crate.  Currently, it has been tested on Linux and MacOS.  For usage, check the unit tests in src/lib.rs.

Future plans include allowing the stemlib to be embedded directly in the binary and optionally converting the input and result to Unicode.
