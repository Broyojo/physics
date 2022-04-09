use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Physics".to_string(),
            width: 700.,
            height: 700.,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(velocity_system)
        .add_system(acceleration_system)
        .add_system(electrostatics_system)
        .add_system(friction_system)
        .add_system(wrap_coordinate_system)
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

    fn add_particle(
        commands: &mut Commands,
        pos: Vec3,
        color: Color,
        mass: f32,
        charge: i32,
        vel: Vec3,
        acc: Vec3,
    ) {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(30.0, 30.0, 0.0),
                    translation: pos,
                    ..Default::default()
                },
                sprite: Sprite {
                    color,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Mass(mass))
            .insert(Charge(charge))
            .insert(Velocity(vel))
            .insert(Acceleration(acc));
    }

    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        add_particle(
            &mut commands,
            Vec3::new(
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-100.0..100.0),
                0.0,
            ),
            Color::rgb(
                rng.gen_range(0.0..255.0),
                rng.gen_range(0.0..255.0),
                rng.gen_range(0.0..255.0),
            ),
            rng.gen_range(-100.0..100.0),
            rng.gen_range(1..100),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        );
    }
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

// not physically correct but I can change later. TODO: make this a drag force
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
        let r12 = transform1.translation - transform2.translation;
        let r21 = transform2.translation - transform1.translation;

        if r12.length() <= 30.0 || r21.length() <= 30.0 {
            continue;
        }

        let r_hat_12 = r12.normalize();
        let r_hat_21 = r21.normalize();

        let f1 = 5.0 * (q1 * q2) as f32 / r12.length_squared() * r_hat_12;
        let f2 = 5.0 * (q1 * q2) as f32 / r21.length_squared() * r_hat_21;

        acc1.0 += f1 / *m1;
        acc2.0 += f2 / *m2;
    }
}

fn wrap_coordinate_system(mut query: Query<&mut Transform>, screen: Res<WindowDescriptor>) {
    for mut transform in query.iter_mut() {
        if transform.translation.x > screen.width {
            transform.translation.x = -screen.width;
        }

        if transform.translation.y > screen.height {
            transform.translation.y = -screen.height;
        }

        if transform.translation.x < -screen.width {
            transform.translation.x = screen.width;
        }

        if transform.translation.y < -screen.height {
            transform.translation.y = screen.height;
        }
    }
}
