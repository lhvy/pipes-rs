use std::collections::HashSet;
use std::num::NonZeroUsize;
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PresetKind {
    Heavy,
    Light,
    Curved,
    Knobby,
    Emoji,
    Outline,
    Dots,
}

#[derive(Clone, Copy)]
pub(super) struct Kind {
    pub(super) up: char,
    pub(super) down: char,
    pub(super) left: char,
    pub(super) right: char,
    pub(super) top_left: char,
    pub(super) top_right: char,
    pub(super) bottom_left: char,
    pub(super) bottom_right: char,
    width: KindWidth,
}

impl Kind {
    fn chars(&self) -> [char; 8] {
        [
            self.up,
            self.down,
            self.left,
            self.right,
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
    }
}

#[derive(Clone, Copy)]
enum KindWidth {
    Auto,
    Custom(NonZeroUsize),
}

impl PresetKind {
    const HEAVY: Kind = Kind {
        up: '┃',
        down: '┃',
        left: '━',
        right: '━',
        top_left: '┏',
        top_right: '┓',
        bottom_left: '┗',
        bottom_right: '┛',
        width: KindWidth::Auto,
    };

    const LIGHT: Kind = Kind {
        up: '│',
        down: '│',
        left: '─',
        right: '─',
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
        width: KindWidth::Auto,
    };

    const CURVED: Kind = Kind {
        up: '│',
        down: '│',
        left: '─',
        right: '─',
        top_left: '╭',
        top_right: '╮',
        bottom_left: '╰',
        bottom_right: '╯',
        width: KindWidth::Auto,
    };

    const KNOBBY: Kind = Kind {
        up: '╽',
        down: '╿',
        left: '╼',
        right: '╾',
        top_left: '┎',
        top_right: '┒',
        bottom_left: '┖',
        bottom_right: '┚',
        width: KindWidth::Auto,
    };

    const EMOJI: Kind = Kind {
        up: '👆',
        down: '👇',
        left: '👈',
        right: '👉',
        top_left: '👌',
        top_right: '👌',
        bottom_left: '👌',
        bottom_right: '👌',
        width: KindWidth::Auto,
    };

    const OUTLINE: Kind = Kind {
        up: '║',
        down: '║',
        left: '═',
        right: '═',
        top_left: '╔',
        top_right: '╗',
        bottom_left: '╚',
        bottom_right: '╝',
        width: KindWidth::Auto,
    };

    const DOTS: Kind = Kind {
        up: '•',
        down: '•',
        left: '•',
        right: '•',
        top_left: '•',
        top_right: '•',
        bottom_left: '•',
        bottom_right: '•',

        // ideally we would use NonZeroUsize::new(2).unwrap() here,
        // but Option::unwrap in const contexts
        // isn’t stable at the moment.
        width: KindWidth::Custom(unsafe { NonZeroUsize::new_unchecked(2) }),
    };

    pub(super) fn kind(&self) -> Kind {
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
}

impl FromStr for PresetKind {
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
pub struct PresetKindSet(pub HashSet<PresetKind>);

impl FromStr for PresetKindSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = HashSet::new();
        for preset_kind in s.split(',') {
            set.insert(PresetKind::from_str(preset_kind)?);
        }
        Ok(Self(set))
    }
}

impl PresetKindSet {
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.kinds().flat_map(|kind| kind.chars())
    }

    pub fn custom_widths(&self) -> impl Iterator<Item = NonZeroUsize> + '_ {
        self.kinds().filter_map(|kind| match kind.width {
            KindWidth::Custom(n) => Some(n),
            KindWidth::Auto => None,
        })
    }

    fn kinds(&self) -> impl Iterator<Item = Kind> + '_ {
        self.0.iter().map(|preset_kind| preset_kind.kind())
    }
}
