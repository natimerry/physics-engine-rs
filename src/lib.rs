pub mod components;
pub mod entity;

use bevy::{prelude::*, window::PrimaryWindow};
use components::*;
#[derive(Debug, Resource)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.81))
    }
}

fn collect_collision_pairs() {}

fn integrate(
    time: Res<Time>,
    mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel, &mut PreSolveVel, &Mass)>,
    gravity: Res<Gravity>,
) {
    if time.delta_seconds() == 0. {
        return;
    }
    for (mut pos, mut prev_pos, mut vel, mut presolve_vel, mass) in query.iter_mut() {
        prev_pos.0 = pos.0;

        let gravitation_force = mass.0 * gravity.0;
        let external_forces = gravitation_force;
        let d_time = time.delta_seconds();
        vel.0 += d_time * external_forces / mass.0; // a  = F/m
        pos.0 += d_time * vel.0;
        presolve_vel.0 = vel.0;
    }
}
fn solve_pos(
    mut query: Query<(Entity, &mut Pos, &Mass, &CircleCollider)>,
    mut contacts: ResMut<Contacts>,
) {
    let mut iter = query.iter_combinations_mut();
    contacts.0.clear();

    while let Some(
        [(entity_a, mut pos_a, mass_a, circle_a), (entity_b, mut pos_b, mass_b, circle_b)],
    ) = iter.fetch_next()
    {
        let ab = pos_b.0 - pos_a.0;
        let combined_radius = circle_a.radius + circle_b.radius;
        if ab.length_squared() < circle_a.radius * circle_b.radius {
            // todo!("Move circles to valid positions")
            return;
        }
        let ab_sqr_len = ab.length_squared();
        if ab_sqr_len < combined_radius * combined_radius {
            // we detect a collision
            contacts.0.push((entity_a, entity_b));
            let ab_length = ab_sqr_len.sqrt();
            let penetration_depth = combined_radius - ab_length;
            let n = ab / ab_length;

            let w_a = 1. / mass_a.0;
            let w_b = 1. / mass_b.0;
            let w_sum = w_a + w_b;

            pos_a.0 -= n * penetration_depth * w_a / w_sum;
            pos_b.0 += n * penetration_depth * w_b / w_sum;
        }
    }
}

fn update_vel(time: Res<Time>, mut query: Query<(&Pos, &PrevPos, &mut Vel)>) {
    if time.delta_seconds() == 0. {
        return;
    }

    for (pos, prev_pos, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - prev_pos.0) / time.delta_seconds();
    }
}
fn solve_vel(
    query: Query<(&mut Vel, &PreSolveVel, &Pos, &Mass, &Restitution)>,
    contacts: Res<Contacts>,
) {
    for (entity_a, entity_b) in contacts.0.iter().cloned() {
        let (
            (mut vel_a, pre_solve_vel_a, pos_a, mass_a, restitute_a),
            (mut vel_b, pre_solve_vel_b, pos_b, mass_b, restitute_b),
        ) = unsafe {
            assert!(entity_a != entity_b);
            (
                query.get_unchecked(entity_a).unwrap(),
                query.get_unchecked(entity_b).unwrap(),
            )
        };
        //
        let n = (pos_a.0 - pos_b.0).normalize(); // get direction of vectors
        let pre_solv_relative_velocity = pre_solve_vel_a.0 - pre_solve_vel_b.0;
        let pre_solv_normal = Vec2::dot(pre_solv_relative_velocity, n);

        let rel_v = vel_a.0 - vel_b.0;
        let normal_vel = Vec2::dot(rel_v, n);

        let restitute = (restitute_a.0 + restitute_b.0) / 2.;

        let w_a = 1. / mass_a.0; // wi = 1/m
        let w_b = 1. / mass_b.0;
        let w_sum = w_a + w_b;

        vel_a.0 += n * (-normal_vel - restitute * pre_solv_normal) * w_a / w_sum;
        vel_b.0 -= n * (-normal_vel - restitute * pre_solv_normal) * w_b / w_sum;
    }
}

pub fn sync_transforms(
    mut query: Query<(&mut Transform, &Pos, &CircleCollider)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (mut transform, pos, collider) in &mut query {
        transform.translation = pos.0.extend(0.);
    }
}

#[derive(Debug, Default)]
pub struct XBPDPhysics;

impl Plugin for XBPDPhysics {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collect_collision_pairs)
            .add_systems(Update, integrate.after(collect_collision_pairs))
            .add_systems(Update, solve_pos.after(integrate))
            .add_systems(Update, update_vel.after(solve_pos))
            .add_systems(Update, solve_vel.after(update_vel))
            .add_systems(Update, sync_transforms.after(solve_vel))
            .init_resource::<Gravity>()
            .init_resource::<Contacts>();
    }
}
