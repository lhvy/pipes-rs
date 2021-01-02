use crate::pipe::PresetKind;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, time::Duration};
#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Config {
    color_mode: Option<ColorMode>,
    delay_ms: Option<u64>,
    reset_threshold: Option<f32>,
    kinds: Option<HashSet<PresetKind>>,
    bold: Option<bool>,
}

impl Config {
    pub(crate) fn color_mode(&self) -> ColorMode {
        self.color_mode.unwrap_or(ColorMode::Ansi)
    }

    pub(crate) fn delay(&self) -> Duration {
        Duration::from_millis(self.delay_ms.unwrap_or(20))
    }

    pub(crate) fn reset_threshold(&self) -> f32 {
        self.reset_threshold.unwrap_or(0.5)
    }

    pub(crate) fn kinds(&self) -> HashSet<PresetKind> {
        self.kinds.clone().unwrap_or_else(|| {
            let mut kinds = HashSet::with_capacity(1);
            kinds.insert(PresetKind::Heavy);
            kinds
        })
    }

    pub(crate) fn bold(&self) -> bool {
        self.bold.unwrap_or(true)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ColorMode {
    Ansi,
    Rgb,
    None,
}
