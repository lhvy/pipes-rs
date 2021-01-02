use std::time::Duration;

pub(crate) struct Config {
    pub(crate) color_mode: ColorMode,
    pub(crate) delay: Duration,
    pub(crate) reset_threshold: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            color_mode: ColorMode::Ansi,
            delay: Duration::from_millis(20),
            reset_threshold: 0.5,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ColorMode {
    Ansi,
    Rgb,
}
