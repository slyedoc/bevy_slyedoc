mod resource_inspector;
mod ui;
use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_inspector_egui::*;
pub use resource_inspector::*;
use std::fmt::Debug;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum EditorState {
    Playing,
    Disabled,
}

#[derive(Inspectable, Default)]
struct Inspector {
    #[inspectable(deletable = false)]
    active: Option<Entity>,
}

#[derive(Inspectable, Default)]
pub struct Editor {
    pub draw_bounds: bool,

    // Windows
    pub egui_settings: bool,
    pub egui_inspection: bool,
}

/// Provides Bevy Editor for Debugging
pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        info!("Toggle Editor - F12");

        app.add_plugin(InspectorPlugin::<Editor>::new().open(false))
            .add_plugin(InspectorPlugin::<Inspector>::new().open(false))
            .add_state(EditorState::Disabled)
            .add_system_set(SystemSet::on_enter(EditorState::Playing).with_system(setup))
            .add_system_set(
                SystemSet::on_update(EditorState::Playing).with_system(ui::toolbar_system),
            )
            .add_system_set(
                SystemSet::on_exit(EditorState::Playing)
                .with_system(close)
                .with_system(ui::close_windows_system),
            )
            .add_system(action_system);
        // .add_system_to_stage(
        //     CoreStage::PostUpdate,
        //     maintain_inspected_entities
        //         // Could need to be after for faster interaction
        //         //.after(bevy_mod_picking::PickingSystem::Focus),
        // );
    }
}

fn setup(mut world_inspection: ResMut<WorldInspectorParams>) {
    world_inspection.enabled = true
}

fn close(mut world_inspection: ResMut<WorldInspectorParams>) {
    world_inspection.enabled = false;
}

// fn maintain_inspected_entities(
//     commands: Commands,
//     mut inspector: ResMut<Inspector>,
//     query: Query<(Entity, &Interaction), Changed<Interaction>>,
// ) {
// let entity = query
//     .iter()
//     .filter(|(_, interaction)| matches!(interaction, Interaction::Clicked))
//     .map(|(entity, _)| entity)
//     .next();

// if let Some(entity) = entity {
//     if let Some(active) = inspector.active {
//         commands.entity(active).remove::<bevy_transform_gizmo::GizmoTransformable>();
//         inspector.active = None;
//     } else {
//         //commands.entity(entity).insert(bevy_transform_gizmo::GizmoTransformable);
//         inspector.active = Some(entity);

//     }
// }
//}

fn action_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<EditorState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F12) {
        match state.current() {
            EditorState::Playing => state.pop().unwrap(),
            EditorState::Disabled => state.push(EditorState::Playing).unwrap(),
        };
    }
}

#[allow(dead_code)]
pub fn run_if_editor(state: Res<State<EditorState>>) -> ShouldRun {
    match state.current() {
        EditorState::Playing => ShouldRun::Yes,
        EditorState::Disabled => ShouldRun::No,
    }
}
