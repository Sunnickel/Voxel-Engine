use crate::player::PlayerPlugin;
use crate::states::GameState;
use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

pub struct GamePlugin;

#[derive(Component)]
pub struct GameTag;

impl GamePlugin {
    pub fn tag() -> GameTag {
        GameTag
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(OnExit(GameState::InGame), exit)
            .add_plugins((PhysicsPlugins::default(), PlayerPlugin));
    }
}

fn setup(mut commands: Commands, mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Confined;
}

fn exit(
    mut commands: Commands,
    query: Query<Entity, With<GameTag>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;

    for e in &query {
        commands.entity(e).despawn();
        info!("Despawning entity {:?}", e);
    }
}
