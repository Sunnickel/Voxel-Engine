use crate::config::GlobalAssets;
use crate::states::{GamePlugin, GameState};
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::fs;
use std::io::Write;
use std::time::Duration;

pub struct AppLoadingPlugin;

#[derive(Component)]
pub struct AppLoadingTag;

impl AppLoadingPlugin {
    pub fn tag() -> AppLoadingTag {
        AppLoadingTag
    }
}

impl Plugin for AppLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AppLoading), setup)
            .add_systems(
                Update,
                check_loading.run_if(
                    in_state(GameState::AppLoading)
                        .and(on_timer(Duration::from_secs_f32(1.0 / 60.0))),
                ),
            )
            .add_systems(OnEnter(GameState::AppLoading), exit);
    }
}

#[derive(Resource, Default)]
pub struct LoadingAssets {
    pub handles: Vec<(String, UntypedHandle)>,
}

impl LoadingAssets {
    pub fn add(&mut self, name: impl Into<String>, handle: impl Into<UntypedHandle>) {
        self.handles.push((name.into(), handle.into()));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    fs::create_dir_all("run").expect("Failed to create directory");
    let mut loading = LoadingAssets::default();

    let font = asset_server.load("fonts/kodo/main.ttf");
    loading.add("font", font.clone());

    commands.insert_resource(loading);
    commands.insert_resource(GlobalAssets { font });
}

fn check_loading(
    loading: Res<LoadingAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let total = loading.handles.len();
    let loaded = loading
        .handles
        .iter()
        .filter(|(_, h)| matches!(asset_server.load_state(h.id()), LoadState::Loaded))
        .count();
    let failed: Vec<&str> = loading
        .handles
        .iter()
        .filter(|(_, h)| matches!(asset_server.load_state(h.id()), LoadState::Failed(_)))
        .map(|(name, _)| name.as_str())
        .collect();

    debug!("\rLoading assets: {}/{}", loaded, total);
    std::io::stdout().flush().unwrap();

    if !failed.is_empty() {
        println!();
        for name in failed {
            error!("Failed to load: {}", name);
        }
    }

    if loaded >= total {
        debug!("\rAll assets loaded!           ");
        next_state.set(GameState::MainMenu);
    }
}

fn exit(mut commands: Commands, query: Query<Entity, With<AppLoadingTag>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
