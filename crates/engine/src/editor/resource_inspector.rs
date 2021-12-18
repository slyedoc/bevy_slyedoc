use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
/// A Helper function to configure inspector plugin resources
pub trait AppInspector {
    /// Custom logic for adding resources
    fn init_inspector_resource<T>(&mut self) -> &mut App
    where
        T: Inspectable + FromWorld + Send + Sync + 'static;

    /// Custom logic for adding resources
    fn insert_inspector_resource<T>(&mut self, data: T) -> &mut App
    where
        T: Inspectable + FromWorld + Send + Sync + 'static;
}

impl AppInspector for App {
    fn init_inspector_resource<T>(&mut self) -> &mut App
    where
        T: Inspectable + FromWorld + Send + Sync + 'static,
    {
        //#[cfg(feature = "editor")]
        self.add_plugin(InspectorPlugin::<T>::new())

        // #[cfg(not(feature = "editor"))]
        // self.init_resource::<T>()
    }

    fn insert_inspector_resource<T>(&mut self, data: T) -> &mut App
    where
        T: Inspectable + FromWorld + Send + Sync + 'static,
    {
        self.insert_resource(data);
        self.add_plugin(InspectorPlugin::<T>::new_insert_manually())
    }
}
