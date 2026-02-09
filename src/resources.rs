use bevy::ecs::entity::Entity;
use bevy::math::IVec3;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::marker::PhantomData;

use crate::marker_traits::MarkerAble;

#[derive(Debug, Clone, PartialEq, Resource, Serialize, Deserialize)]
pub struct TileMap<MARKER: MarkerAble> {
    pub tiles: HashMap<IVec3, Entity>,
    pub config: TileMapConfig,
    pub _marker: PhantomData<MARKER>,
}
impl<MARKER: MarkerAble> TileMap<MARKER> {
    pub fn new(config: TileMapConfig) -> Self {
        Self {
            tiles: HashMap::new(),
            config,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Resource, Serialize, Deserialize)]
pub struct TileMapConfig {
    pub tile_size: Vec3,
    pub source_path: String,
    pub load_from_source: bool,
    pub next_tile_dir: Vec<(Vec3, IVec3)>,
    pub possible_orientations: Vec<Quat>,
}
impl TileMapConfig {
    pub fn with_tile_size(mut self, size: impl Into<Vec3>) -> Self {
        self.tile_size = size.into();
        self
    }
    pub fn with_source_path(mut self, path: impl Into<String>) -> Self {
        self.source_path = path.into();
        self
    }
    pub fn fresh_world(mut self) -> Self {
        self.load_from_source = false;
        self
    }
}
impl Default for TileMapConfig {
    fn default() -> Self {
        Self {
            tile_size: (1.0, 1.0, 1.0).into(),
            source_path: "tile_map.json".into(),
            load_from_source: true,
            next_tile_dir: vec![
                (Vec3::X, IVec3::X),
                (Vec3::Z, IVec3::Z),
                (Vec3::NEG_X, IVec3::NEG_X),
                (Vec3::NEG_Z, IVec3::NEG_Z),
                (Vec3::Y, IVec3::Y),
                (Vec3::NEG_Y, IVec3::NEG_Y),
            ],
            possible_orientations: vec![
                Quat::from_rotation_y(PI / 2.0 * 0.0),
                Quat::from_rotation_y(PI / 2.0 * 1.0),
                Quat::from_rotation_y(PI / 2.0 * 2.0),
                Quat::from_rotation_y(PI / 2.0 * 3.0),
            ],
        }
    }
}
