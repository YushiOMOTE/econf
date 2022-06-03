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
use std::str::FromStr;

#[doc = include_str!("../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub use econf_derive::LoadEnv;

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
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
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

pub fn load<T>(d: T, prefix: &str) -> T
where
    T: LoadEnv,
{
    let mut dup = HashSet::new();
    d.load(prefix, &mut dup)
}

impl LoadEnv for std::time::Duration {
    fn load(self, path: &str, dup: &mut HashSet<String>) -> Self {
        load_and_map(self, path, dup, |s| parse_duration::parse(s))
    }
}
