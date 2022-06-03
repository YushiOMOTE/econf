use econf::LoadEnv;

#[derive(Debug, LoadEnv)]
struct A {
    x: bool,
    y: u64,
}

fn main() {
    let a = A { x: true, y: 42 };
    println!("Before loading env: {:?}", a);

    let a = econf::load(a, "app");
    println!("After loading env: {:?}", a);
}
