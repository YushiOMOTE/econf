use econf::LoadEnv;
use log::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, strum::EnumString, LoadEnv)]
enum X {
    V1,
    V2,
    V3,
}

impl Default for X {
    fn default() -> X {
        X::V1
    }
}

#[derive(Default, Debug, LoadEnv)]
struct B(u32, #[econf(skip)] bool);

#[derive(Default, Debug, LoadEnv)]
struct C;

#[derive(Default, Debug, LoadEnv)]
struct D {
    value1: String,
    value2: bool,
    #[econf(skip)]
    value3: u32,
}

#[derive(Default, Debug, LoadEnv)]
struct E {
    ambiguous: u32,
}

#[derive(Default, Debug, LoadEnv)]
struct A {
    value1: bool,
    value2: char,
    value3: f32,
    value4: f64,
    value5: isize,
    value6: i8,
    value7: i16,
    value8: i32,
    value9: i64,
    value11: usize,
    value12: u8,
    value13: u16,
    value14: u32,
    value15: u64,
    value17: String,
    tuple_struct: B,
    unit: C,
    nested: D,
    prefix: E,
    prefix_ambiguous: bool,
    enum_value: X,
    vec: Vec<u64>,
    set: HashSet<String>,
    map: HashMap<String, u32>,
    tup1: (u32, String),
    tup2: (u32, u64, String),
}

fn main() {
    simple_logger::init().unwrap();

    let a = A::default();
    info!("Before loading env: {:#?}", a);

    let a = econf::load(a, "app");
    info!("After loading env: {:#?}", a);
}
