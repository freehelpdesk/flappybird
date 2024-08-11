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

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct ScoreUI;

pub fn spawn_score_ui(commands: &mut Commands, asset_server: &AssetServer) {
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
        ScoreUI,
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
        ScoreUI,
    ));
}
