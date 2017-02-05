//! This file was generated automatically by the Snowball to Rust compiler
//! http://snowballstem.org/

use SnowballEnv;
use Among;

struct Context {
}

pub fn _stem(env: &mut SnowballEnv) -> bool{
    let mut context = &mut Context{
    };
    println!("_stem: \t\t\t{:?}", env);
    // (, line 7
    // test, line 8
    let v_1 = env.cursor;
    // repeat, line 8
    'replab0: loop{
        let v_2 = env.cursor;
        'lab1: loop{
            // (, line 8
            // or, line 11
            'lab2: loop{
                let v_3 = env.cursor;
                'lab3: loop{
                    // (, line 9
                    // [, line 10
                    env.bra = env.cursor;
                    // literal, line 10
                    if !env.eq_s("\u{00DF}"){
                         break 'lab3;
                    }
                    // ], line 10
                    env.ket = env.cursor;
                    // <-, line 10
                    if !env.slice_from("ss"){
                        return false;
                    }
                    break 'lab2;
                    break 'lab3;
                }
                env.cursor = v_3;;
                // next, line 11
                if env.cursor >= env.limit{
                     break 'lab1;
                }
                env.next_char();
                break 'lab2;
            }
            continue 'replab0;
            break 'lab1;
        }
        env.cursor = v_2;;
        break 'replab0;
    }
    env.cursor = v_1;;
    return true;
}
