use bevy::prelude::*;
use enum2egui::GuiInspect;
use serde::Serialize;

pub trait MarkerAble: 'static + Send + Sync {}
impl<M: 'static + Send + Sync> MarkerAble for M {}

pub trait TileTypeAble:
    'static + GuiInspect + Component + Serialize + for<'de> serde::Deserialize<'de> + Clone + Default
{
}
impl<
    M: 'static
        + GuiInspect
        + Component
        + Serialize
        + for<'de> serde::Deserialize<'de>
        + Clone
        + Default,
> TileTypeAble for M
{
}
