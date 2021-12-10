// From https://github.com/marcusbuffett/bevy_snake

use bevy::{
    app::prelude::*,
    asset::prelude::*,
    core::{FixedTimestep, Name},
    core_pipeline::ClearColor,
    ecs::prelude::*,
    input::Input,
    math::*,
    pbr2::{PbrBundle, StandardMaterial},
    prelude::{Commands, Component, CoreStage, KeyCode, Res, Transform, World},
    render2::{
        camera::{OrthographicCameraBundle, PerspectiveCameraBundle},
        color::Color,
        mesh::{shape, Mesh},
    },
    window::{WindowDescriptor, Windows},
};
use bevy_inspector_egui::Inspectable;
use engine::prelude::*;
use rand::prelude::*;

pub fn run() {
    App::new()
        .add_plugin(StandardEnvironmentPlugin)
        .insert_resource(WindowDescriptor {
            title: "Snake".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_inspector_resource::<ClearColor>(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .init_inspector_resource::<SnakeResources>()
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_startup_system(setup)
        .add_startup_system(spawn_snake)
        // .add_system(
        //     snake_movement_input
        //         .system()
        //         .label(SnakeMovement::Input)
        //         .before(SnakeMovement::Movement),
        // )
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(0.150))
        //         .with_system(snake_movement.label(SnakeMovement::Movement))
        //         .with_system(
        //             snake_eating
        //                 .label(SnakeMovement::Eating)
        //                 .after(SnakeMovement::Movement),
        //         )
        //         .with_system(
        //             snake_growth
        //                 .label(SnakeMovement::Growth)
        //                 .after(SnakeMovement::Eating),
        //         ),
        // )
        // .add_system(game_over.after(SnakeMovement::Movement))
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

fn setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert(CameraController::default())
        .insert(Name::new("camera"));

    // ground
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Name::new("ground"));
}

#[derive(Inspectable)]
struct SnakeResources {
    #[inspectable(min = Vec2::new(0.0, 0.0), max = Vec2::new(100.0, 100.0))]
    arena_size: Vec2,
    head_material: Handle<StandardMaterial>,
    segment_material: Handle<StandardMaterial>,
    food_material: Handle<StandardMaterial>,
}

impl FromWorld for SnakeResources {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        Self {
            arena_size: Vec2::new(10.0, 10.0),
            head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
            food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
        }
    }
}

#[derive(Default, Component, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

// #[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
// pub enum SnakeMovement {
//     Input,
//     Movement,
//     Eating,
//     Growth,
//}

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
    materials: Res<SnakeResources>,
    mut segments: ResMut<SnakeSegments>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    segments.0 = vec![
        commands
            .spawn_bundle(PbrBundle {
                material: materials.head_material.clone(),
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: Vec2::new(10.0, 10.0),
                    flip: false,
                })),
                ..Default::default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(
            commands,
            &materials.segment_material,
            Position { x: 3, y: 2 },
        ),
    ];
}

fn spawn_segment(
    mut commands: Commands,
    material: &Handle<StandardMaterial>,
    position: Position,
) -> Entity {
    commands
        .spawn_bundle(PbrBundle {
            material: material.clone(),
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn snake_movement(
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    segments: ResMut<SnakeSegments>,
    snake_resources: Res<SnakeResources>,
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
            || head_pos.x >= snake_resources.arena_size.x as i32
            || head_pos.y >= snake_resources.arena_size.y as i32
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

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    materials: Res<SnakeResources>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, materials, segments_res, meshes);
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
    materials: Res<SnakeResources>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawn_segment(
            commands,
            &materials.segment_material,
            last_tail_position.0.unwrap(),
        ));
    }
}

fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform)>,
    snake_resources: Res<SnakeResources>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                window.width() as f32,
                snake_resources.arena_size.x as f32,
            ),
            convert(
                pos.y as f32,
                window.height() as f32,
                snake_resources.arena_size.y as f32,
            ),
            0.0,
        );
    }
}

fn food_spawner(mut commands: Commands, snake_resources: Res<SnakeResources>) {
    commands
        .spawn_bundle(PbrBundle {
            material: snake_resources.food_material.clone(),
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * snake_resources.arena_size.x as f32) as i32,
            y: (random::<f32>() * snake_resources.arena_size.y as f32) as i32,
        })
        .insert(Size::square(0.8));
}
