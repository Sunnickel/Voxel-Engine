mod app_loading;
mod game;
mod game_loading;
mod menu;

use bevy::asset::Handle;
use bevy::prelude::{Font, Resource, States};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    AppLoading,
    GameLoading,
    MainMenu,
    InGame,
}

pub use app_loading::AppLoadingPlugin;
pub use menu::MenuPlugin;
pub use game::GamePlugin;
pub use game_loading::GameLoadingPlugin;
