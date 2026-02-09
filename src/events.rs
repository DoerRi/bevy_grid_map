use bevy::prelude::*;
use serde_json::{Value, json};
use std::fs::File;
use std::marker::PhantomData;
use std::path::Path;

use crate::TileMap;
use crate::components::Tile;
use crate::editor::TileHitBox;
use crate::marker_traits::{MarkerAble, TileTypeAble};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct LoadMapEvent<TYP: TileTypeAble, MARKER: MarkerAble>(PhantomData<(TYP, MARKER)>);
impl<TYP: TileTypeAble, MARKER: MarkerAble> LoadMapEvent<TYP, MARKER> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
    pub fn on(_: On<Self>, tile_map: Res<TileMap<MARKER>>, mut commands: Commands) {
        let path = Path::new(&tile_map.config.source_path);
        let Ok(file) = File::open(path) else {
            println!(
                "couldn't load tiles for tile_map, since file {:?} doesn't exist",
                path
            );
            return;
        };

        let Ok(values): Result<Value, _> = serde_json::from_reader(file) else {
            println!("Tile Map source files contains invalid Json ({:?})", path);
            return;
        };
        let Some(Value::Array(tiles)) = values.get("tiles") else {
            return;
        };

        tiles.iter().for_each(|tile| {
            let pos: Option<IVec3> = tile
                .get("pos")
                .map(Clone::clone)
                .and_then(|v| serde_json::from_value(v).ok());
            let orientation: Option<u8> = tile
                .get("dir")
                .map(Clone::clone)
                .and_then(|v| serde_json::from_value(v).ok());
            let typ: Option<TYP> = tile
                .get("typ")
                .map(Clone::clone)
                .and_then(|v| serde_json::from_value(v).ok());
            if let (Some(map_pos), Some(orientation), Some(typ)) = (pos, orientation, typ) {
                commands.spawn((Tile::<MARKER>::new(map_pos, orientation), typ));
            };
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct SaveMapEvent<TYP: TileTypeAble, MARKER: MarkerAble>(PhantomData<(TYP, MARKER)>);
impl<TYP: TileTypeAble, MARKER: MarkerAble> SaveMapEvent<TYP, MARKER> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
    pub fn on(_: On<Self>, tile_map: Res<TileMap<MARKER>>, tiles: Query<(&TYP, &Tile<MARKER>)>) {
        let path = Path::new(&tile_map.config.source_path);
        let Ok(mut file) = File::create(path) else {
            println!(
                "{} \n{} {:?}",
                "couldn't create storage file for tile_map,",
                "since full directory path does not exist for path: {:?}",
                path
            );
            return;
        };

        let tiles_json: Vec<Value> = tiles
            .iter()
            .map(|(typ, tile)| {
                let pos = tile.map_pos;
                let dir = tile.orientation;
                json!({
                    "pos": pos,
                    "dir": dir,
                    "typ": typ
                })
            })
            .collect();
        let value = json!({
            "tiles": tiles_json
        });

        match serde_json::to_writer_pretty(&mut file, &value) {
            Ok(_) => println!("Tile Map saved successfully"),
            Err(e) => println!("Failed to save the Tile Map: {}", e),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct ClearMapEvent<TYP: TileTypeAble, MARKER: MarkerAble> {
    _phantom: PhantomData<(TYP, MARKER)>,
    create_source_tile: bool,
}
impl<TYP: TileTypeAble, MARKER: MarkerAble> ClearMapEvent<TYP, MARKER> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            create_source_tile: false,
        }
    }
    pub fn with_source_tile(mut self) -> Self {
        self.create_source_tile = true;
        self
    }
    pub fn on(
        event: On<Self>,
        tiles: Query<(Entity, &Tile<MARKER>)>,
        editor_hit_boxes: Query<(Entity, &TileHitBox<MARKER>)>,
        mut commands: Commands,
    ) {
        tiles
            .iter()
            .for_each(|(e, _t)| commands.entity(e).despawn());
        editor_hit_boxes
            .iter()
            .for_each(|(e, _t)| commands.entity(e).despawn());

        if event.create_source_tile {
            commands.spawn((Tile::<MARKER>::new(IVec3::ZERO, 0), TYP::default()));
        }
    }
}
