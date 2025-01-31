#[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
pub enum SearchDirection {
    #[default]
    Forward,
    Backward,
}
