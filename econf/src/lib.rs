//! Loads environment variables into your structs in one shot.
//!
//! [![Latest version](https://img.shields.io/crates/v/econf.svg)](https://crates.io/crates/econf)
//! [![Documentation](https://docs.rs/econf/badge.svg)](https://docs.rs/econf)
//! [![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
//! [![Actions Status](https://github.com/YushiOMOTE/econf/workflows/test/badge.svg)](https://github.com/YushiOMOTE/econf/actions)
//!
//! ![](https://github.com/YushiOMOTE/econf/blob/master/assets/logo.png?raw=true)
//!
//! `econf` allows to override struct fields with environment variables easily. This is useful to build up applications that optionally overrides some configuration with environment variables. Here is the basic usage:
//!
//! ```
//! use econf::LoadEnv;
//!
//! #[derive(Debug, LoadEnv)]
//! struct A {
//!     x: bool,
//!     y: u64,
//! }
//!
//! let a = A {
//!     x: true,
//!     y: 42,
//! };
//! println!("Before: {:?}", a);
//!
//! let a = econf::load(a, "PREFIX");
//! println!("After:  {:?}", a);
//! ```
//!
//! ```sh
//! $ ./app
//! Before: A { x: true, y: 42 }
//! After:  A { x: true, y: 42 }
//!
//! $ PREFIX_X=false ./app
//! Before: A { x: true, y: 42 }
//! After:  A { x: false, y: 42 }
//! ```
//!
//! In this example,
//!
//! * `PREFIX_X` is loaded to `x`
//! * `PREFIX_Y` is loaded to `y`
//!
//! The environment variables are all upper-case with `_` separated.
//!
//! # Why econf?
//!
//! There are some existing crates that provide similar features but `econf` is unique in the following ways:
//!
//! * **Supports nesting:** Supports nested structs in an intutive manner with a little constraint.
//! * **Supports containers:** Supports `Vec`, `HashMap` and various types.
//! * **Supplemental:** Loads into existing variables in the code without changing the original logic.
//! * **Contributor friendly:** Simple code base. Comprehensible with a little study on basic macro usage.
//!
//! # Supported types
//!
//! * Boolean: `bool`
//! * Integer: `isize`, `usize`, `i8`, `i16`,`i32`,`i64`,`i128`, `u8`,`u16`,`u32`,`u64`,`u128`
//! * String: `char`, `String`
//! * Float: `f32`, `f64`
//! * Network: `IpAddr`,`Ipv4Addr`,`Ipv6Addr`,`SocketAddr`,`SocketAddrV4`,`SocketAddrV6`
//! * Non-zero types: `NonZeroI128`,`NonZeroI16`,`NonZeroI32`,`NonZeroI64`,`NonZeroI8`,`NonZeroIsize`,`NonZeroU128`, `NonZeroU16`,`NonZeroU32`,`NonZeroU64`,`NonZeroU8`, `NonZeroUsize`
//! * File system: `PathBuf`
//! * Containers: `Vec`, `HashSet`, `HashMap`, `Option`, `BTreeMap`, `BTreeSet`, `BinaryHeap`, `LinkedList`, `VecDeque`, `tuple`
//!     * Containers are parsed as YAML format. See [the tests](https://github.com/YushiOMOTE/econf/blob/master/econf/tests/basics.rs).
//!
//! # Enums
//!
//! Since v0.3.0, econf requires enums to implement [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait. Without this implementation, your program will fail to compile. While you can write the `FromStr` implementation manually, you can alternatively use [strum](https://github.com/Peternator7/strum) crate to automatically generate it. `strum` provides several useful features, making it a generally recommended choice. See [econf/examples/strum.rs](https://github.com/YushiOMOTE/econf/tree/master/econf/examples/strum.rs) for example code.
//!
//! ```
//! use econf::LoadEnv;
//!
//! #[derive(Debug, strum::EnumString, LoadEnv)]
//! #[strum(serialize_all = "kebab-case")]
//! enum AuthMode {
//!     ApiKey,
//!     BasicAuth,
//!     #[strum(ascii_case_insensitive)]
//!     BearerToken,
//!     #[strum(serialize = "oauth", serialize = "OAuth")]
//!     OAuth,
//!     JWT,
//! }
//! ```
//!
//! # Nesting
//!
//! Nested structs are supported.
//!
//! ```
//! # use econf::LoadEnv;
//! #[derive(LoadEnv)]
//! struct A {
//!     v1: usize,
//!     v2: B,
//! }
//!
//! #[derive(LoadEnv)]
//! struct B {
//!     v1: usize,
//!     v2: usize,
//! }
//!
//! let a = A {
//!     v1: 1,
//!     v2: B {
//!         v1: 2,
//!         v2: 3,
//!     },
//! };
//!
//! let a = econf::load(a, "PREFIX");
//! ```
//!
//! In this example,
//!
//! * `PREFIX_V1` is loaded to `a.v1`
//! * `PREFIX_V2_V1` is loaded to `a.v2.v1`
//! * `PREFIX_V2_V2` is loaded to `a.v2.v2`
//!
//! Fields in child structs can be specified by chaining the field names with `_` as a separator.
//! However, there're cases that names conflict. For example,
//!
//! ```
//! # use econf::LoadEnv;
//! #[derive(LoadEnv)]
//! struct A {
//!     v2_v1: usize,
//!     v2: B,
//! }
//!
//! #[derive(LoadEnv)]
//! struct B {
//!     v1: usize,
//!     v2: usize,
//! }
//!
//! let a = A {
//!     v2_v1: 1,
//!     v2: B {
//!         v1: 2,
//!         v2: 3,
//!     },
//! };
//!
//! let a = econf::load(a, "PREFIX");
//! ```
//!
//! Here `PREFIX_V2_V1` corresponds to both `a.v2_v1` and `a.v2.v1`. In this case, `econf` prints warning through [`log facade`](https://docs.rs/log/latest/log/) and the value is loaded to both `a.v2_v1` and `a.v2.v1`.
//!
//! # Skipping fields
//!
//! Fields that do not implement LoadEnv or simply should not be loaded by econf can be skipped by adding the `#[econf(skip)]` helper attribute:
//!
//! ```
//! # use econf::LoadEnv;
//! #[derive(LoadEnv)]
//! struct A {
//!     x: bool,
//!     #[econf(skip)]
//!     y: u64, // will not be loaded by econf
//! }
//! ```
//!
//! # Renaming fields
//!
//! Load a field with the given name instead of its Rust's field name. This is helpful if the environment variable name and Rust's field name don't match:
//!
//! ```
//! # use econf::LoadEnv;
//! #[derive(LoadEnv)]
//! struct A {
//!     x: bool,
//!     #[econf(rename = "ANOTHER_Y")]
//!     y: u64, // will be loaded from an environment variable `ANOTHER_Y`
//! }
//! ```
//!
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::path::PathBuf;

use serde::de::DeserializeOwned;

pub use econf_derive::LoadEnv;

pub use crate::loader::Loader;

mod loader;

/// Makes the type loadable from environment variables.
///
/// [`LoadEnv`](econf_derive::LoadEnv) derive macro automatically implements this trait. Therefore, usually no need to implement this trait manually.
/// In the example below, the type `A` implements this trait and can be loaded by [`load`](load) method.
///
/// ```rust
/// # use econf::LoadEnv;
/// #
/// #[derive(LoadEnv)]
/// struct A {
///     x: bool,
///     y: u64,
/// }
/// ```
///
/// The trait can be manually implemented.
///
/// ```
/// use econf::{LoadEnv, Loader};
///
/// struct A {
///     x: bool,
/// }
///
/// impl LoadEnv for A {
///
///     fn load(self, path: &str, _loader: &mut Loader) -> Self {
///         match std::env::var(path) {
///             Ok(s) if s == "POSITIVE" => A { x: true },
///             Ok(s) if s == "NEGATIVE" => A { x: false },
///             Ok(_) | Err(_) => self,
///         }
///     }
///
/// }
/// ```
///
/// `path` is the environment variable name to be loaded.
/// Return a new value to override the original value (the original value is `self`).
/// Return `self` to use the original value.
///
pub trait LoadEnv
where
    Self: Sized,
{
    fn load(self, path: &str, loader: &mut Loader) -> Self;
}

macro_rules! impl_load_env {
    ($($t:ident),*) => {$(
        impl LoadEnv for $t {
            fn load(self, path: &str, loader: &mut Loader) -> Self {
                loader.load_from_str(self, path)
            }
        }
    )*}
}

impl_load_env! {
    bool, char, String,
    f32, f64,
    isize, usize,
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6,
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, PathBuf
}

macro_rules! impl_load_env_containers {
    ($( $t:ident<$( $p:ident : $tb1:ident $(+ $tb2:ident)* ),*> ),*) => {$(
        impl<$($p),*> LoadEnv for $t<$($p),*>
        where $( $p : $tb1 $(+ $tb2)* ),*
        {
            fn load(self, path: &str, loader: &mut Loader) -> Self {
                loader.load_from_yaml(self, path)
            }
        }
    )*}
}

impl_load_env_containers! {
    Vec<T: DeserializeOwned>,
    HashSet<T: Eq + Hash + DeserializeOwned>,
    HashMap<K: Eq + Hash + DeserializeOwned, V: DeserializeOwned>,
    Option<T: DeserializeOwned>,
    BTreeMap<K: Ord + DeserializeOwned, V: DeserializeOwned>,
    BTreeSet<T: Ord + DeserializeOwned>,
    BinaryHeap<T: Ord + DeserializeOwned>,
    LinkedList<T: DeserializeOwned>,
    VecDeque<T: DeserializeOwned>
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (impl_load_env_tuples! { $($other,)* })
}

macro_rules! impl_load_env_tuples {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<$($name),*> LoadEnv for ($($name,)*)
            where $($name: DeserializeOwned,)*
        {
            fn load(self, path: &str, loader: &mut Loader) -> Self {
                loader.load_from_yaml(self, path)
            }
        }
        peel! { $($name,)* }
    )
}

impl_load_env_tuples! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }

/// Load environment variables to a struct.
///
/// The member variables in struct `data` will be overridden by environment variables
/// that start with the prefix `prefix`. In the example below,
///
/// * `FOO_X` is loaded to `x`.
/// * `FOO_Y` is loaded to `y`.
///
///
/// ```rust
/// # use econf::LoadEnv;
/// #
/// #[derive(Debug, LoadEnv)]
/// struct A {
///     x: bool,
///     y: u64,
/// }
///
/// let a = A {
///     x: true,
///     y: 42,
/// };
///
/// let a = econf::load(a, "FOO");
/// // Here we get `A` with some members overridden by environment variables.
/// ```
///
pub fn load<T>(data: T, prefix: &str) -> T
where
    T: LoadEnv,
{
    let mut loader = Loader::new();
    data.load(prefix, &mut loader)
}

impl LoadEnv for std::time::Duration {
    fn load(self, path: &str, loader: &mut Loader) -> Self {
        loader.load_and_map(self, path, humantime::parse_duration)
    }
}
