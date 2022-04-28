#![feature(fn_traits)]

#[macro_use]
extern crate log;
use raylib::{RaylibHandle, RaylibThread};
use std::{fs::File, io::BufReader, path::Path};

mod entity;
mod module;
mod state;
use entity::*;
use module::*;
use state::*;

// core modules
mod core;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    // initialize raylib
    let (mut rl, thread) = raylib::init()
        // .fullscreen()
        .width(1400)
        .height(800)
        .title("Witch of Erebus")
        .msaa_4x()
        .vsync()
        .build();

    // initialize game state
    static mut MODULES: Vec<Module> = vec![];
    info!("Initializing [Core] modules.");
    unsafe {
        load_core_modules(&mut MODULES, &mut rl, &thread)?;
        let mut state = GlobalState::new(
            &MODULES,
            rl.get_screen_width() as usize,
            rl.get_screen_height() as usize,
        )?;

        // run game loop
        while !rl.window_should_close() {
            state.update();
            state.draw(&mut rl, &thread)?;
        }
    }

    Ok(())
}

fn load_core_modules(
    modules: &mut Vec<Module>,
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
) -> anyhow::Result<()> {
    modules.push(core::player_ships::module(rl, thread)?);

    Ok(())
}

/// Can be configured with the `WOE_ENTITY_DIR` environment variable.
struct LoadModulesReturn {
    pub entity_types: Vec<EntityType>,
    pub paths: Vec<Box<Path>>,
}
fn load_entity_types() -> anyhow::Result<LoadModulesReturn> {
    let dir = match std::env::var("WOE_ENTITY_DIR") {
        Err(_) => String::from("entities"),
        Ok(val) => val,
    };

    let mut entity_types = vec![];
    let mut paths = vec![];

    for entity_path_result in std::fs::read_dir(Path::new(&dir))? {
        let entity_path = entity_path_result?;

        let reader = BufReader::new(File::open(entity_path.path())?);
        let entity_type: EntityTypeJSON = serde_json::from_reader(reader)?;
    }

    Ok(LoadModulesReturn {
        entity_types,
        paths,
    })
}
