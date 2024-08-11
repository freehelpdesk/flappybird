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

use animation::{animate_sprite, AnimationIndices, AnimationTimer};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::*;

use crate::{world::pipes::Pipe, FlappybirdState};

pub mod animation;
pub mod controller;

#[derive(Debug, Component, Default)]
pub struct Player {
    pub name: String,
    pub score: u64,
    pub last_score_time: f32,
}

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
    ));
}

fn deadly_touch(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, Without<Pipe>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    state: Res<State<FlappybirdState>>,
    mut next_state: ResMut<NextState<FlappybirdState>>,
) {
    if state.get() != &FlappybirdState::InGame {
        return;
    }
    for event in collision_events.read().into_iter() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if query.get(*entity1).is_ok() && query.get(*entity2).is_ok() {
                    next_state.set(FlappybirdState::GameOver);
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

use controller::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, animate_sprite)
            .add_systems(
                Update,
                (
                    player_movement,
                    player_movement_restrictions,
                    player_velocity_limiter,
                )
                    .run_if(
                        in_state(FlappybirdState::TapTap)
                            .or_else(in_state(FlappybirdState::InGame)),
                    ),
            )
            .add_systems(
                Update,
                (player_rotation, deadly_touch).run_if(in_state(FlappybirdState::InGame)),
            );
    }
}
