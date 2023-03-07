#![allow(unused_imports, dead_code, non_snake_case)]

// {{{ use
use itertools::Itertools;
use num::{
    abs,
    integer::{gcd, lcm, Roots},
    pow, range_step, range_step_inclusive,
};
use petgraph::{
    algo::{bellman_ford, dijkstra},
    graph::{node_index, DiGraph, UnGraph},
    unionfind::UnionFind,
    visit::{depth_first_search, DfsEvent},
};
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::mem::swap;
use superslice::Ext;
// }}}

// {{{ const
const MOD: usize = 1_000_000_007;
// const MOD: usize = 998_244_353;
const XY2: [(i64, i64); 2] = [(0, 1), (1, 0)];
const XY4: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const XY8: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
// }}}

// {{{ macro
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[allow(unused_macros)]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = my_min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = my_max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
macro_rules! my_min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, my_min!($($rest),+))
    }};
}

#[allow(unused_macros)]
macro_rules! my_max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, my_max!($($rest),+))
    }};
}

#[allow(unused_macros)]
macro_rules! mat {
    ($($e:expr),*) => { Vec::from(vec![$($e),*]) };
    ($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
    ($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
    ($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}
// }}}

fn main() {
    input! {
        
    }
}
