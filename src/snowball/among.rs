use SnowballEnv;

pub struct Among(&'static str, i32, i32, Option<Box<Fn(&SnowballEnv) -> bool + Sync>>);
