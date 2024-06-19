pub(crate) mod app_state;
pub(crate) mod asset_handles;
pub(crate) mod error;
pub(crate) mod events;
pub(crate) mod prelude;

use app_state::{AppState, PauseState};
use asset_handles::AssetHandles;
use bevy::{
    asset,
    input::{keyboard::KeyboardInput, ButtonState},
    log::LogPlugin,
    prelude::*,
    utils::petgraph::matrix_graph::Zero,
    window::WindowResolution,
};
use events::{GameLostEvent, GameResetEvent, GameWonEvent};

fn main() {
    let primary_window = Window {
        resolution: WindowResolution::new(850., 850.),
        title: "Mine Sweeper!".to_string(),
        ..Default::default()
    };
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(primary_window),
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
            })
            .set(LogPlugin {
                #[cfg(feature = "debug")]
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            }),
    );

    #[cfg(feature = "debug")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.insert_resource(minesweeper::options::BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        ..Default::default()
    })
    .insert_resource(AssetHandles::default())
    .add_plugins(minesweeper::MinesweeperPlugin::new(
        AppState::Running,
        PauseState::Paused,
        GameLostEvent,
        GameResetEvent,
        GameWonEvent,
    ))
    .init_state::<AppState>()
    .init_state::<PauseState>()
    .add_event::<GameLostEvent>()
    .add_event::<GameWonEvent>()
    .add_event::<GameResetEvent>()
    .add_systems(
        Startup,
        (spawn_camera, load_assets).run_if(in_state(AppState::Loading)),
    )
    .add_systems(
        Update,
        (
            is_loaded.run_if(in_state(AppState::Loading)),
            handle_input.run_if(in_state(AppState::Running)),
            game_over_handler,
        ),
    )
    .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}

pub fn handle_input(
    mut button_evr: EventReader<KeyboardInput>,
    mut evw: EventWriter<GameResetEvent>,
) {
    for event in button_evr.read() {
        if let ButtonState::Pressed = event.state {
            match event.key_code {
                KeyCode::KeyR => evw.send(GameResetEvent),
                _ => return,
            };
        };
    }
}

pub fn game_over_handler(
    mut evr_game_over: EventReader<GameLostEvent>,
    mut evw_game_reset_event: EventWriter<GameResetEvent>,
) {
    if evr_game_over.read().count().is_zero() {
        return;
    }
    evw_game_reset_event.send(GameResetEvent);
}

pub fn load_assets(asset_server: Res<AssetServer>, mut handles: ResMut<AssetHandles>) {
    handles.push(asset_server.load::<Image>("./minesweeper/0.png").untyped());
    handles.push(asset_server.load::<Image>("./minesweeper/1.png").untyped());
    handles.push(asset_server.load::<Image>("./minesweeper/2.png").untyped());
    handles.push(asset_server.load::<Image>("./minesweeper/3.png").untyped());
    handles.push(asset_server.load::<Image>("./minesweeper/4.png").untyped());
    handles.push(asset_server.load::<Image>("./minesweeper/5.png").untyped());
    handles.push(asset_server.load::<Image>("./minesweeper/6.png").untyped());
    handles.push(asset_server.load::<Image>("./minesweeper/7.png").untyped());
    handles.push(asset_server.load::<Image>("./minesweeper/8.png").untyped());
    handles.push(
        asset_server
            .load::<Image>("./minesweeper/Base.png")
            .untyped(),
    );
    handles.push(
        asset_server
            .load::<Image>("./minesweeper/Bomb.png")
            .untyped(),
    );
    handles.push(
        asset_server
            .load::<Image>("./minesweeper/Flag.png")
            .untyped(),
    );
}

pub fn is_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    handles: Res<AssetHandles>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if handles
        .iter()
        .map(|h| asset_server.get_load_state(h))
        .all(|state| state == Some(asset::LoadState::Loaded))
    {
        commands.insert_resource(minesweeper::texture_handles::TextureHandles {
            tile_0: asset_server.load::<Image>("./minesweeper/0.png"),
            tile_1: asset_server.load::<Image>("./minesweeper/1.png"),
            tile_2: asset_server.load::<Image>("./minesweeper/2.png"),
            tile_3: asset_server.load::<Image>("./minesweeper/3.png"),
            tile_4: asset_server.load::<Image>("./minesweeper/4.png"),
            tile_5: asset_server.load::<Image>("./minesweeper/5.png"),
            tile_6: asset_server.load::<Image>("./minesweeper/6.png"),
            tile_7: asset_server.load::<Image>("./minesweeper/7.png"),
            tile_8: asset_server.load::<Image>("./minesweeper/8.png"),
            tile_base: asset_server.load::<Image>("./minesweeper/Base.png"),
            tile_bomb: asset_server.load::<Image>("./minesweeper/Bomb.png"),
            tile_flag: asset_server.load::<Image>("./minesweeper/Flag.png"),
        });
        next_state.set(AppState::Running)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generic_error() {
        use crate::prelude::*;
        println!("{}", Error::from("Error!"));
    }

    #[test]
    fn test_other_error() {
        use crate::error::ToCrateError;

        println!("{}", std::fmt::Error {}.to_crate_error())
    }
}
