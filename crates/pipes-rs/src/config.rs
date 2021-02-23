use model::pipe::{ColorMode, PresetKind, PresetKindSet};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, time::Duration};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Default, StructOpt)]
#[structopt(name = "pipes-rs")]
pub struct Config {
    /// "ansi", "rgb" or "none"
    #[structopt(short, long)]
    color_mode: Option<ColorMode>,

    ///delay between frames in ms
    #[structopt(short, long = "delay")]
    delay_ms: Option<u64>,

    /// percentage of the screen before resetting (0.0-1.0)
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
}

impl Config {
    pub fn color_mode(&self) -> ColorMode {
        self.color_mode.unwrap_or(ColorMode::Ansi)
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

    pub fn kinds(&self) -> PresetKindSet {
        self.kinds.clone().unwrap_or_else(|| {
            let mut kinds = HashSet::with_capacity(1);
            kinds.insert(PresetKind::Heavy);
            PresetKindSet(kinds)
        })
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

    pub fn combine(self, other: Self) -> Self {
        Self {
            color_mode: other.color_mode.or(self.color_mode),
            delay_ms: other.delay_ms.or(self.delay_ms),
            reset_threshold: other.reset_threshold.or(self.reset_threshold),
            kinds: other.kinds.or(self.kinds),
            bold: other.bold.or(self.bold),
            inherit_style: other.inherit_style.or(self.inherit_style),
            num_pipes: other.num_pipes.or(self.num_pipes),
        }
    }
}
