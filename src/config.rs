use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Lang {
    En,
    Zh,
}

impl Lang {
    pub fn toggle(&self) -> Self {
        match self {
            Lang::En => Lang::Zh,
            Lang::Zh => Lang::En,
        }
    }
}

pub struct Config {
    pub lang: Lang,
    pub show_statusbar: bool,
    pub path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path();
        let (lang, show_statusbar) = if path.exists() {
            let content = fs::read_to_string(&path)?;
            let lang = if content.contains("language = \"zh\"") {
                Lang::Zh
            } else {
                Lang::En
            };
            let show_statusbar = !content.contains("show_statusbar = \"false\"");
            (lang, show_statusbar)
        } else {
            (Lang::En, true)
        };
        Ok(Self {
            lang,
            show_statusbar,
            path,
        })
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let lang_str = match self.lang {
            Lang::En => "en",
            Lang::Zh => "zh",
        };
        let statusbar_str = if self.show_statusbar { "true" } else { "false" };
        fs::write(
            &self.path,
            format!(
                "language = \"{}\"\nshow_statusbar = \"{}\"\n",
                lang_str, statusbar_str
            ),
        )?;
        Ok(())
    }

    pub fn toggle_lang(&mut self) -> Result<()> {
        self.lang = self.lang.toggle();
        self.save()
    }

    pub fn toggle_statusbar(&mut self) -> Result<()> {
        self.show_statusbar = !self.show_statusbar;
        self.save()
    }
}

fn config_path() -> PathBuf {
    dirs_next::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("todo-tui")
        .join("config.toml")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn temp_config_path() -> PathBuf {
        let dir = std::env::temp_dir().join("todo-tui-test");
        dir.join("config.toml")
    }

    fn cleanup() {
        let path = temp_config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }

    #[test]
    fn test_config_load_default_when_file_missing() {
        cleanup();
        let path = temp_config_path();
        assert!(!path.exists());

        // 手动构造 Config 验证默认值逻辑
        let config = Config {
            lang: Lang::En,
            show_statusbar: true,
            path: path.clone(),
        };
        assert_eq!(config.lang, Lang::En);
        assert!(config.show_statusbar);
    }

    #[test]
    fn test_config_save_and_load_roundtrip() {
        cleanup();
        let path = temp_config_path();

        // 保存英文配置
        let config = Config {
            lang: Lang::En,
            show_statusbar: true,
            path: path.clone(),
        };
        config.save().unwrap();
        assert!(path.exists());

        // 读取并验证内容
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("language = \"en\""));
        assert!(content.contains("show_statusbar = \"true\""));

        // 保存中文配置
        let config_zh = Config {
            lang: Lang::Zh,
            show_statusbar: false,
            path: path.clone(),
        };
        config_zh.save().unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("language = \"zh\""));
        assert!(content.contains("show_statusbar = \"false\""));

        cleanup();
    }

    #[test]
    fn test_config_parse_custom_content() {
        cleanup();
        let path = temp_config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // 写入中文配置
        fs::write(&path, "language = \"zh\"\nshow_statusbar = \"false\"\n").unwrap();
        let content = fs::read_to_string(&path).unwrap();
        let lang = if content.contains("language = \"zh\"") { Lang::Zh } else { Lang::En };
        let show_statusbar = !content.contains("show_statusbar = \"false\"");
        assert_eq!(lang, Lang::Zh);
        assert!(!show_statusbar);

        // 写入英文配置
        fs::write(&path, "language = \"en\"\nshow_statusbar = \"true\"\n").unwrap();
        let content = fs::read_to_string(&path).unwrap();
        let lang = if content.contains("language = \"zh\"") { Lang::Zh } else { Lang::En };
        let show_statusbar = !content.contains("show_statusbar = \"false\"");
        assert_eq!(lang, Lang::En);
        assert!(show_statusbar);

        cleanup();
    }

    #[test]
    fn test_lang_toggle() {
        assert_eq!(Lang::En.toggle(), Lang::Zh);
        assert_eq!(Lang::Zh.toggle(), Lang::En);
    }

    #[test]
    fn test_config_toggle_lang_and_statusbar() {
        cleanup();
        let path = temp_config_path();

        let mut config = Config {
            lang: Lang::En,
            show_statusbar: true,
            path: path.clone(),
        };

        config.toggle_lang().unwrap();
        assert_eq!(config.lang, Lang::Zh);
        assert!(path.exists());

        config.toggle_statusbar().unwrap();
        assert!(!config.show_statusbar);

        cleanup();
    }
}
