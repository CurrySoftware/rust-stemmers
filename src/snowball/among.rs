use snowball::SnowballEnv;

pub struct Among(pub &'static str, pub i32, pub i32, pub Option<Box<Fn(&SnowballEnv) -> bool + Sync>>);
