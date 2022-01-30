use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(48, 44, 44)))
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Idle".to_string(),
            ..Default::default()
        })
        .add_startup_system(startup)
        .add_system(update)
        .run();
}

// Change these values
const SPEED: f32 = 100.0;
const AMOUNT: i32 = 50;
const MAX_DIST: f32 = 200.0;

#[derive(Component)]
struct Dots {
    pos: (f32, f32),
    direction: f32,
}

#[derive(Component)]
struct Lines;

// Backend variables (dont change)
const MAX_DIST_DIVIDED_BY_4: f32 = MAX_DIST / 4.0;

fn startup(mut commands: Commands, windows: ResMut<Windows>) {
    let win = windows.get_primary().expect("no primary window");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    for _ in 0..AMOUNT {
        let mut rng = rand::thread_rng();
        let dots = Dots {
            pos: (
                rng.gen::<f32>() * win.width() - (win.width() / 2.0),
                rng.gen::<f32>() * win.height() - (win.height() / 2.0),
            ),
            direction: rng.gen(),
        };
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(dots.pos.0, dots.pos.1, 0.0),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(dots);
    }
}

fn update(
    mut commands: Commands,
    mut query: Query<(&mut Dots, &mut Transform)>,
    lines: Query<Entity, With<Lines>>,
    windows: ResMut<Windows>,
    time: Res<Time>,
) {
    let win = windows.get_primary().expect("no primary window");
    for mut i in query.iter_mut() {
        if i.0.direction > 0.5 {
            i.0.pos.0 = i.0.pos.0
                + ((((i.0.direction % 0.5) - 0.25).abs() - 0.25) * SPEED * time.delta_seconds());
        } else {
            i.0.pos.0 = i.0.pos.0
                - ((((i.0.direction % 0.5) - 0.25).abs() - 0.25) * SPEED * time.delta_seconds());
        }
        if i.0.direction > 0.25 && i.0.direction < 0.75 {
            i.0.pos.1 =
                i.0.pos.1 - (((i.0.direction % 0.5) - 0.25).abs() * SPEED * time.delta_seconds())
        } else {
            i.0.pos.1 =
                i.0.pos.1 + (((i.0.direction % 0.5) - 0.25).abs() * SPEED * time.delta_seconds())
        }
        if i.0.pos.0.abs() > win.width() / 2.0 + MAX_DIST
            || i.0.pos.1.abs() > win.height() / 2.0 + MAX_DIST
        {
            let mut rng = rand::thread_rng();
            let num = (rng.gen::<f32>()
                * ((win.width() + MAX_DIST * 2.0) * 2.0 + (win.height() + MAX_DIST * 2.0) * 2.0)
                + 1.0)
                .floor();
            let mut x = 0.0;
            let mut y = 0.0;
            if num <= win.width() + MAX_DIST * 2.0 {
                y = win.height() / 2.0 + MAX_DIST;
                x = num - (win.width() + MAX_DIST);
            } else if num <= (win.width() + MAX_DIST * 2.0) * 2.0 {
                y = (win.height() / 2.0 + MAX_DIST) * -1.0;
                x = (num - win.width() + MAX_DIST * 2.0) - win.width() + MAX_DIST;
            } else if num <= (win.width() + MAX_DIST * 2.0) * 2.0 + win.height() + MAX_DIST * 2.0 {
                y = num - ((win.width() + MAX_DIST * 2.0) * 2.0);
                x = win.width() / 2.0 + MAX_DIST;
            } else if num
                <= (win.width() + MAX_DIST * 2.0) * 2.0 + (win.height() + MAX_DIST * 2.0) * 2.0
            {
                y = num
                    - ((win.width() + MAX_DIST * 2.0) * 2.0 + win.height() + MAX_DIST * 2.0)
                    - win.height()
                    + MAX_DIST;
                x = (win.width() / 2.0 + MAX_DIST) * -1.0;
            }
            i.0.pos = (x, y);
            i.0.direction = rng.gen();
        }
        i.1.translation.x = i.0.pos.0;
        i.1.translation.y = i.0.pos.1;
    }
    for i in lines.iter() {
        commands.entity(i).despawn();
    }
    for i in query.iter() {
        for x in query.iter() {
            let distance = (((i.0.pos.1 - x.0.pos.1).abs() * (i.0.pos.1 - x.0.pos.1).abs())
                + ((i.0.pos.0 - x.0.pos.0).abs() * (i.0.pos.0 - x.0.pos.0).abs()))
            .sqrt();
            if distance < MAX_DIST {
                let diff =
                    Vec3::new(i.0.pos.0, i.0.pos.1, 0.0) - Vec3::new(x.0.pos.0, x.0.pos.1, 0.0);
                let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: (Vec3::new(i.0.pos.0, i.0.pos.1, 0.0)
                                + Vec3::new(x.0.pos.0, x.0.pos.1, 0.0))
                                / 2.0,
                            scale: Vec3::new(distance, 1.0, 0.0),
                            rotation: Quat::from_rotation_z(angle),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgba(
                                1.0,
                                1.0,
                                1.0,
                                1.0 - ((distance / MAX_DIST_DIVIDED_BY_4).sqrt() / 2.0),
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Lines);
            }
        }
    }
}
