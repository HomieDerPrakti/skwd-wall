#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BrowserGrid {
    pub cols: usize,
    pub rows: usize,
    pub thumb_w: f32,
    pub thumb_h: f32,
    pub gap_x: f32,
    pub gap_y: f32,
    pub corner_radius: f32,
    pub border_width: f32,
}

impl BrowserGrid {
    #[allow(dead_code)]
    pub fn cell_h(&self) -> f32 {
        self.thumb_h + self.gap_y
    }
}

pub const SANDY_SWAP_STYLES: [(&str, &str); 11] = [
    ("vortex", "Vortex"),
    ("hourglass", "Hourglass"),
    ("castle", "Sandcastle"),
    ("saltation", "Saltation"),
    ("pour", "Glass pour"),
    ("orbit", "Maelstrom"),
    ("burst", "Burst"),
    ("weave", "Weave"),
    ("bloom", "Bloom"),
    ("flock", "Murmuration"),
    ("ring", "Ring"),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SandyStyle {
    Vortex,
    Hourglass,
    Castle,
    Saltation,
    Pour,
    Orbit,
    Burst,
    Weave,
    Bloom,
    Flock,
    Ring,
}

impl SandyStyle {
    pub fn from_key(key: &str) -> Self {
        match key {
            "hourglass" => Self::Hourglass,
            "castle" => Self::Castle,
            "saltation" => Self::Saltation,
            "pour" => Self::Pour,
            "orbit" => Self::Orbit,
            "burst" => Self::Burst,
            "weave" => Self::Weave,
            "bloom" => Self::Bloom,
            "flock" => Self::Flock,
            "ring" => Self::Ring,
            _ => Self::Vortex,
        }
    }

    pub fn shader_index(self) -> f32 {
        match self {
            Self::Vortex => 1.0,
            Self::Hourglass => 2.0,
            Self::Castle => 3.0,
            Self::Saltation => 6.0,
            Self::Pour => 7.0,
            Self::Orbit => 8.0,
            Self::Burst => 10.0,
            Self::Weave => 11.0,
            Self::Bloom => 13.0,
            Self::Flock => 16.0,
            Self::Ring => 17.0,
        }
    }
}

pub fn sandy_style_index(style: &str) -> f32 {
    SandyStyle::from_key(style).shader_index()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandMove {
    Corkscrew,
    Cascade,
    Shuffle,
    Ribbon,
    Spiral,
}

impl HandMove {
    pub const ALL: [HandMove; 5] = [
        HandMove::Corkscrew,
        HandMove::Cascade,
        HandMove::Shuffle,
        HandMove::Ribbon,
        HandMove::Spiral,
    ];

    pub fn from_key(value: &str) -> Option<Self> {
        match value {
            "corkscrew" => Some(Self::Corkscrew),
            "cascade" => Some(Self::Cascade),
            "shuffle" => Some(Self::Shuffle),
            "ribbon" => Some(Self::Ribbon),
            "spiral" => Some(Self::Spiral),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|mv| *mv == self).unwrap_or(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandCut {
    Straight,
    Slant,
    Steep,
}

impl HandCut {
    pub fn from_key(value: &str) -> Self {
        match value {
            "slant" => Self::Slant,
            "steep" => Self::Steep,
            _ => Self::Straight,
        }
    }

    pub const fn as_key(self) -> &'static str {
        match self {
            Self::Straight => "straight",
            Self::Slant => "slant",
            Self::Steep => "steep",
        }
    }

    pub const fn factor(self) -> f32 {
        match self {
            Self::Straight => 0.0,
            Self::Slant => 1.2,
            Self::Steep => 2.4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandVariance {
    None,
    Soft,
    Wild,
}

impl HandVariance {
    pub fn from_key(value: &str) -> Self {
        match value {
            "soft" => Self::Soft,
            "wild" => Self::Wild,
            _ => Self::None,
        }
    }

    pub const fn as_key(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Soft => "soft",
            Self::Wild => "wild",
        }
    }

    pub const fn factor(self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Soft => 0.5,
            Self::Wild => 1.05,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandAxis {
    Rows,
    Columns,
}

impl HandAxis {
    pub fn from_key(value: &str) -> Self {
        if value == "columns" { Self::Columns } else { Self::Rows }
    }

    pub const fn as_key(self) -> &'static str {
        match self {
            Self::Rows => "rows",
            Self::Columns => "columns",
        }
    }
}

pub fn format_config_number(value: f64) -> String {
    if value.fract() == 0.0 { format!("{}", value as i64) } else { format!("{value}") }
}

mod tests;
