

# Rust Stemmers

This crate implements some stemmer algorithms found in the [snowball project](http://snowballstem.org/) which are compiled to rust using a [rust-backend](https://github.com/JDemler/snowball) for the snowball language.


# Supported Algorithms

-   Arabic
-   English
-   French
-   German
-   Italian
-   Portuguese
-   Romanian
-   Russian
-   Spanish


# Usage

```rust
extern crate native_stemmers;
use native_stemmers::{Algorithm, Stemmer};

// Create a stemmer for the english language
let en_stemmer = Stemmer::create(Algorithm::English);

// Stemm the word "fruitlessly"
// Please be aware that all algorithms expect their input to only contain lowercase characters.
assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
```

# Issues

Generated code is neither beautiful nor idiomatic nor optimized and full of warnings. 

There are some very low hanging fruit to fix this. 
Contributions to the rust backend or this crate are very welcome.


# Adding a stemmer

It is very simple to add a snowball-stemmer to this library:

1.  Install snowball with [rust-backend](https://github.com/JDemler/snowball) support.
2.  Put the <language>.sbl file containing the snowball-code in the algorithms directory
3.  Add `pub mod <language>;` to `src/snowball/algorithms/mod.rs`
4.  Add an enum-variant to the `Algorithm`-enum
5.  In `Stemmer::create` add a path for your enum-variant
6.  If test-data exists please consider implementing a test case in the tests-module
7.  Run the `recompile_and_test.sh`-script which expects a valid snowball-compiler installation in your path
8.  Send a pull-request


# Related Projects

-   [rust-backend](https://github.com/JDemler/snowball) for the snowball language which generated the code in src/snowball/algorithms.
-   The [stemmer](https://github.com/lise-henry/stemmer-rs) crate provides bindings to the C Snowball implementation.

