use bevy::prelude::*;

#[derive(Bundle)]
pub struct MainCameraBundle {
    camera3d: Camera3d,
    transform: Transform,
}

impl Default for MainCameraBundle {
    fn default() -> Self {
        MainCameraBundle {
            camera3d: Camera3d::default(),
            transform: Transform::from_xyz(0.0, 12.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        }
    }
}

impl MainCameraBundle {
    pub fn spawn(self, commands: &mut Commands) {
        commands.spawn(self);
    }
}
