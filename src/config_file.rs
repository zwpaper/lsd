use crate::print_error;
///! This module provides methods to handle the program's config files and operations related to
///! this.
use std::path::PathBuf;

use serde::Deserialize;
use serde_yaml::Sequence;

use std::fs;

const CONF_DIR: &str = "lsd";
const CONF_FILE_NAME: &str = "config";
const YAML_LONG_EXT: &str = "yaml";

/// A struct to hold an optional file path [String] and an optional [Yaml], and provides methods
/// around error handling in a config file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub classic: Option<bool>,
    pub blocks: Option<Sequence>,
    pub color: Option<Color>,
    pub date: Option<String>, // enum?
    pub dereference: Option<bool>,
    pub display: Option<String>, // enum?
    pub icons: Option<Icons>,
    pub ignore_globs: Option<Sequence>,
    pub indicators: Option<bool>,
    pub layout: Option<String>, // enum?
    pub recursion: Option<Recursion>,
    pub size: Option<String>, // enum?
    pub sorting: Option<Sorting>,
    pub no_symlink: Option<bool>,
    pub total_size: Option<bool>,
    pub symlink_arrow: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Color {
    pub when: String, // enum?
}

#[derive(Debug, Deserialize)]
pub struct Icons {
    pub when: Option<String>, // enum?
    pub theme: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Recursion {
    pub enabled: Option<bool>,
    pub depth: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct Sorting {
    pub column: Option<String>, // enum?
    pub reverse: Option<bool>,
    pub dir_grouping: Option<String>, // enum?
}

impl Config {
    /// This constructs a Config struct with all None
    pub fn with_none() -> Self {
        Self {
            classic: None,
            blocks: None,
            color: None,
            date: None,
            dereference: None,
            display: None,
            icons: None,
            ignore_globs: None,
            indicators: None,
            layout: None,
            recursion: None,
            size: None,
            sorting: None,
            no_symlink: None,
            total_size: None,
            symlink_arrow: None,
        }
    }

    /// This constructs a Config struct with a passed file path [String].
    pub fn with_file(file: String) -> Option<Self> {
        match fs::read(&file) {
            Ok(f) => Self::with_yaml(&String::from_utf8_lossy(&f)),
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {}
                    _ => print_error!("bad config file: {}, {}\n", &file, e),
                };
                None
            }
        }
    }

    /// This constructs a Config struct with a passed [Yaml] str.
    fn with_yaml(yaml: &str) -> Option<Self> {
        match serde_yaml::from_str(yaml) {
            Ok(c) => Some(c),
            Err(e) => {
                print_error!("configuration file format error, {}\n\n", e);
                None
            }
        }
    }

    /// This provides the path for a configuration file, according to the XDG_BASE_DIRS specification.
    /// return None if error like PermissionDenied
    #[cfg(not(windows))]
    fn config_file_path() -> Option<PathBuf> {
        use xdg::BaseDirectories;
        match BaseDirectories::with_prefix(CONF_DIR) {
            Ok(p) => {
                if let Ok(p) = p.place_config_file([CONF_FILE_NAME, YAML_LONG_EXT].join(".")) {
                    return Some(p);
                }
            }
            Err(e) => print_error!("can not open config file: {}", e),
        }
        None
    }

    /// This provides the path for a configuration file, inside the %APPDATA% directory.
    /// return None if error like PermissionDenied
    #[cfg(windows)]
    fn config_file_path() -> Option<PathBuf> {
        if let Some(p) = dirs::config_dir() {
            return Some(
                p.join(CONF_DIR)
                    .join(CONF_FILE_NAME)
                    .with_extension(YAML_LONG_EXT),
            );
        }
        None
    }
}

impl Default for Config {
    fn default() -> Self {
        if let Some(p) = Self::config_file_path() {
            if let Some(c) = Self::with_file(p.to_string_lossy().to_string()) {
                return c;
            }
        }
        Self::with_yaml(
            r#"---
# == Classic ==
# This is a shorthand to override some of the options to be backwards compatible
# with `ls`. It affects the "color"->"when", "sorting"->"dir-grouping", "date"
# and "icons"->"when" options.
# Possible values: false, true
classic: false

# == Blocks ==
# This specifies the columns and their order when using the long and the tree
# layout.
# Possible values: permission, user, group, size, size_value, date, name, inode
blocks:
  - permission
  - user
  - group
  - size
  - date
  - name

# == Color ==
# This has various color options. (Will be expanded in the future.)
color:
  # When to colorize the output.
  # When "classic" is set, this is set to "never".
  # Possible values: never, auto, always
  when: auto

# == Date ==
# This specifies the date format for the date column. The freeform format
# accepts an strftime like string.
# When "classic" is set, this is set to "date".
# Possible values: date, relative, +<date_format>
date: date

# == Dereference ==
# Whether to dereference symbolic links.
# Possible values: false, true
dereference: false

# == Display ==
# What items to display. Do not specify this for the default behavior.
# Possible values: all, almost-all, directory-only
# display: all

# == Icons ==
icons:
  # When to use icons.
  # When "classic" is set, this is set to "never".
  # Possible values: always, auto, never
  when: auto
  # Which icon theme to use.
  # Possible values: fancy, unicode
  theme: fancy

# == Ignore Globs ==
# A list of globs to ignore when listing.
# ignore-globs:
#   - .git

# == Indicators ==
# Whether to add indicator characters to certain listed files.
# Possible values: false, true
indicators: false

# == Layout ==
# Which layout to use. "oneline" might be a bit confusing here and should be
# called "one-per-line". It might be changed in the future.
# Possible values: grid, tree, oneline
layout: grid

# == Recursion ==
recursion:
  # Whether to enable recursion.
  # Possible values: false, true
  enabled: false
  # How deep the recursion should go. This has to be a positive integer. Leave
  # it unspecified for (virtually) infinite.
  # depth: 3

# == Size ==
# Specifies the format of the size column.
# Possible values: default, short, bytes
size: default

# == Sorting ==
sorting:
  # Specify what to sort by.
  # Possible values: extension, name, time, size, version
  column: name
  # Whether to reverse the sorting.
  # Possible values: false, true
  reverse: false
  # Whether to group directories together and where.
  # When "classic" is set, this is set to "none".
  # Possible values: first, last, none
  dir-grouping: none

# == No Symlink ==
# Whether to omit showing symlink targets
# Possible values: false, true
no-symlink: false

# == Total size ==
# Whether to display the total size of directories.
# Possible values: false, true
total-size: false

# == Symlink arrow ==
# Specifies how the symlink arrow display, chars in both ascii and utf8
symlink-arrow: ⇒"#,
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    #[test]
    fn test_read_config_ok() {
        let c = Config::with_yaml("classic: true").unwrap();
        println!("{:?}", c);
        assert!(c.classic.unwrap())
    }

    #[test]
    fn test_read_config_bad_bool() {
        let c = Config::with_yaml("classic: notbool");
        println!("{:?}", c);
        assert!(c.is_none())
    }

    #[test]
    fn test_read_config_file_not_found() {
        let c = Config::with_file("not-existed".to_string());
        assert!(c.is_none())
    }
}
