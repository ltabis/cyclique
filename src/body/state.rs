
pub struct State {
    pub paused: bool,
    pub debug: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            paused: true,
            debug: cfg!(debug_assertions),
        }
    }
}
