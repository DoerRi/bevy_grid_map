use bevy::prelude::*;
use std::marker::PhantomData;

use crate::marker_traits::MarkerAble;

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
}
