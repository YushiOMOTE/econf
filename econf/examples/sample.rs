use econf::LoadEnv;
use log::*;
use std::collections::HashMap;

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
struct Y {
    value1: isize,
    value2: bool,
}

#[derive(Default, Debug, LoadEnv)]
struct A {
    value1: bool,
    value2: u64,
    value3: String,
    enum_value: X,
    nested: Y,
    vec: Vec<u64>,
    map: HashMap<String, u32>,
}

fn main() {
    simple_logger::init().unwrap();

    let a = A::default();
    info!("Before loading env: {:?}", a);

    let a = econf::load(a, "app");
    info!("After loading env: {:?}", a);
}
