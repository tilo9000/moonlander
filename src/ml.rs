use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;

const GRAVITY: f32 = 0.1;
const ENGINE_FACTOR: f32 = 0.02;
const PIXEL_PER_VELOCITY: f32 = 2.;
const MAX_ENGINE_LEVEL: u8 = 9;

pub struct MLPlugin;

impl Plugin for MLPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WindowDescriptor {
            title: "Moon Lander".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_lander.system()))
        .add_system(
            lander_input
                .system()
                .label(LanderMovement::Input)
                .before(LanderMovement::Movement),
        )
        .add_event::<GameOverEvent>()
        .add_system(
            status_text
                .system()
                .label(LanderMovement::Status)
                .after(LanderMovement::Movement),
        )
        .add_system(game_over.system().after(LanderMovement::Status))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(lander_movement.system().label(LanderMovement::Movement)),
        );
    }
}

struct Lander {
    engine: u8,
    velocity: f32,
}

impl Lander {
    fn get_status_text(&self) -> String {
        return format!("Engine: {}\nVelocity: {:.3}", self.engine, self.velocity).to_string();
    }
}

struct Materials {
    lander_material: Handle<ColorMaterial>,
}

struct GameOverEvent;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum LanderMovement {
    Input,
    Movement,
    Status,
    TouchDown,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        lander_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
            },
        ),
        transform: Transform::from_translation(Vec3::new(-100., 200., 0.)),
        ..Default::default()
    });
}

fn spawn_lander(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.lander_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            transform: Transform::from_translation(Vec3::new(0., 200., 0.)),
            ..Default::default()
        })
        .insert(Lander {
            engine: 0,
            velocity: 0.,
        });
}

fn lander_movement(
    mut lander_positions: Query<(&mut Lander, &mut Transform)>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    for (mut lander, mut transform) in lander_positions.iter_mut() {
        // increase lander speed (velocity)
        let acceleration: f32 = GRAVITY - lander.engine as f32 * ENGINE_FACTOR;
        lander.velocity += acceleration;
        // movement is speed times delta (in pixels)
        transform.translation.y -= lander.velocity * PIXEL_PER_VELOCITY;
        // when we hit the boundary, we reset the game
        if transform.translation.y < -250. {
            game_over_writer.send(GameOverEvent)
        }
    }
}

fn status_text(mut text: Query<&mut Text>, lander: Query<&Lander>) {
    // Get the lander for status display
    for l in lander.iter() {
        for mut t in text.iter_mut() {
            t.sections[0].value = l.get_status_text()
        }
    }
}

fn lander_input(keyboard_input: Res<Input<KeyCode>>, mut lander_positions: Query<&mut Lander>) {
    for mut lander in lander_positions.iter_mut() {
        // check keyboard input
        if keyboard_input.pressed(KeyCode::Up) {
            // increase engine level (0..9)
            if lander.engine < MAX_ENGINE_LEVEL {
                lander.engine += 1;
            }
        }
        if keyboard_input.pressed(KeyCode::Down) {
            // decrease engine level
            if lander.engine > 0 {
                lander.engine -= 1;
            }
        }
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    materials: Res<Materials>,
    lander: Query<Entity, With<Lander>>,
) {
    if reader.iter().next().is_some() {
        for ent in lander.iter() {
            commands.entity(ent).despawn();
        }
        spawn_lander(commands, materials);
    }
}
