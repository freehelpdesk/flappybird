/*
MIT License

Copyright (c) 2024 freehelpdesk

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::time::Duration;

use bevy::scene::ron::de;
use bevy::{prelude::*, render::view::window, window::PrimaryWindow};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioControl;
use bevy_kira_audio::{Audio, AudioPlugin};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::{
    parry::query,
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::{
        AdditionalMassProperties, Collider, ColliderMassProperties, ExternalImpulse, RigidBody,
        Velocity,
    },
    rapier::prelude::RigidBodyBuilder,
    render::RapierDebugRenderPlugin,
};
use rand::Rng;
use rust_embed::Embed;
use strum::EnumIter;

fn main() {
    // check to see if 1 week elapsed from today

    App::new()
        .add_plugins((
            EmbeddedAssetPlugin::default(),
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".to_string(),
                        name: Some("Flappy Bird".to_string()),
                        //resolution: (1179. / 4., 2556. / 4.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            #[cfg(debug_assertions)]
            RapierDebugRenderPlugin::default(),
            #[cfg(debug_assertions)]
            WorldInspectorPlugin::new(),
            AudioPlugin,
        ))
        .init_state::<FlappybirdState>()
        .add_systems(Startup, setup)
        .add_systems(Update, title_button_system)
        .add_systems(
            Startup,
            spawn_main_title_screen.run_if(in_state(FlappybirdState::MainTitle)),
        )
        .insert_resource(SpawnTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
        .add_systems(
            Update,
            (
                spawn_pipes,
                move_pipes,
                deadly_touch,
                bird_rotation,
                pipe_reached,
            )
                .run_if(in_state(FlappybirdState::InGame)),
        )
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (
                player_movement,
                movement_restrictions,
                player_velocity_limiter,
            )
                .run_if(
                    in_state(FlappybirdState::TapTap).or_else(in_state(FlappybirdState::InGame)),
                ),
        )
        .add_systems(Update, animate_sprite)
        .add_systems(Update, (move_ground, despawn_and_spawn_ground))
        .add_systems(Update, (move_sky, despawn_and_spawn_sky))
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum FlappybirdState {
    #[default]
    MainTitle,
    TapTap,
    InGame,
    GameOver,
    Settings,
}

#[derive(Component)]
pub struct TitleScreen;

#[derive(Reflect, Debug, Component, Clone, Copy, EnumIter)]
pub enum TitleScreenButtons {
    Play,
    Github,
    Settings,
    Scoreboard,
    Exit,
}

pub fn spawn_title(commands: &mut Commands, asset_server: &AssetServer) {
    println!("Title spawned");
    let title_screen = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            ..default()
        })
        .insert(TitleScreen)
        .with_children(|commands| {
            // Title node
            commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0), // Adjust this value based on your needs
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|commands| {
                    commands.spawn(ImageBundle {
                        style: Style {
                            width: Val::Auto,
                            height: Val::Px(18. * 3.),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("embedded://sprites/ui/flappy.png")),
                        ..Default::default()
                    });
                });

            // Buttons node
            commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        //margin: UiRect::horizontal(Val::Px(10.0)), // Space between buttons
                        ..default()
                    },
                    ..default()
                })
                .with_children(|commands| {
                    commands
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(31. * 3.),
                                height: Val::Px(18. * 3.),
                                margin: UiRect::horizontal(Val::Px(10.0)),
                                ..default()
                            },
                            image: UiImage::new(
                                asset_server.load("embedded://sprites/ui/board.png"),
                            ),
                            ..Default::default()
                        })
                        .insert(TitleScreenButtons::Scoreboard);

                    commands
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(31. * 3.),
                                height: Val::Px(18. * 3.),
                                margin: UiRect::horizontal(Val::Px(10.0)),
                                ..default()
                            },
                            image: UiImage::new(
                                asset_server.load("embedded://sprites/ui/play.png"),
                            ),
                            ..Default::default()
                        })
                        .insert(TitleScreenButtons::Play);

                    commands
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(31. * 3.),
                                height: Val::Px(18. * 3.),
                                margin: UiRect::horizontal(Val::Px(10.0)),
                                ..default()
                            },
                            image: UiImage::new(
                                asset_server.load("embedded://sprites/ui/settings.png"),
                            ),
                            ..Default::default()
                        })
                        .insert(TitleScreenButtons::Settings);
                });
        });
}

pub fn spawn_main_title_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    spawn_title(&mut commands, &asset_server);
}

#[derive(Debug, Component, Default)]
pub struct Github;

fn title_button_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &TitleScreenButtons), Changed<Interaction>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut state: ResMut<NextState<FlappybirdState>>,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    mut title_screen: Query<Entity, With<TitleScreen>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        let window = window_query.get_single().unwrap();
        let (player_entity, mut transform) = player_query.single_mut();
        println!("{:?}", button);
        match *interaction {
            Interaction::Pressed => {
                // Handle button click
                audio.play(
                    asset_server
                        .get_handle("embedded://audio/sfx_swooshing.ogg")
                        .unwrap_or(asset_server.load("embedded://audio/sfx_swooshing.ogg")),
                );
                match button {
                    TitleScreenButtons::Play => {
                        println!("Play button clicked!");
                        // Destroy the title screen, spawn tap tap screen
                        commands
                            .get_entity(title_screen.single())
                            .unwrap()
                            .despawn_recursive();
                        // set the state to TapTap
                        transform.translation =
                            Vec3::new(window.width() / 6., window.height() / 2., 2.); // set player to the gameplay area
                        spawn_score_ui(&mut commands, &asset_server);
                        spawn_taptap_screen(&mut commands, &asset_server);
                        state.set(FlappybirdState::TapTap);
                    }
                    TitleScreenButtons::Github => {
                        println!("Github button clicked!");
                    }
                    TitleScreenButtons::Settings => {
                        println!("Settings button clicked!");
                        state.set(FlappybirdState::Settings);
                    }
                    TitleScreenButtons::Exit => {
                        println!("Exit button clicked!");
                        //state.set(FlappybirdState::InGame);
                    }
                    TitleScreenButtons::Scoreboard => {
                        println!("Scoreboard button clicked!");
                    }
                }
            }
            Interaction::Hovered => {
                // Handle button hover
                //*material = materials.add(Color::srgb(0.5, 0.5, 0.5));
            }
            Interaction::None => {
                // Handle button normal state
                //*material = materials.add(Color::srgb(1.0, 1.0, 1.0));
            }
        }
    }
}

#[derive(Debug, Component)]
pub struct ScoreUI;

fn spawn_score_ui(commands: &mut Commands, asset_server: &AssetServer) {
    let shared_style = Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        top: Val::Percent(25.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };
    commands.spawn((
        TextBundle {
            style: shared_style.clone(),
            text: Text::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("embedded://fonts/inside.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            ..default()
        },
        Score::default(),
    ));
    commands.spawn((
        TextBundle {
            style: shared_style,
            text: Text::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("embedded://fonts/outside.ttf"),
                    font_size: 60.0,
                    color: Color::BLACK,
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            ..default()
        },
        Score::default(),
    ));
}

#[derive(Debug, Component, Default)]
pub struct Score;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Sky;

#[derive(Component)]
struct Speed(f32);

pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let primary_window = window_query.get_single().unwrap();

    let ground_width = 168.;
    let ground_texture = asset_server.load("embedded://sprites/world/land.png");

    let ground_scale = Vec3::splat(3.);

    let number_of_grounds =
        (primary_window.width() / (ground_width * ground_scale.x)).ceil() as u32 + 1;

    for i in 0..number_of_grounds {
        commands.spawn((
            SpriteBundle {
                texture: ground_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        i as f32 * (ground_width * ground_scale.x),
                        56. / 2.,
                        2.,
                    ),
                    scale: ground_scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            Ground,
            Speed(150.0),
            Collider::cuboid(168. / 2., 56. / 2.),
            ActiveEvents::COLLISION_EVENTS,
        ));
    }

    let sky_width = 144.;
    let sky_texture = asset_server.load("embedded://sprites/world/day-sky.png");

    let sky_scale = Vec3::splat(3.);

    let number_of_skys = (primary_window.width() / (sky_width * sky_scale.x)).ceil() as u32 + 1;

    for i in 0..number_of_skys {
        commands.spawn((
            SpriteBundle {
                texture: sky_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        i as f32 * (sky_width * sky_scale.x),
                        primary_window.height() / 2.,
                        0.,
                    ),
                    scale: sky_scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            Sky,
            Speed(20.0),
        ));
    }
}

fn move_ground(
    time: Res<Time>,
    mut query: Query<(&Speed, &mut Transform), With<Ground>>,
    player: Query<&Player>,
) {
    for (speed, mut transform) in query.iter_mut() {
        if let Ok(player) = player.get_single() {
            if !player.is_dead {
                transform.translation.x -= speed.0 * time.delta_seconds();
            }
        }
    }
}

fn move_pipes(
    time: Res<Time>,
    mut query: Query<(&Speed, &mut Transform), With<Pipe>>,
    player: Query<&Player>,
) {
    for (speed, mut transform) in query.iter_mut() {
        if let Ok(player) = player.get_single() {
            if !player.is_dead {
                transform.translation.x -= speed.0 * time.delta_seconds();
            }
        }
    }
}

fn move_sky(
    time: Res<Time>,
    mut sky_query: Query<(&Speed, &mut Transform), With<Sky>>,
    player: Query<&Player>,
) {
    for (speed, mut transform) in sky_query.iter_mut() {
        if let Ok(player) = player.get_single() {
            if !player.is_dead {
                transform.translation.x -= speed.0 * time.delta_seconds();
            }
        }
    }
}

fn despawn_and_spawn_ground(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Ground>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    let texture_handle = asset_server
        .get_handle("embedded://sprites/world/land.png")
        .unwrap();
    let texture_width = 168.0; // Adjust based on your texture width
    let texture_scale = Vec3::splat(3.); // Adjust based on your texture scale

    let effective_width = texture_width * texture_scale.x;

    let mut ground_entities: Vec<(Entity, &Transform)> = query.iter_mut().collect();
    ground_entities.sort_by(|a, b| a.1.translation.x.partial_cmp(&b.1.translation.x).unwrap());

    // Despawn entities that have moved out of the left boundary
    for (entity, transform) in ground_entities.iter() {
        if transform.translation.x <= -effective_width / 2. {
            println!("Despawning entity");
            commands.entity(*entity).despawn();
            // Spawn new ground entities on the right if needed
            let rightmost_x = ground_entities
                .last()
                .map_or(-window.width() / 2., |(_, transform)| {
                    transform.translation.x
                });
            let new_x = rightmost_x + effective_width;
            commands.spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(new_x, 56. / 2., 2.), // Adjust based on your texture width, scale, and position
                        scale: texture_scale,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Collider::cuboid(168. / 2., 56. / 2.),
                ActiveEvents::COLLISION_EVENTS,
                Ground,
                Speed(150.0),
            ));
        }
    }
}

fn despawn_and_spawn_sky(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Sky>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let texture_handle = asset_server
        .get_handle("embedded://sprites/world/day-sky.png")
        .unwrap();
    let texture_width = 144.0; // Adjust based on your texture width
    let texture_scale = Vec3::splat(3.); // Adjust based on your texture scale

    let effective_width = texture_width * texture_scale.x;

    let mut sky_entities: Vec<(Entity, &Transform)> = query.iter_mut().collect();
    sky_entities.sort_by(|a, b| a.1.translation.x.partial_cmp(&b.1.translation.x).unwrap());

    // Despawn entities that have moved out of the left boundary
    for (entity, transform) in sky_entities.iter() {
        if transform.translation.x <= -effective_width / 2. {
            println!("Despawning entity");
            commands.entity(*entity).despawn();
            // Spawn new ground entities on the right if needed
            let rightmost_x = sky_entities
                .last()
                .map_or(-window.width() / 2., |(_, transform)| {
                    transform.translation.x
                });
            let new_x = rightmost_x + effective_width;
            commands.spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(new_x, window.height() / 2., 0.),
                        scale: texture_scale,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Sky,
                Speed(20.0),
            ));
        }
    }
}

#[derive(Debug, Component, Default)]
pub struct Player {
    name: String,
    is_dead: bool,
    is_started: bool,
    score: u64,
    last_score_time: f32,
}

// fn move_ground(time: Res<Time>, mut query: Query<(&Speed, &mut Transform), With<Ground>>) {
//     for (speed, mut transform) in query.iter_mut() {
//         transform.translation.x -= speed.0 * time.delta_seconds();
//     }
// }

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let primary_window = window_query.get_single().unwrap();

    // Load texture for the bird, can be randomized later
    let texture = asset_server.load("embedded://sprites/birds/yellow/bird-sheet.png");
    // the sprite sheet has 4 sprites arranged in a row, and they are all 17px x 12px
    let layout = TextureAtlasLayout::from_grid(UVec2::new(17, 12), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    let mut bird_transform = Transform::from_xyz(
        primary_window.width() / 2.,
        primary_window.height() / 2. + 70.,
        2.,
    );

    bird_transform.scale = Vec3::splat(3.);

    commands.spawn((
        SpriteBundle {
            transform: bird_transform,
            texture: texture.clone(),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_indices.first,
        },
        Player {
            name: "helpdesk".to_string(),
            ..Default::default()
        },
        animation_indices,
        RigidBody::Fixed,
        ExternalImpulse::default(),
        ColliderMassProperties::Density(0.),
        AdditionalMassProperties::Mass(10.0),
        Velocity::default(),
        Collider::capsule_x(1., 6.),
        AnimationTimer(Timer::new(Duration::from_millis(100), TimerMode::Repeating)),
        ScoreTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));
}

pub fn spawn_taptap_screen(commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(TapTap)
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(92. * 3.),
                    height: Val::Px(25. * 3.),
                    position_type: PositionType::Relative,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    ..Default::default()
                },
                image: UiImage::from(asset_server.load("embedded://sprites/ui/getready.png")),
                ..Default::default()
            });

            parent.spawn((ImageBundle {
                style: Style {
                    width: Val::Px(57. * 3.),
                    height: Val::Px(49. * 3.),
                    position_type: PositionType::Relative,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: UiImage::from(asset_server.load("embedded://sprites/ui/taptap.png")),
                ..Default::default()
            },));
        });
}

#[derive(Debug, Component)]
pub struct TapTap;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let primary_window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            primary_window.width() / 2.,
            primary_window.height() / 2.,
            0.,
        ),
        ..Default::default()
    });
}

pub const PLAYER_SPEED: f32 = 500.; // M/S

pub fn player_velocity_limiter(mut player_query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut v) = player_query.get_single_mut() {
        if v.linvel.y > PLAYER_SPEED {
            v.linvel.y = PLAYER_SPEED;
        }
        //println!("m/s: {}", v.linvel);
    }
}

const UPWARD_ROTATION_LIMIT: f32 = 25.; // Adjust for faster upward rotation
const DOWNWARD_ROTATION_LIMIT: f32 = -90.; // Adjust for slower downward rotation

pub fn player_movement(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut ExternalImpulse,
            &mut Velocity,
            &mut Player,
            Entity,
        ),
        With<Player>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
    taptap_query: Query<Entity, With<TapTap>>,
    mut mutable_state: ResMut<NextState<FlappybirdState>>,
    current_state: Res<State<FlappybirdState>>,
) {
    if let Ok((mut transform, mut impulse, mut velocity, player, entity)) =
        player_query.get_single_mut()
    {
        if (keyboard_input.just_pressed(KeyCode::Space)
            || mouse_input.just_pressed(MouseButton::Left))
            && transform.translation.y < window_query.get_single().unwrap().height()
            && !player.is_dead
        {
            if current_state.get() == &FlappybirdState::TapTap {
                for entity in taptap_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                mutable_state.set(FlappybirdState::InGame);
                commands
                    .get_entity(entity)
                    .unwrap()
                    .insert(RigidBody::Dynamic);
                //state.set(FlappybirdState::InGame);
            }
            println!("Space pressed");
            audio.play(
                asset_server
                    .get_handle("embedded://audio/sfx_wing.ogg")
                    .unwrap_or(asset_server.load("embedded://audio/sfx_wing.ogg")),
            );
            if velocity.linvel.y < 0. {
                velocity.linvel.y = 0.;
            }
            impulse.impulse = Vec2::new(0., 4000.);
            transform.rotation = Quat::from_rotation_z(f32::to_radians(UPWARD_ROTATION_LIMIT));
        }
    }
}

fn movement_restrictions(mut query: Query<&mut Velocity, With<Player>>) {
    for mut rb_vel in query.iter_mut() {
        rb_vel.linvel.x = 0.0; // Prevent x-axis movement
    }
}

const ROTATION_INTERPOLATION_SPEED: f32 = 2.; // Base speed of rotation interpolation
const FLAP_HOLD_DURATION: f32 = 0.5; // Duration to hold upward angle in seconds

fn bird_rotation(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform, &Player), With<Player>>,
) {
    for (velocity, mut transform, player) in &mut query {
        if player.is_dead {
            break;
        }

        let delta_time = time.delta_seconds();
        let target_rotation = if velocity.linvel.y > -300. {
            f32::to_radians(UPWARD_ROTATION_LIMIT)
        } else {
            f32::to_radians(DOWNWARD_ROTATION_LIMIT)
        };

        // Calculate the interpolation speed based on the downward velocity
        let interpolation_speed =
            ROTATION_INTERPOLATION_SPEED + velocity.linvel.y.abs() * delta_time;

        // Perform smooth interpolation
        let current_z_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
        let new_z_rotation = current_z_rotation
            + (target_rotation - current_z_rotation) * interpolation_speed * delta_time;
        transform.rotation = Quat::from_rotation_z(new_z_rotation);
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Debug, Component)]
struct Pipe;

fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    player: Query<&Player>,
) {
    timer.0.tick(time.delta());
    if let Ok(player) = player.get_single() {
        if timer.0.just_finished() && !player.is_dead {
            let mut rng = rand::thread_rng();
            let window = window_query.get_single().unwrap();
            println!("Spawning");

            let mut node_collider = Transform::from_xyz(
                window.width() + 50.,
                (window.height() / 2.) + (rng.gen_range(-3..6) as f32 * 40.),
                1.,
            );
            node_collider.scale = Vec3::splat(2.);

            const PIPE_DISTANCE: f32 = 40.;

            commands
                .spawn((
                    SpatialBundle::from(node_collider),
                    Collider::cuboid(15. / 2., PIPE_DISTANCE),
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Pipe,
                    Speed(150.0),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            texture: asset_server
                                .get_handle("embedded://sprites/world/pipe-up.png")
                                .unwrap_or(
                                    asset_server.load("embedded://sprites/world/pipe-up.png"),
                                ),
                            transform: Transform::from_xyz(0., (-512. / 2.) - PIPE_DISTANCE, 0.),
                            ..Default::default()
                        },
                        Collider::cuboid(30. / 2., 512. / 2.),
                        ActiveEvents::COLLISION_EVENTS,
                    ));
                    parent.spawn((
                        SpriteBundle {
                            texture: asset_server
                                .get_handle("embedded://sprites/world/pipe-down.png")
                                .unwrap_or(
                                    asset_server.load("embedded://sprites/world/pipe-down.png"),
                                ),
                            transform: Transform::from_xyz(0., (512. / 2.) + PIPE_DISTANCE, 0.),
                            ..Default::default()
                        },
                        Collider::cuboid(30. / 2., 512. / 2.),
                        ActiveEvents::COLLISION_EVENTS,
                    ));
                });
        }
    }
}

#[derive(Event)]
struct LevelUpEvent(Entity);

#[derive(Component, Deref, DerefMut)]
struct ScoreTimer(Timer);

fn pipe_reached(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, With<Pipe>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &mut ScoreTimer), With<Player>>,
    mut score_query: Query<&mut Text, With<Score>>,
    time: Res<Time>,
) {
    const SCORE_COOLDOWN: f32 = 1.0; // Cooldown time in seconds

    for event in collision_events.read().into_iter() {
        match event {
            CollisionEvent::Started(_, _, _) => {}
            CollisionEvent::Stopped(entity1, entity2, _) => {
                if query.get(*entity1).is_ok() || query.get(*entity2).is_ok() {
                    if let Ok((mut player, _)) = player_query.get_single_mut() {
                        let current_time = time.elapsed_seconds() as f32;

                        if current_time - player.last_score_time >= SCORE_COOLDOWN {
                            player.score += 1;
                            player.last_score_time = current_time; // Update the last score time

                            for mut text in score_query.iter_mut() {
                                text.sections[0].value = player.score.to_string();
                            }

                            audio.play(
                                asset_server
                                    .get_handle("embedded://audio/sfx_point.ogg")
                                    .unwrap_or_else(|| {
                                        asset_server.load("embedded://audio/sfx_point.ogg")
                                    }),
                            );
                        }
                    }
                }
            }
        }
    }
}

fn deadly_touch(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, Without<Pipe>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<&mut Player, With<Player>>,
    mut score_query: Query<&mut Text, With<Score>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if player.is_dead {
            return;
        }
        for event in collision_events.read().into_iter() {
            match event {
                CollisionEvent::Started(entity1, entity2, _) => {
                    if query.get(*entity1).is_ok() && query.get(*entity2).is_ok() {
                        player.is_dead = true;
                        audio.play(
                            asset_server
                                .get_handle("embedded://audio/sfx_hit.ogg")
                                .unwrap_or(asset_server.load("embedded://audio/sfx_hit.ogg")),
                        );
                        println!("touched");
                    }
                }
                CollisionEvent::Stopped(_, _, _) => {}
            }
        }
    }
}