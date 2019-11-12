//! This file was generated automatically by the Snowball to Rust compiler
//! Snowball 2.0.0 - https://snowballstem.org/

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 29] = &[
    Among("a", -1, 1, None),
    Among("e", -1, 1, None),
    Among("ede", 1, 1, None),
    Among("ande", 1, 1, None),
    Among("ende", 1, 1, None),
    Among("ane", 1, 1, None),
    Among("ene", 1, 1, None),
    Among("hetene", 6, 1, None),
    Among("erte", 1, 3, None),
    Among("en", -1, 1, None),
    Among("heten", 9, 1, None),
    Among("ar", -1, 1, None),
    Among("er", -1, 1, None),
    Among("heter", 12, 1, None),
    Among("s", -1, 2, None),
    Among("as", 14, 1, None),
    Among("es", 14, 1, None),
    Among("edes", 16, 1, None),
    Among("endes", 16, 1, None),
    Among("enes", 16, 1, None),
    Among("hetenes", 19, 1, None),
    Among("ens", 14, 1, None),
    Among("hetens", 21, 1, None),
    Among("ers", 14, 1, None),
    Among("ets", 14, 1, None),
    Among("et", -1, 1, None),
    Among("het", 25, 1, None),
    Among("ert", -1, 3, None),
    Among("ast", -1, 1, None),
];

static A_1: &'static [Among<Context>; 2] = &[
    Among("dt", -1, -1, None),
    Among("vt", -1, -1, None),
];

static A_2: &'static [Among<Context>; 11] = &[
    Among("leg", -1, 1, None),
    Among("eleg", 0, 1, None),
    Among("ig", -1, 1, None),
    Among("eig", 2, 1, None),
    Among("lig", 2, 1, None),
    Among("elig", 4, 1, None),
    Among("els", -1, 1, None),
    Among("lov", -1, 1, None),
    Among("elov", 7, 1, None),
    Among("slov", 7, 1, None),
    Among("hetslov", 9, 1, None),
];

static G_v: &'static [u8; 19] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 128];

static G_s_ending: &'static [u8; 4] = &[119, 125, 149, 1];

#[derive(Clone)]
struct Context {
    i_x: usize,
    i_p1: usize,
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    let v_1 = env.cursor;
    let c = env.byte_index_for_hop(3);
    if 0 as i32 > c || c > env.limit as i32 {
        return false;
    }
    env.cursor = c as usize;
    context.i_x = env.cursor;
    env.cursor = v_1;
    'golab0: loop {
        let v_2 = env.cursor;
        'lab1: loop {
            if !env.in_grouping(G_v, 97, 248) {
                break 'lab1;
            }
            env.cursor = v_2;
            break 'golab0;
        }
        env.cursor = v_2;
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    'golab2: loop {
        'lab3: loop {
            if !env.out_grouping(G_v, 97, 248) {
                break 'lab3;
            }
            break 'golab2;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    context.i_p1 = env.cursor;
    'lab4: loop {
        if !(context.i_p1 < context.i_x){
            break 'lab4;
        }
        context.i_p1 = context.i_x;
        break 'lab4;
    }
    return true;
}

fn r_main_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        env.limit_backward = v_2;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_2;
    if among_var == 1 {
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        'lab0: loop {
            let v_3 = env.limit - env.cursor;
            'lab1: loop {
                if !env.in_grouping_b(G_s_ending, 98, 122) {
                    break 'lab1;
                }
                break 'lab0;
            }
            env.cursor = env.limit - v_3;
            if !env.eq_s_b(&"k") {
                return false;
            }
            if !env.out_grouping_b(G_v, 97, 248) {
                return false;
            }
            break 'lab0;
        }
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 3 {
        if !env.slice_from("er") {
            return false;
        }
    }
    return true;
}

fn r_consonant_pair(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_3 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if env.find_among_b(A_1, context) == 0 {
        env.limit_backward = v_3;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_3;
    env.cursor = env.limit - v_1;
    if env.cursor <= env.limit_backward {
        return false;
    }
    env.previous_char();
    env.bra = env.cursor;
    if !env.slice_del() {
        return false;
    }
    return true;
}

fn r_other_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_p1 {
        return false;
    }

    let v_2 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;

    if env.find_among_b(A_2, context) == 0 {
        env.limit_backward = v_2;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_2;

    if !env.slice_del() {
        return false;
    }

    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_x: 0,
        i_p1: 0,
    };

    let v_1 = env.cursor;
    r_mark_regions(env, context);
    env.cursor = v_1;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;

    let v_2 = env.limit - env.cursor;
    r_main_suffix(env, context);
    env.cursor = env.limit - v_2;

    let v_3 = env.limit - env.cursor;
    r_consonant_pair(env, context);
    env.cursor = env.limit - v_3;

    let v_4 = env.limit - env.cursor;
    r_other_suffix(env, context);
    env.cursor = env.limit - v_4;
    env.cursor = env.limit_backward;

    return true;
}
