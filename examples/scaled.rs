/// This example demonstrates how to use the `StrokedText` component to render
/// text with a custom scale in order to reduce blurriness.
use bevy::prelude::*;
use bevy_stroked_text::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_plugins((DefaultPlugins, StrokedTextPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            near: -1000.0,
            scale: 0.25,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(
        StrokedTextBundle::new(StrokedText {
            text: "Hello, world!".to_string(),
            font_size: 32.0,
            text_anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        })
        .with_transform(Transform::from_translation(Vec3::Z).with_scale(Vec3::splat(0.25))),
    );

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(
        StrokedTextBundle::new(StrokedText {
            text: "Custom font".to_string(),
            font,
            font_size: 32.0,
            text_anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        })
        .with_transform(
            Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)).with_scale(Vec3::splat(0.25)),
        ),
    );
}
