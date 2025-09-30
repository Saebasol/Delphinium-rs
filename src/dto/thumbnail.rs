pub enum Size {
    SmallSmall,
    Small,
    SmallBig,
    Big,
}

impl Size {
    pub fn as_str(&self) -> &'static str {
        match self {
            Size::SmallSmall => "smallsmall",
            Size::Small => "small",
            Size::SmallBig => "smallbig",
            Size::Big => "big",
        }
    }
}
