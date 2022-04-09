use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(velocity_system)
        .add_system(acceleration_system)
        .add_system(electrostatics_system)
        //.add_system(friction_system)
        .run();
}

const COULOMB_CONSTANT: f32 = 8987551792.3;
const GRAVITATIONAL_CONSTANT: f32 = 6.674184e-11;
const FRICTION_COEFFICIENT: f32 = 0.1;

#[derive(Component)]
struct Mass(f32); // in kg

#[derive(Component)]
struct Charge(i32); // in coulombs

#[derive(Component)]
struct Velocity(Vec3); // in m/s

#[derive(Component)]
struct Acceleration(Vec3); // in m/s^2

fn setup(mut commands: Commands) {
    // add a camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let ball1_pos = Vec3::new(-40.0, 30.0, 0.0);
    let ball2_pos = Vec3::new(40.0, -30.0, 0.0);
    let ball3_pos = Vec3::new(-70.0, 70.0, 0.0);

    // particle 1
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: ball1_pos,
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Mass(1.0))
        .insert(Charge(1))
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(Acceleration(Vec3::new(0.0, 0.0, 0.0)));

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: ball2_pos,
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 1.0, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Mass(1.0))
        .insert(Charge(1))
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(Acceleration(Vec3::new(0.0, 0.0, 0.0)));

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: ball3_pos,
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Mass(1.0))
        .insert(Charge(-1))
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(Acceleration(Vec3::new(0.0, 0.0, 0.0)));
}

fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in query.iter_mut() {
        transform.translation += vel.0;
    }
}

fn acceleration_system(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut vel, acc) in query.iter_mut() {
        vel.0 += acc.0;
    }
}

// not physically correct but I can change later
fn friction_system(mut query: Query<&mut Velocity>) {
    for mut vel in query.iter_mut() {
        vel.0 *= FRICTION_COEFFICIENT;
    }
}

fn electrostatics_system(mut query: Query<(&mut Acceleration, &Transform, &Mass, &Charge)>) {
    let mut iter = query.iter_combinations_mut();

    while let Some(
        [(mut acc1, transform1, Mass(m1), Charge(q1)), (mut acc2, transform2, Mass(m2), Charge(q2))],
    ) = iter.fetch_next()
    {
        if transform1.translation == transform2.translation {
            continue;
        }
        /*
        let r21 = transform2.translation - transform1.translation;

        let dir12 = r12.normalize();

        let c = 10.0 * (*q1 as f32) * (*q2 as f32);

        let r_squared = r12.length_squared();

        let f12 = c * dir12 / r_squared;
        let f21 = c * -dir12 / r_squared;

        acc1.0 += f21 / *m1;
        acc2.0 += f12 / *m2; */

        let r = transform2.translation - transform1.translation;

        let dir = r.normalize();

        let c = q1 * q2;

        let r2 = r.length_squared();

        let f12 = c as f32 * dir / r2;
        let f21 = c as f32 * -dir / r2;

        acc1.0 += f21 / *m1;
        acc2.0 += f12 / *m2;
    }
}
