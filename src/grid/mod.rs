use bevy::prelude::*;

#[derive(Component)]
pub struct Grid;

#[derive(Component)]
pub struct Cell;

pub struct GridPlugin {
    pub grid_size: Vec2,
    pub cell_size: f32,
    pub space_between_cells: f32,
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        let state = GridState {
            grid_size: self.grid_size,
            cell_size: self.cell_size,
            space_between_cells: self.space_between_cells,
        };

        app.insert_resource(state).add_startup_system(setup);
    }
}

pub struct GridState {
    pub grid_size: Vec2,
    pub cell_size: f32,
    pub space_between_cells: f32,
}

fn setup(state: Res<GridState>, mut commands: Commands) {
    // root node.
    commands
        .spawn_bundle(NodeBundle {
            transform: Transform::from_xyz(
                state.grid_size.x * (state.cell_size + state.space_between_cells) / 2.0,
                state.grid_size.y * (state.cell_size + state.space_between_cells) / 2.0,
                999.0,
            ),
            style: Style {
                size: bevy::prelude::Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Grid)
        .with_children(|parent| {
            for x in 0..state.grid_size.x as usize {
                for y in 0..state.grid_size.y as usize {
                    parent
                        .spawn_bundle(NodeBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    x as f32 * state.cell_size / 2.0,
                                    y as f32 * state.cell_size / 2.0,
                                    999.0,
                                ),
                                ..Default::default()
                            },
                            style: Style {
                                size: Size::new(Val::Px(state.cell_size), Val::Px(state.cell_size)),
                                // align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::rgb(1.0, 0.15, 0.15).into(),
                            ..default()
                        })
                        .insert(Cell);
                }
            }
        });
}
