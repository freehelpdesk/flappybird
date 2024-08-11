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
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{player::Player, ui::score::ScoreUI, world::Speed, FlappybirdState};

use super::SpawnTimer;

#[derive(Debug, Component)]
pub struct Pipe;

fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
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
                            .unwrap_or(asset_server.load("embedded://sprites/world/pipe-up.png")),
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
                            .unwrap_or(asset_server.load("embedded://sprites/world/pipe-down.png")),
                        transform: Transform::from_xyz(0., (512. / 2.) + PIPE_DISTANCE, 0.),
                        ..Default::default()
                    },
                    Collider::cuboid(30. / 2., 512. / 2.),
                    ActiveEvents::COLLISION_EVENTS,
                ));
            });
    }
}

fn move_pipes(time: Res<Time>, mut query: Query<(&Speed, &mut Transform), With<Pipe>>) {
    for (speed, mut transform) in query.iter_mut() {
        transform.translation.x -= speed.0 * time.delta_seconds();
    }
}

fn pipe_reached(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, With<Pipe>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<&mut Player, With<Player>>,
    mut score_query: Query<&mut Text, With<ScoreUI>>,
    time: Res<Time>,
) {
    const SCORE_COOLDOWN: f32 = 1.0; // Cooldown time in seconds

    for event in collision_events.read().into_iter() {
        match event {
            CollisionEvent::Started(_, _, _) => {}
            CollisionEvent::Stopped(entity1, entity2, _) => {
                if query.get(*entity1).is_ok() || query.get(*entity2).is_ok() {
                    if let Ok(mut player) = player_query.get_single_mut() {
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

fn despawn_pipes(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
    let effective_pipe_width = 30. * 3.;
    for (entity, pipe) in query.iter() {
        if pipe.translation.x <= -effective_pipe_width / 2. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
            .add_systems(
                Update,
                (spawn_pipes, move_pipes, pipe_reached, despawn_pipes)
                    .run_if(in_state(FlappybirdState::InGame)),
            );
    }
}
