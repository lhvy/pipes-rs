use rand::Rng;
use std::str::FromStr;
use tincture::ColorSpace;

pub(super) fn gen_random_color(color_mode: ColorMode, palette: Palette) -> Option<terminal::Color> {
    match color_mode {
        ColorMode::Ansi => Some(gen_random_ansi_color()),
        ColorMode::Rgb => Some(gen_random_rgb_color(palette)),
        ColorMode::None => None,
    }
}

fn gen_random_ansi_color() -> terminal::Color {
    let num = rand::thread_rng().gen_range(0..=11);

    match num {
        0 => terminal::Color::Red,
        1 => terminal::Color::DarkRed,
        2 => terminal::Color::Green,
        3 => terminal::Color::DarkGreen,
        4 => terminal::Color::Yellow,
        5 => terminal::Color::DarkYellow,
        6 => terminal::Color::Blue,
        7 => terminal::Color::DarkBlue,
        8 => terminal::Color::Magenta,
        9 => terminal::Color::DarkMagenta,
        10 => terminal::Color::Cyan,
        11 => terminal::Color::DarkCyan,
        _ => unreachable!(),
    }
}

fn gen_random_rgb_color(palette: Palette) -> terminal::Color {
    let hue = rand::thread_rng().gen_range(0.0..=360.0);
    let oklch = tincture::Oklch {
        l: palette.get_lightness(),
        c: palette.get_chroma(),
        h: tincture::Hue::from_degrees(hue).unwrap(),
    };
    let oklab = tincture::Oklab::from(oklch);
    let lrgb: tincture::LinearRgb = tincture::convert(oklab);
    let srgb = tincture::Srgb::from(lrgb);
    debug_assert!(srgb.in_bounds());

    terminal::Color::Rgb {
        r: (srgb.r * 255.0) as u8,
        g: (srgb.g * 255.0) as u8,
        b: (srgb.b * 255.0) as u8,
    }
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    Ansi,
    Rgb,
    None,
}

impl FromStr for ColorMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ansi" => Self::Ansi,
            "rgb" => Self::Rgb,
            "none" => Self::None,
            _ => anyhow::bail!(r#"expected "ansi", "rgb" or "none""#),
        })
    }
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Palette {
    Default,
    Darker,
    Pastel,
}

impl FromStr for Palette {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "default" => Self::Default,
            "darker" => Self::Darker,
            "pastel" => Self::Pastel,
            _ => anyhow::bail!(r#"expected "default", "darker" or "pastel""#),
        })
    }
}

impl Palette {
    pub(super) fn get_lightness(self) -> f32 {
        match self {
            Self::Default => 0.75,
            Self::Darker => 0.65,
            Self::Pastel => 0.8,
        }
    }

    pub(super) fn get_chroma(self) -> f32 {
        match self {
            Self::Default => 0.125,
            Self::Darker => 0.11,
            Self::Pastel => 0.085,
        }
    }
}
