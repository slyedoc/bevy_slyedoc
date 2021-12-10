use bevy::prelude::HandleUntyped;

mod ron_asset;

pub use ron_asset::*;

pub struct AssetsLoading(pub Vec<HandleUntyped>);
