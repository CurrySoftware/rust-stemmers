//! This file was generated automatically by the Snowball to Rust compiler
//! http://snowballstem.org/

use SnowballEnv;
use Among;

static g_v: &'static [u8; 20] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 32, 8];

struct Context {
}

pub fn _stem(env: &mut SnowballEnv) -> bool{
    let mut context = &mut Context{
    };
    println!("_stem: \t\t\t{:?}", env);
    // (, line 13
    // repeat, line 14
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: loop{
            // goto, line 14
            'golab2: loop{
                let v_2 = env.cursor;
                'lab3: loop{
                    // (, line 14
                    if !env.in_grouping(g_v, 97, 252){
                         break 'lab3;
                    }
                    // [, line 15
                    env.bra = env.cursor;
                    // or, line 15
                    'lab4: loop{
                        let v_3 = env.cursor;
                        'lab5: loop{
                            // (, line 15
                            // literal, line 15
                            if !env.eq_s("u"){
                                 break 'lab5;
                            }
                            // ], line 15
                            env.ket = env.cursor;
                            if !env.in_grouping(g_v, 97, 252){
                                 break 'lab5;
                            }
                            // <-, line 15
                            if !env.slice_from("U"){
                                return false;
                            }
                            break 'lab4;
                            break 'lab5;
                        }
                        env.cursor = v_3;;
                        // (, line 16
                        // literal, line 16
                        if !env.eq_s("y"){
                             break 'lab3;
                        }
                        // ], line 16
                        env.ket = env.cursor;
                        if !env.in_grouping(g_v, 97, 252){
                             break 'lab3;
                        }
                        // <-, line 16
                        if !env.slice_from("Y"){
                            return false;
                        }
                        break 'lab4;
                    }
                    env.cursor = v_2;;
                    break 'golab2;
                    break 'lab3;
                }
                env.cursor = v_2;;
                if env.cursor >= env.limit{
                     break 'lab1;
                }
                env.next_char();
            }
            continue 'replab0;
            break 'lab1;
        }
        env.cursor = v_1;;
        break 'replab0;
    }
    return true;
}
