use bevy::{color::palettes::css::*, prelude::*};

pub struct MenuStylingPlugin;
impl Plugin for MenuStylingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, highlight_button_interactions);
    }
}

pub const DEFAULT_BACKGROUND_COLOR: Srgba = Srgba::NONE;

pub const BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

pub const BUTTON_WIDTH: f32 = 200.0;
pub const BUTTON_HEIGHT: f32 = 80.0;

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(BUTTON_WIDTH);
    style.height = Val::Px(BUTTON_HEIGHT);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub fn highlight_button_interactions(
    mut q_buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ColoredButton>),
    >,
) {
    for (interaction, mut background_color) in &mut q_buttons {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = BUTTON_COLOR.into();
            }
        }
    }
}

#[derive(Component)]
pub struct ColoredButton;

pub const PADDING: f32 = 15.0;
pub const BORDER: f32 = 5.0;

pub fn transparent_root() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,

            ..default()
        },

        ..default()
    }
}

pub fn central_panel() -> NodeBundle {
    NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(PADDING)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(BORDER)),
            row_gap: Val::Px(PADDING),
            ..default()
        },
        border_color: BorderColor(Color::WHITE),
        background_color: BackgroundColor::from(ORANGE.with_alpha(0.5)),
        ..default()
    }
}

pub fn button_bundle() -> impl Bundle {
    (
        ButtonBundle {
            style: Style {
                padding: UiRect::all(Val::Px(PADDING)),
                ..default()
            },
            border_color: BorderColor(Color::WHITE),
            background_color: BackgroundColor::from(ORANGE.with_alpha(0.5)),
            ..default()
        },
        ColoredButton,
    )
}

pub fn default_text(text: &str, font_size: f32, asset_server: &AssetServer) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: font_size,
                    color: Color::WHITE,
                },
            )],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    }
}

// pub fn spawn_button<T: Component>(
// 	cmds: &mut ChildBuilder,
// 	asset_server: Res<AssetServer>,
// 	text: &str,
// 	id: Component,
// ) {
// 	cmds.spawn(NodeBundle {
// 		style: Style {
// 			padding: UiRect::all(Val::Px(15.0)),
// 			flex_direction: FlexDirection::Column,
// 			justify_content: JustifyContent::FlexStart,
// 			align_items: AlignItems::Center,
// 			border: UiRect::all(Val::Px(5.0)),
// 			..default()
// 		},
// 		border_color: BorderColor(Color::WHITE),
// 		background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
// 		..default()
// 	})
// 	.with_children(|cmds| {
// 		cmds.spawn(TextBundle::from_section(
// 			"You win!",
// 			TextStyle {
// 				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
// 				font_size: 64.0,
// 				color: default(),
// 			},
// 		));
// 	});
// }
