use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use log::{error, info, warn};
use serde::de::DeserializeOwned;

/// Responsible for loading/parsing environment variables.
pub struct Loader {
    paths: HashSet<String>,
}

impl Loader {
    /// Create the instance.
    pub fn new() -> Self {
        Self {
            paths: HashSet::new(),
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
    pub fn is_duplicated(&mut self, path: &str) -> bool {
        !self.paths.insert(path.into())
    }

    /// Loads an environment variable and overrides the original value if the value is successfully loaded.
    ///
    /// The function does the following:
    ///
    /// * Checks duplication (case insensitive)
    /// * Loads a corresponding environment variable (look up `path` as upper-case)
    /// * Calls `map` function to convert the loaded string to a specific type.
    ///
    /// If loading/conversion is successful, the function returns the new value loaded. Otherwise, returns `fallback_value`.
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
    pub fn load_and_map<T, F, E>(&mut self, fallback_value: T, path: &str, map: F) -> T
        where
            F: FnOnce(&str) -> Result<T, E>,
            E: Display,
    {
        let path = path.to_uppercase();

        if self.is_duplicated(&path) {
            warn!("econf: warning: {} is ambiguous", path);
        }

        match std::env::var(&path) {
            Ok(s) => match map(&s) {
                Ok(v) => {
                    info!("econf: loading {}: found {}", path, s);
                    v
                }
                Err(e) => {
                    error!("econf: loading {}: error on parsing \"{}\": {}", path, s, e);
                    fallback_value
                }
            },
            Err(_) => {
                info!("econf: loading {}: not found", path);
                fallback_value
            }
        }
    }

    /// Loads an environment variable in yaml format then deserializes it to a specific type.
    ///
    /// The function is used to load compound types and collections. Since the yaml is the superset of json,
    /// the function is usable to parse json format.
    ///
    /// If loading/conversion is successful, the function returns the new value loaded. Otherwise, returns `fallback_value`.
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
    pub fn load_from_yaml<T>(&mut self, fallback_value: T, path: &str) -> T
        where
            T: DeserializeOwned,
    {
        self.load_and_map(fallback_value, path, |s| serde_yaml::from_str(s))
    }

    /// Loads an environment variable then converts it to a specific type using [`from_str`](std::str::FromStr::from_str).
    ///
    /// If loading/conversion is successful, the function returns the new value loaded. Otherwise, returns `fallback_value`.
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
    pub fn load_from_str<T>(&mut self, fallback_value: T, path: &str) -> T
        where
            T: FromStr,
            T::Err: Display,
    {
        self.load_and_map(fallback_value, path, |s| T::from_str(s))
    }
}