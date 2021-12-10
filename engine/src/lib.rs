#![allow(clippy::type_complexity)]

mod camera_controller;
mod editor;
mod loaders;
mod materials;
mod shapes;

use bevy::{
    diagnostic::*,
    input::Input,
    prelude::{App, EventWriter, KeyCode, Plugin, Res, ResMut},
    render2::view::Msaa,
    PipelinedDefaultPlugins,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use camera_controller::CameraControllerPlugin;

#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;
use editor::*;
use materials::MaterialPlugin;
use shapes::ShapePlugin;

pub mod prelude {
    pub use crate::{
        camera_controller::*, editor::*, loaders::*, materials::*, shapes::*,
        StandardEnvironmentPlugin,
    };
}

pub struct StandardEnvironmentPlugin;

impl Plugin for StandardEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        // limiting for wasm firefox
        #[cfg(target_arch = "wasm32")]
        app.insert_resource(bevy::pbr2::DirectionalLightShadowMap { size: 2048 });

        app.add_plugins(PipelinedDefaultPlugins)
            .insert_resource(Msaa { samples: 4 })
            .add_plugin(WorldInspectorPlugin::default())
            .add_plugin(EditorPlugin)
            .add_plugin(CameraControllerPlugin)
            .add_plugin(ShapePlugin)
            .add_plugin(MaterialPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            //.add_plugin(LogDiagnosticsPlugin::default());
            .add_system(control_system);

        //#[cfg(not(target_arch = "wasm32"))]
        // app.add_plugin(config::ConfigPlugin);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn control_system(
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<AppExit>,
    key_input: Res<Input<KeyCode>>,
    mut world_inspection: ResMut<WorldInspectorParams>,
) {
    if key_input.just_pressed(KeyCode::F12) {
        world_inspection.enabled = !world_inspection.enabled;
    }
    #[cfg(not(target_arch = "wasm32"))]
    if key_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
