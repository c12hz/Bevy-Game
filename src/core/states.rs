#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
	Loading,
	Loaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
	MainMenu,
	InGame,
}
