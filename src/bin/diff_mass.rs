use bevy::{prelude::*, render::mesh::shape::Circle, sprite::MaterialMesh2dBundle};
use physics_engine_rs::{
    components::{CircleCollider, Mass},
    entity::ParticleBundle,
    *,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(XBPDPhysics::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut texture: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: mesh.add(Circle::new(50.).into()).into(),
                material: texture.add(ColorMaterial::from(Color::RED)).into(),
                transform: Transform::from_translation(Vec3::default()),
                ..default()
            },
            ParticleBundle::new_with_pos_and_vel(Vec2::new(-100., 0.), Vec2::new(30., 0.)),
        ))
        .insert(Mass(100.))
        .insert(CircleCollider { radius: 50. });

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: mesh.add(Circle::new(70.).into()).into(),
                material: texture.add(ColorMaterial::from(Color::BLUE)).into(),
                transform: Transform::from_translation(Vec3::default()),
                ..default()
            },
            ParticleBundle::new_with_pos_and_vel(Vec2::new(100., 0.), Vec2::new(-70., 0.)),
        ))
        .insert(Mass(300.))
        .insert(CircleCollider { radius: 70. });
}
