use bevy::{prelude::*, sprite::Anchor, text::BreakLineOn};

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
    pub justify_text: JustifyText,
    pub linebreak_behavior: BreakLineOn,
    pub stroke_width: f32,
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
            justify_text: JustifyText::default(),
            linebreak_behavior: BreakLineOn::WordBoundary,
            stroke_width: 1.0,
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

    pub fn with_justify(mut self, justify: JustifyText) -> Self {
        self.text.justify_text = justify;
        self
    }

    pub fn with_linebreak(mut self, linebreak: BreakLineOn) -> Self {
        self.text.linebreak_behavior = linebreak;
        self
    }
}

trait TextLineBreakExt {
    fn with_linebreak(self, linebreak: BreakLineOn) -> Self;
}

impl TextLineBreakExt for Text {
    fn with_linebreak(mut self, linebreak: BreakLineOn) -> Self {
        self.linebreak_behavior = linebreak;
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
                    cursor_text.justify = stroked_text.justify_text;
                    cursor_text.linebreak_behavior = stroked_text.linebreak_behavior;
                    if transform.translation.z < 0. {
                        cursor_text.sections[0].style.color = stroked_text.stroke_color;
                    } else {
                        cursor_text.sections[0].style.color = stroked_text.color;
                    }
                }
            }
        } else {
            commands.entity(entity).with_children(|builder| {
                builder.spawn(Text2dBundle {
                    text: Text::from_section(
                        stroked_text.text.clone(),
                        TextStyle {
                            font: stroked_text.font.clone(),
                            font_size: stroked_text.font_size,
                            color: stroked_text.color,
                        },
                    )
                    .with_justify(stroked_text.justify_text)
                    .with_linebreak(stroked_text.linebreak_behavior),
                    text_anchor: stroked_text.text_anchor,
                    ..default()
                });
                let s = stroked_text.stroke_width;
                for offset in [
                    Vec3::new(s, s, -1.),
                    Vec3::new(-s, -s, -1.),
                    Vec3::new(s, -s, -1.),
                    Vec3::new(-s, s, -1.),
                    Vec3::new(0., s, -1.),
                    Vec3::new(0., -s, -1.),
                    Vec3::new(s, 0., -1.),
                    Vec3::new(-s, 0., -1.),
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
