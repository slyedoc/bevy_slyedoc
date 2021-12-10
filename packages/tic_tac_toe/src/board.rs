use crate::{GameConfig, LocationRaycastSet};
use bevy::{
    core::prelude::*,
    ecs::prelude::*,
    math::*,
    pbr2::*,
    prelude::{Assets, BuildChildren, Commands, GlobalTransform, ResMut, Transform},
    render2::mesh::{shape, Mesh},
};
use bevy_mod_raycast::RayCastMesh;

#[derive(Component)]
pub struct Board;

#[derive(Component)]
pub struct Location {
    #[allow(dead_code)]
    id: u32,
    #[allow(dead_code)]
    state: LocationState,
}

pub enum LocationState {
    Empty,
    #[allow(dead_code)]
    X,
    #[allow(dead_code)]
    O,
}


pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    game_config: ResMut<GameConfig>,
) {
    // Board
    let board_half_size = game_config.board_size * 0.5;
    let cell_size = game_config.board_size / 3.0;
    let cell_half_size = cell_size * 0.5;
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(0.0, 3.0, 0.0)),
            GlobalTransform::default(),
            Name::new("Board"),
            Board,
        ))
        .with_children(|builder| {
            // locations
            for i in 0..3 {
                for j in 0..3 {
                    let x = -board_half_size + cell_half_size + (i as f32 * cell_size);
                    let y = -board_half_size + cell_half_size + (j as f32 * cell_size);
                    builder
                        .spawn_bundle(PbrBundle {
                            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                            mesh: meshes.add(Mesh::from(shape::Quad {
                                size: Vec2::new(
                                    cell_size * game_config.cell_fill,
                                    cell_size * game_config.cell_fill,
                                ),
                                flip: false,
                            })),
                            material: game_config.cell_material.clone(),
                            ..Default::default()
                        })
                        .insert(Location {
                            id: (i * 3 + j) as u32,
                            state: LocationState::Empty,
                        })
                        .insert(RayCastMesh::<LocationRaycastSet>::default())
                        .insert(Name::new(format!("Location {}", i * 3 + j)));
                }
            }

            // lines - vertical
            builder
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_translation(Vec3::new(-cell_half_size, 0.0, 0.0)),
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(game_config.grid_line_width, game_config.board_size),
                        flip: false,
                    })),
                    material: game_config.line_material.clone(),
                    ..Default::default()
                })
                .insert(Name::new("line_y_0"));

            builder
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_translation(Vec3::new(cell_half_size, 0.0, 0.0)),
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(game_config.grid_line_width, game_config.board_size),
                        flip: false,
                    })),
                    material: game_config.line_material.clone(),
                    ..Default::default()
                })
                .insert(Name::new("line_y_1"));

            // lines - horizontal
            builder
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, cell_half_size, 0.0)),
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(game_config.board_size, game_config.grid_line_width),
                        flip: false,
                    })),
                    material: game_config.line_material.clone(),
                    ..Default::default()
                })
                .insert(Name::new("line_x_0"));

            builder
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, -cell_half_size, 0.0)),
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(game_config.board_size, game_config.grid_line_width),
                        flip: false,
                    })),
                    material: game_config.line_material.clone(),
                    ..Default::default()
                })
                .insert(Name::new("line_x_1"));
        });
}
