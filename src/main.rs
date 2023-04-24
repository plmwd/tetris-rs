use bevy::{ecs::system::Command, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const BLOCK_SIZE: f32 = 20.;
const PLAYFIELD_WIDTH: i32 = 10;
const PLAYFIELD_HEIGHT: i32 = 20;

#[derive(Reflect, Component, Default)]
struct Collider;

#[derive(Reflect, Component, Default)]
struct Coordinates(IVec2);

impl Coordinates {
    fn new(x: i32, y: i32) -> Self {
        Coordinates(IVec2 { x, y })
    }
}

impl From<(i32, i32)> for Coordinates {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

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
                        x: BLOCK_SIZE * coords.0.x as f32,
                        y: BLOCK_SIZE * coords.0.y as f32,
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

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
enum TetrominoKind {
    #[default]
    I,
    J,
    L,
    O,
    T,
    S,
    Z,
}

#[derive(Bundle, Default)]
struct TetrominoBundle {
    transform_bundle: TransformBundle,
    kind: TetrominoKind,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    name: Name,
}

impl TetrominoBundle {
    fn new(kind: TetrominoKind, x: i32, y: i32) -> Self {
        Self {
            kind,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(x as f32 * BLOCK_SIZE, y as f32 * BLOCK_SIZE, 0.0),
                ..default()
            },
            name: Name::new("Tetromino"),
            ..default()
        }
    }
}

impl TetrominoKind {
    fn color(&self) -> Color {
        match self {
            TetrominoKind::I => Color::CYAN,
            TetrominoKind::J => Color::BLUE,
            TetrominoKind::L => Color::ORANGE,
            TetrominoKind::O => Color::YELLOW,
            TetrominoKind::T => Color::PURPLE,
            TetrominoKind::S => Color::GREEN,
            TetrominoKind::Z => Color::RED,
        }
    }
}

struct SpawnTetrominoCommand {
    kind: TetrominoKind,
    coords: Coordinates,
}

impl Command for SpawnTetrominoCommand {
    fn write(self, world: &mut World) {
        let x = self.coords.0.x;
        let y = self.coords.0.y;
        let color = self.kind.color();

        world
            .spawn(TetrominoBundle::new(self.kind.clone(), x, y))
            .with_children(|parent| {
                match self.kind {
                    TetrominoKind::I => {
                        parent.spawn(BlockBundle::new(Coordinates::new(0, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(2, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(3, 0), color));
                    }
                    TetrominoKind::J => {
                        parent.spawn(BlockBundle::new(Coordinates::new(0, 1), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(0, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(2, 0), color));
                    }
                    TetrominoKind::L => {
                        parent.spawn(BlockBundle::new(Coordinates::new(0, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(2, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(2, 1), color));
                    }
                    TetrominoKind::O => {
                        parent.spawn(BlockBundle::new(Coordinates::new(0, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(0, 1), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 1), color));
                    }
                    TetrominoKind::T => {
                        parent.spawn(BlockBundle::new(Coordinates::new(0, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(2, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 1), color));
                    }
                    TetrominoKind::S => {
                        parent.spawn(BlockBundle::new(Coordinates::new(0, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 1), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(2, 1), color));
                    }
                    TetrominoKind::Z => {
                        parent.spawn(BlockBundle::new(Coordinates::new(0, 1), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 1), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(1, 0), color));
                        parent.spawn(BlockBundle::new(Coordinates::new(2, 0), color));
                    }
                };
            });
    }
}

trait TetrisCommandsExt {
    fn spawn_tetromino(&mut self, kind: TetrominoKind, coords: Coordinates);
}

impl<'w, 's> TetrisCommandsExt for Commands<'w, 's> {
    fn spawn_tetromino(&mut self, kind: TetrominoKind, coords: Coordinates) {
        self.add(SpawnTetrominoCommand { kind, coords })
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .register_type::<TetrominoKind>()
        .register_type::<Coordinates>()
        .register_type::<Collider>()
        .run();
}

// Startup system to setup the scene and spawn all relevant entities.
fn setup(mut commands: Commands) {
    // Spawn a camera looking at the entities to show what's happening in this example.
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            PLAYFIELD_WIDTH as f32 * BLOCK_SIZE / 2.,
            PLAYFIELD_HEIGHT as f32 * BLOCK_SIZE / 2.,
            1000.,
        )),
        ..default()
    });

    commands
        .spawn((
            Name::new("Walls"),
            TransformBundle::default(),
            Visibility::default(),
            ComputedVisibility::default(),
        ))
        .with_children(|parent| {
            for y in -1..=PLAYFIELD_HEIGHT {
                parent.spawn(BlockBundle::new(Coordinates::new(-1, y), Color::PURPLE));
                parent.spawn(BlockBundle::new(
                    Coordinates::new(PLAYFIELD_WIDTH, y),
                    Color::PURPLE,
                ));
            }

            for x in 0..PLAYFIELD_WIDTH {
                parent.spawn(BlockBundle::new(Coordinates::new(x, -1), Color::PURPLE));
            }
        });

    commands.spawn_tetromino(TetrominoKind::I, (0, 0).into());
}
