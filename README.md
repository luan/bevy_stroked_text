# bevy_stroked_text

[![crates.io](https://img.shields.io/crates/v/bevy_stroked_text.svg)](https://crates.io/crates/bevy_stroked_text)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![docs.rs](https://img.shields.io/docsrs/bevy_stroked_text)](https://docs.rs/bevy_stroked_text)

A Bevy plugin for stroked text rendering. This plugin is experimental and may not have performance or quality suitable for production use.
The way this plugin works currently is by rendering 8 copies of the text with a 1 pixel offset in each direction, then rendering the original text on top of that. This is not the most efficient way to render stroked text, but it is the easiest to implement and works well enough for now.

![CleanShot 2024-02-08 at 22 55 49@2x](https://github.com/luan/bevy_stroked_text/assets/223760/8d553eaf-3778-46a2-8f99-ffede2f72adc)

## Usage

Add the plugin to your app

```rust ignore
App::new()
    .add_plugins((DefaultPlugins, StrokedTextPlugin))
    .run();
```

Add a StrokedTextBundle to your entity

```rust ignore
    commands.spawn(
        StrokedTextBundle::new(StrokedText {
            text: "Hello, world!".to_string(),
            font_size: 32.0,
            text_anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        })
        .with_transform(Transform::from_translation(Vec3::Z).with_scale(Vec3::splat(0.25))),
    );
```

## Bevy Version

| bevy | bevy_stroked_text |
| ---- | ----------------- |
| 0.12 | 0.10, main        |

## License

`bevy_stroked_text` is dual-licensed under either

- MIT License (./LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributions

PRs welcome!
