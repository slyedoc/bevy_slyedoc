use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::Inspectable;
use engine::prelude::*;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

// TODO: track down off by one issue with left and bottom

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

#[wasm_bindgen]
pub fn run() {
    App::new()        
        .add_plugin(EnginePlugin {
            title: "Snake".to_string(),
        })
        .insert_inspector_resource::<ClearColor>(ClearColor(Color::BLACK))
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .init_inspector_resource::<SnakeConfig>()
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_startup_system(setup)
        .add_startup_system(spawn_snake)
        .add_system(
            snake_movement_input
                .label(SnakeMovement::Input)
                .before(SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake_movement.label(SnakeMovement::Movement))
                .with_system(
                    snake_eating
                        .label(SnakeMovement::Eating)
                        .after(SnakeMovement::Movement),
                )
                .with_system(
                    snake_growth
                        .label(SnakeMovement::Growth)
                        .after(SnakeMovement::Eating),
                ),
        )
        .add_system(game_over.after(SnakeMovement::Movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(position_translation),
        )
        .run();
}

#[derive(Inspectable)]
struct SnakeConfig {
    #[inspectable(min = 0.0)]
    cell_size: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    snake_head_scale: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    snake_segment_scale: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    food_scale: f32,
    #[inspectable(min = Vec2::new(0.0, 0.0), max = Vec2::new(100.0, 100.0), speed = 1.0)]
    board_size: Vec2,

    background_material: Handle<StandardMaterial>,
    boarder_material: Handle<StandardMaterial>,
    boarder_corner_material: Handle<StandardMaterial>,
    head_material: Handle<StandardMaterial>,

    segment_material: Handle<StandardMaterial>,
    food_material: Handle<StandardMaterial>,
}

impl FromWorld for SnakeConfig {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        Self {
            cell_size: 1.0,
            board_size: Vec2::new(32.0, 18.0),
            snake_head_scale: 1.0,
            snake_segment_scale: 0.8,
            food_scale: 0.65,
            background_material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.9, 0.9, 0.9),
                ..Default::default()
            }),
            boarder_material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.2, 0.2),
                ..Default::default()
            }),
            boarder_corner_material: materials.add(StandardMaterial {
                base_color: Color::DARK_GRAY,
                ..Default::default()
            }),
            head_material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.7, 0.7, 0.7),
                ..Default::default()
            }),
            segment_material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.3, 0.3),
                ..Default::default()
            }),
            food_material: materials.add(StandardMaterial {
                base_color: Color::RED,
                ..Default::default()
            }),
        }
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, config: Res<SnakeConfig>) {
    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0., 50.),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(Name::new("Camera"));

    // Board
    let cell_size_half = config.cell_size * 0.5;
    let board_size = config.board_size * config.cell_size;
    let board_half_size = board_size * 0.5;
    commands
        .spawn_bundle((
            Transform::default(),
            GlobalTransform::default(),
            Name::new("Board"),
        ))
        .with_children(|builder| {
            // background
            builder
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(0.0, 0.0, -cell_size_half),
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(board_size.x, board_size.y),
                        flip: false,
                    })),
                    material: config.background_material.clone(),
                    ..Default::default()
                })
                .insert(Name::new("Background"));

            // Setup Sides
            [
                (
                    // top
                    0.,
                    board_half_size.y + cell_size_half,
                    board_size.x,
                    config.cell_size,
                ),
                (
                    // bottom
                    0.,
                    -board_half_size.y - cell_size_half,
                    board_size.x,
                    config.cell_size,
                ),
                (
                    // left
                    -board_half_size.x - cell_size_half,
                    0.,
                    config.cell_size,
                    board_size.y,
                ),
                (
                    // right
                    board_half_size.x + cell_size_half,
                    0.,
                    config.cell_size,
                    board_size.y,
                ),
            ]
            .iter()
            .for_each(|(x, y, x_length, y_length)| {
                builder
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_xyz(*x, *y, 0.),
                        mesh: meshes.add(Mesh::from(shape::Box::new(*x_length, *y_length, 1.0))),
                        material: config.boarder_material.clone(),
                        ..Default::default()
                    })
                    .insert(Name::new("Side"));
            });

            // Setup corners and lights
            [
                (
                    // top left
                    -board_half_size.x - cell_size_half,
                    board_half_size.y + cell_size_half,
                ),
                (
                    // top right
                    board_half_size.x + cell_size_half,
                    board_half_size.y + cell_size_half,
                ),
                (
                    // bottom left
                    -board_half_size.x - cell_size_half,
                    -board_half_size.y - cell_size_half,
                ),
                (
                    // bottom right
                    board_half_size.x + cell_size_half,
                    -board_half_size.y - cell_size_half,
                ),
            ]
            .iter()
            .for_each(|(x, y)| {
                builder
                    // Corner
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_xyz(*x, *y, 0.),
                        mesh: meshes.add(Mesh::from(shape::Box::new(config.cell_size, 1.0, 1.0))),
                        material: config.boarder_corner_material.clone(),
                        ..Default::default()
                    })
                    .insert(Name::new("Corner"));

                // Light
                builder
                    .spawn_bundle(PointLightBundle {
                        point_light: PointLight {
                            color: Color::WHITE,
                            intensity: 800.0,
                            range: 100.0,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(*x, *y, 5.),
                        ..Default::default()
                    })
                    .insert(Name::new("Light - Corner"));
            });

            // Setup Center Light
            builder
                .spawn_bundle(PointLightBundle {
                    point_light: PointLight {
                        color: Color::WHITE,
                        intensity: 1000.0,
                        range: 100.0,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0., 0., 10.),
                    ..Default::default()
                })
                .insert(Name::new("Light - Center"));
        });
}

#[derive(Default, Component, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

struct GameOverEvent;
struct GrowthEvent;

#[derive(Default)]
struct LastTailPosition(Option<Position>);

#[derive(Component)]
struct SnakeSegment;
#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

#[derive(Component)]
struct Food;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

fn spawn_snake(
    mut commands: Commands,
    materials: Res<SnakeConfig>,
    mut segments: ResMut<SnakeSegments>,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<SnakeConfig>,
) {
    segments.0 = vec![
        commands
            .spawn_bundle(PbrBundle {
                material: materials.head_material.clone(),
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: config.cell_size * config.snake_head_scale,
                })),
                ..Default::default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Name::new("Head"))
            .id(),
        spawn_segment(
            commands,
            meshes.add(Mesh::from(shape::Cube {
                size: config.cell_size * config.snake_segment_scale,
            })),
            &materials.segment_material,
            Position { x: 3, y: 2 },
        ),
    ];
}

fn spawn_segment(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    position: Position,
) -> Entity {
    commands
        .spawn_bundle(PbrBundle {
            material: material.clone(),
            mesh,
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Name::new("Segment"))
        .id()
}

fn snake_movement(
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    segments: ResMut<SnakeSegments>,
    snake_resources: Res<SnakeConfig>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x >= snake_resources.board_size.x as i32
            || head_pos.y >= snake_resources.board_size.y as i32
        {
            game_over_writer.send(GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    materials: Res<SnakeConfig>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
    meshes: ResMut<Assets<Mesh>>,
    config: Res<SnakeConfig>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, materials, segments_res, meshes, config);
    }
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
    materials: Res<SnakeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<SnakeConfig>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawn_segment(
            commands,
            meshes.add(Mesh::from(shape::Cube {
                size: config.cell_size * config.snake_segment_scale,
            })),
            &materials.segment_material,
            last_tail_position.0.unwrap(),
        ));
    }
}

fn position_translation(
    mut q: Query<(&Position, &mut Transform)>,
    snake_resources: Res<SnakeConfig>,
) {
    let board_size = snake_resources.board_size * snake_resources.cell_size;
    let board_size_half = board_size * 0.5;

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            -board_size_half.x + (pos.x as f32 * snake_resources.cell_size),
            -board_size_half.y + (pos.y as f32 * snake_resources.cell_size),
            0.,
        );
    }
}

fn food_spawner(
    mut commands: Commands,
    config: Res<SnakeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            material: config.food_material.clone(),
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: config.cell_size * config.food_scale,
            })),
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * config.board_size.x as f32) as i32,
            y: (random::<f32>() * config.board_size.y as f32) as i32,
        })
        .insert(Name::new("Food"));
}
