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

use crate::{ui::taptap::TapTapUI, FlappybirdState};

use super::Player;

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
        (&mut Transform, &mut ExternalImpulse, &mut Velocity, Entity),
        With<Player>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
    taptap_query: Query<Entity, With<TapTapUI>>,
    mut mutable_state: ResMut<NextState<FlappybirdState>>,
    current_state: Res<State<FlappybirdState>>,
) {
    if let Ok((mut transform, mut impulse, mut velocity, entity)) = player_query.get_single_mut() {
        if (keyboard_input.just_pressed(KeyCode::Space)
            || mouse_input.just_pressed(MouseButton::Left))
            && transform.translation.y < window_query.get_single().unwrap().height()
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

pub fn player_movement_restrictions(mut query: Query<&mut Velocity, With<Player>>) {
    for mut rb_vel in query.iter_mut() {
        rb_vel.linvel.x = 0.0; // Prevent x-axis movement
    }
}

const ROTATION_INTERPOLATION_SPEED: f32 = 2.; // Base speed of rotation interpolation
pub fn player_rotation(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    for (velocity, mut transform) in &mut query {
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
