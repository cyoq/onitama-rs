#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InProgress,
    GameEnd,
    Out,
}
