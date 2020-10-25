//! This module defines the [Theme]. To set it up from [ArgMatches], a [Yaml] and its [Default]
//! value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use std::path::Path;
use yaml_rust::Yaml;

/// the path of the color theme file
///
/// .config/lsd/ + path if relative
/// path if absolute
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct ColorTheme {
    /// where is the color theme file
    pub path: Path,
}

impl Configurable<Self> for ColorTheme {
    /// Get a potential `Theme` value from [ArgMatches].
    ///
    /// If the "theme" argument is passed, this returns a `Theme` with valid value `color` and `icon` in
    /// a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if let Some(path) = matches.value_of("color-theme") {
            p = Path::new(path);
            if p.is_relative() {
                Some(Path::new())
            } else {
                Some(p)
            }
        } else {
            None
        }
    }

    /// Get a potential `Theme` value from a [Config].
    ///
    /// If the Config's [Yaml] contains the [String](Yaml::String) value pointed to by
    /// "theme/color:", this returns its value as the value of the `ColorTheme`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            match &yaml["theme"]["color"] {
                Yaml::BadValue => None,
                Yaml::String(value) => Some(Self(Path::new(*value))),
                _ => {
                    config.print_wrong_type_warning("theme/color", "string");
                    None
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `ColorTheme` is `theme.yaml`.
impl Default for ColorTheme {
    fn default() -> Self {
        Self{
            path: Path::new()
        }
    }
}

#[cfg(test)]
mod test_color_theme {
    use super::ColorTheme;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_color_theme_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, ColorTheme::from_arg_matches(&matches));
    }

    #[test]
    fn test_color_theme_from_arg_matches_ok() {
        let argv = vec!["lsd", "--color-theme", "theme-ok.yaml"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Path::new("theme-ok.yaml")),
            ColorTheme::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_color_theme_from_config_none() {
        assert_eq!(None, ColorTheme::from_config(&Config::with_none()));
    }

    #[test]
    fn test_color_theme_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, ColorTheme::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_color_theme_from_config_ok() {
        let yaml_string = "styles:\n  color-theme: theme-ok.yaml";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(Path::new("theme-ok.yam")),
            ColorTheme::from_config(&Config::with_yaml(yaml))
        );
    }
}
