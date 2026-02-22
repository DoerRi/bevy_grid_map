use bevy::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::marker::PhantomData;
use std::path::Path;

use crate::components::Tile;
use crate::editor::TileHitBox;
use crate::marker_traits::{MarkerAble, TileTypeAble};
use crate::{TileMap, utils};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct LoadMapEvent<TYP: TileTypeAble, MARKER: MarkerAble> {
    phantom_data: PhantomData<(TYP, MARKER)>,
    format: utils::FileFormat,
}
impl<TYP: TileTypeAble, MARKER: MarkerAble> LoadMapEvent<TYP, MARKER> {
    pub fn new(format: utils::FileFormat) -> Self {
        Self {
            phantom_data: PhantomData,
            format,
        }
    }
    pub fn on(event: On<Self>, tile_map: Res<TileMap<MARKER>>, commands: Commands) {
        match event.format {
            utils::FileFormat::Json => Self::load_json(tile_map, commands),
            utils::FileFormat::Csv => Self::load_csv(tile_map, commands),
        };
    }
    fn load_json(tile_map: Res<TileMap<MARKER>>, mut commands: Commands) {
        let path = format!("{}.json", tile_map.config.source_path);
        let path = Path::new(&path);
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
    fn load_csv(tile_map: Res<TileMap<MARKER>>, mut commands: Commands) {
        let path = format!("{}.csv", tile_map.config.source_path);
        let path = Path::new(&path);

        let Ok(file) = File::open(path) else {
            println!(
                "couldn't load tiles for tile_map, since file {:?} doesn't exist",
                path
            );
            return;
        };

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        for result in reader.records() {
            let Ok(record) = result else { continue };

            let Ok(x) = record[0].parse::<i32>() else {
                continue;
            };
            let Ok(y) = record[1].parse::<i32>() else {
                continue;
            };
            let Ok(z) = record[2].parse::<i32>() else {
                continue;
            };
            let Ok(dir) = record[3].parse::<u8>() else {
                continue;
            };

            let typ: Result<TYP, _> = serde_json::from_str(&record[4]);
            let Ok(typ) = typ else { continue };

            let map_pos = IVec3::new(x, y, z);

            commands.spawn((Tile::<MARKER>::new(map_pos, dir), typ));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct SaveMapEvent<TYP: TileTypeAble, MARKER: MarkerAble> {
    phantom_data: PhantomData<(TYP, MARKER)>,
    format: utils::FileFormat,
}
impl<TYP: TileTypeAble, MARKER: MarkerAble> SaveMapEvent<TYP, MARKER> {
    pub fn new(format: utils::FileFormat) -> Self {
        Self {
            phantom_data: PhantomData,
            format,
        }
    }
    pub fn on(
        event: On<Self>,
        tile_map: Res<TileMap<MARKER>>,
        tiles: Query<(&TYP, &Tile<MARKER>)>,
    ) {
        match event.format {
            utils::FileFormat::Json => Self::save_json(tile_map, tiles),
            utils::FileFormat::Csv => Self::save_csv(tile_map, tiles),
        };
    }
    fn save_json(tile_map: Res<TileMap<MARKER>>, tiles: Query<(&TYP, &Tile<MARKER>)>) {
        let path = format!("{}.json", tile_map.config.source_path);
        let path = Path::new(&path);
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
                serde_json::json!({ "pos": pos, "dir": dir, "typ": typ })
            })
            .collect();
        let value = serde_json::json!({ "tiles": tiles_json });
        match serde_json::to_writer_pretty(&mut file, &value) {
            Ok(_) => println!("Tile Map saved successfully"),
            Err(e) => println!("Failed to save the Tile Map: {}", e),
        }
    }
    fn save_csv(tile_map: Res<TileMap<MARKER>>, tiles: Query<(&TYP, &Tile<MARKER>)>) {
        let path = format!("{}.csv", tile_map.config.source_path);
        let path = Path::new(&path);

        let Ok(file) = File::create(path) else {
            println!(
                "couldn't create storage file for tile_map, directory may not exist: {:?}",
                path
            );
            return;
        };

        let mut writer = csv::WriterBuilder::new()
            .has_headers(true)
            .from_writer(file);

        // Write header
        let _ = writer.write_record(["x", "y", "z", "dir", "typ"]);

        for (typ, tile) in tiles.iter() {
            let pos = tile.map_pos;
            let dir = tile.orientation;

            let Ok(typ_json) = serde_json::to_string(typ) else {
                continue;
            };

            let _ = writer.write_record(&[
                pos.x.to_string(),
                pos.y.to_string(),
                pos.z.to_string(),
                dir.to_string(),
                typ_json,
            ]);
        }

        match writer.flush() {
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
