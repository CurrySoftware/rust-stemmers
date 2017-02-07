//! This library provides rust implementations for some stemmer algorithms
//! written in the [snowball language](https://snowballstem.org/).
//!
//!
//! All algorithms expect the input to already be lowercased.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! rust-stemmers = "0.1"
//! ```
//!
//! ```rust
//! extern crate rust_stemmers;
//!
//! use rust_stemmers::{Algorithm, Stemmer};
//!
//! fn main() {
//!    let en_stemmer = Stemmer::create(Algorithm::English);
//!    assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
//! }
//! ```


use std::borrow::Cow;

mod snowball;

use snowball::SnowballEnv;
use snowball::algorithms;

/// Enum of all supported algorithms.
/// Check the [Snowball-Website](https://snowballstem.org/) for details.
pub enum Algorithm {
    Arabic,
    English,
    French,
    German,
    Italian,
    Portuguese,
    Romanian,
    Russian,
    Spanish,
}

/// Wrapps a usable interface around the actual stemmer implementation
pub struct Stemmer {
    stemmer: Box<Fn(&mut SnowballEnv) -> bool>,
}

impl Stemmer {
    /// Create a new stemmer from an algorithm
    pub fn create(lang: Algorithm) -> Self {
        match lang {
            Algorithm::Arabic => Stemmer { stemmer: Box::new(algorithms::arabic::_stem) },
            Algorithm::English => Stemmer { stemmer: Box::new(algorithms::english::_stem) },
            Algorithm::French => Stemmer { stemmer: Box::new(algorithms::french::_stem) },
            Algorithm::German => Stemmer { stemmer: Box::new(algorithms::german::_stem) },
            Algorithm::Italian => Stemmer { stemmer: Box::new(algorithms::italian::_stem) },
            Algorithm::Portuguese => Stemmer { stemmer: Box::new(algorithms::portuguese::_stem) },
            Algorithm::Romanian => Stemmer { stemmer: Box::new(algorithms::romanian::_stem) },
            Algorithm::Russian => Stemmer { stemmer: Box::new(algorithms::russian::_stem) },
            Algorithm::Spanish => Stemmer { stemmer: Box::new(algorithms::spanish::_stem) },
        }
    }

    /// Stem a single word
    /// Please note, that the input is expected to be all lowercase (if that is applicable).
    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current()
    }
}



#[cfg(test)]
mod tests {
    use super::{Stemmer, Algorithm};

    fn stemms_to(lhs: &str, rhs: &str, stemmer: Algorithm) {
        assert_eq!(Stemmer::create(stemmer).stem(lhs), rhs);
    }

    #[test]
    fn german_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_ger.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_ger.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::German);
        }
    }

    #[test]
    fn english_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_en.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_en.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::English);
        }
    }

    #[test]
    fn french_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_fr.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_fr.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::French);
        }
    }

    #[test]
    fn spanish_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_es.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_es.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Spanish);
        }
    }

    #[test]
    fn portuguese_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_pt.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_pt.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Portuguese);
        }
    }

    #[test]
    fn italian_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_it.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_it.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Italian);
        }
    }

    #[test]
    fn romanian_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_ro.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_ro.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Romanian);
        }
    }

    #[test]
    fn russian_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_ru.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_ru.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Russian);
        }
    }

    #[test]
    fn arabic_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_ar.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_ar.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Arabic);
        }
    }

}
