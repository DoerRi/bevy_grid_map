use bevy_grid_map::prelude::*;
mod utils;
use bevy::prelude::*;
use derive_more::Display;
use enum2egui::GuiInspect;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utils::{VoxelVec, setup_camera};

use bevy_vox_scene::VoxScenePlugin;
use enum2egui::Gui;

fn main() {
    App::new()
        // Default Plugins
        .add_plugins((DefaultPlugins, VoxScenePlugin::default()))
        .add_systems(Startup, setup_camera)
        // Plugins for Egui Interface
        .add_plugins((
            bevy_inspector_egui::DefaultInspectorConfigPlugin,
            bevy_egui::EguiPlugin::default(),
        ))
        // TileMap Plugin in editor mode
        .add_plugins((TileMapPlugin::<TileType, ()>::new()
            .in_edit_mod()
            .with_config(TileMapConfig::default().with_tile_size(Vec3 {
                x: 16.0,
                y: 8.0,
                z: 16.0,
            })),))
        .add_observer(on_add_tile_type)
        .run();
}

// ----------------------------- //
// Tile Map                      //
// ----------------------------- //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)] // Default
#[derive(Component, Gui, Display)] // egui + bevy
pub enum TileType {
    Oil,
    Cement,
    Stone,
    Tree,
    #[display("Grass")]
    Grass {
        slop: SlopType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)] // Default
#[derive(Component, Gui, Display, Default)] // egui + bevy
pub enum SlopType {
    #[default]
    Flat,
    Straight,
    Convex,
    Concave,
}

impl bevy_grid_map::prelude::TileTypeAble for TileType {}
impl Default for TileType {
    fn default() -> Self {
        Self::Grass {
            slop: SlopType::Flat,
        }
    }
}

pub fn on_add_tile_type(
    event: On<Add, TileType>,
    mut commands: Commands,
    asset_sever: Res<AssetServer>,
    tiles: Query<(&TileType, &Tile<()>)>,
    tile_map: Res<TileMap<()>>,
) {
    let Ok((typ, tile)) = tiles.get(event.entity) else {
        return;
    };

    let (scene_path, delta_pos) = match typ {
        TileType::Oil => ("terrain.vox#Oil", Vec3::ZERO),
        TileType::Cement => ("terrain.vox#Flat-Stone", Vec3::ZERO),
        TileType::Stone => ("terrain.vox#Stone", Vec3::ZERO),
        TileType::Tree => ("terrain.vox#Tree-1", VoxelVec::y(24 - 8)),
        TileType::Grass { slop } => match slop {
            SlopType::Flat => ("terrain.vox#Grass-Flat", Vec3::ZERO),
            SlopType::Straight => ("terrain.vox#Grass-Straight", Vec3::ZERO),
            SlopType::Convex => ("terrain.vox#Grass-Convex", Vec3::ZERO),
            SlopType::Concave => ("terrain.vox#Grass-Concave", Vec3::ZERO),
        },
    };
    let scene = SceneRoot(asset_sever.load(scene_path));
    let rotation = tile_map.orientation_to_rotation(tile.orientation);
    commands.entity(event.entity).insert((
        scene,
        Visibility::Visible,
        Transform::default()
            .with_rotation(rotation)
            .with_translation(
                tile.map_pos.as_vec3() * VoxelVec::voxel((16, 8, 16)) + delta_pos + VoxelVec::y(8),
            ),
    ));
}
