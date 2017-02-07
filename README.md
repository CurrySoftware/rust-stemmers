

# Native Stemmers

This crate implements some stemmer algorithms found in the [snowball project](http://snowballstem.org/) which are compiled to rust using a [rust-backend](https://github.com/JDemler/snowball) for the snowball language.


# Supported Algorithms

-   Arabic
-   English
-   French
-   German
-   Italian
-   Portuguese
-   Romanian
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


# Related Projects

The [stemmer](https://github.com/lise-henry/stemmer-rs) crate provides bindings to the C Snowball implementation. 

