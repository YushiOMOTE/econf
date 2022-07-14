# econf

Loads environment variables into your structs in one shot.

[![Latest version](https://img.shields.io/crates/v/econf.svg)](https://crates.io/crates/econf)
[![Documentation](https://docs.rs/econf/badge.svg)](https://docs.rs/econf)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Actions Status](https://github.com/YushiOMOTE/econf/workflows/test/badge.svg)](https://github.com/YushiOMOTE/econf/actions)

![](https://github.com/YushiOMOTE/econf/blob/master/assets/logo.png?raw=true)

`econf` allows to override struct fields with environment variables easily. This is useful to build up applications that optionally overrides some configuration with environment variables. Here is the basic usage:

```rust
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

    let a = econf::load(a, "PREFIX");
    println!("After:  {:?}", a);
}
```

```sh
$ ./app
Before: A { x: true, y: 42 }
After:  A { x: true, y: 42 }

$ PREFIX_X=false ./app
Before: A { x: true, y: 42 }
After:  A { x: false, y: 42 }
```

In this example,

* `PREFIX_X` is loaded to `x`
* `PREFIX_Y` is loaded to `y`

The environment variables are all upper-case with `_` separated.

## Why econf?

There are some existing crates that provide similar features but `econf` is unique in the following ways:

* **Supports nesting:** Supports nested structs in an intutive manner with a little constraint.
* **Supports containers:** Supports `Vec`, `HashMap` and various types.
* **Supplemental:** Loads into existing variables in the code without changing the original logic.
* **Contributor friendly:** Simple code base. Comprehensible with a little study on basic macro usage.

## Supported types

* Boolean: `bool`
* Integer: `isize`, `usize`, `i8`, `i16`,`i32`,`i64`,`i128`, `u8`,`u16`,`u32`,`u64`,`u128`
* String: `char`, `String`
* Float: `f32`, `f64`
* Network: `IpAddr`,`Ipv4Addr`,`Ipv6Addr`,`SocketAddr`,`SocketAddrV4`,`SocketAddrV6`
* Non-zero types: `NonZeroI128`,`NonZeroI16`,`NonZeroI32`,`NonZeroI64`,`NonZeroI8`,`NonZeroIsize`,`NonZeroU128`, `NonZeroU16`,`NonZeroU32`,`NonZeroU64`,`NonZeroU8`, `NonZeroUsize`
* File system: `PathBuf`
* Containers: `Vec`, `HashSet`, `HashMap`, `Option`, `BTreeMap`, `BTreeSet`, `BinaryHeap`, `LinkedList`, `VecDeque`, `tuple`
    * Containers are parsed as YAML format. See [the tests](https://github.com/YushiOMOTE/econf/blob/master/econf/tests/basics.rs).

## Nesting

Nested structs are supported.

```rust
#[derive(LoadEnv)]
struct A {
    v1: usize,
    v2: B,
}

#[derive(LoadEnv)]
struct B {
    v1: usize,
    v2: usize,
}

fn main() {
    let a = A {
        v1: 1,
        v2: B {
            v1: 2,
            v2: 3,
        },
    };

    let a = econf::load(a, "PREFIX");
}
```

In this example,

* `PREFIX_V1` is loaded to `a.v1`
* `PREFIX_V2_V1` is loaded to `a.v2.v1`
* `PREFIX_V2_V2` is loaded to `a.v2.v2`

Fields in child structs can be specified by chaining the field names with `_` as a separator.
However, there're cases that names conflict. For example,

```rust
#[derive(LoadEnv)]
struct A {
    v2_v1: usize,
    v2: B,
}

#[derive(LoadEnv)]
struct B {
    v1: usize,
    v2: usize,
}

fn main() {
    let a = A {
        v2_v1: 1,
        v2: B {
            v1: 2,
            v2: 3,
        },
    };

    let a = econf::load(a, "PREFIX");
}
```

Here `PREFIX_V2_V1` corresponds to both `a.v2_v1` and `a.v2.v1`. In this case, `econf` prints warning through [`log facade`](https://docs.rs/log/latest/log/) and the value is loaded to both `a.v2_v1` and `a.v2.v1`.

## Skipping fields

Fields that do not implement LoadEnv or simply should not be loaded by econf can be skipped by adding the `#[econf(skip)]` helper attribute:

```rust
#[derive(LoadEnv)]
struct A {
    x: bool,
    #[econf(skip)]
    y: u64, // will not be loaded by econf
}
```

## Renaming fields

Load a field with the given name instead of its Rust's field name. This is helpful if the environment variable name and Rust's field name don't match:

```
#[derive(LoadEnv)]
struct A {
    x: bool,
    #[econf(rename = "ANOTHER_Y")]
    y: u64, // will be loaded from an environment variable `ANOTHER_Y`
}
```

License: MIT
