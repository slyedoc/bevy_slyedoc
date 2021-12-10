use bevy::{prelude::*, render::camera::Camera};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::{ops::Range, time::Duration};
use engine::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugin(StandardEnvironmentPlugin)
            .insert_resource(TubeLastGapOffset(0.0))
            .add_state(FlappyState::Loading)
            .add_system_set(
                SystemSet::on_enter(FlappyState::Loading).with_system(setup_environment.system()),
            )
            .add_system_set(
                SystemSet::on_update(FlappyState::Playing)
                    .with_system(scroll_tubes)
                    .with_system(catchup_bird)
                    .with_system(update_human),
            )
            .add_system_set(
                SystemSet::on_enter(FlappyState::Resetting)
                .with_system(clear_environment),
            )
        .run();


        println!("Press Space to jump, Escape to exit");
}


impl Plugin for FlappyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
    }
}




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FlappyState {
    Loading,
    Playing,
    Resetting,
}

struct Bird {
    index: usize,
}
struct Tube {
    current: bool,
    top_lip: f32,
    bottom_lip: f32,
}

struct TubeLastGapOffset(f32);
struct Population(usize);

const RAPIER_SCALE: f32 = 50.0; // Very useful to zoom in and out to see whats going on
                                // Also see https://rapier.rs/docs/user_guides/bevy_plugin/common_mistakes/#why-is-everything-moving-in-slow-motion
const TUBE_SIZE_HALF_X: f32 = 1.0;
const TUBE_SIZE_HALF_Y: f32 = 10.0;
const TUBE_SPACING: f32 = 12.0;
const TUBE_GAP_SIZE_HALF: f32 = 2.0; // Control gap size between tubes in a set
const TUBE_GAP_OFFSET_MAX: f32 = 6.0; // Control gap range off of y axis
const TUBE_GAP_CLAMP_HALF: f32 = 10.0; // Removes impossible height changes
const TUBE_SPEED: f32 = 0.10;
const TUBE_COUNT: usize = 5;
const TUBE_DESPAWN_LIMIT: f32 = -2.0 * TUBE_SPACING;
const BIRD_SIZE_HALF: V2<f32> = V2 { x: 0.0, y: 0.0 };
const ACTION_FORCE: f32 = 250.0;
const BIRD_LIMIT_X: Range<f32> = -1.0..4.0;
const BIRD_LIMIT_Y: Range<f32> = -8.0..8.0;

fn update_human(
    keyboard_input: Res<Input<KeyCode>>,
    mut birds: Query<(&RigidBodyPosition, &mut RigidBodyVelocity), With<Bird>>,
    params: Res<IntegrationParameters>,
    mut state: ResMut<State<FlappyState>>,
) {
    for (rb_pos, mut rb_vel) in birds.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            rb_vel.linvel = Vec2::new(0.0, ACTION_FORCE * params.dt).into();
        }

        if is_bird_dead(rb_pos.position.translation.x, rb_pos.position.translation.y) {
            state.set(FlappyState::Resetting).unwrap();
        }
    }
}



fn setup_environment(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    config: ResMut<FlappyConfig>,
    population: Res<Population>,
    mut state: ResMut<State<FlappyState>>,
    camera: Query<&Camera>,
    mut gap_offset: ResMut<TubeLastGapOffset>,
) {
    rapier_config.scale = RAPIER_SCALE;

    if config.render && camera.iter().count() == 0 {
        let mut camera = OrthographicCameraBundle::new_2d();
        camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 50.0));
        commands.spawn_bundle(camera);
    }

    // Create the Birds
    for i in 0..population.0 {
        commands
            .spawn_bundle(RigidBodyBundle {
                position: Vec2::new(0.0, 0.0).into(),
                body_type: RigidBodyType::Dynamic,
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(BIRD_SIZE_HALF.x, BIRD_SIZE_HALF.y),
                collider_type: ColliderType::Solid,
                flags: ColliderFlags {
                    collision_groups: InteractionGroups::new(0b0001, 0b0010),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete)
            .insert(ColliderDebugRender::from(Color::RED))
            .insert(Bird { index: i })
            .id();
    }

    // Create tubes
    for x in 0..TUBE_COUNT {
        spawn_tube_set(
            &mut commands,
            (x + 1) as f32 * TUBE_SPACING,
            &mut gap_offset,
        );
    }

    state.set(FlappyState::Playing).unwrap();
}

fn spawn_tube_set(commands: &mut Commands, pos_x: f32, last_gap_offset: &mut TubeLastGapOffset) {
    // figure out where the tubes should be
    let mut rng = rand::thread_rng();
    let gap_offset = rng
        .gen_range(-TUBE_GAP_OFFSET_MAX..TUBE_GAP_OFFSET_MAX)
        .clamp(
            last_gap_offset.0 - TUBE_GAP_CLAMP_HALF,
            last_gap_offset.0 + TUBE_GAP_CLAMP_HALF,
        ); // Remove impossible height changes
    last_gap_offset.0 = gap_offset;

    let spacing = TUBE_SIZE_HALF_Y + TUBE_GAP_SIZE_HALF;
    let top_pos = Vec2::new(0.0, spacing + gap_offset);
    let bottom_pos = Vec2::new(0.0, -spacing + gap_offset);

    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec2::new(pos_x, 0.0).into(),
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .with_children(|mut parent| {
            create_child_tubes(&mut parent, top_pos);
            create_child_tubes(&mut parent, bottom_pos);
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::from(Color::GREEN))
        .insert(Tube {
            top_lip: TUBE_GAP_SIZE_HALF + gap_offset,
            bottom_lip: -TUBE_GAP_SIZE_HALF + gap_offset,
            current: false,
        })
        .id();
}

fn create_child_tubes(parent: &mut ChildBuilder, pos: Vec2) {
    parent
        .spawn_bundle(ColliderBundle {
            position: pos.into(),
            collider_type: ColliderType::Solid,
            shape: ColliderShape::cuboid(TUBE_SIZE_HALF_X, TUBE_SIZE_HALF_Y),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(0b0010, 0b0001),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::from(Color::GREEN));
}

fn scroll_tubes(
    mut commands: Commands,
    mut tubes: Query<(Entity, &mut RigidBodyPosition, &mut Tube)>,
    mut lines: ResMut<DebugLines>,
    mut gap_offset: ResMut<TubeLastGapOffset>,
) {
    for (e, mut rb_pos, mut tube) in tubes.iter_mut() {
        rb_pos.position.translation.x -= TUBE_SPEED;

        let x = rb_pos.position.translation.x;

        // despawn when off screen and spawn new tube
        if x < TUBE_DESPAWN_LIMIT {
            commands.entity(e).despawn_recursive();

            spawn_tube_set(
                &mut commands,
                TUBE_COUNT as f32 * TUBE_SPACING + TUBE_DESPAWN_LIMIT,
                &mut gap_offset,
            );
        }

        // Update Tube Status
        if x > 0.0 && x < TUBE_SPACING {
            tube.current = true;

            // Draw Debug Line
            let start = Vec3::new(0.0, tube.top_lip * RAPIER_SCALE, 0.0);
            let end = Vec3::new(0.0, tube.bottom_lip * RAPIER_SCALE, 0.0);
            lines.line_colored(start, end, 0.0, Color::BLUE);
        } else {
            tube.current = false;
        }
    }
}

// The bird can get stuck behind tubes and falls behind and skid off top of tubes
// I could just trigger game over when a touch occurs but I like the effect
// This is a system to move bird back to x=0, so it can be used more than once or twice
fn catchup_bird(mut bird: Query<&mut RigidBodyPosition, With<Bird>>) {
    for mut rb_pos in bird.iter_mut() {
        // poor mans lerp
        rb_pos.position.translation.x -= rb_pos.position.translation.x / 60.0;
    }
}

fn clear_environment(
    mut commands: Commands,
    query_set: QuerySet<(Query<Entity, With<Bird>>, Query<Entity, With<Tube>>)>,
    mut state: ResMut<State<FlappyState>>,
) {
    for e in query_set.q0().iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_set.q1().iter() {
        commands.entity(e).despawn_recursive();
    }

    state.set(FlappyState::Loading).unwrap();
}
