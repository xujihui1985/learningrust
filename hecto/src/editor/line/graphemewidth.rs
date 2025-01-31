
#[derive(Clone, Copy)]
pub enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    pub const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2),
        }
    }
}

impl From<GraphemeWidth> for usize {
    fn from(width: GraphemeWidth) -> Self {
        match width {
            GraphemeWidth::Half => 1,
            GraphemeWidth::Full => 2,
        }
    }
}
