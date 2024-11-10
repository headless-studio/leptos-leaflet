use std::fmt::{Display, Formatter};

/// Options for configuring the appearance of a path join.
#[derive(Debug, Copy, Clone)]
pub enum LineJoin {
    Arcs,
    Bevel,
    Miter,
    MiterClip,
    Round,
}

impl Default for LineJoin {
    fn default() -> Self {
        Self::Round
    }
}

impl Display for LineJoin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LineJoin::Arcs => write!(f, "arcs"),
            LineJoin::Bevel => write!(f, "bevel"),
            LineJoin::Miter => write!(f, "miter"),
            LineJoin::MiterClip => write!(f, "miter-clip"),
            LineJoin::Round => write!(f, "round"),
        }
    }
}

impl AsRef<str> for LineJoin {
    fn as_ref(&self) -> &str {
        match self {
            LineJoin::Arcs => "arcs",
            LineJoin::Bevel => "bevel",
            LineJoin::Miter => "miter",
            LineJoin::MiterClip => "miter-clip",
            LineJoin::Round => "round",
        }
    }
}

/// Options for configuring the appearance of a path cap.
#[derive(Debug, Copy, Clone)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl Default for LineCap {
    fn default() -> Self {
        Self::Round
    }
}

impl Display for LineCap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LineCap::Butt => write!(f, "butt"),
            LineCap::Round => write!(f, "round"),
            LineCap::Square => write!(f, "square"),
        }
    }
}

impl AsRef<str> for LineCap {
    fn as_ref(&self) -> &str {
        match self {
            LineCap::Butt => "butt",
            LineCap::Round => "round",
            LineCap::Square => "square",
        }
    }
}

/// Options for configuring the appearance of a path fill.
#[derive(Debug, Copy, Clone)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

impl Default for FillRule {
    fn default() -> Self {
        Self::EvenOdd
    }
}

impl Display for FillRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FillRule::NonZero => write!(f, "nonzero"),
            FillRule::EvenOdd => write!(f, "evenodd"),
        }
    }
}

impl AsRef<str> for FillRule {
    fn as_ref(&self) -> &str {
        match self {
            FillRule::NonZero => "nonzero",
            FillRule::EvenOdd => "evenodd",
        }
    }
}

impl From<LineJoin> for String {
    fn from(value: LineJoin) -> Self {
        format!("{}", value)
    }
}

impl From<LineCap> for String {
    fn from(value: LineCap) -> Self {
        format!("{}", value)
    }
}

impl From<FillRule> for String {
    fn from(value: FillRule) -> Self {
        format!("{}", value)
    }
}
