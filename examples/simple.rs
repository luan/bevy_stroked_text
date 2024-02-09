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
    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        StrokedTextBundle::new(StrokedText {
            text: "Hello, world!".to_string(),
            font_size: 32.0,
            text_anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        })
        .with_transform(Transform::from_translation(Vec3::Z)),
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
        .with_transform(Transform::from_translation(Vec3::new(0.0, 100.0, 0.0))),
    );
}
