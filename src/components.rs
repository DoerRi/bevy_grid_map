use crate::marker_traits::MarkerAble;
use crate::resources::TileMap;
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Component)]
pub struct Tile<MARKER: MarkerAble> {
    pub map_pos: IVec3,
    pub orientation: u8,
    _marker: PhantomData<MARKER>,
}
impl<MARKER: MarkerAble> Tile<MARKER> {
    pub fn new(map_pos: IVec3, orientation: u8) -> Self {
        Self {
            map_pos,
            orientation,
            _marker: PhantomData,
        }
    }
    pub fn on_added(
        event: On<Add, Self>,
        tiles: Query<&Self>,
        mut tile_map: ResMut<TileMap<MARKER>>,
    ) {
        let entity = event.entity;
        let tile = tiles.get(entity).unwrap();
        tile_map.tiles.insert(tile.map_pos, entity);
    }
}
