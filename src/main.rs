use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Idle".to_string(),
            ..Default::default()
        })
        .add_startup_system(startup)
        .add_system(update)
        .run();
}

#[derive(Component)]
struct Dots {
    pos: (f32,f32),
    direction: f32,
    color: u8
}

const SPEED: f32 = 100.0;

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    let dots = Dots {pos: (0.0,0.0), direction: 0.10, color: 255};
    commands.spawn_bundle(
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
    }).insert(dots);
}

fn update(mut query: Query<(&mut Dots, &mut Transform)>, windows: ResMut<Windows>, time: Res<Time>) {
    let win = windows.get_primary().expect("no primary window");
    for mut i in query.iter_mut() {
        if i.0.direction > 0.5 {
            i.0.pos.0 = i.0.pos.0 + ((((i.0.direction%0.5)-0.25).abs()-0.25)*SPEED*time.delta_seconds());
        }
        else {
            i.0.pos.0 = i.0.pos.0 - ((((i.0.direction%0.5)-0.25).abs()-0.25)*SPEED*time.delta_seconds());
        }
        if i.0.direction > 0.25 && i.0.direction < 0.75 {
            i.0.pos.1 = i.0.pos.1 - (((i.0.direction%0.5)-0.25).abs()*SPEED*time.delta_seconds())
        }
        else {
            i.0.pos.1 = i.0.pos.1 + (((i.0.direction%0.5)-0.25).abs()*SPEED*time.delta_seconds())
        }
        i.1.translation.x = i.0.pos.0;
        i.1.translation.y = i.0.pos.1;
    }
}
