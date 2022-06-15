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
//! fn main() {
//!     let a = A {
//!         x: true,
//!         y: 42,
//!     };
//!     println!("Before: {:?}", a);
//!
//!     let a = econf::load(a, "PREFIX");
//!     println!("After:  {:?}", a);
//! }
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
//! There are some existing crates that provide similar features but `econf` is unique in the following ways:
//!
//! * **Supports nesting:** Supports nested structs in an intutive manner with a little constraint.
//! * **Supports compound types:** Supports `tuple`, `Vec`, `HashMap` and various types.
//! * **Supplemental:** Loads supplementally into existing variables in the code without changing the original logic.
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
//! * Containers: `Vec`, `HashSet`, `HashMap`, `Option`, `BTreeMap`, `BTreeSet`, `BinaryHeap`, `LinkedList`, `VecDeque`
//!     * Containers are parsed as YAML format. See [the tests](./econf/tests/basics.rs).
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
use log::*;
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::Display;
use std::hash::Hash;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::path::PathBuf;
use std::str::FromStr;

pub use econf_derive::LoadEnv;

/// Makes the type loadable from environment variables.
///
/// [`LoadEnv`](econf_derive::LoadEnv) derive macro automatically implements this trait. In the example below,
/// the type `A` implements this trait and can be loaded by [`load`](load) method.
/// Recommends to use the derive macro instead of manually implementing this trait.
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
pub trait LoadEnv
where
    Self: Sized,
{
    fn load(self, path: &str, dup: &mut HashSet<String>) -> Self;
}

fn load_and_map<T, F, E>(value: T, path: &str, dup: &mut HashSet<String>, map: F) -> T
where
    F: FnOnce(&str) -> Result<T, E>,
    E: Display,
{
    let path = path.to_uppercase();

    if dup.get(&path).is_some() {
        warn!("econf: warning: {} is ambiguous", path);
    }
    dup.insert(path.clone());

    match std::env::var(path.clone()) {
        Ok(s) => match map(&s) {
            Ok(v) => {
                info!("econf: loading {}: found {}", path, s);
                v
            }
            Err(e) => {
                error!("econf: loading {}: error on parsing \"{}\": {}", path, s, e);
                value
            }
        },
        Err(_) => {
            info!("econf: loading {}: not found", path);
            value
        }
    }
}

fn load_as_yaml<T>(value: T, path: &str, dup: &mut HashSet<String>) -> T
where
    T: DeserializeOwned,
{
    load_and_map(value, path, dup, |s| serde_yaml::from_str(s))
}

fn load_as_str<T>(value: T, path: &str, dup: &mut HashSet<String>) -> T
where
    T: FromStr,
    T::Err: Display,
{
    load_and_map(value, path, dup, |s| T::from_str(s))
}

macro_rules! impl_load_env {
    ($($t:ident),*) => {$(
        impl LoadEnv for $t {
            fn load(self, path: &str, dup: &mut HashSet<String>) -> Self {
                load_as_str(self, path, dup)
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
            fn load(self, path: &str, dup: &mut HashSet<String>) -> Self {
                load_as_yaml(self, path, dup)
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
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<$($name),*> LoadEnv for ($($name,)*)
            where $($name: DeserializeOwned,)*
        {
            fn load(self, path: &str, dup: &mut HashSet<String>) -> Self {
                load_as_yaml(self, path, dup)
            }
        }
        peel! { $($name,)* }
    )
}

tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }

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
/// fn main() {
///     let a = A {
///         x: true,
///         y: 42,
///     };
///
///     let a = econf::load(a, "FOO");
///     // Here we get `A` with some members overridden by environment variables.
/// }
/// ```
///
pub fn load<T>(data: T, prefix: &str) -> T
where
    T: LoadEnv,
{
    let mut dup = HashSet::new();
    data.load(prefix, &mut dup)
}

impl LoadEnv for std::time::Duration {
    fn load(self, path: &str, dup: &mut HashSet<String>) -> Self {
        load_and_map(self, path, dup, |s| parse_duration::parse(s))
    }
}
