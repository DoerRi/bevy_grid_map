use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::Slider;
use rand::random;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::{
    TileMap,
    components::Tile,
    events::{ClearMapEvent, LoadMapEvent, SaveMapEvent},
    marker_traits::{MarkerAble, TileTypeAble},
};

pub struct TileMapEditorPlugin<TYP: TileTypeAble, MARKER: MarkerAble>(PhantomData<(TYP, MARKER)>);

impl<TYP: TileTypeAble, MARKER: MarkerAble> Default for TileMapEditorPlugin<TYP, MARKER> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<TYP: TileTypeAble, MARKER: MarkerAble> Plugin for TileMapEditorPlugin<TYP, MARKER> {
    fn build(&self, app: &mut App) {
        app.add_observer(add_tile_map_hit_box::<TYP, MARKER>);
        app.insert_resource(TileMapEditor::<TYP, MARKER>::default());
        app.add_systems(Startup, TileMapEditor::<TYP, MARKER>::setup);
        app.add_systems(
            bevy_egui::EguiPrimaryContextPass,
            TileMapEditor::<TYP, MARKER>::ui_system,
        );
        app.add_systems(
            Update,
            (
                TileMapEditor::<TYP, MARKER>::user_input_system,
                TileMapEditor::<TYP, MARKER>::update,
            ),
        );
    }
}

#[derive(Resource, Debug)]
pub struct TileMapEditor<TYP: TileTypeAble, MARKER: MarkerAble> {
    pub mode: EditorMode,
    pub selected_tile_type: TYP,
    pub random_orientation: bool,
    pub selected_orientation: usize,
    show_tile_hit_box: bool,
    _marker: PhantomData<MARKER>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Paint,
    Attach,
    Erase,
}

impl<TYP: TileTypeAble, MARKER: MarkerAble> Default for TileMapEditor<TYP, MARKER> {
    fn default() -> Self {
        Self {
            mode: EditorMode::Paint,
            selected_tile_type: TYP::default(),
            random_orientation: true,
            selected_orientation: 0,
            show_tile_hit_box: false,
            _marker: PhantomData,
        }
    }
}
impl<TYP: TileTypeAble, MARKER: MarkerAble> TileMapEditor<TYP, MARKER> {
    fn user_input_system(
        buttons: Res<ButtonInput<MouseButton>>,
        windows: Query<&Window>,
        camera: Query<(&Camera, &GlobalTransform)>,
        hit_boxes: Query<(&TileHitBox<MARKER>, Entity)>,
        mut ray_cast: MeshRayCast,
        editor: Res<TileMapEditor<TYP, MARKER>>,
        tile_map: Res<TileMap<MARKER>>,
        mut commands: Commands,
    ) {
        if !buttons.just_pressed(MouseButton::Left) {
            return;
        }
        // Create Ray for ray_casting
        let window = windows.single().unwrap();
        let Some(curser_pos) = window.cursor_position() else {
            return;
        };
        let (camera, camera_transform) = camera.single().unwrap();
        let Ok(ray) = camera.viewport_to_world(camera_transform, curser_pos) else {
            return;
        };

        // Cast Ray and get tile entity
        let filter = |e| hit_boxes.get(e).is_ok();
        let settings = MeshRayCastSettings::default()
            .with_filter(&filter)
            .with_visibility(RayCastVisibility::Any);
        let Some((result, ray_hit)) = ray_cast.cast_ray(ray, &settings).first() else {
            return;
        };
        let tile_entity = hit_boxes.get(*result).unwrap().0.parent;
        let tile_pos = hit_boxes.get(*result).unwrap().0.map_pos;

        let typ = &editor.selected_tile_type;

        let orientation = match editor.random_orientation {
            true => random::<u8>() % (tile_map.config.possible_orientations.len() as u8),
            false => editor.selected_orientation as u8,
        };
        match editor.mode {
            EditorMode::Attach => {
                let map_pos = {
                    let (_, delta_pos) = tile_map
                        .config
                        .next_tile_dir
                        .iter()
                        .reduce(|a, b| {
                            match a.0.angle_between(ray_hit.normal)
                                < b.0.angle_between(ray_hit.normal)
                            {
                                true => a,
                                false => b,
                            }
                        })
                        .unwrap();
                    delta_pos + tile_pos
                };
                commands.spawn((Tile::<MARKER>::new(map_pos, orientation), typ.clone()));
            }
            EditorMode::Erase => {
                commands.entity(*result).despawn();
                commands.entity(tile_entity).despawn_children().despawn();
            }
            EditorMode::Paint => {
                let map_pos = tile_pos;
                commands.entity(tile_entity).remove::<(TYP, Tile<MARKER>)>();
                commands
                    .entity(tile_entity)
                    .insert((Tile::<MARKER>::new(map_pos, orientation), typ.clone()));
            }
        }
    }

    pub fn setup(mut commands: Commands) {
        commands.spawn((Tile::<MARKER>::new(IVec3::ZERO, 0), TYP::default()));
    }

    pub fn update(
        mut tile_hit_boxes: Query<&mut Visibility, With<TileHitBox<MARKER>>>,
        editor: Res<Self>,
    ) {
        tile_hit_boxes.iter_mut().for_each(|mut visibility| {
            *visibility = match editor.show_tile_hit_box {
                true => Visibility::Visible,
                false => Visibility::Hidden,
            };
        });
    }

    pub fn ui_system(
        mut contexts: EguiContexts,
        mut editor_state: ResMut<TileMapEditor<TYP, MARKER>>,
        tile_map: ResMut<TileMap<MARKER>>,
        mut commands: Commands,
    ) {
        let Ok(ctx) = contexts.ctx_mut() else {
            return;
        };

        egui::Window::new("Tile Map Editor")
            .default_pos([10.0, 10.0])
            .show(ctx, |ui| {
                ui.heading("Editor Mode");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut editor_state.mode, EditorMode::Paint, "🎨 Paint");
                    ui.selectable_value(&mut editor_state.mode, EditorMode::Attach, "🔗 Attach");
                    ui.selectable_value(&mut editor_state.mode, EditorMode::Erase, "🗑️ Erase");
                });

                ui.checkbox(&mut editor_state.show_tile_hit_box, "Show Tile Box");
                if ui.button("Clear Map").clicked() {
                    let event = ClearMapEvent::<TYP, MARKER>::new().with_source_tile();
                    commands.trigger(event);
                }

                ui.separator();
                ui.heading("Tile Type");
                editor_state.selected_tile_type.ui_mut(ui);
                ui.separator();

                ui.heading("Orientation");
                ui.checkbox(&mut editor_state.random_orientation, "Random Orientation");
                if editor_state.random_orientation == false {
                    ui.horizontal(|ui| {
                        ui.label("Orientation (degrees):");
                        ui.add(Slider::new(
                            &mut editor_state.selected_orientation,
                            0..=tile_map.config.possible_orientations.len() - 1,
                        ));
                    });
                }

                ui.separator();
                ui.heading("Tile Map Info");
                ui.label(format!("Tiles placed: {}", tile_map.tiles.len()));
                ui.label(format!("Tile size: {}", tile_map.config.tile_size));

                ui.separator();
                ui.heading("Save/Load");
                if ui.button("Save Map").clicked() {
                    commands.trigger(SaveMapEvent::<TYP, MARKER>::new());
                }

                if ui.button("Load Map").clicked() {
                    commands.trigger(LoadMapEvent::<TYP, MARKER>::new());
                }

                ui.separator();
                ui.label("Left Click: Paint | Right Click: Erase");
            });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Component)]
pub struct TileHitBox<MARKER: MarkerAble> {
    pub parent: Entity,
    pub map_pos: IVec3,
    _marker: PhantomData<MARKER>,
}
fn add_tile_map_hit_box<TYP: TileTypeAble, MARKER: MarkerAble>(
    event: On<Add, Tile<MARKER>>,
    tile_map: Res<TileMap<MARKER>>,
    tiles: Query<&Tile<MARKER>>,
    editor: Res<TileMapEditor<TYP, MARKER>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let entity = event.entity;
    let tile = tiles.get(entity).unwrap();

    let transform = Transform::from_translation(tile.map_pos.as_vec3() * tile_map.config.tile_size);
    let hit_box = TileHitBox::<MARKER> {
        map_pos: tile.map_pos.clone(),
        parent: entity,
        _marker: PhantomData,
    };
    let mesh = Mesh3d(meshes.add(Cuboid::from_size(tile_map.config.tile_size)));
    let material = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Color::linear_rgba(1.0, 0.0, 0.0, 0.9),
        alpha_mode: AlphaMode::Add,
        ..Default::default()
    }));

    let visibility = match editor.show_tile_hit_box {
        true => Visibility::Visible,
        false => Visibility::Hidden,
    };

    let _ = commands.spawn((hit_box, mesh, material, transform, visibility));
}
