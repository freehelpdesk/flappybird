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
pub struct TapTapUI;

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
        .insert(TapTapUI)
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
