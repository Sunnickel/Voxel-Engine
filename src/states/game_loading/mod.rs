use crate::config::{BlockRegistry, Seed, SpawnChunkGenerated, WorldReady};
use crate::states::GameState;
use crate::world::seeding::SeedPlugin;
use crate::world::{setup_chunk, setup_sky, WorldPlugin};
use bevy::prelude::*;

use crate::world::bioms::load_bioms;
use crate::world::blocks::load_blocks;

pub struct GameLoadingPlugin;

#[derive(Component)]
pub struct GameLoadingTag;

impl GameLoadingPlugin {
    pub fn tag() -> GameLoadingTag {
        GameLoadingTag
    }
}

impl Plugin for GameLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameLoading),
            (setup, setup_chunk, setup_sky).chain(),
        )
        .add_plugins((SeedPlugin, WorldPlugin))
        .add_systems(
            Update,
            loading_update.run_if(in_state(GameState::GameLoading)),
        )
        .add_systems(OnExit(GameState::GameLoading), exit);
    }
}

pub fn setup(
    mut commands: Commands,
    mut seed: ResMut<Seed>,
    block_registry: ResMut<BlockRegistry>,
) {
    commands.spawn((Text::new("Loading..."), GameLoadingPlugin::tag()));

    load_blocks(&mut commands);
    load_bioms(&mut commands, &block_registry);

    seed.0 = 1234;
}

fn loading_update(
    mut next: ResMut<NextState<GameState>>,
    generated: ResMut<SpawnChunkGenerated>,
    mut world_ready: ResMut<WorldReady>,
) {
    if world_ready.0 {
        return;
    }

    if generated.0 {
        info!("Chunk colliders ready, world is ready");
        world_ready.0 = true;
        next.set(GameState::InGame);
    }
}

fn exit(mut commands: Commands, query: Query<Entity, With<GameLoadingTag>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
