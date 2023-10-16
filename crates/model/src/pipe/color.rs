use std::ops::Range;

#[derive(Clone, Copy)]
pub struct Color {
    pub terminal: terminal::Color,
    pub(crate) oklch: Option<tincture::Oklch>,
}

impl Color {
    pub(crate) fn update(&mut self, hue_shift: f32) {
        if let Some(oklch) = &mut self.oklch {
            oklch.h += hue_shift.to_radians();
            let oklab = tincture::oklch_to_oklab(*oklch);
            let lrgb = tincture::oklab_to_linear_srgb(oklab);
            let srgb = tincture::linear_srgb_to_srgb(lrgb);
            self.terminal = terminal::Color::Rgb {
                r: (srgb.r * 255.0) as u8,
                g: (srgb.g * 255.0) as u8,
                b: (srgb.b * 255.0) as u8,
            };
        }
    }
}

pub(super) fn gen_random_color(color_mode: ColorMode, palette: Palette) -> Option<Color> {
    match color_mode {
        ColorMode::Ansi => Some(gen_random_ansi_color()),
        ColorMode::Rgb => Some(gen_random_rgb_color(palette)),
        ColorMode::None => None,
    }
}

fn gen_random_ansi_color() -> Color {
    let num = rng::gen_range(0..12);

    Color {
        terminal: match num {
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
        },
        oklch: None,
    }
}

fn gen_random_rgb_color(palette: Palette) -> Color {
    let hue = rng::gen_range_float(palette.get_hue_range());
    let lightness = rng::gen_range_float(palette.get_lightness_range());

    let oklch = tincture::Oklch {
        l: lightness,
        c: palette.get_chroma(),
        h: hue.to_radians(),
    };
    let oklab = tincture::oklch_to_oklab(oklch);
    let lrgb = tincture::oklab_to_linear_srgb(oklab);
    let srgb = tincture::linear_srgb_to_srgb(lrgb);
    debug_assert!(
        (0.0..=1.0).contains(&srgb.r)
            && (0.0..=1.0).contains(&srgb.g)
            && (0.0..=1.0).contains(&srgb.b)
    );

    Color {
        terminal: terminal::Color::Rgb {
            r: (srgb.r * 255.0) as u8,
            g: (srgb.g * 255.0) as u8,
            b: (srgb.b * 255.0) as u8,
        },
        oklch: Some(oklch),
    }
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    Ansi,
    Rgb,
    None,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Palette {
    Default,
    Darker,
    Pastel,
    Matrix,
}

impl Palette {
    pub(super) fn get_hue_range(self) -> Range<f32> {
        match self {
            Self::Matrix => 145.0..145.0,
            _ => 0.0..360.0,
        }
    }

    pub(super) fn get_lightness_range(self) -> Range<f32> {
        match self {
            Self::Default => 0.75..0.75,
            Self::Darker => 0.65..0.65,
            Self::Pastel => 0.8..0.8,
            Self::Matrix => 0.5..0.9,
        }
    }

    pub(super) fn get_chroma(self) -> f32 {
        match self {
            Self::Default => 0.125,
            Self::Darker => 0.11,
            Self::Pastel => 0.085,
            Self::Matrix => 0.11,
        }
    }
}
