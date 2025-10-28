use bevy::prelude::*;

#[derive(Component)]
struct Object {
    size: f32,
    position: (f32, f32),
    movement: (f32, f32),
    mass: f32,
}

fn create_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Object {
            size: 6371., // km
            position: (0.0, 0.0),
            movement: (0.0, 0.0),
            mass: 6000., // * 10^21 kg
        },
        (
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),
        ),
    ));
    commands.spawn((
        Object {
            size: 1737. * 3.,               // km
            position: (380000., 0.),        // km
            movement: (0., 1. * 60. * 60.), // km per hour, one frame per hour
            mass: 73.5,                     // * 10^21 kg
        },
        (
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 1.))),
        ),
    ));
    commands.spawn((
        Object {
            size: 1737. * 3.,                // km
            position: (400000., 0.),         // km
            movement: (0., 0.5 * 60. * 60.), // km per hour, one frame per hour
            mass: 1.5,                       // * 10^21 kg
        },
        (
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(1., 0., 0.))),
        ),
    ));
}

fn draw(query: Query<(&Object, &mut Transform)>) {
    const SCALING_FACTOR: f32 = 1000.;
    for (blob, mut transform) in query {
        *transform = Transform::from_xyz(
            blob.position.0 / SCALING_FACTOR,
            blob.position.1 / SCALING_FACTOR,
            0.0,
        )
        .with_scale(Vec3::splat(blob.size / SCALING_FACTOR));
    }
}

fn update_movement_vectors(mut query: Query<&mut Object>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([mut object1, mut object2]) = iter.fetch_next() {
        let G = 66.7 * 60. * 60. * 60. * 60.;
        let v = (
            object1.position.0 - object2.position.0,
            object1.position.1 - object2.position.1,
        );
        let r = ops::sqrt(v.0 * v.0 + v.1 * v.1);
        let F = G * object1.mass * object2.mass / (r * r);

        object1.movement.0 -= v.0 * F / object1.mass / r;
        object1.movement.1 -= v.1 * F / object1.mass / r;
        object2.movement.0 += v.0 * F / object2.mass / r;
        object2.movement.1 += v.1 * F / object2.mass / r;
    }
}

fn move_objects(query: Query<&mut Object>) {
    for mut object in query {
        object.position.0 += object.movement.0;
        object.position.1 += object.movement.1;
    }
}

fn print_positions(query: Query<&mut Object>) {}

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, create_objects)
        .add_systems(FixedUpdate, (update_movement_vectors, move_objects, draw))
        .add_systems(FixedUpdate, (print_positions))
        .add_plugins(DefaultPlugins)
        .run();
}
