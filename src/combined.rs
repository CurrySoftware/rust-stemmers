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
    // (, line 16
    // test, line 18
    let v_1 = env.cursor;
    // repeat, line 18
    'replab0: loop{
        let v_2 = env.cursor;
        'lab1: loop{
            // (, line 18
            // or, line 21
            'lab2: loop{
                let v_3 = env.cursor;
                'lab3: loop{
                    // (, line 19
                    // [, line 20
                    env.bra = env.cursor;
                    // literal, line 20
                    if !env.eq_s("\u{00DF}"){
                         break 'lab3;
                    }
                    // ], line 20
                    env.ket = env.cursor;
                    // <-, line 20
                    if !env.slice_from("ss"){
                        return false;
                    }
                    break 'lab2;
                    break 'lab3;
                }
                env.cursor = v_3;;
                // next, line 21
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
    // repeat, line 24
    'replab4: loop{
        let v_4 = env.cursor;
        'lab5: loop{
            // goto, line 24
            'golab6: loop{
                let v_5 = env.cursor;
                'lab7: loop{
                    // (, line 24
                    if !env.in_grouping(g_v, 97, 252){
                         break 'lab7;
                    }
                    // [, line 25
                    env.bra = env.cursor;
                    // or, line 25
                    'lab8: loop{
                        let v_6 = env.cursor;
                        'lab9: loop{
                            // (, line 25
                            // literal, line 25
                            if !env.eq_s("u"){
                                 break 'lab9;
                            }
                            // ], line 25
                            env.ket = env.cursor;
                            if !env.in_grouping(g_v, 97, 252){
                                 break 'lab9;
                            }
                            // <-, line 25
                            if !env.slice_from("U"){
                                return false;
                            }
                            break 'lab8;
                            break 'lab9;
                        }
                        env.cursor = v_6;;
                        // (, line 26
                        // literal, line 26
                        if !env.eq_s("y"){
                             break 'lab7;
                        }
                        // ], line 26
                        env.ket = env.cursor;
                        if !env.in_grouping(g_v, 97, 252){
                             break 'lab7;
                        }
                        // <-, line 26
                        if !env.slice_from("Y"){
                            return false;
                        }
                        break 'lab8;
                    }
                    env.cursor = v_5;;
                    break 'golab6;
                    break 'lab7;
                }
                env.cursor = v_5;;
                if env.cursor >= env.limit{
                     break 'lab5;
                }
                env.next_char();
            }
            continue 'replab4;
            break 'lab5;
        }
        env.cursor = v_4;;
        break 'replab4;
    }
    return true;
}
