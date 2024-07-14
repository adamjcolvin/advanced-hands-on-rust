use bevy::prelude::*;

#[derive(Component)]
struct Dragon;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("dragon.png"),
            ..Default::default()
        })
        .insert(Dragon);
}

fn movement(keyboard: Res<Input<KeyCode>>, mut dragon_query: Query<&mut Transform, With<Dragon>>) {
    let delta = if keyboard.pressed(KeyCode::Left) {
        Vec2::new(-1.0, 0.0)
    } else if keyboard.pressed(KeyCode::Right) {
        Vec2::new(1.0, 0.0)
    } else if keyboard.pressed(KeyCode::Down) {
        Vec2::new(0.0, -1.0)
    } else if keyboard.pressed(KeyCode::Up) {
        Vec2::new(0.0, 1.0)
    } else {
        Vec2::ZERO
    };

    dragon_query.for_each_mut(|mut transform| {
        transform.translation += delta.extend(0.0);
    })
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}
