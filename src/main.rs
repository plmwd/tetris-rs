use bevy::{ecs::system::Command, prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const BLOCK_SIZE: f32 = 20.;
const PLAYFIELD_WIDTH: u8 = 10;
const PLAYFIELD_HEIGHT: u8 = 20;

#[derive(Component, Default)]
struct Collider;

#[derive(Component, Default)]
struct Coordinates {
    x: i16,
    y: i16,
}

enum WallLocation {}

#[derive(Bundle, Default)]
struct BlockBundle {
    sprite_bundle: SpriteBundle,
    coords: Coordinates,
    collider: Collider,
}

impl BlockBundle {
    fn new(coords: Coordinates, color: Color) -> Self {
        BlockBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite { color, ..default() },
                transform: Transform {
                    scale: Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 0.),
                    translation: Vec3 {
                        x: BLOCK_SIZE * coords.x as f32,
                        y: BLOCK_SIZE * coords.y as f32,
                        z: 0.,
                    },
                    ..default()
                },
                ..default()
            },
            coords,
            ..default()
        }
    }
}

enum TetrominoKind {
    I,
    J,
    L,
    O,
    T,
    Y,
    Z,
}

struct SpawnTetrominoCommand {
    kind: TetrominoKind,
    coords: Coordinates,
}

impl Command for SpawnTetrominoCommand {
    fn write(self, world: &mut World) {
        match self.kind {
            TetrominoKind::I => todo!(),
            TetrominoKind::J => todo!(),
            TetrominoKind::L => todo!(),
            TetrominoKind::O => todo!(),
            TetrominoKind::T => todo!(),
            TetrominoKind::Y => todo!(),
            TetrominoKind::Z => todo!(),
        }
    }
}

trait TetrisCommandsExt {
    fn spawn_tetromino(&mut self, kind: TetrominoKind, coords: Coordinates);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .run();
}

// Startup system to setup the scene and spawn all relevant entities.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // Spawn a camera looking at the entities to show what's happening in this example.
    let window = window.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            PLAYFIELD_WIDTH as f32 * BLOCK_SIZE / 2.,
            PLAYFIELD_HEIGHT as f32 * BLOCK_SIZE / 2.,
            1000.,
        )),
        ..default()
    });

    for y in -1..=PLAYFIELD_HEIGHT as i16 {
        commands.spawn(BlockBundle::new(Coordinates { x: -1, y }, Color::PURPLE));
        commands.spawn(BlockBundle::new(
            Coordinates {
                x: PLAYFIELD_WIDTH as i16,
                y,
            },
            Color::PURPLE,
        ));
    }

    for x in 0..PLAYFIELD_WIDTH as i16 {
        commands.spawn(BlockBundle::new(Coordinates { x, y: -1 }, Color::PURPLE));
    }
}
