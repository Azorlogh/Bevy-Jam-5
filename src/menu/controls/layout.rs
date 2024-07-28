use bevy::prelude::*;

use crate::{
    input::Action,
    menu::styling::{
        default_text, ColoredButton, BUTTON_COLOR, BUTTON_STYLE, DEFAULT_BACKGROUND_COLOR,
    },
};

use super::{ActionButton, ControlsBack, ControlsMenu, KeyText};

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _option_menu_entity = build_menu(&mut commands, &asset_server);
}

pub fn despawn_menu(mut commands: Commands, q_menu: Query<Entity, With<ControlsMenu>>) {
    if let Ok(menu_entity) = q_menu.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

pub fn build_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let menu_entity = commands
        .spawn((
            Name::new("Controls Menu"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: DEFAULT_BACKGROUND_COLOR.into(),
                ..default()
            },
            ControlsMenu,
        ))
        .with_children(|parent| {
            // FORWARD
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ActionButton(Action::Forward),
                    ColoredButton,
                ))
                .with_children(|parent| {
                    parent.spawn(default_text("Forward ", 32.0, asset_server));
                    parent.spawn((default_text("", 32.0, asset_server), KeyText));
                });
            // BACKWARD
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ActionButton(Action::Backward),
                    ColoredButton,
                ))
                .with_children(|parent| {
                    parent.spawn(default_text("Backward ", 32.0, asset_server));
                    parent.spawn((default_text("", 32.0, asset_server), KeyText));
                });
            // LEFT
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ActionButton(Action::Left),
                    ColoredButton,
                ))
                .with_children(|parent| {
                    parent.spawn(default_text("Left ", 32.0, asset_server));
                    parent.spawn((default_text("", 32.0, asset_server), KeyText));
                });
            // Right
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ActionButton(Action::Right),
                    ColoredButton,
                ))
                .with_children(|parent| {
                    parent.spawn(default_text("Right ", 32.0, asset_server));
                    parent.spawn((default_text("", 32.0, asset_server), KeyText));
                });
            // Jump
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ActionButton(Action::Jump),
                    ColoredButton,
                ))
                .with_children(|parent| {
                    parent.spawn(default_text("Jump ", 32.0, asset_server));
                    parent.spawn((default_text("", 32.0, asset_server), KeyText));
                });
            // Crouch
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ActionButton(Action::Crouch),
                    ColoredButton,
                ))
                .with_children(|parent| {
                    parent.spawn(default_text("Crouch ", 32.0, asset_server));
                    parent.spawn((default_text("", 32.0, asset_server), KeyText));
                });
            // Interact
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ActionButton(Action::Interact),
                    ColoredButton,
                ))
                .with_children(|parent| {
                    parent.spawn(default_text("Interact ", 32.0, asset_server));
                    parent.spawn((default_text("", 32.0, asset_server), KeyText));
                });
            // Place beacon
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ActionButton(Action::PlaceBeacon),
                    ColoredButton,
                ))
                .with_children(|parent| {
                    parent.spawn(default_text("Place beacon ", 32.0, asset_server));
                    parent.spawn((default_text("", 32.0, asset_server), KeyText));
                });
            // BACK
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ControlsBack,
                    ColoredButton,
                ))
                .with_children(|parent| {
                    parent.spawn(default_text("Back", 32.0, asset_server));
                });
        })
        .id();

    menu_entity
}
