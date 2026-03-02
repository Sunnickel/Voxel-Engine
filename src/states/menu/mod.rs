use crate::config::GlobalAssets;
use crate::states::GameState;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy_lunex::prelude::*;

pub struct MenuPlugin;

#[derive(Component)]
pub struct MenuTag;

impl MenuPlugin {
    pub const fn tag() -> MenuTag {
        MenuTag
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(OnExit(GameState::MainMenu), exit);
    }
}

fn setup(mut commands: Commands, global_assets: Res<GlobalAssets>) {
    commands.spawn((
        Camera2d,
        UiSourceCamera::<0>,
        Transform::from_translation(Vec3::Z * 1000.0),
        RenderLayers::from_layers(&[0, 1]),
        MenuPlugin::tag(),
    ));

    commands
        .spawn((
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
            MenuPlugin::tag(),
        ))
        .with_children(|ui| {
            // Title
            ui.spawn((
                Name::new("Title"),
                UiLayout::window()
                    .anchor(Anchor::CENTER)
                    .pos(Rl((50.0, 30.0)))
                    .pack(),
                UiTextSize::from(Rh(8.0)),
                Text2d::new("Voxel Engine"),
                TextFont {
                    font: global_assets.font.clone(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            ui.spawn((
                Name::new("Play"),
                UiLayout::window()
                    .anchor(Anchor::CENTER)
                    .pos(Rl((50.0, 50.0)))
                    .size(Rl((20.0, 8.0)))
                    .pack(),
                UiColor::new(vec![
                    (UiBase::id(), Color::srgb(0.5, 0.5, 0.5)),
                    (UiHover::id(), Color::srgb(0.0, 0.4, 0.0)),
                ]),
                UiHover::new().forward_speed(20.0).backward_speed(8.0),
                Sprite::default(),
            ))
                .with_children(|btn| {
                    btn.spawn((
                        Name::new("Label"),
                        UiLayout::window()
                            .anchor(Anchor::CENTER)
                            .pos(Rl((50.0, 50.0)))
                            .pack(),
                        UiTextSize::from(Rh(60.0)),
                        Text2d::new("Play"),
                        TextFont {
                            font: global_assets.font.clone(),
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    ));
                })
                .observe(
                    |trigger: On<Pointer<Over>>, mut query: Query<&mut UiHover>| {
                        if let Ok(mut hover) = query.get_mut(trigger.entity) {
                            hover.enable = true;
                        }
                    },
                )
                .observe(
                    |trigger: On<Pointer<Out>>, mut query: Query<&mut UiHover>| {
                        if let Ok(mut hover) = query.get_mut(trigger.entity) {
                            hover.enable = false;
                        }
                    },
                )
                .observe(
                    |_: On<Pointer<Click>>, mut next: ResMut<NextState<GameState>>| {
                        next.set(GameState::GameLoading);
                    },
                );

            ui.spawn((
                Name::new("Quit"),
                UiLayout::window()
                    .anchor(Anchor::CENTER)
                    .pos(Rl((50.0, 62.0)))
                    .size(Rl((20.0, 8.0)))
                    .pack(),
                UiColor::new(vec![
                    (UiBase::id(), Color::srgb(0.5, 0.5, 0.5)),
                    (UiHover::id(), Color::srgb(0.0, 0.4, 0.0)),
                ]),
                UiHover::new().forward_speed(20.0).backward_speed(8.0),
                Sprite::default(),
            ))
                .with_children(|btn| {
                    btn.spawn((
                        Name::new("Label"),
                        UiLayout::window()
                            .anchor(Anchor::CENTER)
                            .pos(Rl((50.0, 50.0)))
                            .pack(),
                        UiTextSize::from(Rh(60.0)),
                        Text2d::new("Quit"),
                        TextFont {
                            font: global_assets.font.clone(),
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    ));
                })
                .observe(
                    |trigger: On<Pointer<Over>>, mut query: Query<&mut UiHover>| {
                        if let Ok(mut hover) = query.get_mut(trigger.entity) {
                            hover.enable = true;
                        }
                    },
                )
                .observe(
                    |trigger: On<Pointer<Out>>, mut query: Query<&mut UiHover>| {
                        if let Ok(mut hover) = query.get_mut(trigger.entity) {
                            hover.enable = false;
                        }
                    },
                )
                .observe(|_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>| {
                    let _ = exit.write(AppExit::Success);
                });
        });
}

fn exit(mut commands: Commands, query: Query<Entity, With<MenuTag>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
