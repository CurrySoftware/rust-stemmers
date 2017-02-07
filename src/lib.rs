use std::borrow::Cow;

mod snowball;

use snowball::SnowballEnv;


pub enum Stemmers {
    Arabic,
    English,
    French,
    German,
    Italian,
    Portuguese,
    Romanian,
    Spanish
}

pub struct Stemmer{
    stemmer: Box<Fn(&mut SnowballEnv) -> bool>
}

impl Stemmer{
    pub fn create(lang: Stemmers) -> Self {
        match lang {
            Stemmers::Arabic => Stemmer{ stemmer: Box::new(snowball::algorithms::arabic::_stem)},
            Stemmers::English => Stemmer{ stemmer: Box::new(snowball::algorithms::english::_stem)},
            Stemmers::French => Stemmer{ stemmer: Box::new(snowball::algorithms::french::_stem)},
            Stemmers::German => Stemmer{ stemmer: Box::new(snowball::algorithms::german::_stem)},
            Stemmers::Italian => Stemmer{ stemmer: Box::new(snowball::algorithms::italian::_stem)},
            Stemmers::Portuguese => Stemmer{ stemmer: Box::new(snowball::algorithms::portuguese::_stem)},
            Stemmers::Romanian => Stemmer{ stemmer: Box::new(snowball::algorithms::romanian::_stem)},
            Stemmers::Spanish => Stemmer{ stemmer: Box::new(snowball::algorithms::spanish::_stem)},            
        }
    }

    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current()
    }
}



#[cfg(test)]
mod tests {
    use snowball::SnowballEnv;

    fn stemms_to<F: Fn(&mut SnowballEnv) -> bool>(lhs: &str, rhs: &str, stemmer: F) {
        let mut env = SnowballEnv::create(lhs);
        println!("{:?} -> {:?}", lhs, rhs);
        stemmer(&mut env);
        assert_eq!(env.get_current(), rhs);
    }

    #[test]
    fn german_test() {
        use snowball::algorithms::german::_stem;
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("voc_ger.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("res_ger.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(), res.unwrap().as_str(), _stem);
        }
    }

    #[test]
    fn english_test() {
        use english::_stem;
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("voc_en.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("res_en.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(), res.unwrap().as_str(), _stem);
        }
    }

    #[test]
    fn french_test() {
        use french::_stem;
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("voc_fr.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("res_fr.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(), res.unwrap().as_str(), _stem);
        }
    }

    #[test]
    fn spanish_test() {
        use spanish::_stem;
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("voc_es.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("res_es.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(), res.unwrap().as_str(), _stem);
        }
    }

    #[test]
    fn portuguese_test() {
        use portuguese::_stem;
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("voc_pt.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("res_pt.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(), res.unwrap().as_str(), _stem);
        }
    }

    #[test]
    fn italian_test() {
        use italian::_stem;
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("voc_it.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("res_it.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(), res.unwrap().as_str(), _stem);
        }
    }

    #[test]
    fn romanian_test() {
        use romanian::_stem;
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("voc_ro.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("res_ro.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(), res.unwrap().as_str(), _stem);
        }
    }

    #[test]
    fn arabic_test() {
        use arabic::_stem;
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("voc_ar.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("res_ar.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(), res.unwrap().as_str(), _stem);
        }
    }

}
