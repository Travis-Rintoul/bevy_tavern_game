use crate::{
    actors::{CustomerActorBundle, PlayerBundle},
    cameras::MainCameraBundle,
    core::{Scenes, UIState},
    devices::StoveDeviceBundle,
};
use bevy::{prelude::*, state::state_scoped::clear_state_scoped_entities};

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Scenes::TestScene), setup)
            .add_systems(
                OnExit(Scenes::TestScene),
                clear_state_scoped_entities::<Scenes>,
            );
    }
}

pub fn setup(
    mut ui_state: ResMut<NextState<UIState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(MainCameraBundle::default())
        .insert(Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 0.0),
    ));

    let width = 150.0;
    let length = 150.0;

    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(width, 1.0, length)),
    ));

    let radius = 0.5;
    let height = 2.0;
    commands.spawn(PlayerBundle::default()).insert((
        Transform::from_xyz(0.0, 1.0, 0.0),
        Mesh3d(meshes.add(Cylinder::new(radius, height))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        })),
    ));

    commands
        .spawn(StoveDeviceBundle::default())
        .insert((
            Transform::from_xyz(-5.0, 1.0, -5.0),
            Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 0.6),
                ..default()
            })),
        ))
        .with_children(|parent| {});

    commands.spawn(CustomerActorBundle::default()).insert((
        Transform::from_xyz(5.0, 1.0, 5.0),
        Mesh3d(meshes.add(Cylinder::new(radius, height))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 1.0),
            ..default()
        })),
    ));

    ui_state.set(UIState::Exploration);
}
