use bevy::{ecs::event::Events, prelude::*, window::WindowResized};
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Chemistry Simulation".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_system(window_resize)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(velocity_system)
        .add_system(acceleration_system)
        .add_system(electrostatics_system)
        .add_system(friction_system)
        .add_system(wrap_coordinate_system)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

const COULOMB_CONSTANT: f32 = 50.0;
const FRICTION_COEFFICIENT: f32 = 0.1;
const ELECTRON_MASS: f32 = 5.0;
const PROTON_MASS: f32 = 20.0;

#[derive(Component)]
struct Mass(f32); // in kg

#[derive(Component)]
struct Charge(i32); // in coulombs

#[derive(Component)]
struct Velocity(Vec3); // in m/s

#[derive(Component)]
struct Acceleration(Vec3); // in m/s^2

#[derive(Component)]
struct Radius(f32); // in m

fn setup(mut commands: Commands, window: Res<WindowDescriptor>) {
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
        radius: f32,
    ) {
        commands
            .spawn()
            .insert_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    radius,
                    ..Default::default()
                },
                DrawMode::Fill(FillMode::color(color)),
                Transform::from_translation(pos),
            ))
            .insert(Mass(mass))
            .insert(Charge(charge))
            .insert(Velocity(vel))
            .insert(Acceleration(acc))
            .insert(Radius(radius));
    }

    let mut rng = rand::thread_rng();

    println!("{}x{}", window.width, window.height);

    // for _ in 0..10 {
    //     let b = rng.gen_bool(0.5);
    //     add_particle(
    //         &mut commands,
    //         Vec3::new(
    //             rng.gen_range(-window.width / 2.0..=window.width / 2.0),
    //             rng.gen_range(-window.height / 2.0..=window.height / 2.0),
    //             0.0,
    //         ),
    //         Color::rgb(
    //             if b { 255.0 } else { 0.0 },
    //             0.0,
    //             if b { 0.0 } else { 255.0 },
    //         ),
    //         if b { 20.0 } else { 10.0 },
    //         if b { 1 } else { -1 },
    //         Vec3::new(0.0, 0.0, 0.0),
    //         Vec3::new(0.0, 0.0, 0.0),
    //         if b { 20.0 } else { 10.0 },
    //     )
    // }

    /*
    commands: &mut Commands,
    pos: Vec3,
    color: Color,
    mass: f32,
    charge: i32,
    vel: Vec3,
    acc: Vec3,
    radius: f32,
    */

    for i in -10..10 {
        for j in -10..10 {
            let b = (i + j) % 2 == 0;
            add_particle(
                &mut commands,
                Vec3::new((i * 30) as f32, (j * 30) as f32, 0.0),
                Color::rgb(
                    if b { 255.0 } else { 0.0 },
                    0.0,
                    if b { 0.0 } else { 255.0 },
                ),
                if b { 20.0 } else { 10.0 },
                if b { 1 } else { -1 },
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                if b { 20.0 } else { 10.0 },
            )
        }
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

fn friction_system(mut query: Query<&mut Velocity>) {
    for mut vel in query.iter_mut() {
        vel.0 *= FRICTION_COEFFICIENT;
    }
}

fn electrostatics_system(
    mut query: Query<(&mut Acceleration, &Transform, &Mass, &Charge, &Radius)>,
) {
    let mut iter = query.iter_combinations_mut();

    while let Some(
        [(mut acc1, transform1, Mass(m1), Charge(q1), Radius(r1)), (mut acc2, transform2, Mass(m2), Charge(q2), Radius(r2))],
    ) = iter.fetch_next()
    {
        let r12 = transform1.translation - transform2.translation;
        let r21 = transform2.translation - transform1.translation;

        if r12.length() < r1 + r2 {
            continue;
        }

        let r_hat_12 = r12.normalize();
        let r_hat_21 = r21.normalize();

        let f1 = COULOMB_CONSTANT * (q1 * q2) as f32 / r12.length_squared() * r_hat_12;
        let f2 = COULOMB_CONSTANT * (q1 * q2) as f32 / r21.length_squared() * r_hat_21;

        acc1.0 += f1 / *m1;
        acc2.0 += f2 / *m2;
    }
}

fn wrap_coordinate_system(
    mut query: Query<(&mut Transform, &Radius)>,
    window: Res<WindowDescriptor>,
) {
    for (mut transform, Radius(radius)) in query.iter_mut() {
        if transform.translation.x > window.width / 2.0 + radius {
            transform.translation.x = -window.width / 2.0 - radius;
        }

        if transform.translation.y > window.height / 2.0 + radius {
            transform.translation.y = -window.height / 2.0 - radius;
        }

        if transform.translation.x < -window.width / 2.0 - radius {
            transform.translation.x = window.width / 2.0 + radius;
        }

        if transform.translation.y < -window.height / 2.0 - radius {
            transform.translation.y = window.height / 2.0 + radius;
        }
    }
}

fn window_resize(resize_event: Res<Events<WindowResized>>, mut window: ResMut<WindowDescriptor>) {
    let mut event_reader = resize_event.get_reader();
    for event in event_reader.iter(&resize_event) {
        window.width = event.width.try_into().unwrap();
        window.height = event.height.try_into().unwrap();
    }
}
