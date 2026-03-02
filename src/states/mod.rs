mod app_loading;
mod game;
mod game_loading;
mod menu;

use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    AppLoading,
    GameLoading,
    MainMenu,
    InGame,
}

pub use app_loading::AppLoadingPlugin;
pub use game::GamePlugin;
pub use game_loading::GameLoadingPlugin;
pub use menu::MenuPlugin;
