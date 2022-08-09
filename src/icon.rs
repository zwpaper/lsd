use crate::flags::{IconOption, IconTheme as FlagTheme};
use crate::meta::{FileType, Name};
use crate::theme::{icon::IconTheme, Theme};

pub struct Icons {
    icon_separator: String,
    theme: Option<IconTheme>,
}

// In order to add a new icon, write the unicode value like "\ue5fb" then
// run the command below in vim:
//
// s#\\u[0-9a-f]*#\=eval('"'.submatch(0).'"')#
impl Icons {
    pub fn new(tty: bool, when: IconOption, theme: FlagTheme, icon_separator: String) -> Self {
        let icon_theme = match (tty, when, theme) {
            (_, IconOption::Never, _) | (false, IconOption::Auto, _) => None,
            (_, _, FlagTheme::Fancy) => {
                if let Ok(t) = Theme::from_path::<IconTheme>("icons") {
                    Some(t)
                } else {
                    Some(IconTheme::default())
                }
            }
            (_, _, FlagTheme::Unicode) => Some(IconTheme::unicode()),
        };

        Self {
            icon_separator,
            theme: icon_theme,
        }
    }

    pub fn get(&self, name: &Name) -> String {
        match &self.theme {
            None => String::new(),
            Some(t) => {
                // Check file types
                let file_type: FileType = name.file_type();
                let icon = match file_type {
                    FileType::SymLink { is_dir: true } => &t.icons_by_filetype.symlink_dir,
                    FileType::SymLink { is_dir: false } => &t.icons_by_filetype.symlink_file,
                    FileType::Socket => &t.icons_by_filetype.socket,
                    FileType::Pipe => &t.icons_by_filetype.pipe,
                    FileType::CharDevice => &t.icons_by_filetype.device_char,
                    FileType::BlockDevice => &t.icons_by_filetype.device_block,
                    FileType::Special => &t.icons_by_filetype.special,
                    _ => {
                        if let Some(icon) = t
                            .icons_by_name
                            .get(name.file_name().to_lowercase().as_str())
                        {
                            icon
                        } else if let Some(icon) = name
                            .extension()
                            .and_then(|ext| t.icons_by_extension.get(ext.to_lowercase().as_str()))
                        {
                            icon
                        } else {
                            match file_type {
                                FileType::Directory { .. } => &t.icons_by_filetype.dir,
                                // If a file has no extension and is executable, show an icon.
                                // Except for Windows, it marks everything as an executable.
                                #[cfg(not(windows))]
                                FileType::File { exec: true, .. } => {
                                    &t.icons_by_filetype.executable
                                }
                                _ => &t.icons_by_filetype.file,
                            }
                        }
                    }
                };

                format!("{}{}", icon, self.icon_separator)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{IconTheme, Icons};
    use crate::flags::{IconOption, IconTheme as FlagTheme};
    use crate::meta::Meta;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn get_no_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(false, IconOption::Never, FlagTheme::Fancy, " ".to_string());
        let icon = icon.get(&meta.name);

        assert_eq!(icon, "");
    }

    #[test]
    fn get_default_file_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(false, IconOption::Always, FlagTheme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, "\u{f016} "); // 
    }

    #[test]
    fn get_default_file_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(
            false,
            IconOption::Always,
            FlagTheme::Unicode,
            " ".to_string(),
        );
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{1f4c4}", icon.icon_separator));
    }

    #[test]
    fn get_directory_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(file_path, false).unwrap();

        let icon = Icons::new(false, IconOption::Always, FlagTheme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, "\u{f115} "); // 
    }

    #[test]
    fn get_directory_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(file_path, false).unwrap();

        let icon = Icons::new(
            false,
            IconOption::Always,
            FlagTheme::Unicode,
            " ".to_string(),
        );
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{1f4c2}", icon.icon_separator));
    }

    #[test]
    fn get_directory_icon_with_ext() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(file_path, false).unwrap();

        let icon = Icons::new(false, IconOption::Always, FlagTheme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, "\u{f115} "); // 
    }

    #[test]
    fn get_icon_by_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        for (file_name, file_icon) in &IconTheme::get_default_icons_by_name() {
            let file_path = tmp_dir.path().join(file_name);
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path, false).unwrap();

            let icon = Icons::new(false, IconOption::Always, FlagTheme::Fancy, " ".to_string());
            let icon_str = icon.get(&meta.name);

            assert_eq!(icon_str, format!("{}{}", file_icon, icon.icon_separator));
        }
    }

    #[test]
    fn get_icon_by_extension() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        for (ext, file_icon) in &IconTheme::get_default_icons_by_extension() {
            let file_path = tmp_dir.path().join(format!("file.{}", ext));
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path, false).unwrap();

            let icon = Icons::new(false, IconOption::Always, FlagTheme::Fancy, " ".to_string());
            let icon_str = icon.get(&meta.name);

            assert_eq!(icon_str, format!("{}{}", file_icon, icon.icon_separator));
        }
    }
}
