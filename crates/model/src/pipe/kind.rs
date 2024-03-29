use std::num::NonZeroUsize;
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Heavy,
    Light,
    Curved,
    Knobby,
    Emoji,
    Outline,
    Dots,
}

impl Kind {
    pub fn up(self) -> char {
        self.chars()[0]
    }

    pub fn down(self) -> char {
        self.chars()[1]
    }

    pub fn left(self) -> char {
        self.chars()[2]
    }

    pub fn right(self) -> char {
        self.chars()[3]
    }

    pub fn top_left(self) -> char {
        self.chars()[4]
    }

    pub fn top_right(self) -> char {
        self.chars()[5]
    }

    pub fn bottom_left(self) -> char {
        self.chars()[6]
    }

    pub fn bottom_right(self) -> char {
        self.chars()[7]
    }

    fn chars(self) -> [char; 8] {
        match self {
            Self::Heavy => Self::HEAVY,
            Self::Light => Self::LIGHT,
            Self::Curved => Self::CURVED,
            Self::Knobby => Self::KNOBBY,
            Self::Emoji => Self::EMOJI,
            Self::Outline => Self::OUTLINE,
            Self::Dots => Self::DOTS,
        }
    }

    fn width(self) -> KindWidth {
        if self == Self::Dots {
            KindWidth::Custom(NonZeroUsize::new(2).unwrap())
        } else {
            KindWidth::Auto
        }
    }

    const HEAVY: [char; 8] = ['┃', '┃', '━', '━', '┏', '┓', '┗', '┛'];
    const LIGHT: [char; 8] = ['│', '│', '─', '─', '┌', '┐', '└', '┘'];
    const CURVED: [char; 8] = ['│', '│', '─', '─', '╭', '╮', '╰', '╯'];
    const KNOBBY: [char; 8] = ['╽', '╿', '╼', '╾', '┎', '┒', '┖', '┚'];
    const EMOJI: [char; 8] = ['👆', '👇', '👈', '👉', '👌', '👌', '👌', '👌'];
    const OUTLINE: [char; 8] = ['║', '║', '═', '═', '╔', '╗', '╚', '╝'];
    const DOTS: [char; 8] = ['•', '•', '•', '•', '•', '•', '•', '•'];
}

#[derive(Clone, Copy)]
enum KindWidth {
    Auto,
    Custom(NonZeroUsize),
}

impl FromStr for Kind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "heavy" => Self::Heavy,
            "light" => Self::Light,
            "curved" => Self::Curved,
            "knobby" => Self::Knobby,
            "emoji" => Self::Emoji,
            "outline" => Self::Outline,
            "dots" => Self::Dots,
            _ => anyhow::bail!(
                r#"unknown pipe kind (expected “heavy”, “light”, “curved”, “knobby”, “emoji”, “outline” or “dots”)"#,
            ),
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct KindSet(Vec<Kind>);

impl FromStr for KindSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Vec::new();

        for kind in s.split(',') {
            let kind = Kind::from_str(kind)?;

            if !set.contains(&kind) {
                set.push(kind);
            }
        }

        Ok(Self(set))
    }
}

impl KindSet {
    pub fn from_one(kind: Kind) -> Self {
        Self(vec![kind])
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = Kind> + '_ {
        self.0.iter().copied()
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.0.iter().flat_map(|kind| kind.chars())
    }

    pub fn custom_widths(&self) -> impl Iterator<Item = NonZeroUsize> + '_ {
        self.0.iter().filter_map(|kind| match kind.width() {
            KindWidth::Custom(n) => Some(n),
            KindWidth::Auto => None,
        })
    }
}
