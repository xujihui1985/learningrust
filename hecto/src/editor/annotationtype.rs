#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationType {
    Match,
    SelectedMatch,
    Number,
    Keyword,
    Type,
    KnownValue,
    Char,
    LifetimeSpecifier,
    Comment,
}