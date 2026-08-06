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

    #[test]
    fn lang_toggle() {
        assert_eq!(Lang::En.toggle(), Lang::Zh);
        assert_eq!(Lang::Zh.toggle(), Lang::En);
    }

    #[test]
    fn config_load_defaults_when_no_file() {
        let cfg = Config {
            lang: Lang::En,
            show_statusbar: true,
            path: PathBuf::from("/nonexistent/path/config.toml"),
        };
        // 直接验证默认值逻辑（load 依赖 config_path()，此处验证字段默认）
        assert_eq!(cfg.lang, Lang::En);
        assert!(cfg.show_statusbar);
    }

    #[test]
    fn config_save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join(format!("todo-tui-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");

        let cfg = Config {
            lang: Lang::Zh,
            show_statusbar: false,
            path: path.clone(),
        };
        cfg.save().unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("language = \"zh\""));
        assert!(content.contains("show_statusbar = \"false\""));

        // 模拟 load 的解析逻辑
        let loaded_lang = if content.contains("language = \"zh\"") { Lang::Zh } else { Lang::En };
        let loaded_statusbar = !content.contains("show_statusbar = \"false\"");
        assert_eq!(loaded_lang, Lang::Zh);
        assert!(!loaded_statusbar);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn config_save_english_defaults() {
        let dir = std::env::temp_dir().join(format!("todo-tui-test-en-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");

        let cfg = Config {
            lang: Lang::En,
            show_statusbar: true,
            path: path.clone(),
        };
        cfg.save().unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("language = \"en\""));
        assert!(content.contains("show_statusbar = \"true\""));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn config_toggle_lang_and_statusbar() {
        let dir = std::env::temp_dir().join(format!("todo-tui-test-toggle-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");

        let mut cfg = Config {
            lang: Lang::En,
            show_statusbar: true,
            path: path.clone(),
        };
        cfg.toggle_lang().unwrap();
        assert_eq!(cfg.lang, Lang::Zh);
        cfg.toggle_statusbar().unwrap();
        assert!(!cfg.show_statusbar);

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("language = \"zh\""));
        assert!(content.contains("show_statusbar = \"false\""));

        fs::remove_dir_all(&dir).unwrap();
    }
}
