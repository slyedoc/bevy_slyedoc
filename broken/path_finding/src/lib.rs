use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use engine::prelude::*;
use pathfinding::prelude::*;
use std::ops::Not;

#[derive(Inspectable, Copy, Clone)]
struct Board {
    x: u32,
    y: u32,
}
impl Default for Board {
    fn default() -> Self {
        Self { x: 10, y: 10 }
    }
}

pub fn run() {
    App::new()
        .add_plugin(EnginePlugin {
            title: "Pathfinding".to_string(),
        })
        .insert_inspector_resource::<ClearColor>(ClearColor(Color::GRAY))
        .add_event::<ToggleBlockEvent>()
        .init_inspector_resource::<Board>()
        .init_resource::<Materials>()
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PostUpdate, grid_to_transform.system())
        .add_system(mouse_click_system)
        .add_system(toggle_block)
        .add_system(pathfinding)
        .run();
}

#[derive(Component)]
struct Start;
#[derive(Component)]
struct End;
#[derive(Component)]
struct Block;

struct Materials {
    path: Handle<StandardMaterial>,
    block: Handle<StandardMaterial>,
    background: Handle<StandardMaterial>
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        Self {
            block: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            }),
            path: materials.add(StandardMaterial {
                base_color: Color::rgb(1., 1., 1.),
                ..Default::default()
            }),
            background: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..Default::default()
            }),
        }
    }
}

#[derive(Component, Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    const fn try_new(x: i32, y: i32, board: Board) -> Option<Self> {
        if x < 0 || y < 0 || x >= board.x as i32 || y >= board.y as i32 {
            None
        } else {
            Some(Self {
                x: x as i32,
                y: y as i32,
            })
        }
    }

    const fn min(self) -> bool {
        self.x == 0 && self.y == 0
    }

    const fn max(self, board: Board) -> bool {
        self.x == board.x as i32 - 1 && self.y == board.y as i32 - 1
    }
}

struct ToggleBlockEvent {
    pos: Pos,
}

#[derive(Component)]
struct Path;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    path_materials: Res<Materials>,
    board: Res<Board>,
) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert(CameraController::default())
        .insert(Name::new("Camera"));

    // Start
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 35.,
            })),
            material: path_materials.path.clone(),
            ..Default::default()
        })
        .insert(Pos::try_new(0, 0, *board).unwrap())
        .insert(Start)
        .insert(Name::new("Start"));

    // End
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 35.,
                //flip: false,
            })),
            material: path_materials.path.clone(),
            ..Default::default()
        })
        .insert(Pos::try_new(9, 9, *board).unwrap())
        .insert(End)
        .insert(Name::new("End"));

    // Background?
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(-20., -20., 1.),
        mesh: meshes.add(Mesh::from(shape::Cube {
            size: 400.,
        })),
        material: path_materials.background.clone(),
        ..Default::default()
    })
    .insert(Name::new("Background?"));
}

fn grid_to_transform(mut query: Query<(&Pos, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = ((pos.x as i32 * 40) - 200) as f32;
        transform.translation.y = ((pos.y as i32 * 40) - 200) as f32;
        transform.translation.z = 2.;
    }
}

fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut my_events: EventWriter<ToggleBlockEvent>,
    board: Res<Board>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.get_primary() {
            if let Some(cursor_pos) = window.cursor_position() {
                let x = (cursor_pos.x as i32 - 180) / 40;
                let y = (cursor_pos.y as i32 - 85) / 40;

                if let Some(pos) = Pos::try_new(x as i32, y as i32,*board) {
                    my_events.send(ToggleBlockEvent { pos });
                }
            }
        }
    }
}

fn toggle_block(
    mut my_events: EventReader<ToggleBlockEvent>,
    blocks: Query<(Entity, &Pos), With<Block>>,
    mut commands: Commands,
    path_material: Res<Materials>,
    board: Res<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in my_events.iter() {
        let event: &ToggleBlockEvent = event;
        if event.pos.min() || event.pos.max(*board) {
            continue;
        }
        match blocks.iter().find(|(_, pos)| pos == &&event.pos) {
            None => {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad {
                            size: Vec2::new(35., 35.),
                            flip: false,
                        })),
                        material: path_material.block.clone(),
                        ..Default::default()
                    })
                    .insert(event.pos)
                    .insert(Block);
            }
            Some((entity, _)) => {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

/// Pathfinding logic
/// find shortest path between Start and End
#[allow(clippy::too_many_arguments)]
fn pathfinding(
    start: Query<&Pos, With<Start>>,
    end: Query<&Pos, With<End>>,
    blocks: Query<&Pos, With<Block>>,
    paths: Query<Entity, With<Path>>,
    mut commands: Commands,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    board: Res<Board>
) {
    let start = start.get_single().expect("No start block");
    let end = end.get_single().expect("No end block");

    let blocks = blocks.iter().collect::<Vec<_>>();

    let result = bfs(
        start,
        |p| {
            let &Pos { x, y } = p;
            vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
                .into_iter()
                .filter_map(|(x, y)| Pos::try_new(x, y, *board))
                .filter(|pos| blocks.contains(&pos).not())
        },
        |p| p == end,
    );

    for entity in paths.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if let Some(path) = result {
        for pos in path {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(5., 5.),
                        flip: false,
                    })),
                    material: materials.path.clone(),
                    ..Default::default()
                })
                .insert(pos)
                .insert(Path);
        }
    }
}
