use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use my_library::{Random, RandomNumberGenerator};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
    #[default]
    Player,
    Cpu,
}

#[derive(Resource)]
struct GameAssets {
    dice: Handle<TextureAtlas>,
}

#[derive(Clone, Copy, Resource)]
struct Scores {
    player: usize,
    cpu: usize,
}

#[derive(Component)]
struct HandDie;

#[derive(Resource)]
struct HandTimer(Timer);

fn setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    let atlas = TextureAtlas::from_grid(
        asset_server.load("dice.png"),
        Vec2::new(52.0, 52.0),
        6,
        1,
        None,
        None,
    );
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(GameAssets { dice: atlas_handle });
    commands.insert_resource(Scores { cpu: 0, player: 0 });
    commands.insert_resource(HandTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

fn display_score(scores: Res<Scores>, mut egui_context: EguiContexts) {
    egui::Window::new("Total Scores").show(egui_context.ctx_mut(), |ui| {
        ui.label(&format!("Player: {}", scores.player));
        ui.label(&format!("CPU: {}", scores.cpu));
    });
}

fn spawn_die(
    hand_query: &Query<(Entity, &TextureAtlasSprite, With<HandDie>)>,
    commands: &mut Commands,
    assets: &GameAssets,
    new_roll: usize,
    color: Color,
) {
    let rolled_die = hand_query.iter().count() as f32 * 52.0;
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: assets.dice.clone(),
            transform: Transform::from_xyz(rolled_die - 400.0, 60.0, 1.0),
            sprite: TextureAtlasSprite {
                index: new_roll - 1,
                color,
                ..default()
            },
            ..default()
        })
        .insert(HandDie);
}

fn clear_die(
    hand_query: &Query<(Entity, &TextureAtlasSprite, With<HandDie>)>,
    commands: &mut Commands,
) {
    hand_query
        .iter()
        .for_each(|(entity, _, _)| commands.entity(entity).despawn());
}

fn player(
    hand_query: Query<(Entity, &TextureAtlasSprite, With<HandDie>)>,
    mut commands: Commands,
    mut rng: ResMut<RandomNumberGenerator>,
    assets: Res<GameAssets>,
    mut scores: ResMut<Scores>,
    mut state: ResMut<NextState<GamePhase>>,
    mut egui_context: EguiContexts,
) {
    egui::Window::new("Play Options").show(egui_context.ctx_mut(), |ui| {
        let hand_score: usize = hand_query.iter().map(|(_, ts, _)| ts.index + 1).sum();
        ui.label(&format!("Score for this hand: {hand_score}"));

        if ui.button("Roll Dice").clicked() {
            let new_roll = rng.range(1..7);
            if new_roll == 1 {
                //End turn!
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Cpu);
            } else {
                spawn_die(&hand_query, &mut commands, &assets, new_roll, Color::WHITE);
            }
        }
        if ui.button("Pass - Keep Hand Score").clicked() {
            let hand_total: usize = hand_query.iter().map(|(_, ts, _)| ts.index + 1).sum();
            scores.player += hand_total;
            clear_die(&hand_query, &mut commands);
            state.set(GamePhase::Cpu);
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn cpu(
    hand_query: Query<(Entity, &TextureAtlasSprite, With<HandDie>)>,
    mut state: ResMut<NextState<GamePhase>>,
    mut scores: ResMut<Scores>,
    mut rng: ResMut<RandomNumberGenerator>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut timer: ResMut<HandTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let hand_total: usize = hand_query.iter().map(|(_, ts, _)| ts.index + 1).sum();
        if hand_total < 20 && scores.cpu + hand_total < 100 {
            let new_roll = rng.range(1..=6);
            if new_roll == 1 {
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Player);
            } else {
                spawn_die(&hand_query, &mut commands, &assets, new_roll, Color::BLUE);
            }
        } else {
            scores.cpu += hand_total;
            state.set(GamePhase::Player);
            hand_query
                .iter()
                .for_each(|(entity, _, _)| commands.entity(entity).despawn());
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(Random)
        .add_systems(Startup, setup)
        .add_state::<GamePhase>()
        .add_systems(Update, display_score)
        .add_systems(Update, player.run_if(in_state(GamePhase::Player)))
        .add_systems(Update, cpu.run_if(in_state(GamePhase::Cpu)))
        .run()
}
