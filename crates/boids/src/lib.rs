#![allow(warnings)]
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::{Inspectable, InspectableRegistry};
use engine::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    let mut app = App::new();
    app
        .add_plugin(EnginePlugin {
            title: "Boids".to_string(),
        })
        .init_inspector_resource::<BoidConfig>()
        .insert_inspector_resource::<ClearColor>(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_inspector_resource::<AmbientLight>(AmbientLight {
            color: Color::WHITE,
            brightness: 0.05,
        })
        .add_startup_system(setup)
        .add_system(boids_flocking_system.label("flocking"))
        .add_system(boid_heading_system.label("heading").after("flocking"))
        .add_system(ls_adjustment.before("flocking"))
        .add_system(heading_system.after("heading"));

    // registering custom component to be able to edit it in inspector
    let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();
    registry.register::<Boid>();
    registry.register::<Flock>();

    app.run();
}

// Heading component (velocity vector)
#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

impl std::ops::Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn heading_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    query.for_each_mut(|(heading, mut transform)| {
        transform.translation += heading.0.extend(0.) * time.delta_seconds();
    });
}

// Boid component
#[derive(Component, Inspectable, Default)]
pub struct Boid {
    pub force: Vec2, // Sum of the forces
}

#[derive(Component, Inspectable, Default, PartialEq, Eq)]
pub struct Flock(usize);

fn steer(current_vel: Vec2, dir: Vec2, max_velocity: f32, max_acceleration: f32) -> Vec2 {
    (dir * max_velocity - current_vel).clamp_length_max(max_acceleration)
}

fn percieve(pos: Vec2, dir: Vec2, other: Vec2, neighbor_radius: f32, field_of_vision: f32) -> bool {
    pos.distance_squared(other) < neighbor_radius * neighbor_radius
        && (pos + dir).angle_between(other) < field_of_vision / 2.
}

// System to calculate each boids forces
fn boids_flocking_system(
    mut query: Query<(&mut Boid, &Transform, &Velocity, Entity, &Flock)>,
    inner_query: Query<(&Transform, &Velocity, Entity, &Flock), With<Boid>>, /* This query
                                                                              * requires the
                                                                              * Boid component
                                                                              * but doesn't
                                                                              * borrow it */
    raycastable: Query<&Raycastable>,
    config: Res<BoidConfig>,
) {

    query.for_each_mut(|(mut boid, transform_a, velocity_a, entity_a, flock_a)| {
        let pos_a = transform_a.translation.truncate();
        let velocity_a = velocity_a.0;
        let dir_a = velocity_a.normalize();
        let mut acceleration = Vec2::ZERO;
        let mut flockmate_count = 0;
        let mut foreign_count = 0;
        let mut pos_total = Vec2::ZERO;
        let mut heading_total = Vec2::ZERO;
        let mut avoidance_total = Vec2::ZERO;
        let mut interflock_avoidance_total = Vec2::ZERO;

        inner_query.for_each(|(transform_b, velocity_b, entity_b, flock_b)| {
            if entity_a == entity_b {
                return;
            }
            let pos_b = transform_b.translation.truncate();
            let direction_b = velocity_b.normalize();
            if percieve(
                pos_a,
                dir_a,
                pos_b,
                config.neighbor_radius,
                config.field_of_vision,
            ) {
                let offset = pos_b - pos_a;
                let sqr_dist = offset.length_squared();
                if flock_a == flock_b {
                    flockmate_count += 1;
                    pos_total += pos_b;
                    heading_total += direction_b;
                    if sqr_dist < config.avoid_radius * config.avoid_radius {
                        avoidance_total -= offset / sqr_dist;
                    }
                } else {
                    foreign_count += 1;
                    interflock_avoidance_total -= offset / sqr_dist;
                }
            }
        });

        if foreign_count > 0 {
            let foreign_count = foreign_count as f32;
            let avg_interflock_avoidance = interflock_avoidance_total / foreign_count;
            acceleration += steer(
                velocity_a,
                avg_interflock_avoidance,
                config.max_velocity,
                config.max_acceleration,
            ) * config.interflock_separation_force;
        }

        if flockmate_count > 0 {
            let flockmate_count = flockmate_count as f32;
            let avg_pos = pos_total / flockmate_count;
            let avg_avoidance = avoidance_total / flockmate_count;
            let avg_heading = heading_total / flockmate_count;
            let offset_to_avg_pos = avg_pos - pos_a;

            acceleration += steer(
                velocity_a,
                avg_avoidance,
                config.max_velocity,
                config.max_acceleration,
            ) * config.separation_force;
            acceleration += steer(
                velocity_a,
                offset_to_avg_pos,
                config.max_velocity,
                config.max_acceleration,
            ) * config.cohesion_force;
            acceleration += steer(
                velocity_a,
                avg_heading,
                config.max_velocity,
                config.max_acceleration,
            ) * config.align_force;
        }
        let target_pos = Vec2::ZERO;
        acceleration += steer(
            velocity_a,
            target_pos - pos_a,
            config.max_velocity,
            config.max_acceleration,
        ) * config.target_force;

        if raycast(raycastable.iter(), pos_a, dir_a * config.collision_radius) {
            let dir_angle = f32::atan2(dir_a.y, dir_a.x);
            let direction = (0..(std::f32::consts::TAU / config.turn_find_step) as usize)
                .map(|i| {
                    if i % 2 == 0 {
                        config.turn_find_step * (i / 2) as f32
                    } else {
                        -config.turn_find_step * ((i + 1) / 2) as f32
                    }
                })
                .map(|v| v + dir_angle)
                .map(vec_from_angle)
                .find(|&dir| !raycast(raycastable.iter(), pos_a, dir * config.collision_radius))
                .unwrap_or(dir_a);

            acceleration += steer(
                velocity_a,
                direction,
                config.max_velocity,
                config.max_acceleration,
            ) * config.collision_avoidance_force;
        }
        boid.force = acceleration;
    });
}
fn vec_from_angle(x: f32) -> Vec2 {
    Vec2::new(f32::cos(x), f32::sin(x))
}

fn raycast<'a>(
    raycastable: impl IntoIterator<Item = &'a Raycastable>,
    start: Vec2,
    offset: Vec2,
) -> bool {
    raycastable.into_iter().any(|r| match r {
        Raycastable::LS(ls) => segment_segment_intersection(*ls, LineSegment { start, offset }),
    })
}

#[derive(Component)]
struct MainCamera;

// System to apply the boid force to the heading/rotation component
fn boid_heading_system(
    time: Res<Time>,
    mut query: Query<(&Boid, &mut Velocity, &mut Transform)>,
    cam: Query<&OrthographicProjection, With<MainCamera>>,
    config: Res<BoidConfig>,
) {
    query.for_each_mut(|(boid, mut velocity, mut transform)| {
        let rotation = &mut transform.rotation;

        // Update the heading (velocity)
        velocity.0 += boid.force * time.delta_seconds();
        velocity.0 = velocity.0.clamp_length(config.min_velocity, config.max_velocity);

        // Compute the rotation according to the heading
        let angle = f32::atan2(velocity.y, velocity.x);
        *rotation = Quat::from_euler(EulerRot::YXZ, 0., 0., angle);

        let pos = &mut transform.translation;
        let proj = cam.single();
        if pos.x < proj.left * proj.scale {
            pos.x = proj.right * proj.scale
        }
        if pos.x > proj.right * proj.scale {
            pos.x = proj.left * proj.scale
        }
        if pos.y < proj.bottom * proj.scale {
            pos.y = proj.top * proj.scale
        }
        if pos.y > proj.top * proj.scale {
            pos.y = proj.bottom * proj.scale
        }
    });
}

#[derive(Inspectable)]
struct BoidConfig {
    // The number of boids we will spawn
    num_boids: usize,

    // These constants influence the boid movement
    max_velocity: f32,
    min_velocity: f32,
    max_acceleration: f32,
    neighbor_radius: f32,
    field_of_vision: f32,
    avoid_radius: f32,
    interflock_separation_force: f32,
    separation_force: f32,
    align_force: f32,
    cohesion_force: f32,
    target_force: f32,
    collision_avoidance_force: f32,
    collision_radius: f32,
    turn_find_step: f32,

    #[inspectable(min = Vec2::new(0.0, 0.0))]
    map_size: Vec2,
    materials: Vec<Handle<StandardMaterial>>,
    #[inspectable(ignore)]
    mesh: Handle<Mesh>,
}

impl FromWorld for BoidConfig {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let texture = asset_server.load("icon.png");
        Self {
            map_size: Vec2::new(10.0, 10.0),
            num_boids: 300,
            max_velocity: 16.,
            min_velocity: 4.,
            max_acceleration: 6.,
            neighbor_radius: 20.,
            field_of_vision: (2. / 3.) * std::f32::consts::TAU,
            avoid_radius: 5.,
            interflock_separation_force: 15.,
            separation_force: 10.,
            align_force: 1.,
            cohesion_force: 1.,
            target_force: 0.001,
            collision_avoidance_force: 20.,
            collision_radius: 10.,
            turn_find_step: (std::f32::consts::PI / 180.) * 45.,

            materials: [
                Color::RED,
                Color::GREEN,
                Color::BLUE,
                Color::GOLD,
                Color::PINK,
                Color::WHITE,
                Color::PURPLE,
                Color::TEAL,
                Color::BEIGE,
            ]
            .iter()
            .map(|c| {
                materials.add(StandardMaterial {
                    base_color: *c,
                    base_color_texture: Some(texture.clone()),
                    unlit: true,
                    ..Default::default()
                })
            })
            .collect::<Vec<_>>(),
            mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::splat(1.0), flip: false  })),
        }
    }
}

// Set up a scene
fn setup(mut commands: Commands, config: Res<BoidConfig>, mut _meshes: ResMut<Assets<Mesh>>) {
    commands
        // Camera, follows the last boid
        .spawn_bundle(OrthographicCameraBundle {
            orthographic_projection: OrthographicProjection {
                scale: 40.,
                scaling_mode: ScalingMode::FixedVertical,
                ..Default::default()
            },
            ..OrthographicCameraBundle::new_3d()
        })
        //.spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert(MainCamera)
        //.insert(CameraController::default())
        .insert(Name::new("Camera"));


    let random_pm = |v: f32| rand::random::<f32>() * 2. * v - v;
    for i in 0..config.num_boids {
        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_xyz(random_pm(10.), random_pm(10.), -5.),
                material: config.materials[i % config.materials.len()].clone(),
                mesh: config.mesh.clone(),
                ..Default::default()
            })
            .insert(Boid::default())
            .insert(Velocity({
                let angle = rand::random::<f32>() * std::f32::consts::TAU;
                Vec2::new(angle.cos(), angle.sin()) * config.min_velocity
            }))
            .insert(Flock(i % config.materials.len()))
            .insert(Name::new(format!("Bird {}", i)));
    }
    // commands.spawn_batch((0..NUM_BOIDS).map(move |i| {
    //     let handle = config.materials[i % config.materials.len()].clone();
    //     GroupBundle {
    //         // TODO: Texture
    //         pbr: PbrBundle {
    //             transform: Transform::from_xyz(random_pm(10.), random_pm(10.), 0.),
    //             material: handle,
    //             mesh,
    //             ..Default::default()
    //         },
    //         velocity: Velocity({
    //             let angle = rand::random::<f32>() * std::f32::consts::TAU;
    //             Vec2::new(angle.cos(), angle.sin()) * MIN_VELOCITY
    //         }),
    //         flock: Flock(i % config.materials.len()),
    //         ..Default::default()
    //     }
    // }));

    commands
        .spawn()
        .insert(Raycastable::LS(LineSegment::default()))
        .insert(Left);
    commands
        .spawn()
        .insert(Raycastable::LS(LineSegment::default()))
        .insert(Right);
    commands
        .spawn()
        .insert(Raycastable::LS(LineSegment::default()))
        .insert(Top);
    commands
        .spawn()
        .insert(Raycastable::LS(LineSegment::default()))
        .insert(Bottom);
}

#[allow(clippy::type_complexity)]
fn ls_adjustment(
    mut left: Query<&mut Raycastable, (With<Left>, Without<Right>, Without<Top>, Without<Bottom>)>,
    mut right: Query<&mut Raycastable, (Without<Left>, With<Right>, Without<Top>, Without<Bottom>)>,
    mut top: Query<&mut Raycastable, (Without<Left>, Without<Right>, With<Top>, Without<Bottom>)>,
    mut bottom: Query<
        &mut Raycastable,
        (Without<Left>, Without<Right>, Without<Top>, With<Bottom>),
    >,
    cam: Query<&OrthographicProjection, With<MainCamera>>,
    _config: Res<BoidConfig>,
) {
    let Raycastable::LS(left) = &mut *left.single_mut();
    let Raycastable::LS(right) = &mut *right.single_mut();
    let Raycastable::LS(top) = &mut *top.single_mut();
    let Raycastable::LS(bottom) = &mut *bottom.single_mut();

    let cam = cam.single();
    *left = LineSegment {
        start: Vec2::new(cam.left, cam.top) * cam.scale,
        offset: Vec2::Y * -2. * cam.scale,
    };

    *right = LineSegment {
        start: Vec2::new(cam.right, cam.top) * cam.scale,
        offset: Vec2::Y * -2. * cam.scale,
    };
    *top = LineSegment {
        start: Vec2::new(cam.left, cam.top) * cam.scale,
        offset: Vec2::X * (cam.right - cam.left) * cam.scale,
    };
    *bottom = LineSegment {
        start: Vec2::new(cam.left, cam.bottom) * cam.scale,
        offset: Vec2::X * (cam.right - cam.left) * cam.scale,
    };

    // info!("top: {:?}", top);
    // info!("right: {:?}", right);
    // info!("top {:?}", top);
    // info!("bottom {:?}", bottom);
}

#[derive(Component)]
struct Left;
#[derive(Component)]
struct Right;
#[derive(Component)]
struct Top;
#[derive(Component)]
struct Bottom;

#[derive(Component)]
enum Raycastable {
    LS(LineSegment),
}

#[derive(Clone, Copy, Debug, Default)]
struct LineSegment {
    start: Vec2,
    offset: Vec2,
}

// https://stackoverflow.com/a/565282
fn segment_segment_intersection(p: LineSegment, q: LineSegment) -> bool {
    let LineSegment {
        start: p,
        offset: r,
    } = p;
    let LineSegment {
        start: q,
        offset: s,
    } = q;
    let r_x_s = r.perp_dot(s);
    let q_minus_p = q - p;
    if r_x_s == 0. {
        let qmp_x_r = q_minus_p.perp_dot(r);
        if qmp_x_r == 0. {
            // collinear
            let r_dot_r = r.dot(r);
            let r_over_rdr = r / r_dot_r;
            let t0 = q_minus_p.dot(r_over_rdr);
            let t1 = t0 + s.dot(r_over_rdr);
            #[allow(clippy::needless_bool)]
            if ((0.0..1.0).contains(&t0) || (0.0..1.0).contains(&t1))
                || (t0 < 0. && t1 > 1.)
                || (t0 > 1. && t0 < 0.)
            {
                // overlapping
                true
            } else {
                // disjoint
                false
            }
        } else {
            // parallel & non-intersecting
            false
        }
    } else {
        let t = q_minus_p.perp_dot(s / r_x_s);
        let u = q_minus_p.perp_dot(r / r_x_s);
        if 0. <= t && t <= 1. && 0. <= u && u <= 1. {
            // intersection @ (p + tr, q + us)
            true
        } else {
            // no intersection within segments
            false
        }
    }
}
