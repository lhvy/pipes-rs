use anyhow::Context;
use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};
use model::pipe::{ColorMode, Palette, PresetKind, PresetKindSet};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::PathBuf, time::Duration};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Default, StructOpt)]
#[structopt(name = "pipes-rs")]
pub(crate) struct Config {
    /// “ansi”, “rgb” or “none”
    #[structopt(short, long)]
    color_mode: Option<ColorMode>,

    /// “default”, “darker”, “pastel” or “matrix”
    #[structopt(long)]
    palette: Option<Palette>,

    /// delay between frames in milliseconds
    #[structopt(short, long = "delay")]
    delay_ms: Option<u64>,

    /// portion of screen covered before resetting (0.0–1.0)
    #[structopt(short, long)]
    reset_threshold: Option<f32>,

    /// kinds of pipes separated by commas, e.g. heavy,curved
    #[structopt(short, long)]
    kinds: Option<PresetKindSet>,

    /// whether to use bold (true/false)
    #[structopt(short, long)]
    bold: Option<bool>,

    /// whether pipes should retain style after hitting the edge (true/false)
    #[structopt(short, long)]
    inherit_style: Option<bool>,

    /// number of pipes
    #[structopt(name = "pipe-num", short, long)]
    num_pipes: Option<u32>,

    /// chance of a pipe turning (0.0–1.0)
    #[structopt(short, long)]
    turn_chance: Option<f32>,
}

impl Config {
    pub(crate) fn read() -> anyhow::Result<Self> {
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
            author: "CookieCoder15".to_string(),
            app_name: "pipes-rs".to_string(),
        })?
        .in_config_dir("config.toml");

        Ok(path)
    }

    fn read_from_disk(path: PathBuf) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents).context("failed to read config")?)
    }

    pub(crate) fn color_mode(&self) -> ColorMode {
        self.color_mode.unwrap_or(ColorMode::Ansi)
    }

    pub(crate) fn palette(&self) -> Palette {
        self.palette.unwrap_or(Palette::Default)
    }

    pub(crate) fn delay(&self) -> Duration {
        Duration::from_millis(self.delay_ms.unwrap_or(20))
    }

    pub(crate) fn reset_threshold(&self) -> Option<f32> {
        if self.reset_threshold == Some(0.0) {
            None
        } else {
            Some(self.reset_threshold.unwrap_or(0.5))
        }
    }

    pub(crate) fn kinds(&self) -> PresetKindSet {
        self.kinds.clone().unwrap_or_else(|| {
            let mut kinds = HashSet::with_capacity(1);
            kinds.insert(PresetKind::Heavy);
            PresetKindSet(kinds)
        })
    }

    pub(crate) fn bold(&self) -> bool {
        self.bold.unwrap_or(true)
    }

    pub(crate) fn inherit_style(&self) -> bool {
        self.inherit_style.unwrap_or(false)
    }

    pub(crate) fn num_pipes(&self) -> u32 {
        self.num_pipes.unwrap_or(1)
    }

    pub(crate) fn turn_chance(&self) -> f32 {
        self.turn_chance.unwrap_or(0.15)
    }

    pub(crate) fn combine(self, other: Self) -> Self {
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
