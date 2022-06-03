# econf

Loads environment variables into your structs in one shot.

[![Latest version](https://img.shields.io/crates/v/econf.svg)](https://crates.io/crates/econf)
[![Documentation](https://docs.rs/econf/badge.svg)](https://docs.rs/econf)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Actions Status](https://github.com/YushiOMOTE/econf/workflows/test/badge.svg)](https://github.com/YushiOMOTE/econf/actions)

![](https://github.com/YushiOMOTE/econf/blob/master/assets/logo.png?raw=true)

`econf` allows to override struct fields with environment variables easily. This is useful to build up applications that optionally overrides some configuration with environment variables. Here is the basic usage:

``` rust
use econf::LoadEnv;

#[derive(Debug, LoadEnv)]
struct A {
    x: bool,
    y: u64,
}

fn main() {
    let a = A {
        x: true,
        y: 42,
    };
    println!("Before: {:?}", a);

    let a = econf::load(a, "app");
    println!("After:  {:?}", a);
}
```

``` sh
$ ./app
Before: A { x: true, y: 42 }
After:  A { x: true, y: 42 }

$ APP_X=false ./app
Before: A { x: true, y: 42 }
After:  A { x: false, y: 42 }
```

There are some existing crates that provide similar features but `econf` is unique in the following ways:

* **Supports nesting:** Supports nested structs in an intutive manner.
* **Supports compound types:** Supports `tuple`, `array`, `Vec`, `HashMap` and various types.
* **Supplemental:** Loads supplementally into existing variables in the code without changing the original logic.
* **Maintainer friendly:** Simple code base. Comprehensible with a little study on basic macro usage.
