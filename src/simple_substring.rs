//! This file was generated automatically by the Snowball to Rust compiler
//! http://snowballstem.org/

use SnowballEnv;
use Among;

static a_0: &'static [Among; 4] = &[
    Among("s", -1, 3, None),
    Among("ies", 0, 2, None),
    Among("sses", 0, 1, None),
    Among("ss", 0, -1, None)
];

struct Context {
}

pub fn _stem(env: &mut SnowballEnv) -> bool{
    let mut context = &mut Context{
    };
    println!("_stem: \t\t\t{:?}", env);
    // (, line 5
    // [, line 6
    env.ket = env.cursor;
    // substring, line 6
    let among_var = env.find_among_b(a_0);
    if among_var == 0{
        return false
    }
    // ], line 6
    env.bra = env.cursor;
    if among_var == 0{
        return false
}

    else if among_var == 1{
        // (, line 7
        // <-, line 7
        if !env.slice_from("ss"){
            return false;
        }
    }
    else if among_var == 2{
        // (, line 8
        // <-, line 8
        if !env.slice_from("i"){
            return false;
        }
    }
    else if among_var == 3{
        // (, line 10
        // delete, line 10
        if !env.slice_del(){
            return false;
}
    }
    return true;
}
