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
    pub path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path();
        let lang = if path.exists() {
            let content = fs::read_to_string(&path)?;
            if content.contains("language = \"zh\"") {
                Lang::Zh
            } else {
                Lang::En
            }
        } else {
            Lang::En
        };
        Ok(Self { lang, path })
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let lang_str = match self.lang {
            Lang::En => "en",
            Lang::Zh => "zh",
        };
        fs::write(&self.path, format!("language = \"{}\"\n", lang_str))?;
        Ok(())
    }

    pub fn toggle_lang(&mut self) -> Result<()> {
        self.lang = self.lang.toggle();
        self.save()
    }
}

fn config_path() -> PathBuf {
    dirs_next::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("todo-tui")
        .join("config.toml")
}
