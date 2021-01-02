use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Config {
    color_mode: Option<ColorMode>,
    delay_ms: Option<u64>,
    reset_threshold: Option<f32>,
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
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ColorMode {
    Ansi,
    Rgb,
    None,
}
