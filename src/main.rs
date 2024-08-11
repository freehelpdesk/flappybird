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

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;
use ui::titlescreen::TitlescreenPlugin;
use world::ground::{Ground, GroundPlugin};
use world::pipes::PipePlugin;
use world::sky::{Sky, SkyPlugin};
use world::Speed;

pub mod player;
pub mod ui;
pub mod world;

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
            //#[cfg(debug_assertions)]
            //WorldInspectorPlugin::new(),
            AudioPlugin,
            PlayerPlugin,
            TitlescreenPlugin,
            PipePlugin,
            SkyPlugin,
            GroundPlugin,
        ))
        .init_state::<FlappybirdState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_camera)
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
