use anyhow::Context;
use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};
use model::pipe::{ColorMode, Kind, KindSet, Palette};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, time::Duration};
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Default, StructOpt)]
#[structopt(name = "pipes-rs", setting = AppSettings::ColoredHelp)]
pub struct Config {
    /// what kind of terminal coloring to use
    #[structopt(short, long, possible_values = &["ansi", "rgb", "none"])]
    pub color_mode: Option<ColorMode>,

    /// the color palette used assign colors to pipes
    #[structopt(long, possible_values = &["default", "darker", "pastel", "matrix"])]
    pub palette: Option<Palette>,

    /// delay between frames in milliseconds
    #[structopt(short, long = "delay")]
    pub delay_ms: Option<u64>,

    /// portion of screen covered before resetting (0.0–1.0)
    #[structopt(short, long)]
    pub reset_threshold: Option<f32>,

    /// kinds of pipes separated by commas, e.g. heavy,curved
    #[structopt(short, long)]
    pub kinds: Option<KindSet>,

    /// whether to use bold
    #[structopt(short, long, possible_values = &["true", "false"], value_name = "boolean")]
    pub bold: Option<bool>,

    /// whether pipes should retain style after hitting the edge
    #[structopt(short, long, possible_values = &["true", "false"], value_name = "boolean")]
    pub inherit_style: Option<bool>,

    /// number of pipes
    #[structopt(name = "pipe-num", short, long)]
    pub num_pipes: Option<u32>,

    /// chance of a pipe turning (0.0–1.0)
    #[structopt(short, long)]
    pub turn_chance: Option<f32>,
}

impl Config {
    pub fn read() -> anyhow::Result<Self> {
        let config = Self::read_from_disk_with_default()?.combine(Self::from_args());
        config.validate()?;

        Ok(config)
    }

    fn read_from_disk_with_default() -> anyhow::Result<Self> {
        let path = Self::path()?;

        if path.exists() {
            Self::read_from_disk(path)
        } else {
            Ok(Config::default())
        }
    }

    fn path() -> anyhow::Result<PathBuf> {
        let path = Xdg::new(AppStrategyArgs {
            top_level_domain: "io.github".to_string(),
            author: "lhvy".to_string(),
            app_name: "pipes-rs".to_string(),
        })?
        .in_config_dir("config.toml");

        Ok(path)
    }

    fn read_from_disk(path: PathBuf) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents).context("failed to read config")
    }

    fn validate(&self) -> anyhow::Result<()> {
        if let Some(reset_threshold) = self.reset_threshold() {
            if !(0.0..=1.0).contains(&reset_threshold) {
                anyhow::bail!("reset threshold should be within 0 and 1")
            }
        }
        if !(0.0..=1.0).contains(&self.turn_chance()) {
            anyhow::bail!("turn chance should be within 0 and 1")
        }

        Ok(())
    }

    pub fn color_mode(&self) -> ColorMode {
        self.color_mode.unwrap_or(ColorMode::Ansi)
    }

    pub fn palette(&self) -> Palette {
        self.palette.unwrap_or(Palette::Default)
    }

    pub fn delay(&self) -> Duration {
        Duration::from_millis(self.delay_ms.unwrap_or(20))
    }

    pub fn reset_threshold(&self) -> Option<f32> {
        if self.reset_threshold == Some(0.0) {
            None
        } else {
            Some(self.reset_threshold.unwrap_or(0.5))
        }
    }

    pub fn kinds(&self) -> KindSet {
        self.kinds
            .clone()
            .unwrap_or_else(|| KindSet::from_one(Kind::Heavy))
    }

    pub fn bold(&self) -> bool {
        self.bold.unwrap_or(true)
    }

    pub fn inherit_style(&self) -> bool {
        self.inherit_style.unwrap_or(false)
    }

    pub fn num_pipes(&self) -> u32 {
        self.num_pipes.unwrap_or(1)
    }

    pub fn turn_chance(&self) -> f32 {
        self.turn_chance.unwrap_or(0.15)
    }

    fn combine(self, other: Self) -> Self {
        Self {
            color_mode: other.color_mode.or(self.color_mode),
            palette: other.palette.or(self.palette),
            delay_ms: other.delay_ms.or(self.delay_ms),
            reset_threshold: other.reset_threshold.or(self.reset_threshold),
            kinds: other.kinds.or(self.kinds),
            bold: other.bold.or(self.bold),
            inherit_style: other.inherit_style.or(self.inherit_style),
            num_pipes: other.num_pipes.or(self.num_pipes),
            turn_chance: other.turn_chance.or(self.turn_chance),
        }
    }
}
