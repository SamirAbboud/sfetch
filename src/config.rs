use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LogoConfig {
    Name(String),
    Custom {
        logo: String,
        main_color: usize,
    },
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub layout: String,
    pub colorize_functions: bool,
    pub text_color: String,
    pub functions_color: usize,
    pub logo: LogoConfig,
    pub logo_info_whitespace: usize,
    pub lstrip_info: bool,
    pub print_logo: bool,
    pub logo_position: String,
}

const DEFAULT_LAYOUT: &str = r#"
Hardware & OS
  >  Distro: {distro(true)}
  >  Kernel: {kernel(false)}
  > 󰌢 Laptop: {model(true)}
  > 󰍹 Monitor: {monitor()}
  >  GPU: {gpu(true, false)}
  >  CPU: {cpu(2, false, false)}
  >  Memory: {memory(true, 2, true)}
  >  Disk: {disk("/", true, true, true, 2)}
  >  Driver: {gpu_driver(false)}

Software & Misc
  >  WM: {wm(true)}
  >  Shell: {shell(true)}
  >  Terminal: {terminal()}
  > 󰔚 Uptime: {uptime(false)}
  >  Packages: {packages()}
  >  Theme: {gtk_theme()}
  >  Icons: {icon_theme()}
  >  Font: {gtk_font()}

          {colors(true, "  ", true)}
"#;

const DEFAULT_CONFIG: &str = include_str!("../config/config.toml");

impl Default for Config {
    fn default() -> Self {
        Self {
            layout: DEFAULT_LAYOUT.to_string(),
            colorize_functions: true,
            text_color: "logo".to_string(),
            functions_color: 7,
            logo: LogoConfig::Name("auto".to_string()),
            logo_info_whitespace: 5,
            lstrip_info: false,
            print_logo: true,
            logo_position: "left".to_string(),
        }
    }
}

pub fn load(path: Option<PathBuf>) -> io::Result<Config> {
    let Some(path) = path else {
        return Ok(Config::default());
    };

    let content = fs::read_to_string(path)?;
    let config = toml::from_str(&content)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    Ok(config)
}

pub fn publish(path: &Path) -> io::Result<()> {
    if path.exists() {
        println!("Config already exists: {}", path.display());
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, DEFAULT_CONFIG)?;

    println!("Config published to: {}", path.display());

    Ok(())
}
