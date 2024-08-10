//! Illustrates how to rotate an object around an axis.

use bevy::{
    asset::AssetMetaCheck, prelude::*
};

use std::f32::consts::TAU;

// use std::f32::consts::TAU;

// Define a component to designate a rotation speed to an entity.
// #[derive(Component)]
// struct Rotatable {
//     speed: f32,
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_scene_entities)
        .run();
}

#[derive(Component)]
struct MovedScene;

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn a cube to rotate.
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Cuboid::default()),
    //         material: materials.add(Color::WHITE),
    //         transform: Transform::from_translation(Vec3::ZERO),
    //         ..default()
    //     },
    //     Rotatable { speed: 0.3 },
    // ));

    // Spawn a camera looking at the entities to show what's happening in this example.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add a light source so we can see clearly.
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Spawn a second scene, and add a tag component to be able to target it later
    commands.spawn((
        SceneBundle {
            scene: asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("rat.glb")),
            ..default()
        },
        MovedScene,
    ));

}
// This system will rotate any entity in the scene with a Rotatable component around its y-axis.

// This system will move all entities that are descendants of MovedScene (which will be all entities spawned in the scene)
fn move_scene_entities(
    time: Res<Time>,
    moved_scene: Query<Entity, With<MovedScene>>,
    // children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
) {

    for entity in &moved_scene {
        if let Ok(mut transform) = transforms.get_mut(entity) {

            transform.rotate_y( 0.3 * TAU * time.delta_seconds());
        // here you set the Transform of only the parent, which will be propagated to children
        }
    }

}