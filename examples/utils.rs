use bevy::prelude::*;

use std::f32::consts::FRAC_PI_4;
pub(crate) struct VoxelVec;
impl VoxelVec {
    pub fn voxel(size: impl Into<IVec3>) -> Vec3 {
        let size = size.into();
        size.as_vec3()
    }
    pub fn y(height: i32) -> Vec3 {
        Self::voxel((0, height, 0))
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Transform::from_xyz(100.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, -FRAC_PI_4, -FRAC_PI_4, 0.0)),
        GlobalTransform::default(),
    ));
}
