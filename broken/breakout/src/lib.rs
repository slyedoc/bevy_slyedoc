use bevy::{
    math::*,
    pbr2::{AmbientLight, StandardMaterial},
    prelude::{App, Assets, Commands, GlobalTransform, ResMut, Transform},
    render2::{
        camera::PerspectiveCameraBundle,
        color::Color,
        mesh::{shape, Mesh},
        view::{ComputedVisibility, Visibility},
    },
};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_twin_games::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugin(StandardEnvironmentPlugin)
        .add_plugin(InspectorPlugin::<BreakoutConfig>::new().open(false))
        .insert_resource(Score(0))
        .add_state(BreakoutState::Loading)
            .add_system_set(
                SystemSet::on_enter(BreakoutState::Loading)
                    .with_system(setup_environment.system())
                    .with_system(spawn_board.system())
                    .with_system(spawn_player.system())
                    .with_system(spawn_ball.system()),
            )
            .add_system_set(
                SystemSet::on_update(BreakoutState::Playing)
                    .with_system(update_ball.system())
                    .with_system(ball_collision.system())
                    .with_system(ball_bounds_check.system()),
            )
            .add_system_set(
                SystemSet::on_enter(BreakoutState::Resetting)
                    .with_system(clean_environment.system()),
            ).add_system_set(
                SystemSet::on_update(BreakoutState::Playing)
                    .with_system(player_movement_human.system())
                    .with_system(other_keyboard_input.system()),
            );
            println!("Press A or D, or Left or Right Arrow\nR to reset\nEscape to exit");
      
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        materials.add(Color::GREEN.into()),
    ));

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default());

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });
}


use crate::helpers::{range_lerp, V2};
use bevy::{ecs::component::Component, prelude::*};
use bevy_inspector_egui::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[derive(Inspectable, Debug)]
pub struct BreakoutConfig {
    // environment settings
    #[inspectable(ignore)]
    pub render: bool,
    #[inspectable(ignore)]
    pub human: bool,
    pub rapier_scale: f32,

    // breakout settings
    pub player_size_half: Vec2,
    pub player_speed: f32,
    pub player_color: Color,
    pub board_size_half: Vec2,
    pub board_line_size_half: f32,
    pub board_color: Color,
    pub brick_grid: V2<usize>,
    pub brick_color: Color,
    pub ball_size_half: f32,
    pub ball_init_x_range: (f32, f32),
    pub ball_init_y: f32,
    pub ball_speed: f32,
    pub ball_y_basis: f32,
    pub ball_basis_engage: f32,
}

impl Default for BreakoutConfig {
    fn default() -> Self {
        Self {
            render: true,
            human: true,
            rapier_scale: 50.0,
            player_size_half: Vec2::new(1.0, 0.1),
            player_speed: 2.0,
            player_color: Color::BLUE,
            board_size_half: Vec2::new(4.0, 6.0),
            board_line_size_half: 0.1,
            board_color: Color::BLUE,
            brick_grid: V2 { x: 6, y: 8 },
            brick_color: Color::GRAY,
            ball_size_half: 0.1,
            ball_init_x_range: (-2.0, 2.0),
            ball_init_y: 5.0,
            ball_speed: 200.0,
            ball_y_basis: 0.01,
            ball_basis_engage: 0.8,
        }
    }
}

#[allow(dead_code)]
pub struct BreakoutInstance {
    pub index: usize,
    pub origin: Vec2,
}

struct Brick;

#[derive(Component)]
pub struct Player {
    index: usize,
}
struct Score(usize);

#[derive(Component)]
struct Ball;
#[derive(Component)]
struct Hit;
#[derive(Component)]
struct BoardBottom;
#[derive(Component)]
struct BoardOther;
#[derive(Component)]
struct BreakoutCleanup;

pub struct BreakoutPlugin {
    pub render: bool,
    pub human: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BreakoutState {
    Loading,
    Playing,
    Resetting,
}

fn setup_environment(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    config: Res<BreakoutConfig>,
    mut state: ResMut<State<BreakoutState>>,
    mut score: ResMut<Score>,
) {
    rapier_config.scale = config.rapier_scale;
    rapier_config.gravity = Vec2::ZERO.into();
    score.0 = 0;

    if config.render {
        let mut camera = OrthographicCameraBundle::new_2d();
        camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 50.0));
        commands.spawn_bundle(camera).insert(BreakoutCleanup);
    }
    state.set(BreakoutState::Playing).unwrap();
}

fn spawn_board(mut commands: Commands, config: Res<BreakoutConfig>) {
    // draw board
    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .with_children(|parent| {
            // Top
            create_board_side(
                parent,
                Vec2::new(0.0, config.board_size_half.y),
                Vec2::new(
                    config.board_size_half.x + config.board_line_size_half,
                    config.board_line_size_half,
                ),
                BoardOther,
                config.board_color,
            );
            // Bottom
            create_board_side(
                parent,
                Vec2::new(0.0, -config.board_size_half.y),
                Vec2::new(
                    config.board_size_half.x + config.board_line_size_half,
                    config.board_line_size_half,
                ),
                BoardBottom,
                config.board_color,
            );
            // Left
            create_board_side(
                parent,
                Vec2::new(-config.board_size_half.x, 0.0),
                Vec2::new(config.board_line_size_half, config.board_size_half.y),
                BoardOther,
                config.board_color,
            );
            // Right
            create_board_side(
                parent,
                Vec2::new(config.board_size_half.x, 0.0),
                Vec2::new(config.board_line_size_half, config.board_size_half.y),
                BoardOther,
                config.board_color,
            );

            let size_x: f32 = config.board_size_half.x / (config.brick_grid.x + 2) as f32;
            let size_y: f32 = config.board_size_half.y * 0.5 / (config.brick_grid.y + 2) as f32;
            // Create Bricks
            for x in 0..config.brick_grid.x {
                for y in 0..config.brick_grid.y {
                    let pos_x = range_lerp(
                        (x + 1) as f32,
                        0.0,
                        (config.brick_grid.x + 1) as f32,
                        -config.board_size_half.x,
                        config.board_size_half.x,
                    );
                    let pos_y = range_lerp(
                        (y + 1) as f32,
                        0.0,
                        (config.brick_grid.y + 1) as f32,
                        0.0,
                        config.board_size_half.y,
                    );

                    create_brick(
                        parent,
                        Vec2::new(pos_x, pos_y),
                        Vec2::new(size_x, size_y),
                        config.brick_color,
                    );
                }
            }
        })
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);
}

fn create_board_side(
    parent: &mut ChildBuilder,
    pos: Vec2,
    size_half: Vec2,
    component: impl Component,
    color: Color,
) {
    parent
        .spawn_bundle(ColliderBundle {
            position: pos.into(),
            collider_type: ColliderType::Solid,
            shape: ColliderShape::cuboid(size_half.x, size_half.y),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::from(color))
        .insert(BreakoutCleanup)
        .insert(component);
}

fn create_brick(parent: &mut ChildBuilder, pos: Vec2, size_half: Vec2, color: Color) {
    parent
        .spawn_bundle(ColliderBundle {
            position: pos.into(),
            collider_type: ColliderType::Solid,
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 1.0,
                ..Default::default()
            },
            shape: ColliderShape::cuboid(size_half.x, size_half.y),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::from(color))
        .insert(Brick)
        .insert(BreakoutCleanup);
}

fn spawn_player(mut commands: Commands, config: Res<BreakoutConfig>) {
    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec2::new(
                0.0,
                -config.board_size_half.y + (config.board_size_half.y * 0.1),
            )
            .into(),
            body_type: RigidBodyType::KinematicPositionBased,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            collider_type: ColliderType::Solid,
            shape: ColliderShape::cuboid(config.player_size_half.x, config.player_size_half.y),
            material: ColliderMaterial {
                restitution: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        //.insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::from(config.player_color))
        .insert(Player { index: 0 })
        .insert(BreakoutCleanup);
}

fn spawn_ball(mut commands: Commands, config: Res<BreakoutConfig>) {
    let mut rnd = rand::thread_rng();
    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec2::new(
                0.0,
                -config.board_size_half.y + (config.board_size_half.y * 0.2),
            )
            .into(),
            mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
            activation: RigidBodyActivation::cannot_sleep(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            },
            damping: RigidBodyDamping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            // Create random launch vector
            velocity: RigidBodyVelocity {
                linvel: Vec2::new(
                    rnd.gen_range(config.ball_init_x_range.0..config.ball_init_x_range.1),
                    config.ball_init_y,
                )
                .into(),
                angvel: 0.0,
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            collider_type: ColliderType::Solid,
            shape: ColliderShape::ball(config.ball_size_half),
            flags: (ActiveEvents::CONTACT_EVENTS).into(),
            material: ColliderMaterial {
                friction: 0.0, // you lose all ball control on paddle at 0
                restitution: 1.0,
                restitution_combine_rule: CoefficientCombineRule::Max,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::from(config.player_color))
        .insert(Ball)
        .insert(BreakoutCleanup);
}

// Keep the ball speed somewhat constant and  avoid getting stuck by back and forth
fn update_ball(
    mut balls: Query<&mut RigidBodyVelocity, With<Ball>>,
    params: Res<IntegrationParameters>,
    config: Res<BreakoutConfig>,
) {
    for mut rb_vel in balls.iter_mut() {
        // Normalize ball speed, currently picked at random
        let mag = rb_vel.linvel.norm();
        let speed = config.ball_speed * params.dt;
        if mag != speed {
            rb_vel.linvel *= speed / mag;
        }

        // This will curve that ball up when its going more left to right that up and down
        // so it can't get stuck, relies on the speed normalizing above
        if rb_vel.linvel[0].abs() > config.ball_basis_engage * speed {
            rb_vel.linvel[1] += if rb_vel.linvel[1].is_sign_positive() {
                config.ball_y_basis
            } else {
                -config.ball_y_basis
            };
        }
    }
}

// The ball can get away using the paddle to force it though a wall, this checks for that
fn ball_bounds_check(
    balls: Query<&RigidBodyPosition, With<Ball>>,
    config: Res<BreakoutConfig>,
    mut state: ResMut<State<BreakoutState>>,
) {
    for rb_pos in balls.iter() {
        if rb_pos.position.translation.x.abs() > config.board_size_half.x
            || rb_pos.position.translation.y.abs() > config.board_size_half.y
        {
            state.set(BreakoutState::Resetting).unwrap()
        }
    }
}

// So Rapier will provide the collisions, but only the entity id
// We can either record entity ids when we create them, or mark the entities
// You could also query narrow phase, but using the ContactEvent is a bit clearer
// This causes an additional 1-frame-lag
fn ball_collision(
    mut commands: Commands,
    mut contact_events: EventReader<ContactEvent>,
    brick_hits: Query<Entity, (With<Brick>, With<Hit>)>,
    bottom_hits: Query<Entity, (With<BoardBottom>, With<Hit>)>,
    extra_hits: Query<Entity, (Without<Brick>, Without<BoardBottom>, With<Hit>)>,
    mut score: ResMut<Score>,
    mut state: ResMut<State<BreakoutState>>,
    config: Res<BreakoutConfig>,
) {
    // Mark every contact event entity, will process them next frame
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                commands.entity(h1.entity()).insert(Hit);
                commands.entity(h2.entity()).insert(Hit);
            }
            _ => (),
        }
    }

    for b in brick_hits.iter() {
        commands.entity(b).despawn_recursive();
        score.0 += 1;

        // This exit condition only works assuming no bugs with hits
        // being using it this way to debug
        if score.0 == config.brick_grid.x * config.brick_grid.y {
            state.set(BreakoutState::Resetting).unwrap();
            return;
        }
    }
    for _ in bottom_hits.iter() {
        state.set(BreakoutState::Resetting).unwrap();
        return;
    }
    for ext in extra_hits.iter() {
        commands.entity(ext).remove::<Hit>();
    }
}

fn other_keyboard_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<BreakoutState>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        state.set(BreakoutState::Resetting).unwrap();
        // TODO: You get stuck in a loop without updating keyboard
        // https://github.com/bevyengine/bevy/issues/1700
        keyboard_input.reset(KeyCode::Escape);
    }
}

fn clean_environment(
    mut commands: Commands,
    cleanup: Query<Entity, With<BreakoutCleanup>>,
    mut state: ResMut<State<BreakoutState>>,
) {
    for e in cleanup.iter() {
        commands.entity(e).despawn();
    }
    state.set(BreakoutState::Loading).unwrap();
}

pub fn player_movement_human(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<&mut RigidBodyPosition, With<Player>>,
    params: Res<IntegrationParameters>,
    config: Res<BreakoutConfig>,
) {
    let movement = config.player_speed * params.dt;
    let limit = config.board_size_half.x - config.player_size_half.x - config.board_line_size_half;
    for mut rb_pos in players.iter_mut() {
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x : f32 = if left {
            rb_pos.position.translation.x - movement
        } else if right {
            rb_pos.position.translation.x + movement
        } else {
            0.0
        };
        if x != 0.0 {
            rb_pos.next_position.translation.x = x.clamp(-limit, limit);
        }
    }
}
