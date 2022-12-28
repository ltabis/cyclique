use bevy::prelude::*;

use crate::{game_state::GameState, simulation::events::switch_simulation_state};

const NORMAL_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                switch_simulation_state(&mut state);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn update_button_text(
    state: ResMut<State<GameState>>,
    interaction_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
) {
    for children in &interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match state.current() {
            GameState::Paused => {
                text.sections[0].value = "Run".to_string();
            }
            GameState::Run => {
                text.sections[0].value = "Pause".to_string();
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(35.0)),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Px(20.),
                    ..default()
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(TextBundle::from_section(
                "Run",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}
