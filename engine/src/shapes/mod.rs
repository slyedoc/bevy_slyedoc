// Nearly all of this is from bevy renderer
// duplicating for easy of use in egui, maybe replace with wrapper?
mod capsule;
mod icosphere;
mod torus;
mod uvsphere;

use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices, primitives::Aabb}, math::vec2};
use bevy_inspector_egui::{Inspectable, InspectableRegistry};

pub use capsule::{Capsule, CapsuleUvProfile};
pub use icosphere::Icosphere;
pub use torus::Torus;
pub use uvsphere::UVSphere;

pub struct ShapePlugin;
impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shape_change_detection_system);

        // registering custom component to be able to edit it in inspector
        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();

        registry.register::<ShapeInstance>();
        registry.register::<Quad>();

    }
}

fn shape_change_detection_system(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeInstance), Or<(Added<ShapeInstance>, Changed<ShapeInstance>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (e, shape) in query.iter_mut() {
        let h = meshes.add(shape.value.mesh());
        commands.entity(e).remove::<Handle<Mesh>>();
        commands.entity(e).remove::<Aabb>();
        commands.entity(e).insert(h);
    }
}

#[derive(Debug, Component, Inspectable, Copy, Clone)]
pub struct ShapeInstance {
    pub value: Shape,
}

#[derive(Debug, Inspectable, Clone, Copy)]
pub enum Shape {
    Quad(Quad),
    Plane(Plane),
    Icosphere(Icosphere),
    Capsule(Capsule),
}

impl Shape {
    pub fn mesh(&self) -> Mesh {
        match self {
            Shape::Quad(quad) => Mesh::from(*quad),
            Shape::Plane(plane) => Mesh::from(*plane),
            Shape::Icosphere(icosphere) => Mesh::from(*icosphere),
            Shape::Capsule(capsule) => Mesh::from(*capsule),
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Quad(Quad::default())
    }
}

/// A rectangle on the XY plane.
#[derive(Debug, Inspectable, Copy, Clone)]
pub struct Quad {
    /// Full width and height of the rectangle.
    #[inspectable(min = Vec2::ZERO)]
    pub size: Vec2,
    /// Flips the texture coords of the resulting vertices.
    pub flip: bool,
}

impl Default for Quad {
    fn default() -> Self {
        Quad::new(Vec2::ONE)
    }
}

impl Quad {
    pub fn new(size: Vec2) -> Self {
        Self { size, flip: false }
    }

    pub fn flipped(size: Vec2) -> Self {
        Self { size, flip: true }
    }
}

impl From<Quad> for Mesh {
    fn from(quad: Quad) -> Self {
        let extent_x = quad.size.x / 2.0;
        let extent_y = quad.size.y / 2.0;

        let north_west = vec2(-extent_x, extent_y);
        let north_east = vec2(extent_x, extent_y);
        let south_west = vec2(-extent_x, -extent_y);
        let south_east = vec2(extent_x, -extent_y);
        let vertices = if quad.flip {
            [
                (
                    [south_east.x, south_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [1.0, 1.0],
                ),
                (
                    [north_east.x, north_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [1.0, 0.0],
                ),
                (
                    [north_west.x, north_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0],
                ),
                (
                    [south_west.x, south_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 1.0],
                ),
            ]
        } else {
            [
                (
                    [south_west.x, south_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 1.0],
                ),
                (
                    [north_west.x, north_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0],
                ),
                (
                    [north_east.x, north_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [1.0, 0.0],
                ),
                (
                    [south_east.x, south_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [1.0, 1.0],
                ),
            ]
        };

        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let mut positions = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut uvs = Vec::<[f32; 2]>::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

/// A square on the XZ plane.
#[derive(Debug, Inspectable, Copy, Clone)]
pub struct Plane {
    /// The total side length of the square.
    #[inspectable(min = 0.0)]
    pub size: f32,
}

impl Default for Plane {
    fn default() -> Self {
        Plane { size: 1.0 }
    }
}

impl From<Plane> for Mesh {
    fn from(plane: Plane) -> Self {
        let extent = plane.size / 2.0;

        let vertices = [
            ([extent, 0.0, -extent], [0.0, 1.0, 0.0], [1.0, 1.0]),
            ([extent, 0.0, extent], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ([-extent, 0.0, extent], [0.0, 1.0, 0.0], [0.0, 0.0]),
            ([-extent, 0.0, -extent], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ];

        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}
