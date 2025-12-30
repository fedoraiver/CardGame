mod component;
mod game_play;
mod main_menu;
mod resources;
mod states;
mod systems;
mod util;
mod visual_effect;

use crate::game_play::*;
use crate::main_menu::*;
use crate::visual_effect::crt_post_processing::PostProcessPlugin;
use resources::*;
use states::*;
use systems::*;

use bevy::window::*;
use bevy::{log::*, prelude::*};

fn main() {
    let mut app = App::new();

    // TODO: cursor 图案
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                level: Level::INFO,
                ..Default::default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "PokerAutomata".into(),
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..default()
                }),
                ..default()
            }),
    );

    app.init_resource::<ZIndexManager>();
    app.init_state::<AppState>();

    app.add_plugins(MainMenuPlugin);
    app.add_plugins(GamePlayPlugin);
    app.add_plugins(PostProcessPlugin);

    app.add_systems(Startup, register_my_observers);
    app.add_systems(Startup, register_cards_aseprite_metadata);
    app.add_systems(Update, toggle_pause_state);
    app.add_systems(Update, quit_game);

    app.run();
}
