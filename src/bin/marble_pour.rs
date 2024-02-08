use bevy::{
    prelude::*, render::mesh::shape::Circle, sprite::MaterialMesh2dBundle, window::PrimaryWindow,
};
use physics_engine_rs::{
    components::{CircleCollider, Mass, Pos},
    entity::ParticleBundle,
    XBPDPhysics,
};
use rand::Rng;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn marble_pour(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity>,  
) {
    let radius = 20.;

    let mut batch: Vec<(MaterialMesh2dBundle<ColorMaterial>, ParticleBundle)> = Vec::new();
    for _i in 0..100 {
        let mut rng = rand::thread_rng();
        let pos = Vec2::new(
            rng.gen_range(-300..300) as f32,
            rng.gen_range(300..301) as f32,
        ) * 2.
            + Vec2::Y * 3.;
        let vel = Vec2::new(
            rng.gen_range(-50..50) as f32,
            rng.gen_range(-100..-50) as f32,
        );

        batch.push((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)).into(),
                transform: Transform::from_translation(Vec3::default()),
                ..default()
            },
            ParticleBundle {
                mass: Mass(1.),
                collider: CircleCollider { radius },
                ..ParticleBundle::new_with_pos_and_vel(pos, vel) 
            },
        ));
    }

    commands.spawn_batch(batch);

    // .insert(Mass(100.))
    // .insert(CircleCollider { radius: 50. });
}

fn despawn_marbles(
    mut commands: Commands,
    query: Query<(Entity, &Pos)>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = win_query.get_single().unwrap();
    for (entity, pos) in query.iter() {
        if pos.0.y < -0.5 * window.height() {
            commands.entity(entity).despawn();
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, marble_pour)
        .add_systems(Update, despawn_marbles)
        .add_plugins(XBPDPhysics::default())
        .run();
}
