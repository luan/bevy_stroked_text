use bevy::{prelude::*, sprite::Anchor};

pub struct StrokedTextPlugin;

impl Plugin for StrokedTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, refresh_text_system);
    }
}

#[derive(Component, Debug, Clone)]
pub struct StrokedText {
    pub text: String,
    pub color: Color,
    pub stroke_color: Color,
    pub font: Handle<Font>,
    pub font_size: f32,
    pub text_anchor: Anchor,
}

impl Default for StrokedText {
    fn default() -> Self {
        StrokedText {
            text: "".to_string(),
            color: Color::WHITE,
            stroke_color: Color::BLACK,
            font: Default::default(),
            font_size: 32.0,
            text_anchor: Anchor::Center,
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct StrokedTextBundle {
    /// The text to be rendered along with its font, color, and size.
    pub text: StrokedText,
    /// The transform of the text.
    pub transform: Transform,
    /// The global transform of the text.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the text.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}

impl StrokedTextBundle {
    pub fn new(stroked_text: StrokedText) -> Self {
        StrokedTextBundle {
            text: stroked_text,
            ..Default::default()
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text.text = text.to_string();
        self
    }

    pub fn with_font(mut self, font: Handle<Font>) -> Self {
        self.text.font = font;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.text.font_size = font_size;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.text.color = color;
        self
    }

    pub fn with_stroke_color(mut self, stroke_color: Color) -> Self {
        self.text.stroke_color = stroke_color;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

fn refresh_text_system(
    mut commands: Commands,
    mut query: Query<(Entity, &StrokedText, Option<&Children>), Changed<StrokedText>>,
    mut child_text_query: Query<(&mut Text, &Transform)>,
) {
    for (entity, stroked_text, children) in &mut query {
        if let Some(children) = children {
            for &child in children.iter() {
                if let Ok((mut cursor_text, transform)) = child_text_query.get_mut(child) {
                    cursor_text.sections[0].value = stroked_text.text.clone();
                    cursor_text.sections[0].style.font_size = stroked_text.font_size;
                    if transform.translation.z < 0. {
                        cursor_text.sections[0].style.color = stroked_text.stroke_color;
                    } else {
                        cursor_text.sections[0].style.color = stroked_text.color;
                    }
                }
            }
        } else {
            commands.entity(entity).with_children(|builder| {
                info!("Creating text entity {:?}", stroked_text);
                builder.spawn(Text2dBundle {
                    text: Text::from_section(
                        stroked_text.text.clone(),
                        TextStyle {
                            font: stroked_text.font.clone(),
                            font_size: stroked_text.font_size,
                            color: stroked_text.color,
                        },
                    ),
                    text_anchor: stroked_text.text_anchor,
                    ..default()
                });
                for offset in [
                    Vec3::new(1., 1., -1.),
                    Vec3::new(-1., -1., -1.),
                    Vec3::new(1., -1., -1.),
                    Vec3::new(-1., 1., -1.),
                    Vec3::new(0., 1., -1.),
                    Vec3::new(0., -1., -1.),
                    Vec3::new(1., 0., -1.),
                    Vec3::new(-1., 0., -1.),
                ] {
                    builder.spawn(Text2dBundle {
                        text: Text::from_section(
                            stroked_text.text.clone(),
                            TextStyle {
                                font: stroked_text.font.clone(),
                                font_size: stroked_text.font_size,
                                color: stroked_text.stroke_color,
                            },
                        ),
                        transform: Transform::from_translation(offset),
                        text_anchor: stroked_text.text_anchor,
                        ..default()
                    });
                }
            });
        }
    }
}
