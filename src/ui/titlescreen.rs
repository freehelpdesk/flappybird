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
use strum::EnumIter;

use crate::{
    player::Player,
    ui::{score::spawn_score_ui, taptap::spawn_taptap_screen},
    FlappybirdState,
};

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
    commands
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

pub fn spawn_main_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_title(&mut commands, &asset_server);
}

#[derive(Debug, Component, Default)]
pub struct Github;

fn title_button_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &TitleScreenButtons), Changed<Interaction>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut state: ResMut<NextState<FlappybirdState>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    title_screen: Query<Entity, With<TitleScreen>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        let window = window_query.single();
        let mut transform = player_query.single_mut();
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

pub struct TitlescreenPlugin;

impl Plugin for TitlescreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_title_screen)
            .add_systems(
                Update,
                title_button_system.run_if(in_state(FlappybirdState::MainTitle)),
            );
    }
}
