use anyhow::Context;
use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};
use model::pipe::{ColorMode, Kind, KindSet, Palette};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub color_mode: Option<ColorMode>,
    pub palette: Option<Palette>,
    pub rainbow: Option<u8>,
    pub delay_ms: Option<u64>,
    pub fps: Option<f32>,
    pub reset_threshold: Option<f32>,
    pub kinds: Option<KindSet>,
    pub bold: Option<bool>,
    pub inherit_style: Option<bool>,
    pub num_pipes: Option<u32>,
    pub turn_chance: Option<f32>,
}

impl Config {
    pub fn read() -> anyhow::Result<Self> {
        let config = Self::read_from_disk_with_default()?;
        config.validate()?;

        Ok(config)
    }

    fn read_from_disk_with_default() -> anyhow::Result<Self> {
        let path = Self::path()?;

        if !path.exists() {
            return Ok(Config::default());
        }

        Self::read_from_disk(path)
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

    pub fn validate(&self) -> anyhow::Result<()> {
        if let Some(reset_threshold) = self.reset_threshold() {
            if !(0.0..=1.0).contains(&reset_threshold) {
                anyhow::bail!("reset threshold should be within 0 and 1")
            }
        }

        if !(0.0..=1.0).contains(&self.turn_chance()) {
            anyhow::bail!("turn chance should be within 0 and 1")
        }

        if self.delay_ms.is_some() && self.fps.is_some() {
            anyhow::bail!("both delay and FPS can’t be set simultaneously");
        }

        Ok(())
    }

    pub fn color_mode(&self) -> ColorMode {
        self.color_mode.unwrap_or(ColorMode::Ansi)
    }

    pub fn palette(&self) -> Palette {
        self.palette.unwrap_or(Palette::Default)
    }

    pub fn rainbow(&self) -> u8 {
        self.rainbow.unwrap_or(0)
    }

    pub fn tick_length(&self) -> Duration {
        if let Some(fps) = self.fps {
            if fps == 0.0 {
                return Duration::ZERO;
            }
            return Duration::from_secs_f32(1.0 / fps);
        }

        if let Some(delay_ms) = self.delay_ms {
            return Duration::from_millis(delay_ms); // assume rendering a frame takes no time
        }

        Duration::from_secs_f32(1.0 / 50.0) // default to 50 FPS
    }

    pub fn reset_threshold(&self) -> Option<f32> {
        match self.reset_threshold {
            Some(n) if n == 0.0 => None,
            Some(n) => Some(n),
            None => Some(0.5),
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
}
