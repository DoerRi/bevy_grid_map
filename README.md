# bevy_grid_map

A Bevy plugin for creating, editing, and saving grid-based tile maps with ease. Perfect for building tile-based games, procedural level editors, and world-building tools.

## Features

- **Grid-Based Tile System**: Create and manage tile-based maps on a 3D grid (IVec3 coordinates)
- **Interactive Editor**: Built-in EGUI-based editor for editing maps in real-time
- **Save/Load System**: Serialize tile maps to JSON format and load them back
- **Configurable Tiles**: Support for custom tile types with full serialization support
- **Tile Orientation**: Built-in support for tile rotations (4-way by default, customizable)
- **Flexible Markers**: Generic marker system for organizing tiles into categories or layers
- **Hot Reloading**: Load maps on startup or manage them manually

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bevy_grid_map = { path = "path/to/bevy_grid_map" }
```

## Quick Start

### 1. Define Your Tile Type

Create a custom tile type that implements `TileTypeAble`:

```rust
use bevy_grid_map::prelude::*;
use serde::{Deserialize, Serialize};
use derive_more::Display;
use enum2egui::Gui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Component, Gui, Display)]
pub enum TileType {
    Grass,
    Stone,
    Water,
    Tree,
}

impl TileTypeAble for TileType {}

impl Default for TileType {
    fn default() -> Self {
        Self::Grass
    }
}
```

### 2. Add the Plugin to Your App

```rust
use bevy::prelude::*;
use bevy_grid_map::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            TileMapPlugin::<TileType, ()>::new()
                .in_edit_mod()  // Enable the editor UI
                .with_config(
                    TileMapConfig::default()
                        .with_tile_size(Vec3::new(1.0, 1.0, 1.0))
                )
        )
        .run();
}
```

### 3. Respond to Tile Events

```rust
use bevy::prelude::*;
use bevy_grid_map::prelude::*;

fn on_tile_added(
    event: On<Add, TileType>,
    mut commands: Commands,
    tiles: Query<(&TileType, &Tile<()>)>,
) {
    if let Ok((tile_type, tile)) = tiles.get(event.entity) {
        println!("Tile added at {:?}: {:?}", tile.map_pos, tile_type);
        // Spawn visual representation, sound, etc.
    }
}
```

## Configuration

Customize the tile map behavior using `TileMapConfig`:

```rust
let config = TileMapConfig::default()
    .with_tile_size(Vec3::new(16.0, 8.0, 16.0))  // Tile dimensions
    .with_source_path("my_map.json")              // Save/load file
    .fresh_world();                               // Don't load on startup

TileMapPlugin::<TileType, ()>::new()
    .with_config(config)
```

## API Overview

### Core Components

- **`Tile<MARKER>`**: Component placed on tile entities containing grid position and orientation
- **`TileMap<MARKER>`**: Resource storing all tiles and configuration
- **`TileMapConfig`**: Defines tile size, save path, and orientations

### Events

- **`LoadMapEvent<TYP, MARKER>`**: Load map from file
- **`SaveMapEvent<TYP, MARKER>`**: Save map to file
- **`ClearMapEvent<TYP, MARKER>`**: Clear all tiles from the map

### Traits

- **`TileTypeAble`**: Implement this for your tile type
- **`MarkerAble`**: Optional marker system for tile organization

## Dependencies

- **bevy** 0.18.0 - Game engine
- **bevy_egui** 0.39.1 - UI framework integration
- **bevy-inspector-egui** 0.36.0 - Debug inspector
- **serde** / **serde_json** - Serialization
- **enum2egui** 0.33.0 - Automatic EGUI UI generation for enums

## Examples

See the `examples/` directory for complete working examples:

- **basic.rs** - Complete tile map editor with custom tile types and slopes
- **tile_map.json** - Example map file

Run examples with:

```bash
cargo run --example basic
```

## Editor Controls

<img width="1276" height="709" alt="Screenshot From 2026-02-09 16-52-43" src="https://github.com/user-attachments/assets/95a173ad-5769-44d1-90f3-8fcf214bb024" />

When `in_edit_mod()` is enabled, the EGUI interface provides:

- Visual tile map editing
- Tile type selection and painting
- Tile orientation/rotation control
- Map save/load buttons
- Inspector for debugging

## Map File Format

Maps are saved as JSON and can be manually edited:

```json
{
  "tiles": {
    "0,0,0": {
      "tile_type": "Grass",
      "orientation": 0
    }
  },
  "config": {
    "tile_size": [1.0, 1.0, 1.0],
    "source_path": "tile_map.json"
  }
}
```

## License

See LICENSE file for details.
