mod components;
mod editor;
mod events;
mod marker_traits;
mod resources;
mod utils;

use bevy::prelude::*;

use std::marker::PhantomData;

use crate::editor::TileMapEditorPlugin;
use crate::events::{ClearMapEvent, LoadMapEvent, SaveMapEvent};
use crate::marker_traits::{MarkerAble, TileTypeAble};
use crate::prelude::Tile;
use crate::resources::{TileMap, TileMapConfig};

pub struct TileMapPlugin<TYP: TileTypeAble, MARKER: MarkerAble> {
    pub edit: bool,
    pub config: TileMapConfig,
    _typ: PhantomData<TYP>,
    _marker: PhantomData<MARKER>,
}
impl<TYP: TileTypeAble, MARKER: MarkerAble> TileMapPlugin<TYP, MARKER> {
    pub fn new() -> Self {
        Self {
            edit: false,
            config: TileMapConfig::default(),
            _typ: PhantomData,
            _marker: PhantomData,
        }
    }
    pub fn in_edit_mod(mut self) -> Self {
        self.edit = true;
        self
    }
    pub fn with_config(mut self, config: TileMapConfig) -> Self {
        self.config = config;
        self
    }
}
impl<TYP: TileTypeAble, MARKER: MarkerAble> Plugin for TileMapPlugin<TYP, MARKER> {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap::<MARKER>::new(self.config.clone()));

        app.add_observer(LoadMapEvent::<TYP, MARKER>::on);
        app.add_observer(SaveMapEvent::<TYP, MARKER>::on);
        app.add_observer(ClearMapEvent::<TYP, MARKER>::on);
        app.add_observer(Tile::<MARKER>::on_added);

        if self.edit {
            app.add_plugins(TileMapEditorPlugin::<TYP, MARKER>::default());
        }

        if self.config.load_from_source {
            app.add_systems(Startup, |mut commands: Commands| {
                commands.trigger(LoadMapEvent::<TYP, MARKER>::new(utils::FileFormat::Csv));
                commands.trigger(LoadMapEvent::<TYP, MARKER>::new(utils::FileFormat::Json));
            });
        }
    }
}

pub mod prelude {
    pub use crate::TileMapPlugin;
    pub use crate::components::Tile;
    pub use crate::events::*;
    pub use crate::marker_traits::*;
    pub use crate::resources::{TileMap, TileMapConfig};
    pub use crate::utils::FileFormat;
}
