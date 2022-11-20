#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Paused,
    Run,
}

pub struct Settings {
    pub debug: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            debug: cfg!(debug_assertions),
        }
    }
}
