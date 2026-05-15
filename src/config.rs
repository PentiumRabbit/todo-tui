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
