use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use log::{error, info, warn};
use serde::de::DeserializeOwned;

/// Responsible for loading/parsing environment variables.
pub struct Loader {
    names: HashSet<String>,
}

impl Loader {
    /// Create the instance.
    pub fn new() -> Self {
        Self {
            names: HashSet::new(),
        }
    }

    /// Check the name conflict of environment variables being loaded.
    ///
    /// ```
    /// # use econf::Loader;
    /// let mut loader = Loader::new();
    ///
    /// assert!(!loader.is_duplicated("PREFIX_X"));
    /// assert!(!loader.is_duplicated("PREFIX_Y"));
    /// assert!(loader.is_duplicated("PREFIX_X"));
    /// ```
    ///
    pub fn is_duplicated(&mut self, name: &str) -> bool {
        !self.names.insert(name.into())
    }

    /// Loads an environment variable and converts it to a specific type.
    ///
    /// The function does the following:
    ///
    /// * Checks the duplication of environment variable names loaded so far (case insensitive)
    /// * Loads the environment variable (look up `name` as upper-case)
    /// * Calls `map` function to convert the loaded string to a specific type.
    ///
    /// If loading/conversion is successful, the function returns the new value loaded. Otherwise, returns `fallback`.
    ///
    /// ```
    /// # use econf::Loader;
    /// let mut loader = Loader::new();
    ///
    /// std::env::set_var("BAR", "2");
    /// std::env::set_var("BUZZ", "A");
    ///
    /// assert_eq!(loader.load_and_map(1, "FOO", |v| v.parse()), 1);
    /// assert_eq!(loader.load_and_map(1, "BAR", |v| v.parse()), 2);
    /// assert_eq!(loader.load_and_map(1, "BUZZ", |v| v.parse()), 1);
    /// ```
    ///
    pub fn load_and_map<T, F, E>(&mut self, fallback: T, name: &str, map: F) -> T
        where
            F: FnOnce(&str) -> Result<T, E>,
            E: Display,
    {
        let name = name.to_uppercase();

        if self.is_duplicated(&name) {
            warn!("econf: warning: {} is ambiguous", name);
        }

        match std::env::var(&name) {
            Ok(s) => match map(&s) {
                Ok(v) => {
                    info!("econf: loading {}: found {}", name, s);
                    v
                }
                Err(e) => {
                    error!("econf: loading {}: error on parsing \"{}\": {}", name, s, e);
                    fallback
                }
            },
            Err(_) => {
                info!("econf: loading {}: not found", name);
                fallback
            }
        }
    }

    /// Loads an environment variable in yaml format then deserializes it to a specific type.
    ///
    /// The function is used to load compound types and collections. Since the yaml is the superset of json,
    /// the function is usable to parse json format.
    ///
    /// If loading/conversion is successful, the function returns the new value loaded. Otherwise, returns `fallback`.
    ///
    /// ```
    /// # use econf::Loader;
    /// # use std::collections::HashMap;
    /// let mut loader = Loader::new();
    ///
    /// std::env::set_var("FOO", "[2, 2, 3]");
    /// std::env::set_var("BAR", "{1: 3, 2: 2}");
    /// std::env::set_var("BUZZ", "broken");
    ///
    /// assert_eq!(loader.load_from_yaml(vec![1usize, 2, 3], "FOO"), vec![2, 2, 3]);
    /// assert_eq!(loader.load_from_yaml(vec![1usize, 2, 3], "FOO2"), vec![1, 2, 3]);
    /// assert_eq!(loader.load_from_yaml(HashMap::from([(1usize, 1usize), (2, 2)]), "BAR"), HashMap::from([(1, 3), (2, 2)]));
    /// assert_eq!(loader.load_from_yaml(HashMap::from([(1usize, 1usize), (2, 2)]), "BAR2"), HashMap::from([(1, 1), (2, 2)]));
    /// assert_eq!(loader.load_from_yaml(vec![1usize, 2, 3], "BUZZ"), vec![1, 2, 3]);
    /// ```
    ///
    pub fn load_from_yaml<T>(&mut self, fallback: T, name: &str) -> T
        where
            T: DeserializeOwned,
    {
        self.load_and_map(fallback, name, |s| serde_yaml::from_str(s))
    }

    /// Loads an environment variable then converts it to a specific type using [`from_str`](std::str::FromStr::from_str).
    ///
    /// If loading/conversion is successful, the function returns the new value loaded. Otherwise, returns `fallback`.
    ///
    /// ```
    /// # use econf::Loader;
    /// # use std::collections::HashMap;
    /// let mut loader = Loader::new();
    ///
    /// std::env::set_var("FOO", "2");
    /// std::env::set_var("BAR", "true");
    /// std::env::set_var("BUZZ", "A");
    ///
    /// assert_eq!(loader.load_from_str(1, "FOO"), 2);
    /// assert_eq!(loader.load_from_str(1, "FOO2"), 1);
    /// assert_eq!(loader.load_from_str(false, "BAR"), true);
    /// assert_eq!(loader.load_from_str(false, "BAR2"), false);
    /// assert_eq!(loader.load_from_str(1, "BUZZ"), 1);
    /// ```
    ///
    pub fn load_from_str<T>(&mut self, fallback: T, name: &str) -> T
        where
            T: FromStr,
            T::Err: Display,
    {
        self.load_and_map(fallback, name, |s| T::from_str(s))
    }
}
