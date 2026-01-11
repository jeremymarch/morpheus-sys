### morpheus-sys
This crate provides a Rust wrapper around the Morpheus morphology parser/lemmatizer library.  Morpheus is an important tool used by many textual databases and analysis systems, but it is coded in the C89 dialect of C which can be difficult to work with.  This crate provides a safe and idiomatic Rust interface to the Morpheus library, making it easier to use in Rust projects.

To use include this crate in your project add
```
[dependencies]
morpheus-sys = "0.1.0"
```
to your Cargo.toml file.  

You will also want to copy the Morpheus stem library to a location accessible to your program.  

Call `morpheus_check("FE/RW", Some("path/to/morpheus/stemlib"))` to analyze the betacode encoded data in the first parameter.  It will return an Option<String> with the result.  You can also leave the second parameter as None and specify the path to the Morpheus stem library using the `MORPHLIB` environment variable.

Future plans include allowing the stemlib to be embedded directly in the binary and optionally converting the input and result to Unicode.
