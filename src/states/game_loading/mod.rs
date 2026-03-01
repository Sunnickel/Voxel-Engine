use crate::config::{BlockRegistry, Seed, SpawnChunkGenerated, WorldReady};
use crate::states::GameState;
use crate::world::chunks::Chunk;
use crate::world::seeding::SeedPlugin;
use crate::world::WorldPlugin;
use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::world::blocks::BlockDef;
use std::fs;
use std::path::Path;

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
        app.add_systems(OnEnter(GameState::GameLoading), setup)
            .add_plugins((SeedPlugin, WorldPlugin))
            .add_systems(
                Update,
                loading_update.run_if(in_state(GameState::GameLoading)),
            )
            .add_systems(OnExit(GameState::GameLoading), exit);
    }
}

fn setup(mut commands: Commands, mut seed: ResMut<Seed>) {
    commands.spawn((Text::new("Loading..."), GameLoadingPlugin::tag()));

    let mut registry = BlockRegistry::default();

    let dir = Path::new("assets/blocks");

    let entries = fs::read_dir(dir)
        .expect("blocks folder missing");

    for entry in entries {
        let path = entry.unwrap().path();

        if path.extension().and_then(|s| s.to_str()) != Some("ron") {
            continue;
        }

        let text = fs::read_to_string(&path)
            .expect("failed reading block ron");

        let def: BlockDef =
            ron::from_str(&text).expect("invalid block ron");

        let id = registry.defs.len() as u16;

        registry.name_to_id.insert(def.id.clone(), id);
        registry.defs.push(def);
    }

    registry.air = registry.get_or_air("air");

    commands.insert_resource(registry);

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
